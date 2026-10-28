//! Assets: uploaded files and images. A file lives in
//! `<files>/<id>/<filename>`, its transforms in `<files>/<id>/t/` where
//! they are generated on the first request. Changing the focal point
//! deletes them.
//!
//! - `/assets/<id>/<filename>`: the original
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
use axum::routing::{get, put};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{ImageFormat, ImageReader};
use rusqlite::{Connection, OptionalExtension, Row, params};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::auth::Authed;
use crate::db::now;
use crate::error::{Error, Result};
use crate::schema::Transform;

mod routes;
#[cfg(test)]
pub(crate) mod tests;
mod transform;

pub use routes::api_routes;
#[cfg(test)]
use routes::file_response;
pub use routes::file_routes;
pub use transform::crop_rect;
pub use transform::transform_size;
use transform::*;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
	pub id: i64,
	pub filename: String,
	/// `image` or `file`
	pub kind: String,
	pub mime: String,
	pub size: i64,
	pub width: Option<u32>,
	pub height: Option<u32>,
	/// like Craft's asset title, empty for one from the filename
	pub title: String,
	pub alt: String,
	pub focal_x: f64,
	pub focal_y: f64,
	pub created_at: i64,
}

impl Asset {
	/// The path of the original or of a transform.
	pub fn url(&self, transform: Option<&str>) -> String {
		match transform {
			Some(t) => format!("/assets/{}/{t}/{}", self.id, self.filename),
			None => format!("/assets/{}/{}", self.id, self.filename),
		}
	}

	/// The size of the image after a transform.
	pub fn size_after(&self, t: &Transform) -> Option<(u32, u32)> {
		Some(transform_size(self.width?, self.height?, t))
	}
}

const COLUMNS: &str = "id, filename, kind, mime, size, width, height, alt, \
	focal_x, focal_y, created_at, title";

fn from_row(r: &Row) -> rusqlite::Result<Asset> {
	Ok(Asset {
		id: r.get(0)?,
		filename: r.get(1)?,
		kind: r.get(2)?,
		mime: r.get(3)?,
		size: r.get(4)?,
		width: r.get(5)?,
		height: r.get(6)?,
		alt: r.get(7)?,
		focal_x: r.get(8)?,
		focal_y: r.get(9)?,
		created_at: r.get(10)?,
		title: r.get(11)?,
	})
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

/// Stores an upload: the row first for the id, then the file.
pub fn store(
	conn: &Connection,
	files: &FsPath,
	filename: &str,
	data: &[u8],
) -> Result<Asset> {
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

	conn.execute(
		"INSERT INTO assets (filename, kind, mime, size, width, height,
		created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
		params![filename, kind, mime, data.len(), width, height, now()],
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
