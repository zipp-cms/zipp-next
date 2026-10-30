//! Assets: uploaded files and images. A file lives in
//! `<files>/<id>/<filename>`, its transforms in `<files>/<id>/t/` where
//! they are generated on the first request. Changing the focal point or
//! the edits (see [`Edits`]) deletes them.
//!
//! - `/assets/<id>/<filename>`: the image with its edits, `?original`
//!   the file as it was uploaded
//! - `/assets/<id>/<transform>/<filename>`: a named transform of the
//!   schema, `_thumb` is the one of the studio

use std::fs;
use std::io::Cursor;
use std::path::{Path as FsPath, PathBuf};

use axum::Json;
use axum::Router;
use axum::extract::{DefaultBodyLimit, Multipart, Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{ImageFormat, ImageReader};
use rusqlite::{Connection, OptionalExtension, Row, params};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::db::now;
use crate::error::{Error, Result};
use crate::schema::Transform;

pub mod api;
#[cfg(test)]
mod api_tests;
mod edits;
mod manage;
mod routes;
#[cfg(test)]
pub(crate) mod tests;
mod transform;

pub use edits::{Canvas, Crop, Edits, Resize};
pub use manage::{AssetChange, Usage, delete, replace, search, update, usage};

pub use routes::api_routes;
#[cfg(test)]
use routes::file_response;
pub use routes::file_routes;
pub use transform::crop_rect;
pub use transform::transform_size;
use transform::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
	pub id: i64,
	pub filename: String,
	/// `image` or `file`
	pub kind: String,
	pub mime: String,
	pub size: i64,
	/// the size with the edits, what transforms start from
	pub width: Option<u32>,
	pub height: Option<u32>,
	/// the size of the uploaded file
	pub original_width: Option<u32>,
	pub original_height: Option<u32>,
	/// the image editor's, see [`Edits`]
	pub edits: Option<Edits>,
	/// the email of who uploaded it
	pub uploaded_by: Option<String>,
	/// changes with the edits and the focal point, urls carry it so
	/// browsers don't keep showing the image from before; empty without
	pub version: String,
	/// like Craft's asset title, empty for one from the filename
	pub title: String,
	pub alt: String,
	pub focal_x: f64,
	pub focal_y: f64,
	pub created_at: i64,
}

impl Asset {
	/// The path of the image or of a transform.
	pub fn url(&self, transform: Option<&str>) -> String {
		let path = match transform {
			Some(t) => format!("/assets/{}/{t}/{}", self.id, self.filename),
			None => format!("/assets/{}/{}", self.id, self.filename),
		};
		match self.version.as_str() {
			"" => path,
			v => format!("{path}?v={v}"),
		}
	}

	/// The size of the image after a transform.
	pub fn size_after(&self, t: &Transform) -> Option<(u32, u32)> {
		Some(transform_size(self.width?, self.height?, t))
	}

	/// The type of the images made of it: a transparent canvas needs a
	/// png when the original is a jpeg.
	pub fn output_mime(&self) -> &str {
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
	(SELECT email FROM users WHERE id = uploaded_by)";

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
		version: version(&edits, focal_x, focal_y),
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
		alt: r.get(7)?,
		focal_x,
		focal_y,
		created_at: r.get(10)?,
		title: r.get(11)?,
	})
}

/// A short hash of what changes the image, empty for an untouched one.
fn version(edits: &Option<Edits>, focal_x: f64, focal_y: f64) -> String {
	use std::hash::{Hash, Hasher};
	if edits.is_none() && (focal_x, focal_y) == (0.5, 0.5) {
		return String::new();
	}
	let mut h = std::collections::hash_map::DefaultHasher::new();
	serde_json::to_string(edits)
		.unwrap_or_default()
		.hash(&mut h);
	focal_x.to_bits().hash(&mut h);
	focal_y.to_bits().hash(&mut h);
	format!("{:x}", h.finish() & 0xffff_ffff)
}

