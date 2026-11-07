//! What editors do with assets: find, change, replace, delete them and
//! see where they are used.

use std::collections::BTreeMap;

use serde_json::Value;

use super::*;
use crate::content::validate;
use crate::db::escape_like;
use crate::entries::{check_relations, links_of};
use crate::schema::{ASSET, Schema};

/// Where to look for assets: all folders, or one (`None` the top).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum InFolder {
	All,
	Folder(Option<i64>),
}

/// With `kind` only assets of it (see [`Kind`]); with `extensions` only
/// files that end so (lowercase, without dot).
pub(crate) fn search(
	conn: &Connection,
	search: &str,
	kind: Option<&str>,
	folder: InFolder,
	extensions: &[String],
) -> Result<Vec<Asset>> {
	let kind = match kind {
		None => String::new(),
		Some(k) => format!("AND {}", Kind::parse(k)?.sql()),
	};
	let (any, folder) = match folder {
		InFolder::All => (true, None),
		InFolder::Folder(f) => (false, f),
	};
	// one LIKE per extension, after the three params
	let ends = if extensions.is_empty() {
		String::new()
	} else {
		let likes: Vec<String> = (0..extensions.len())
			.map(|i| format!("lower(filename) LIKE ?{}", i + 4))
			.collect();
		format!("AND ({})", likes.join(" OR "))
	};
	let sql = format!(
		"SELECT {COLUMNS} FROM assets
		WHERE deleted_at IS NULL AND filename LIKE ?1 ESCAPE '\\' {kind}
		AND (?2 OR folder_id IS ?3) {ends}
		ORDER BY id DESC LIMIT 200"
	);
	let search = format!("%{}%", escape_like(search));
	let mut values: Vec<Box<dyn rusqlite::ToSql>> =
		vec![Box::new(search), Box::new(any), Box::new(folder)];
	for e in extensions {
		values.push(Box::new(format!("%.{e}")));
	}
	let mut stmt = conn.prepare(&sql)?;
	let assets = stmt
		.query_map(rusqlite::params_from_iter(values), from_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(assets)
}

/// What assets are, to filter by: the kinds of [`Asset`] and Craft's
/// kinds of files a site asks for.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Kind {
	Image,
	Video,
	/// neither an image nor a video
	File,
	/// a file but audio, the studio's documents
	Document,
	Pdf,
	/// Word documents
	Word,
}

/// The extensions of Craft's kind `word`.
const WORD: &[&str] = &["doc", "docx", "docm", "dot", "dotm", "dotx", "odt"];

impl Kind {
	pub(crate) fn parse(kind: &str) -> Result<Kind> {
		Ok(match kind {
			"image" => Kind::Image,
			"video" => Kind::Video,
			"file" => Kind::File,
			"document" => Kind::Document,
			"pdf" => Kind::Pdf,
			"word" => Kind::Word,
			k => {
				return Err(Error::Request(format!(
					"kind {k}: image, video, file, document, pdf or word"
				)));
			}
		})
	}

	/// The condition on a row of `assets`, without values.
	fn sql(self) -> String {
		let ends = |exts: &[&str]| {
			let likes: Vec<String> = exts
				.iter()
				.map(|e| format!("lower(filename) LIKE '%.{e}'"))
				.collect();
			format!("({})", likes.join(" OR "))
		};
		match self {
			Kind::Image => "kind = 'image'".into(),
			Kind::Video => "kind = 'video'".into(),
			Kind::File => "kind = 'file'".into(),
			Kind::Document => {
				"(kind = 'file' AND mime NOT LIKE 'audio/%')".into()
			}
			Kind::Pdf => ends(&["pdf"]),
			Kind::Word => ends(WORD),
		}
	}

	pub(crate) fn of(self, a: &Asset) -> bool {
		let ext = a
			.filename
			.rsplit_once('.')
			.map(|(_, e)| e.to_ascii_lowercase())
			.unwrap_or_default();
		match self {
			Kind::Image => a.kind == "image",
			Kind::Video => a.kind == "video",
			Kind::File => a.kind == "file",
			Kind::Document => a.kind == "file" && !a.mime.starts_with("audio/"),
			Kind::Pdf => ext == "pdf",
			Kind::Word => WORD.contains(&ext.as_str()),
		}
	}
}

/// Assets of some folders, kinds and words, for GraphQL's `assets`.
#[derive(Debug, Clone, Default)]
pub(crate) struct AssetList {
	/// the ids of folders, `None` all
	pub folders: Option<Vec<i64>>,
	/// any of them, none all
	pub kinds: Vec<Kind>,
	/// each in the filename, title or alt text
	pub words: Vec<String>,
	/// `title`, `filename`, `dateCreated`, `size` or `id`, maybe with
	/// `ASC` or `DESC`, the newest first without
	pub order_by: Option<String>,
	pub limit: Option<i64>,
	pub offset: Option<i64>,
}

