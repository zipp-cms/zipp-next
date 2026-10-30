//! What editors do with assets: find, change, replace, delete them and
//! see where they are used.

use super::*;

/// The newest assets whose filename contains `search`, of a kind.
/// Where to look for assets: all folders, or one (`None` the top).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InFolder {
	All,
	Folder(Option<i64>),
}

/// With `extensions` only files that end so (lowercase, without dot).
pub fn search(
	conn: &Connection,
	search: &str,
	kind: Option<&str>,
	folder: InFolder,
	extensions: &[String],
) -> Result<Vec<Asset>> {
	let (any, folder) = match folder {
		InFolder::All => (true, None),
		InFolder::Folder(f) => (false, f),
	};
	// one LIKE per extension, after the four params
	let ends = if extensions.is_empty() {
		String::new()
	} else {
		let likes: Vec<String> = (0..extensions.len())
			.map(|i| format!("lower(filename) LIKE ?{}", i + 5))
			.collect();
		format!("AND ({})", likes.join(" OR "))
	};
	let sql = format!(
		"SELECT {COLUMNS} FROM assets
		WHERE filename LIKE ?1 ESCAPE '\\' AND (?2 IS NULL OR kind = ?2)
		AND (?3 OR folder_id IS ?4) {ends}
		ORDER BY id DESC LIMIT 200"
	);
	// `%` and `_` in the search are themselves
	let escaped: String = search
		.chars()
		.flat_map(|c| match c {
			'\\' | '%' | '_' => vec!['\\', c],
			c => vec![c],
		})
		.collect();
	let search = format!("%{escaped}%");
	let mut values: Vec<Box<dyn rusqlite::ToSql>> = vec![
		Box::new(search),
		Box::new(kind.map(String::from)),
		Box::new(any),
		Box::new(folder),
	];
	for e in extensions {
		values.push(Box::new(format!("%.{e}")));
	}
	let mut stmt = conn.prepare(&sql)?;
	let assets = stmt
		.query_map(rusqlite::params_from_iter(values), from_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(assets)
}

/// What an editor changes of an asset.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetChange {
	/// unchanged when missing
	pub title: Option<String>,
	pub alt: String,
	pub focal_x: f64,
	pub focal_y: f64,
	/// a new name, unchanged when missing
	pub filename: Option<String>,
	/// the image editor's, none resets to the original
	#[serde(default)]
	pub edits: Option<Edits>,
}

/// Changes an asset: a new name moves the file, new edits or a new
/// focal point delete the transforms.
pub fn update(
	conn: &Connection,
	files: &FsPath,
	id: i64,
	change: AssetChange,
) -> Result<Asset> {
	let old = get_asset(conn, id)?;
	let (fx, fy) = (
		change.focal_x.clamp(0.0, 1.0),
		change.focal_y.clamp(0.0, 1.0),
	);
	let edits = change.edits.filter(|e| !e.is_empty());
	if let Some(e) = &edits {
		check_edits(&old, e)?;
	}
	let dir = asset_dir(files, id);
	let filename = match change.filename.as_deref().map(clean_filename) {
		Some(name) if name != old.filename => {
			if dir.join(&name).exists() {
				return Err(Error::Request(format!("{name} exists already")));
			}
			name
		}
		_ => old.filename.clone(),
	};
	let json = edits.as_ref().map(|e| serde_json::to_string(e).unwrap());

	// the row first, the file moves before the commit: when it can't, the
	// row stays as it was
	let tx = conn.unchecked_transaction()?;
	tx.execute(
		"UPDATE assets SET title = IFNULL(?1, title), alt = ?2, focal_x = ?3,
		focal_y = ?4, filename = ?5, edits = ?6 WHERE id = ?7",
		params![
			change.title.as_deref().map(str::trim),
			change.alt,
			fx,
			fy,
			filename,
			json,
			id
		],
	)?;
	if filename != old.filename {
		fs::rename(dir.join(&old.filename), dir.join(&filename))?;
	}
	if let Err(e) = tx.commit() {
		// back to the name the row still has
		let _ = fs::rename(dir.join(&filename), dir.join(&old.filename));
		return Err(e.into());
	}

	// a new name, edits or focal point: the transforms are made again
	if (old.focal_x, old.focal_y) != (fx, fy)
		|| old.edits != edits
		|| old.filename != filename
	{
		let _ = fs::remove_dir_all(dir.join(TRANSFORMS));
	}
	get_asset(conn, id)
}

