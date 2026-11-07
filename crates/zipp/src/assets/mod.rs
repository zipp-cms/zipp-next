//! Assets: uploaded files and images. A file lives in
//! `<files>/<id>/<filename>`, its transforms in `<files>/<id>/t/` where
//! they are generated on the first request. Changing the focal point or
//! the edits (see `Edits`) deletes them.
//!
//! - `/assets/<id>/<filename>`: the image with its edits, `?original`
//!   the file as it was uploaded
//! - `/assets/<id>/<transform>/<filename>`: a named transform of the
//!   schema, `_thumb` is the one of the studio

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path as FsPath, PathBuf};

use image::ImageReader;
use rusqlite::{Connection, Row, params, params_from_iter};
use serde::{Deserialize, Serialize};

use crate::db::{marks, now};
use crate::error::{Error, Result};
use crate::schema::{Format, Transform};

pub(crate) mod api;
#[cfg(test)]
mod api_tests;
mod archive;
#[cfg(test)]
mod archive_tests;
mod edits;
mod files;
mod folders;
mod manage;
mod routes;
mod sort;
#[cfg(test)]
mod sort_tests;
#[cfg(test)]
pub(crate) mod tests;
mod transform;
mod upload;

pub(crate) use archive::{
	Limits, Unzipped, ZipOf, read_zip, unzip, write_zip, zip_files,
};
pub(crate) use edits::Edits;
#[cfg(test)]
use edits::{Canvas, Crop, Resize};
use folders::folder;
pub(crate) use folders::{
	Folder, check_folder, create_folder, delete_folder, folder_at,
	folder_named, folder_path, folder_paths, folders, folders_under,
	move_assets, top_count, update_folder,
};
pub(crate) use manage::{
	AssetChange, AssetList, InFolder, Kind, Usage, count, delete, find, list,
	replace, search, update, usage,
};

#[cfg(test)]
use files::file_headers;
pub(crate) use files::file_routes;
pub(crate) use routes::api_routes;
pub(crate) use sort::sort_assets;
pub use transform::transform_size;
use transform::*;
pub use upload::{UploadRules, upload_as};
use upload::{first_file, spool};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Asset {
	pub id: i64,
	pub filename: String,
	/// `image`, `video` or `file`
	pub kind: String,
	pub mime: String,
	pub size: i64,
	/// the size with the edits, what transforms start from
	pub width: Option<u32>,
	pub height: Option<u32>,
	/// the size of the uploaded file
	pub original_width: Option<u32>,
	pub original_height: Option<u32>,
	/// the image editor's, see `Edits`
	pub(crate) edits: Option<Edits>,
	/// the email of who uploaded it
	pub uploaded_by: Option<String>,
	/// changes with the file, the edits and the focal point, urls and the
	/// names of transforms carry it, so browsers don't keep showing the
	/// image from before; empty for an untouched upload
	pub version: String,
	/// the folder it is in, `None` on the top
	pub folder_id: Option<i64>,
	/// like Craft's asset title, empty for one from the filename
	pub title: String,
	/// per site handle, see [`Asset::alt_in`]
	pub alt: BTreeMap<String, String>,
	pub focal_x: f64,
	pub focal_y: f64,
	/// the values of the asset fields, see
	/// [`Schema::asset_fields`](crate::Schema::asset_fields)
	#[serde(default)]
	pub fields: serde_json::Value,
	pub created_at: i64,
}

impl Asset {
	/// The path of the image or of a transform.
	pub fn url(&self, transform: Option<&str>) -> String {
		self.versioned(match transform {
			Some(t) => format!("/assets/{}/{t}/{}", self.id, self.filename),
			None => format!("/assets/{}/{}", self.id, self.filename),
		})
	}

	/// The path of a transform in `format`, else in the transform's
	/// format, else in the type of the image.
	pub(crate) fn format_url(
		&self,
		t: &Transform,
		format: Option<Format>,
	) -> String {
		match format.or(t.format) {
			Some(f) => self.versioned(format!(
				"/assets/{}/{}/{}",
				self.id,
				t.handle,
				self.format_filename(f)
			)),
			None => self.url(Some(&t.handle)),
		}
	}

