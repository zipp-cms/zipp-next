use axum::extract::{Path, State};
use axum::http::{StatusCode, Uri, header};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{Json, Router};

use super::Files;
use super::api::*;
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::Authed;

async fn file(State(state): State<AppState>, uri: Uri) -> Response {
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

	let frames =
		format!("frame-ancestors 'self' {}", state.frame_ancestors.join(" "));
	(
		[
			(header::CONTENT_TYPE, mime),
			(header::CACHE_CONTROL, cache),
			// no other site may frame the studio and click for an editor
			(header::CONTENT_SECURITY_POLICY, frames.as_str()),
		],
		file.data,
	)
		.into_response()
}

/// The editor of a field type of another crate, see
/// [`StudioModule`](crate::field_type::StudioModule).
async fn field_module(
	State(state): State<AppState>,
	Path(file): Path<String>,
) -> Response {
	let name = file.strip_suffix(".js").unwrap_or(&file);
	module(state.schema.field_types.get(name).map(|t| t.studio().js))
}

/// The studio module of a plugin, see [`PluginStudio`](crate::PluginStudio).
async fn plugin_module(
	State(state): State<AppState>,
	Path(file): Path<String>,
) -> Response {
	let name = file.strip_suffix(".js").unwrap_or(&file);
	let studio = state.schema.plugins.get(name).and_then(|p| p.studio());
	module(studio.map(|s| s.js))
}

/// An ES module of another crate, asked again on every load: it changes
/// with the crate, not with a hash in its name.
fn module(js: Option<&'static str>) -> Response {
	let Some(js) = js else {
		return StatusCode::NOT_FOUND.into_response();
	};
	let headers = [
		(header::CONTENT_TYPE, "text/javascript"),
		(header::CACHE_CONTROL, "no-cache"),
	];
	(headers, js).into_response()
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/_fields/{file}", get(field_module))
		.route("/_plugins/{file}", get(plugin_module))
		.fallback(file)
}

async fn schema_route(
	State(state): State<AppState>,
	_auth: Authed,
	Req(_): Req<SchemaReq>,
) -> ApiResult<SchemaReq> {
	Ok(Json(SchemaResp {
		roles: state.schema.roles.clone(),
		offline: state.schema.offline.clone(),
		dev: state.dev,
		icons: (*state.icons).clone(),
		plugins: state
			.schema
			.plugins
			.0
			.iter()
			.filter_map(|p| {
				let s = p.studio()?;
				Some(PluginResp {
					name: p.name().into(),
					data: s.data,
				})
			})
			.collect(),
		schema: (*state.schema).clone(),
		site_urls: (*state.site_urls).clone(),
		ai_models: match state.ai {
			Some(_) => crate::ai::MODELS
				.iter()
				.map(|(id, name)| AiModel {
					id: id.to_string(),
					name: name.to_string(),
				})
				.collect(),
			None => vec![],
		},
		docs: state
			.schema
			.docs
			.iter()
			.map(|d| DocResp {
				handle: d.handle.clone(),
				name: d.name.clone(),
				html: crate::content::markdown_html(&d.markdown),
			})
			.collect(),
	}))
}

/// What the studio loads first: the schema and its settings.
pub fn api_routes() -> Router<AppState> {
	Router::new().api::<SchemaReq, _>(schema_route)
}
