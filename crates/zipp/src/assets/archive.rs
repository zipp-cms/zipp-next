//! Assets as zips: several or a folder with its subfolders downloaded as
//! one, and an uploaded zip unpacked into folders. The folders are only
//! names in the database, a path of a zip never becomes a path on disk.

use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek, Write};

use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

use super::*;

/// What a zip holds.
#[derive(Debug, Clone, PartialEq)]
pub enum ZipOf {
	/// these assets, side by side
	Assets(Vec<i64>),
	/// a folder: its assets on the top, its subfolders as folders
	Folder(i64),
}

/// The assets of a zip with their paths in it, the ones in the trash
/// left out. A name that is taken in its folder gets `-2`, `-3`: zips
/// are opened where case doesn't count, so names compare without it.
pub fn zip_files(
	conn: &Connection,
	of: &ZipOf,
) -> Result<Vec<(String, Asset)>> {
	let mut taken = HashSet::new();
	let assets = match of {
		ZipOf::Assets(ids) => get_many(conn, ids)?
			.into_iter()
			.map(|a| (String::new(), a))
			.collect(),
		ZipOf::Folder(id) => {
			check_folder(conn, Some(*id))?;
			// the folders below it with their paths, parents first
			let all = folders(conn)?;
			let mut dirs = vec![(*id, String::new())];
			let mut i = 0;
			while let Some((parent, path)) = dirs.get(i).cloned() {
				for f in all.iter().filter(|f| f.parent_id == Some(parent)) {
					let here = format!("{path}{}", f.name);
					taken.insert(here.to_lowercase());
					dirs.push((f.id, here + "/"));
				}
				i += 1;
			}

			let mut stmt = conn.prepare(&format!(
				"SELECT {COLUMNS} FROM assets
				WHERE folder_id = ?1 AND deleted_at IS NULL ORDER BY id"
			))?;
			let mut out = vec![];
			for (folder, path) in dirs {
				for a in stmt.query_map([folder], from_row)? {
					out.push((path.clone(), a?));
				}
			}
			out
		}
	};
	Ok(assets
		.into_iter()
		.map(|(dir, a)| (unique(&mut taken, &dir, &a.filename), a))
		.collect())
}

/// `dir` and `name`, with `-2`, `-3` before the extension while taken.
fn unique(taken: &mut HashSet<String>, dir: &str, name: &str) -> String {
	let (stem, ext) = match name.rsplit_once('.') {
		Some((s, e)) if !s.is_empty() => (s, format!(".{e}")),
		_ => (name, String::new()),
	};
	let mut path = format!("{dir}{name}");
	let mut n = 1;
	while !taken.insert(path.to_lowercase()) {
		n += 1;
		path = format!("{dir}{stem}-{n}{ext}");
	}
	path
}

/// Writes the files of [`zip_files`] as they were uploaded into a zip.
/// Images, video, audio and zips are compressed already, they are only
/// stored. A file missing on disk is left out.
pub fn write_zip<W: Write + Seek>(
	files: &FsPath,
	entries: &[(String, Asset)],
	out: W,
) -> Result<W> {
	let mut zip = ZipWriter::new(out);
	for (path, a) in entries {
		let packed = a.kind == "image"
			|| ["video/", "audio/", "application/zip"]
				.iter()
				.any(|m| a.mime.starts_with(m));
		let options = SimpleFileOptions::default()
			.compression_method(match packed {
				true => CompressionMethod::Stored,
				false => CompressionMethod::Deflated,
			})
			.large_file(a.size >= i64::from(u32::MAX));
		let path_on_disk = asset_dir(files, a.id).join(&a.filename);
		let mut file = match fs::File::open(&path_on_disk) {
			Ok(f) => f,
			Err(e) => {
				tracing::warn!("{}: {e}", path_on_disk.display());
				continue;
			}
		};
		zip.start_file(path.as_str(), options).map_err(written)?;
		std::io::copy(&mut file, &mut zip)?;
	}
	zip.finish().map_err(written)
}

fn written(e: zip::result::ZipError) -> Error {
	Error::Internal(format!("the zip: {e}"))
}

fn broken(e: impl std::fmt::Display) -> Error {
	Error::Request(format!("the zip: {e}"))
}

/// What an unpacked zip may hold at most: a zip bomb is refused before
/// a file is stored.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
	pub files: usize,
	/// bytes of all files unpacked
	pub size: u64,
	/// bytes of one file unpacked, what an upload may be
	pub file: u64,
}

impl Default for Limits {
	fn default() -> Self {
		Self {
			files: 2000,
			size: 1024 * 1024 * 1024,
			file: MAX_UPLOAD as u64,
		}
	}
}

/// What an unpacked zip gave.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unzipped {
	pub assets: Vec<Asset>,
	/// files that were left out and why, like an image too big
	pub skipped: Vec<String>,
}