	/// The name of a transform in another type: the extension says it,
	/// `piano.jpg` in WebP is `piano.webp`.
	pub(crate) fn format_filename(&self, format: Format) -> String {
		let stem = match self.filename.rsplit_once('.') {
			Some((stem, _)) => stem,
			None => &self.filename,
		};
		format!("{stem}.{}", format.extension())
	}

	fn versioned(&self, path: String) -> String {
		match self.version.as_str() {
			"" => path,
			v => format!("{path}?v={v}"),
		}
	}

	/// The alt text of a site, else the one of the primary site.
	pub fn alt_in(&self, site: &str, primary: &str) -> &str {
		self.alt
			.get(site)
			.or_else(|| self.alt.get(primary))
			.map_or("", |a| a.as_str())
	}

	/// The size of the image after a transform.
	pub(crate) fn size_after(&self, t: &Transform) -> Option<(u32, u32)> {
		Some(transform_size(self.width?, self.height?, t))
	}

	/// The type of the images made of it: a transparent canvas needs a
	/// png when the original is a jpeg.
	pub(crate) fn output_mime(&self) -> &str {
		match &self.edits {
			Some(e) if e.transparent() && self.mime == "image/jpeg" => {
				"image/png"
			}
			_ => &self.mime,
		}
	}
}

const COLUMNS: &str = "id, filename, kind, mime, size, width, height, alt, \
	focal_x, focal_y, created_at, title, edits, \
	(SELECT email FROM users WHERE id = uploaded_by), folder_id, fields, \
	replaced";

fn from_row(r: &Row) -> rusqlite::Result<Asset> {
	let (width, height): (Option<u32>, Option<u32>) = (r.get(5)?, r.get(6)?);
	let edits: Option<Edits> = r
		.get::<_, Option<String>>(12)?
		.and_then(|e| serde_json::from_str(&e).ok());
	// the size the edits give, the transforms start from it
	let edited = match (&edits, width, height) {
		(Some(e), Some(w), Some(h)) => Some(e.size(w, h)),
		_ => width.zip(height),
	};
	let (focal_x, focal_y): (f64, f64) = (r.get(8)?, r.get(9)?);
	Ok(Asset {
		version: version(r.get(16)?, &edits, focal_x, focal_y),
		id: r.get(0)?,
		filename: r.get(1)?,
		kind: r.get(2)?,
		mime: r.get(3)?,
		size: r.get(4)?,
		width: edited.map(|s| s.0),
		height: edited.map(|s| s.1),
		original_width: width,
		original_height: height,
		edits,
		uploaded_by: r.get(13)?,
		folder_id: r.get(14)?,
		alt: serde_json::from_str(&r.get::<_, String>(7)?).unwrap_or_default(),
		focal_x,
		focal_y,
		created_at: r.get(10)?,
		title: r.get(11)?,
		fields: serde_json::from_str(&r.get::<_, String>(15)?)
			.unwrap_or_default(),
	})
}

/// A short hash of what changes the image, empty for an untouched one.
fn version(
	replaced: i64,
	edits: &Option<Edits>,
	focal_x: f64,
	focal_y: f64,
) -> String {
	use std::hash::{Hash, Hasher};
	if replaced == 0 && edits.is_none() && (focal_x, focal_y) == (0.5, 0.5) {
		return String::new();
	}
	let mut h = std::collections::hash_map::DefaultHasher::new();
	replaced.hash(&mut h);
	serde_json::to_string(edits)
		.unwrap_or_default()
		.hash(&mut h);
	focal_x.to_bits().hash(&mut h);
	focal_y.to_bits().hash(&mut h);
	format!("{:x}", h.finish() & 0xffff_ffff)
}

pub fn get_asset(conn: &Connection, id: i64) -> Result<Asset> {
	let sql = format!(
		"SELECT {COLUMNS} FROM assets WHERE id = ?1 AND deleted_at IS NULL"
	);
	Ok(conn.query_row(&sql, [id], from_row)?)
}

