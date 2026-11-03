//! Serving the files and the transforms of assets, see the module docs of
//! [`super`]. A transform is generated on its first request and kept.

use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, header};
use axum::response::Response;
use axum::routing::get;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use super::*;
use crate::AppState;

pub(super) fn transform_by_name(
	state: &AppState,
	name: &str,
) -> Option<Transform> {
	match name {
		"_thumb" => Some(Transform::new("_thumb").width(400).height(400)),
		_ => state.schema.transform_named(name),
	}
}

#[derive(Debug, Deserialize)]
pub(super) struct OriginalParam {
	/// the file as it was uploaded, without the edits
	original: Option<String>,
}

pub(super) async fn serve_original(
	State(state): State<AppState>,
	Path((id, filename)): Path<(i64, String)>,
	Query(p): Query<OriginalParam>,
	req: axum::extract::Request,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	if asset.filename != filename {
		return Err(Error::NotFound);
	}
	// the edited image is a transform of its own size
	if asset.edits.is_some() && asset.kind == "image" && p.original.is_none() {
		let t = Transform::new("_edited");
		return transformed(&state, asset, "_edited", t, None, req).await;
	}
	let path = asset_dir(&state.files, id).join(&asset.filename);
	serve_file(path, &asset.mime, req).await
}

/// A file from the disk with ranges, a video plays and seeks without
/// loading the whole file, and its date: a cache asks again after a
/// while, a deleted file goes from it.
async fn serve_file(
	path: PathBuf,
	mime: &str,
	req: axum::extract::Request,
) -> Result<Response> {
	let parsed = mime.parse().map_err(|_| Error::NotFound)?;
	let resp = ServeFile::new_with_mime(path, &parsed)
		.oneshot(req)
		.await
		.map_err(|e| Error::Internal(e.to_string()))?;
	if resp.status() == StatusCode::NOT_FOUND {
		return Err(Error::NotFound);
	}
	let (mut parts, body) = resp.into_parts();
	for (name, value) in file_headers(mime) {
		parts.headers.insert(name, value.parse().unwrap());
	}
	Ok(Response::from_parts(parts, axum::body::Body::new(body)))
}

pub(super) async fn serve_transform(
	State(state): State<AppState>,
	Path((id, name, filename)): Path<(i64, String, String)>,
	req: axum::extract::Request,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	let t = transform_by_name(&state, &name).ok_or(Error::NotFound)?;
	if asset.kind != "image" {
		return Err(Error::NotFound);
	}
	// the image's name, or its name with the extension of another type
	let format = match filename == asset.filename {
		true => None,
		false => Some(
			Format::ALL
				.into_iter()
				.find(|f| asset.format_filename(*f) == filename)
				.ok_or(Error::NotFound)?,
		),
	};
	transformed(&state, asset, &name, t, format, req).await
}

/// A transform of an image, generated on the first request, in `format`
/// or the type of the image.
async fn transformed(
	state: &AppState,
	mut asset: Asset,
	name: &str,
	t: Transform,
	format: Option<Format>,
	req: axum::extract::Request,
) -> Result<Response> {
	let mut out = transform_path(state, &asset, name, format);
	if !out.exists() {
		let _permit = GENERATING.acquire().await.unwrap();
		// as it is now, it may have changed while this one waited
		asset = get_asset(&state.db.conn(), asset.id)?;
		out = transform_path(state, &asset, name, format);
		// not made already while this one waited
		if !out.exists() {
			let original =
				asset_dir(&state.files, asset.id).join(&asset.filename);
			let mime = mime_of(&asset, format).to_string();
			let (a, o) = (asset.clone(), out.clone());
			tokio::task::spawn_blocking(move || {
				generate(&a, &original, &t, &mime, &o)
			})
			.await??;
		}
	}

	serve_file(out, mime_of(&asset, format), req).await
}

/// The type of a transform: `format`'s or the image's.
fn mime_of(asset: &Asset, format: Option<Format>) -> &str {
	match format {
		Some(f) => f.mime(),
		None => asset.output_mime(),
	}
}

/// Where a transform of an asset is kept, with its version: one made of
/// the image before an edit is never served after it.
fn transform_path(
	state: &AppState,
	asset: &Asset,
	name: &str,
	format: Option<Format>,
) -> PathBuf {
	let filename = match format {
		Some(f) => asset.format_filename(f),
		None => asset.filename.clone(),
	};
	let file = match asset.version.as_str() {
		"" => format!("{name}-{filename}"),
		v => format!("{name}-{v}-{filename}"),
	};
	asset_dir(&state.files, asset.id)
		.join(TRANSFORMS)
		.join(file)
}

/// Files are served from the studio's origin: nothing may run as a page
/// there. Images, video, audio and pdf show in the browser, everything
/// else (svg too) downloads. Caches keep them five minutes, then ask
/// with the date (`Last-Modified`): one in the trash isn't shown long.
pub(super) fn file_headers(mime: &str) -> [(header::HeaderName, String); 5] {
	let inline = (mime.starts_with("image/") && mime != "image/svg+xml")
		|| mime.starts_with("video/")
		|| mime.starts_with("audio/")
		|| mime == "application/pdf";
	[
		(header::CONTENT_TYPE, mime.to_string()),
		(header::CACHE_CONTROL, "public, max-age=300".into()),
		(header::X_CONTENT_TYPE_OPTIONS, "nosniff".into()),
		(header::CONTENT_SECURITY_POLICY, "sandbox".into()),
		(
			header::CONTENT_DISPOSITION,
			if inline { "inline" } else { "attachment" }.into(),
		),
	]
}

/// Transforms generated at once, each decodes a whole image.
pub(super) static GENERATING: tokio::sync::Semaphore =
	tokio::sync::Semaphore::const_new(2);

/// The files, under `/assets`.
pub(crate) fn file_routes() -> Router<AppState> {
	Router::new()
		.route("/assets/{id}/{filename}", get(serve_original))
		.route("/assets/{id}/{transform}/{filename}", get(serve_transform))
}
