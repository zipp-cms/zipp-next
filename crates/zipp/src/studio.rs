//! The studio, built from `studio/` of the repository into
//! `crates/zipp/studio` (`npm run build`), so a site needs no npm.

use axum::Router;
use axum::http::{StatusCode, Uri, header};
use axum::response::{IntoResponse, Redirect, Response};

use crate::AppState;

#[derive(rust_embed::Embed)]
#[folder = "studio/"]
struct Files;

async fn file(uri: Uri) -> Response {
	let path = uri.path().trim_start_matches('/');

	// the pages of the studio are under /admin, any other path is a file
	let (path, file) = match Files::get(path) {
		Some(f) => (path, f),
		None if path.is_empty() => {
			return Redirect::to("/admin").into_response();
		}
		None if path == "admin" || path.starts_with("admin/") => {
			("index.html", Files::get("index.html").unwrap())
		}
		None => return StatusCode::NOT_FOUND.into_response(),
	};

	let mime = match path.rsplit_once('.').map(|(_, ext)| ext) {
		Some("html") => "text/html; charset=utf-8",
		Some("js") => "text/javascript",
		Some("css") => "text/css",
		Some("svg") => "image/svg+xml",
		_ => "application/octet-stream",
	};
	// file names of assets contain their hash
	let cache = if path.starts_with("_studio/") {
		"public, max-age=31536000, immutable"
	} else {
		"no-cache"
	};

	(
		[(header::CONTENT_TYPE, mime), (header::CACHE_CONTROL, cache)],
		file.data,
	)
		.into_response()
}

pub fn routes() -> Router<AppState> {
	Router::new().fallback(file)
}