/// The assets in the trash, the last deleted first.
pub(crate) fn trashed(
	conn: &Connection,
) -> Result<Vec<crate::trash::TrashedAsset>> {
	let mut stmt = conn.prepare(&format!(
		"SELECT {COLUMNS}, deleted_at,
			(SELECT email FROM users WHERE id = deleted_by)
		FROM assets WHERE deleted_at IS NOT NULL
		ORDER BY deleted_at DESC, id DESC"
	))?;
	let rows = stmt.query_map([], |r| {
		Ok(crate::trash::TrashedAsset {
			asset: from_row(r)?,
			deleted_at: r.get(17)?,
			deleted_by: r.get(18)?,
		})
	})?;
	Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// The assets of these ids in the same order, missing ones are left out.
pub(crate) fn get_many(conn: &Connection, ids: &[i64]) -> Result<Vec<Asset>> {
	let sql = format!(
		"SELECT {COLUMNS} FROM assets WHERE id IN ({}) AND deleted_at IS NULL",
		marks(ids.len())
	);
	let mut stmt = conn.prepare(&sql)?;
	let mut found = HashMap::new();
	for a in stmt.query_map(params_from_iter(ids), from_row)? {
		let a = a?;
		found.insert(a.id, a);
	}
	// in the order of the ids
	Ok(ids.iter().filter_map(|id| found.get(id).cloned()).collect())
}

/// The folder of an asset: its file and [`TRANSFORMS`].
pub(crate) fn asset_dir(files: &FsPath, id: i64) -> std::path::PathBuf {
	files.join(id.to_string())
}

/// The folder of an asset's transforms, made again when they change. It
/// starts with a dot, which [`clean_filename`] never leaves, so no file
/// can take its name.
pub(crate) const TRANSFORMS: &str = ".t";

/// Only letters, digits, dots, dashes and underscores, so the name is a
/// safe path and url.
pub(crate) fn clean_filename(name: &str) -> String {
	let name: String = name
		.rsplit(['/', '\\'])
		.next()
		.unwrap_or_default()
		.chars()
		.map(|c| match c {
			'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '_' => c,
			_ => '-',
		})
		.collect();
	match name.trim_matches('.') {
		"" => "file".into(),
		n => n.into(),
	}
}

/// The type of a file that is no image, by its name: never what the
/// upload claims.
fn file_mime(filename: &str) -> &'static str {
	let ext = filename
		.rsplit_once('.')
		.map(|(_, e)| e.to_ascii_lowercase());
	match ext.as_deref() {
		Some("pdf") => "application/pdf",
		Some("mp4") => "video/mp4",
		Some("webm") => "video/webm",
		Some("mov") => "video/quicktime",
		Some("mp3") => "audio/mpeg",
		Some("svg") => "image/svg+xml",
		Some("zip") => "application/zip",
		Some("txt") => "text/plain",
		Some("csv") => "text/csv",
		Some("docx") => {
			"application/vnd.openxmlformats-officedocument.wordprocessingml.document"
		}
		Some("xlsx") => {
			"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
		}
		_ => "application/octet-stream",
	}
}

/// The most bytes of an upload, of a file or a zip.
pub(crate) const MAX_UPLOAD: usize = 200 * 1024 * 1024;

/// Images above this many pixels are refused, decoding them for a
/// transform would take too much memory.
const MAX_PIXELS: u64 = 40_000_000;

/// The longest side of an image, longer ones are refused like big ones.
const MAX_IMAGE_SIDE: u32 = 65_535;

/// What decoding an image may take: the biggest image an upload may have,
/// in 16 bit colors.
fn decode_limits() -> image::Limits {
	let mut limits = image::Limits::default();
	limits.max_image_width = Some(MAX_IMAGE_SIDE);
	limits.max_image_height = Some(MAX_IMAGE_SIDE);
	limits.max_alloc = Some(MAX_PIXELS * 8 + 64 * 1024 * 1024);
	limits
}

/// What an upload is: its clean name, kind, type and size.
struct Upload {
	filename: String,
	kind: &'static str,
	mime: String,
	width: Option<u32>,
	height: Option<u32>,
}

