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
	focal_x, focal_y, created_at";

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

/// The size of a `width` x `height` image after a transform, never bigger
/// than the image.
pub fn transform_size(width: u32, height: u32, t: &Transform) -> (u32, u32) {
	let (w, h) = (width as f64, height as f64);
	let scale_w = t.width.map_or(f64::MAX, |tw| tw as f64 / w);
	let scale_h = t.height.map_or(f64::MAX, |th| th as f64 / h);

	if t.crop {
		let (tw, th) = (t.width.unwrap() as f64, t.height.unwrap() as f64);
		let s = (w / tw).min(h / th).min(1.0);
		return ((tw * s).round() as u32, (th * s).round() as u32);
	}

	let s = scale_w.min(scale_h).min(1.0);
	(
		((w * s).round() as u32).max(1),
		((h * s).round() as u32).max(1),
	)
}

/// The part of the image a crop to `out_w` x `out_h` keeps: the biggest
/// rect of that ratio, as close to the focal point as it fits.
pub fn crop_rect(
	width: u32,
	height: u32,
	out_w: u32,
	out_h: u32,
	focal: (f64, f64),
) -> (u32, u32, u32, u32) {
	let ratio = out_w as f64 / out_h as f64;
	let (cw, ch) = if width as f64 / height as f64 > ratio {
		((height as f64 * ratio).round() as u32, height)
	} else {
		(width, (width as f64 / ratio).round() as u32)
	};
	let x = (focal.0 * width as f64 - cw as f64 / 2.0)
		.clamp(0.0, (width - cw) as f64);
	let y = (focal.1 * height as f64 - ch as f64 / 2.0)
		.clamp(0.0, (height - ch) as f64);
	(x.round() as u32, y.round() as u32, cw, ch)
}

/// Writes the transform of an image to `out`.
fn generate(
	asset: &Asset,
	original: &FsPath,
	t: &Transform,
	out: &FsPath,
) -> Result<()> {
	let internal = |e: &dyn std::fmt::Display| Error::Internal(e.to_string());
	let img = image::open(original).map_err(|e| internal(&e))?;
	let format = ImageFormat::from_path(original).map_err(|e| internal(&e))?;
	let (w, h) = (img.width(), img.height());
	let (out_w, out_h) = transform_size(w, h, t);

	let img = if t.crop {
		let focal = (asset.focal_x, asset.focal_y);
		let (x, y, cw, ch) = crop_rect(w, h, out_w, out_h, focal);
		img.crop_imm(x, y, cw, ch).resize_exact(
			out_w,
			out_h,
			FilterType::Lanczos3,
		)
	} else {
		img.resize_exact(out_w, out_h, FilterType::Lanczos3)
	};

	// write next to it and rename, two requests may generate it at once
	fs::create_dir_all(out.parent().unwrap()).map_err(|e| internal(&e))?;
	let tmp = out.with_extension(format!("tmp{}", rand::random::<u32>()));
	if format == ImageFormat::Jpeg {
		let file = fs::File::create(&tmp).map_err(|e| internal(&e))?;
		let encoder = JpegEncoder::new_with_quality(file, 82);
		img.to_rgb8()
			.write_with_encoder(encoder)
			.map_err(|e| internal(&e))?;
	} else {
		img.save_with_format(&tmp, format)
			.map_err(|e| internal(&e))?;
	}
	fs::rename(&tmp, out).map_err(|e| internal(&e))?;
	Ok(())
}

fn transform_by_name(state: &AppState, name: &str) -> Option<Transform> {
	match name {
		"_thumb" => Some(Transform::new("_thumb").width(400).height(400)),
		_ => state.schema.get_transform(name).cloned(),
	}
}

async fn serve_original(
	State(state): State<AppState>,
	Path((id, filename)): Path<(i64, String)>,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	if asset.filename != filename {
		return Err(Error::NotFound);
	}
	let path = state.files.join(id.to_string()).join(&asset.filename);
	let data = fs::read(path).map_err(|_| Error::NotFound)?;
	Ok(file_response(&asset.mime, data))
}