impl AssetList {
	/// The `WHERE` and its values.
	fn filter(&self) -> (String, Vec<String>) {
		let mut and = vec!["deleted_at IS NULL".to_string()];
		if let Some(folders) = &self.folders {
			let ids: Vec<String> = folders.iter().map(i64::to_string).collect();
			and.push(format!("folder_id IN ({})", ids.join(", ")));
		}
		if !self.kinds.is_empty() {
			let any: Vec<String> = self.kinds.iter().map(|k| k.sql()).collect();
			and.push(format!("({})", any.join(" OR ")));
		}
		and.extend((1..=self.words.len()).map(word_sql));
		let values = self.words.iter().map(|w| format!("%{}%", escape_like(w)));
		(and.join(" AND "), values.collect())
	}

	/// Craft's `orderBy`, as SQL.
	fn order(&self) -> Result<String> {
		let Some(o) = &self.order_by else {
			return Ok("created_at DESC, id DESC".into());
		};
		let mut words = o.split_whitespace();
		let column = match words.next().unwrap_or_default() {
			// an asset without a title has the one of its filename
			"title" => {
				"CASE title WHEN '' THEN filename ELSE title END \
				COLLATE NOCASE"
			}
			"filename" => "filename COLLATE NOCASE",
			"dateCreated" | "createdAt" => "created_at",
			"size" => "size",
			"id" => "id",
			c => {
				return Err(Error::Request(format!(
					"orderBy {c}: title, filename, dateCreated, size or id"
				)));
			}
		};
		let dir = match words.next().map(str::to_ascii_uppercase).as_deref() {
			None | Some("ASC") => "ASC",
			Some("DESC") => "DESC",
			Some(d) => return Err(Error::Request(format!("orderBy {d}"))),
		};
		Ok(format!("{column} {dir}, id {dir}"))
	}
}