/// An upload by its content, only the header of an image is read.
fn inspect(filename: &str, file: &FsPath) -> Result<Upload> {
	let filename = clean_filename(filename);
	let image = fs::File::open(file)
		.map(|f| ImageReader::new(std::io::BufReader::new(f)))
		.and_then(|r| r.with_guessed_format())
		.ok()
		.and_then(|r| {
			let f = r.format()?;
			let (w, h) = r.into_dimensions().ok()?;
			Some((f, w, h))
		});
	let (kind, mime, width, height) = match image {
		Some((f, w, h)) => {
			("image", f.to_mime_type().to_string(), Some(w), Some(h))
		}
		None => {
			let mime = file_mime(&filename);
			// Craft's kinds, a video plays where an image shows
			let kind = match mime.starts_with("video/") {
				true => "video",
				false => "file",
			};
			(kind, mime.to_string(), None, None)
		}
	};
	if let (Some(w), Some(h)) = (width, height)
		&& (u64::from(w) * u64::from(h) > MAX_PIXELS
			|| w.max(h) > MAX_IMAGE_SIDE)
	{
		return Err(Error::Request(format!(
			"{filename}: the image is too big ({w}×{h}), at most 40 megapixels"
		)));
	}
	Ok(Upload {
		filename,
		kind,
		mime,
		width,
		height,
	})
}

/// A file in the making in `files`, like an upload while it comes in:
/// hidden, so backups leave it out, and gone unless it is stored.
pub(crate) fn temp_file(files: &FsPath) -> Result<tempfile::NamedTempFile> {
	use std::os::unix::fs::PermissionsExt;
	fs::create_dir_all(files)?;
	// readable like any other file once it is stored
	let file = tempfile::Builder::new()
		.permissions(fs::Permissions::from_mode(0o644))
		.tempfile_in(files)?;
	Ok(file)
}

/// Stores an upload, see [`store_file`].
#[cfg(any(test, feature = "craft"))]
pub(crate) fn store(
	conn: &Connection,
	files: &FsPath,
	filename: &str,
	data: &[u8],
	user: Option<i64>,
) -> Result<Asset> {
	let mut tmp = temp_file(files)?;
	std::io::Write::write_all(&mut tmp, data)?;
	store_file(conn, files, filename, tmp, user)
}

/// Stores an upload that came in as `tmp`: the row first for the id, then
/// the file moves in. `user` is who uploads, none for imports.
pub(crate) fn store_file(
	conn: &Connection,
	files: &FsPath,
	filename: &str,
	tmp: tempfile::NamedTempFile,
	user: Option<i64>,
) -> Result<Asset> {
	let Upload {
		filename,
		kind,
		mime,
		width,
		height,
	} = inspect(filename, tmp.path())?;
	let size = tmp.as_file().metadata()?.len();
	conn.execute(
		"INSERT INTO assets (filename, kind, mime, size, width, height,
		created_at, uploaded_by) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
		params![filename, kind, mime, size, width, height, now(), user],
	)?;
	let id = conn.last_insert_rowid();

	let dir = asset_dir(files, id);
	let written = fs::create_dir_all(&dir)
		.map_err(|e| e.to_string())
		.and_then(|_| {
			tmp.persist(dir.join(&filename)).map_err(|e| e.to_string())
		});
	if let Err(e) = written {
		conn.execute("DELETE FROM assets WHERE id = ?1", [id])?;
		return Err(Error::Internal(e));
	}

	get_asset(conn, id)
}

/// Where uploads go, `--files`.
pub(crate) fn files_dir(path: PathBuf) -> std::io::Result<PathBuf> {
	fs::create_dir_all(&path)?;
	Ok(path)
}

/// Fills in where assets are used for content from before zipp kept it
/// (`asset_relations`): once, when that is empty and there is content.
pub(crate) fn index_usage(
	conn: &Connection,
	schema: &crate::schema::Schema,
) -> Result<()> {
	let known: i64 =
		conn.query_row("SELECT COUNT(*) FROM asset_relations", [], |r| {
			r.get(0)
		})?;
	if known > 0 {
		return Ok(());
	}
	crate::entries::reindex_all(conn, schema)
}