async fn serve_transform(
	State(state): State<AppState>,
	Path((id, name, filename)): Path<(i64, String, String)>,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	let t = transform_by_name(&state, &name).ok_or(Error::NotFound)?;
	if asset.filename != filename || asset.kind != "image" {
		return Err(Error::NotFound);
	}

	let dir = state.files.join(id.to_string());
	let original = dir.join(&asset.filename);
	let out = dir.join("t").join(format!("{name}-{}", asset.filename));
	if !out.exists() {
		let _permit = GENERATING.acquire().await.unwrap();
		let (a, o) = (asset.clone(), out.clone());
		tokio::task::spawn_blocking(move || generate(&a, &original, &t, &o))
			.await
			.map_err(|e| Error::Internal(e.to_string()))??;
	}

	let data = fs::read(out).map_err(|e| Error::Internal(e.to_string()))?;
	Ok(file_response(&asset.mime, data))
}

/// Files are served from the studio's origin: nothing may run as a page
/// there. Images, video, audio and pdf show in the browser, everything
/// else (svg too) downloads.
fn file_response(mime: &str, data: Vec<u8>) -> Response {
	let inline = (mime.starts_with("image/") && mime != "image/svg+xml")
		|| mime.starts_with("video/")
		|| mime.starts_with("audio/")
		|| mime == "application/pdf";
	(
		[
			(header::CONTENT_TYPE, mime.to_string()),
			(header::CACHE_CONTROL, "public, max-age=86400".into()),
			(header::X_CONTENT_TYPE_OPTIONS, "nosniff".into()),
			(header::CONTENT_SECURITY_POLICY, "sandbox".into()),
			(
				header::CONTENT_DISPOSITION,
				if inline { "inline" } else { "attachment" }.into(),
			),
		],
		data,
	)
		.into_response()
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

/// Transforms generated at once, each decodes a whole image.
static GENERATING: tokio::sync::Semaphore =
	tokio::sync::Semaphore::const_new(2);

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

async fn upload(
	State(state): State<AppState>,
	_auth: Authed,
	mut multipart: Multipart,
) -> Result<Json<Vec<Asset>>> {
	let mut assets = vec![];
	while let Some(field) = multipart
		.next_field()
		.await
		.map_err(|e| Error::Request(e.to_string()))?
	{
		let filename = field.file_name().unwrap_or("file").to_string();
		let data = field
			.bytes()
			.await
			.map_err(|e| Error::Request(e.to_string()))?;
		let conn = state.db.conn();
		assets.push(store(&conn, &state.files, &filename, &data)?);
	}
	Ok(Json(assets))
}

#[derive(Debug, Deserialize)]
struct ListParams {
	search: Option<String>,
	ids: Option<String>,
	kind: Option<String>,
}

async fn list(
	State(state): State<AppState>,
	_auth: Authed,
	Query(p): Query<ListParams>,
) -> Result<Json<Vec<Asset>>> {
	let conn = state.db.conn();
	if let Some(ids) = p.ids {
		let ids: Vec<i64> =
			ids.split(',').filter_map(|i| i.parse().ok()).collect();
		return Ok(Json(get_many(&conn, &ids)?));
	}

	let sql = format!(
		"SELECT {COLUMNS} FROM assets
		WHERE filename LIKE ?1 AND (?2 IS NULL OR kind = ?2)
		ORDER BY id DESC LIMIT 200"
	);
	let search = format!("%{}%", p.search.unwrap_or_default());
	let mut stmt = conn.prepare(&sql)?;
	let assets = stmt
		.query_map(params![search, p.kind], from_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(Json(assets))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateReq {
	alt: String,
	focal_x: f64,
	focal_y: f64,
}

async fn update(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
	Json(req): Json<UpdateReq>,
) -> Result<Json<Asset>> {
	let conn = state.db.conn();
	let old = get_asset(&conn, id)?;
	let (fx, fy) = (req.focal_x.clamp(0.0, 1.0), req.focal_y.clamp(0.0, 1.0));
	conn.execute(
		"UPDATE assets SET alt = ?1, focal_x = ?2, focal_y = ?3 WHERE id = ?4",
		params![req.alt, fx, fy, id],
	)?;
	if (old.focal_x, old.focal_y) != (fx, fy) {
		let _ = fs::remove_dir_all(state.files.join(id.to_string()).join("t"));
	}
	Ok(Json(get_asset(&conn, id)?))
}

async fn delete(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	let conn = state.db.conn();
	if conn.execute("DELETE FROM assets WHERE id = ?1", [id])? == 0 {
		return Err(Error::NotFound);
	}
	let _ = fs::remove_dir_all(state.files.join(id.to_string()));
	Ok(())
}

/// The api, under `/api`.
pub fn api_routes() -> Router<AppState> {
	Router::new()
		.route(
			"/assets",
			get(list)
				.post(upload)
				.layer(DefaultBodyLimit::max(200 * 1024 * 1024)),
		)
		.route("/assets/{id}", put(update).delete(delete))
}

/// The files, under `/assets`.
pub fn file_routes() -> Router<AppState> {
	Router::new()
		.route("/assets/{id}/{filename}", get(serve_original))
		.route("/assets/{id}/{transform}/{filename}", get(serve_transform))
}

/// Where uploads go, `--files`.
pub fn files_dir(path: PathBuf) -> std::io::Result<PathBuf> {
	fs::create_dir_all(&path)?;
	Ok(path)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn cleans_filenames() {
		assert_eq!(clean_filename("../../etc/Hä llo.JPG"), "H--llo.JPG");
		assert_eq!(clean_filename(".."), "file");
	}

	#[test]
	fn sizes_after_transforms() {
		let fit = Transform::new("m").width(1500).height(1000);
		assert_eq!(transform_size(3000, 1000, &fit), (1500, 500));
		assert_eq!(transform_size(800, 600, &fit), (800, 600));
		let w = Transform::new("s").width(600);
		assert_eq!(transform_size(1200, 900, &w), (600, 450));

		let crop = Transform::new("sq").width(500).height(500).crop();
		assert_eq!(transform_size(2000, 1000, &crop), (500, 500));
		// smaller than the transform: the biggest square that fits
		assert_eq!(transform_size(400, 300, &crop), (300, 300));
	}

	#[test]
	fn crops_around_the_focal_point() {
		// a square out of a wide image, focal point right
		assert_eq!(
			crop_rect(2000, 1000, 500, 500, (0.9, 0.5)),
			(1000, 0, 1000, 1000)
		);
		// centered
		assert_eq!(
			crop_rect(2000, 1000, 500, 500, (0.5, 0.5)),
			(500, 0, 1000, 1000)
		);
		// a wide one out of a tall image, focal point at the top
		assert_eq!(
			crop_rect(1000, 2000, 1000, 500, (0.5, 0.0)),
			(0, 0, 1000, 500)
		);
	}

	#[test]
	fn stores_and_transforms_an_image() {
		let db = crate::db::Db::memory().unwrap();
		let dir = std::env::temp_dir()
			.join(format!("zipp-test-{}", rand::random::<u32>()));
		let mut png = vec![];
		image::RgbImage::new(300, 200)
			.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
			.unwrap();

		let a = store(&db.conn(), &dir, "test image.png", &png).unwrap();
		assert_eq!(
			(a.kind.as_str(), a.width, a.height),
			("image", Some(300), Some(200))
		);
		assert_eq!(
			a.url(Some("small")),
			format!("/assets/{}/small/test-image.png", a.id)
		);

		let t = Transform::new("sq").width(100).height(100).crop();
		let out = dir.join("out.png");
		generate(&a, &dir.join(a.id.to_string()).join(&a.filename), &t, &out)
			.unwrap();
		let img = image::open(&out).unwrap();
		assert_eq!((img.width(), img.height()), (100, 100));

		let f = store(&db.conn(), &dir, "notes.txt", b"hello").unwrap();
		assert_eq!((f.kind.as_str(), f.mime.as_str()), ("file", "text/plain"));
		// html is no type of ours, whatever the upload says
		let f = store(&db.conn(), &dir, "x.html", b"<script>").unwrap();
		assert_eq!(f.mime, "application/octet-stream");
		fs::remove_dir_all(dir).unwrap();
	}

	#[test]
	fn files_never_run_as_pages() {
		let r = file_response("image/svg+xml", vec![]);
		let h = r.headers();
		assert_eq!(h[header::CONTENT_DISPOSITION], "attachment");
		assert_eq!(h[header::CONTENT_SECURITY_POLICY], "sandbox");
		assert_eq!(h[header::X_CONTENT_TYPE_OPTIONS], "nosniff");
		let r = file_response("image/png", vec![]);
		assert_eq!(r.headers()[header::CONTENT_DISPOSITION], "inline");
	}
}