pub fn get_asset(conn: &Connection, id: i64) -> Result<Asset> {
	let sql = format!("SELECT {COLUMNS} FROM assets WHERE id = ?1");
	Ok(conn.query_row(&sql, [id], from_row)?)
}

/// The assets of these ids in the same order, missing ones are left out.
pub fn get_many(conn: &Connection, ids: &[i64]) -> Result<Vec<Asset>> {
	let mut out = vec![];
	for id in ids {
		let sql = format!("SELECT {COLUMNS} FROM assets WHERE id = ?1");
		if let Some(a) = conn.query_row(&sql, [id], from_row).optional()? {
			out.push(a);
		}
	}
	Ok(out)
}

/// Only letters, digits, dots, dashes and underscores, so the name is a
/// safe path and url.
pub fn clean_filename(name: &str) -> String {
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

/// Images above this many pixels are refused, decoding them for a
/// transform would take too much memory.
const MAX_PIXELS: u64 = 40_000_000;

/// What an upload is: its clean name, kind, type and size.
struct Upload {
	filename: String,
	kind: &'static str,
	mime: String,
	width: Option<u32>,
	height: Option<u32>,
}

fn inspect(filename: &str, data: &[u8]) -> Result<Upload> {
	let filename = clean_filename(filename);
	let image = image::guess_format(data).ok().and_then(|f| {
		let reader = ImageReader::with_format(Cursor::new(data), f);
		let (w, h) = reader.into_dimensions().ok()?;
		Some((f, w, h))
	});
	let (kind, mime, width, height) = match image {
		Some((f, w, h)) => {
			("image", f.to_mime_type().to_string(), Some(w), Some(h))
		}
		None => ("file", file_mime(&filename).to_string(), None, None),
	};
	if let (Some(w), Some(h)) = (width, height)
		&& u64::from(w) * u64::from(h) > MAX_PIXELS
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

/// Stores an upload: the row first for the id, then the file. `user` is
/// who uploads, none for imports.
pub fn store(
	conn: &Connection,
	files: &FsPath,
	filename: &str,
	data: &[u8],
	user: Option<i64>,
) -> Result<Asset> {
	let Upload {
		filename,
		kind,
		mime,
		width,
		height,
	} = inspect(filename, data)?;
	conn.execute(
		"INSERT INTO assets (filename, kind, mime, size, width, height,
		created_at, uploaded_by) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
		params![filename, kind, mime, data.len(), width, height, now(), user],
	)?;
	let id = conn.last_insert_rowid();

	let dir = files.join(id.to_string());
	let written = fs::create_dir_all(&dir)
		.and_then(|_| fs::write(dir.join(&filename), data));
	if let Err(e) = written {
		conn.execute("DELETE FROM assets WHERE id = ?1", [id])?;
		return Err(Error::Internal(e.to_string()));
	}

	get_asset(conn, id)
}

/// Where uploads go, `--files`.
pub fn files_dir(path: PathBuf) -> std::io::Result<PathBuf> {
	fs::create_dir_all(&path)?;
	Ok(path)
}

/// Fills in where assets are used for content from before zipp kept it
/// (`asset_relations`): once, when that is empty and there is content.
pub fn index_usage(
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
	let mut stmt = conn.prepare(
		"SELECT e.id, s.site, e.type, s.fields FROM entries e
		JOIN entry_sites s ON s.entry_id = e.id",
	)?;
	let rows = stmt
		.query_map([], |r| {
			Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get::<_, String>(3)?))
		})?
		.collect::<rusqlite::Result<Vec<(i64, String, String, String)>>>()?;
	for (id, site, ty, fields) in rows {
		let fields = serde_json::from_str(&fields).unwrap_or_default();
		crate::entries::reindex(conn, schema, id, &site, &ty, &fields)?;
	}
	Ok(())
}