/// The assets of a list, in its order.
pub(crate) fn list(conn: &Connection, q: &AssetList) -> Result<Vec<Asset>> {
	let (filter, values) = q.filter();
	// SQLite needs a limit before an offset, -1 is none
	let sql = format!(
		"SELECT {COLUMNS} FROM assets WHERE {filter} ORDER BY {}
		LIMIT {} OFFSET {}",
		q.order()?,
		q.limit.unwrap_or(-1),
		q.offset.unwrap_or(0).max(0),
	);
	let mut stmt = conn.prepare(&sql)?;
	let assets = stmt
		.query_map(rusqlite::params_from_iter(values), from_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(assets)
}

/// How many assets a list has, without its limit.
pub(crate) fn count(conn: &Connection, q: &AssetList) -> Result<i64> {
	let (filter, values) = q.filter();
	let sql = format!("SELECT COUNT(*) FROM assets WHERE {filter}");
	let n =
		conn.query_row(&sql, rusqlite::params_from_iter(values), |r| r.get(0))?;
	Ok(n)
}

/// That the filename, title or alt text contain the word of value `i`.
fn word_sql(i: usize) -> String {
	format!(
		"(filename LIKE ?{i} ESCAPE '\\' OR title LIKE ?{i} ESCAPE '\\' \
		OR EXISTS (SELECT 1 FROM json_each(alt) \
			WHERE value LIKE ?{i} ESCAPE '\\'))"
	)
}

/// Assets whose filename, title or alt text contain every word, the
/// newest first, for the studio's search.
pub(crate) fn find(
	conn: &Connection,
	words: &[String],
	limit: i64,
) -> Result<Vec<Asset>> {
	if words.is_empty() {
		return Ok(vec![]);
	}
	let likes: Vec<String> = (1..=words.len()).map(word_sql).collect();
	let sql = format!(
		"SELECT {COLUMNS} FROM assets WHERE deleted_at IS NULL AND {}
		ORDER BY id DESC LIMIT {limit}",
		likes.join(" AND ")
	);
	let values = words.iter().map(|w| format!("%{}%", escape_like(w)));
	let mut stmt = conn.prepare(&sql)?;
	let assets = stmt
		.query_map(rusqlite::params_from_iter(values), from_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(assets)
}

/// What an editor changes of an asset.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AssetChange {
	/// unchanged when missing
	pub title: Option<String>,
	/// per site handle, of all sites
	pub alt: BTreeMap<String, String>,
	pub focal_x: f64,
	pub focal_y: f64,
	/// a new name, unchanged when missing
	pub filename: Option<String>,
	/// the image editor's, none resets to the original
	#[serde(default)]
	pub edits: Option<Edits>,
	/// the values of the asset fields, unchanged when missing
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub fields: Option<Value>,
}

/// Changes an asset: a new name moves the file, new edits or a new
/// focal point delete the transforms. The values of the asset fields
/// are checked like those of a block, for the user `user_id` (`None`: the
/// system).
pub(crate) fn update(
	conn: &Connection,
	files: &FsPath,
	schema: &Schema,
	id: i64,
	change: AssetChange,
	user_id: Option<i64>,
) -> Result<Asset> {
	let old = get_asset(conn, id)?;
	let fields = match &change.fields {
		Some(v) => {
			let fields = check_fields(conn, schema, v, &old.fields, user_id)?;
			Some(fields.to_string())
		}
		None => None,
	};
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
	// of the sites there are, the empty ones fall back to the primary's
	let alt: BTreeMap<&str, &str> = change
		.alt
		.iter()
		.filter(|(site, _)| schema.get_site(site).is_some())
		.map(|(site, a)| (site.as_str(), a.trim()))
		.filter(|(_, a)| !a.is_empty())
		.collect();

	// the row first, the file moves before the commit: when it can't, the
	// row stays as it was
	let tx = conn.unchecked_transaction()?;
	tx.execute(
		"UPDATE assets SET title = IFNULL(?1, title), alt = ?2, focal_x = ?3,
		focal_y = ?4, filename = ?5, edits = ?6, fields = IFNULL(?7, fields)
		WHERE id = ?8",
		params![
			change.title.as_deref().map(str::trim),
			serde_json::to_string(&alt).unwrap(),
			fx,
			fy,
			filename,
			json,
			fields,
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

/// The values of the asset fields cleaned up, or what is wrong with
/// them; nothing to check without asset fields.
pub(crate) fn check_fields(
	conn: &Connection,
	schema: &Schema,
	fields: &Value,
	old: &Value,
	user_id: Option<i64>,
) -> Result<Value> {
	let Some(comp) = schema.get(ASSET) else {
		return Ok(Value::Object(Default::default()));
	};
	let mut fields = validate(schema, comp, fields).map_err(Error::Invalid)?;
	let before = links_of(schema, comp, old);
	let errors =
		check_relations(conn, schema, ASSET, &mut fields, user_id, &before)?;
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}
	Ok(fields)
}

/// Edits only of images, and not bigger than [`MAX_PIXELS`].
fn check_edits(asset: &Asset, e: &Edits) -> Result<()> {
	if asset.kind != "image" {
		return Err(Error::Request("only images can be edited".into()));
	}
	e.check().map_err(Error::Request)?;
	let (w, h) = (
		asset.original_width.unwrap_or(1),
		asset.original_height.unwrap_or(1),
	);
	e.check_size(w, h).map_err(Error::Request)
}

/// Another file for the asset, like Craft's replace file: where it is
/// used stays, the edits and the focal point start new.
pub(crate) fn replace(
	conn: &Connection,
	files: &FsPath,
	id: i64,
	filename: &str,
	tmp: tempfile::NamedTempFile,
) -> Result<Asset> {
	let old = get_asset(conn, id)?;
	let up = inspect(filename, tmp.path())?;
	let size = tmp.as_file().metadata()?.len();
	let dir = asset_dir(files, id);

	// the row first, the new file goes in before the commit; the old one
	// only goes when both worked
	let tx = conn.unchecked_transaction()?;
	tx.execute(
		"UPDATE assets SET filename = ?1, kind = ?2, mime = ?3, size = ?4,
		width = ?5, height = ?6, edits = NULL, focal_x = 0.5, focal_y = 0.5,
		replaced = replaced + 1 WHERE id = ?7",
		params![up.filename, up.kind, up.mime, size, up.width, up.height, id],
	)?;
	fs::create_dir_all(&dir)?;
	tmp.persist(dir.join(&up.filename))
		.map_err(|e| Error::Internal(e.to_string()))?;
	tx.commit()?;
	if old.filename != up.filename {
		let _ = fs::remove_file(dir.join(&old.filename));
	}
	let _ = fs::remove_dir_all(dir.join(TRANSFORMS));

	get_asset(conn, id)
}

/// An entry that uses an asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Usage {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	/// the sites it is used in
	pub sites: Vec<String>,
	/// who made the entry
	pub author_id: Option<i64>,
}

/// The entries that use an asset, as published, the title of the
/// primary site first.
pub(crate) fn usage(
	conn: &Connection,
	primary: &str,
	id: i64,
) -> Result<Vec<Usage>> {
	let mut stmt = conn.prepare(
		"SELECT e.id, e.section, e.type, group_concat(DISTINCT r.site),
		(SELECT title FROM entry_sites s WHERE s.entry_id = e.id
			ORDER BY s.site = ?2 DESC LIMIT 1), e.author_id
		FROM asset_relations r JOIN entries e ON e.id = r.entry_id
		WHERE r.asset_id = ?1 AND e.deleted_at IS NULL
		GROUP BY e.id ORDER BY e.id",
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
				author_id: r.get(5)?,
			})
		})?
		.collect::<rusqlite::Result<_>>()?;
	Ok(found)
}

/// Deletes an asset and its files for good, see [`crate::trash`].
pub(crate) fn delete(conn: &Connection, files: &FsPath, id: i64) -> Result<()> {
	if conn.execute("DELETE FROM assets WHERE id = ?1", [id])? == 0 {
		return Err(Error::NotFound);
	}
	let _ = fs::remove_dir_all(asset_dir(files, id));
	Ok(())
}