/// Edits only of images, and not bigger than [`MAX_PIXELS`].
fn check_edits(asset: &Asset, e: &Edits) -> Result<()> {
	if asset.kind != "image" {
		return Err(Error::Request("only images can be edited".into()));
	}
	e.check().map_err(Error::Request)?;
	let (ow, oh) = (
		asset.original_width.unwrap_or(1),
		asset.original_height.unwrap_or(1),
	);
	let (w, h) = e.size(ow, oh);
	// a crop past the image is made before a resize makes it smaller
	let crop = Edits {
		resize: None,
		canvas: None,
		..e.clone()
	};
	let (cw, ch) = crop.size(ow, oh);
	if u64::from(w) * u64::from(h) > MAX_PIXELS
		|| u64::from(cw) * u64::from(ch) > MAX_PIXELS
	{
		return Err(Error::Request(
			"the edited image is bigger than 40 megapixels".into(),
		));
	}
	Ok(())
}

/// Another file for the asset, like Craft's replace file: where it is
/// used stays, the edits and the focal point start new.
pub fn replace(
	conn: &Connection,
	files: &FsPath,
	id: i64,
	filename: &str,
	data: &[u8],
) -> Result<Asset> {
	let old = get_asset(conn, id)?;
	let up = inspect(filename, data)?;

	let dir = asset_dir(files, id);
	let tmp = dir.join(format!(".replace{}", rand::random::<u32>()));
	fs::write(&tmp, data)?;

	// the row first, the new file goes in before the commit; the old one
	// only goes when both worked
	let tx = conn.unchecked_transaction()?;
	let updated = tx.execute(
		"UPDATE assets SET filename = ?1, kind = ?2, mime = ?3, size = ?4,
		width = ?5, height = ?6, edits = NULL, focal_x = 0.5, focal_y = 0.5
		WHERE id = ?7",
		params![
			up.filename,
			up.kind,
			up.mime,
			data.len(),
			up.width,
			up.height,
			id
		],
	);
	let moved = updated
		.map_err(Error::from)
		.and_then(|_| {
			fs::rename(&tmp, dir.join(&up.filename)).map_err(Error::from)
		})
		.and_then(|_| tx.commit().map_err(Error::from));
	if let Err(e) = moved {
		let _ = fs::remove_file(&tmp);
		return Err(e);
	}
	if old.filename != up.filename {
		let _ = fs::remove_file(dir.join(&old.filename));
	}
	let _ = fs::remove_dir_all(dir.join(TRANSFORMS));

	get_asset(conn, id)
}

/// An entry that uses an asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Usage {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	/// the sites it is used in
	pub sites: Vec<String>,
}

/// The entries that use an asset, as published, the title of the
/// primary site first.
pub fn usage(conn: &Connection, primary: &str, id: i64) -> Result<Vec<Usage>> {
	let mut stmt = conn.prepare(
		"SELECT e.id, e.section, e.type, group_concat(DISTINCT r.site),
		(SELECT title FROM entry_sites s WHERE s.entry_id = e.id
			ORDER BY s.site = ?2 DESC LIMIT 1)
		FROM asset_relations r JOIN entries e ON e.id = r.entry_id
		WHERE r.asset_id = ?1 GROUP BY e.id ORDER BY e.id",
	)?;
	let found = stmt
		.query_map(params![id, primary], |r| {
			let sites: String = r.get(3)?;
			Ok(Usage {
				id: r.get(0)?,
				section: r.get(1)?,
				ty: r.get(2)?,
				sites: sites.split(',').map(String::from).collect(),
				title: r.get(4)?,
			})
		})?
		.collect::<rusqlite::Result<_>>()?;
	Ok(found)
}

/// Deletes an asset and its files.
pub fn delete(conn: &Connection, files: &FsPath, id: i64) -> Result<()> {
	if conn.execute("DELETE FROM assets WHERE id = ?1", [id])? == 0 {
		return Err(Error::NotFound);
	}
	let _ = fs::remove_dir_all(asset_dir(files, id));
	Ok(())
}