/// A file or folder of a zip to unpack.
struct Item {
	index: usize,
	/// the folders it is in, of the zip
	dirs: Vec<String>,
	/// `None` for a folder
	name: Option<String>,
}

/// The files of a zip as assets into `folder`, its folders as folders
/// in it (merged with the ones of the same name). Hidden files, macOS'
/// `__MACOSX` and links are left out; paths out of the zip, encrypted
/// files and more than `limits` refuse the whole zip. All or nothing,
/// only a file that is no fitting asset is skipped.
pub fn unzip(
	conn: &Connection,
	files: &FsPath,
	data: &[u8],
	folder: Option<i64>,
	user: Option<i64>,
	limits: &Limits,
) -> Result<Unzipped> {
	check_folder(conn, folder)?;
	let mut archive = ZipArchive::new(Cursor::new(data)).map_err(broken)?;
	let items = plan(&mut archive, limits)?;

	// the rows go with the transaction, the files written so far by hand
	let tx = conn.unchecked_transaction()?;
	let mut stored = vec![];
	let unpacked =
		unpack(&tx, files, &mut archive, &items, folder, user, &mut stored)
			.and_then(|u| Ok(tx.commit().map(|_| u)?));
	if unpacked.is_err() {
		for id in stored {
			let _ = fs::remove_dir_all(asset_dir(files, id));
		}
	}
	unpacked
}

/// What of the zip is unpacked, checked against the limits by the sizes
/// it says.
fn plan<R: Read + Seek>(
	archive: &mut ZipArchive<R>,
	limits: &Limits,
) -> Result<Vec<Item>> {
	let mut items = vec![];
	let (mut count, mut size) = (0, 0u64);
	for index in 0..archive.len() {
		let f = archive.by_index_raw(index).map_err(broken)?;
		let path = f.name().map_err(broken)?.replace('\\', "/");
		let parts: Vec<&str> = path
			.split('/')
			.filter(|p| !p.trim().is_empty() && *p != ".")
			.collect();
		if path.starts_with('/') || parts.contains(&"..") {
			return Err(Error::Request(format!(
				"the zip: {path} is a path out of it"
			)));
		}
		// what macOS adds, hidden files and folders
		let hidden = parts.first() == Some(&"__MACOSX")
			|| parts.iter().any(|p| p.starts_with('.'));
		if hidden || parts.is_empty() || f.is_symlink() {
			continue;
		}
		if f.encrypted() {
			return Err(Error::Request(format!(
				"the zip: {path} is encrypted"
			)));
		}

		let dirs = parts.iter().map(|p| p.trim().to_string());
		if f.is_dir() {
			items.push(Item {
				index,
				dirs: dirs.collect(),
				name: None,
			});
			continue;
		}
		if f.size() > limits.file {
			return Err(Error::Request(format!(
				"the zip: {path} is bigger than {} MB",
				limits.file / 1024 / 1024
			)));
		}
		count += 1;
		size += f.size();
		let mut dirs: Vec<String> = dirs.collect();
		let name = dirs.pop();
		items.push(Item { index, dirs, name });
	}
	if count > limits.files {
		return Err(Error::Request(format!(
			"the zip has more than {} files",
			limits.files
		)));
	}
	if size > limits.size {
		return Err(Error::Request(format!(
			"the zip is bigger than {} MB unpacked",
			limits.size / 1024 / 1024
		)));
	}
	Ok(items)
}

fn unpack<R: Read + Seek>(
	conn: &Connection,
	files: &FsPath,
	archive: &mut ZipArchive<R>,
	items: &[Item],
	folder: Option<i64>,
	user: Option<i64>,
	stored: &mut Vec<i64>,
) -> Result<Unzipped> {
	// the folders by their path in the zip, in lowercase
	let mut ids: HashMap<String, Option<i64>> =
		HashMap::from([(String::new(), folder)]);
	let mut skipped = vec![];
	for item in items {
		let mut into = folder;
		let mut path = String::new();
		for name in &item.dirs {
			path = format!("{path}/{}", name.to_lowercase());
			into = match ids.get(&path) {
				Some(id) => *id,
				None => Some(folder_named(conn, into, name)?),
			};
			ids.insert(path.clone(), into);
		}
		let Some(name) = &item.name else { continue };

		// zip refuses a file that holds more than it says, the limits
		// of the plan hold
		let mut data = vec![];
		let mut f = archive.by_index(item.index).map_err(broken)?;
		f.read_to_end(&mut data).map_err(broken)?;

		match store(conn, files, name, &data, user) {
			Ok(a) => {
				stored.push(a.id);
				move_assets(conn, &[a.id], into)?;
			}
			Err(Error::Request(why)) => skipped.push(why),
			Err(e) => return Err(e),
		}
	}
	Ok(Unzipped {
		assets: get_many(conn, stored)?,
		skipped,
	})
}
