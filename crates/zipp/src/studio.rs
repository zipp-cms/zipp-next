//! The studio, built from `studio/` of the repository into
//! `crates/zipp/studio` (`npm run build`), so a site needs no npm.

use axum::extract::{Path, State};
use axum::http::{StatusCode, Uri, header};
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{Json, Router};

use serde::Serialize;

use crate::AppState;
use crate::auth::Authed;
use crate::schema::Schema;

#[derive(rust_embed::Embed)]
#[folder = "studio/"]
struct Files;

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
	match state.schema.field_types.get(name) {
		Some(t) => (
			[
				(header::CONTENT_TYPE, "text/javascript"),
				(header::CACHE_CONTROL, "no-cache"),
			],
			t.studio().js,
		)
			.into_response(),
		None => StatusCode::NOT_FOUND.into_response(),
	}
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/_fields/{file}", get(field_module))
		.fallback(file)
}

/// A doc of the code, as html.
#[derive(Debug, Serialize)]
struct DocResp {
	handle: String,
	name: String,
	html: String,
}

#[derive(Debug, Serialize)]
struct AiModel {
	id: String,
	name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SchemaResp {
	#[serde(flatten)]
	schema: Schema,
	/// for the preview button of the studio
	site_urls: crate::graphql::SiteUrls,
	/// the models of the AI chat, none if it is off
	ai_models: Vec<AiModel>,
	docs: Vec<DocResp>,
	/// the roles, the studio shows what the user's may do
	roles: Vec<crate::schema::Role>,
	/// sites not on the website yet
	offline: Vec<String>,
	/// SEO defaults of sections and entry types, shown in the setup
	seo: Option<crate::schema::seo::Seo>,
	me: Me,
}

/// The logged in user, `role: None` is an admin.
#[derive(Debug, Serialize)]
struct Me {
	id: i64,
	email: String,
	role: Option<String>,
}

async fn schema_route(
	State(state): State<AppState>,
	auth: Authed,
) -> crate::error::Result<Json<SchemaResp>> {
	let email = state.db.conn().query_row(
		"SELECT email FROM users WHERE id = ?1",
		[auth.user_id],
		|r| r.get(0),
	)?;
	Ok(Json(SchemaResp {
		roles: state.schema.roles.clone(),
		offline: state.schema.offline.clone(),
		seo: state.schema.seo.clone(),
		me: Me {
			id: auth.user_id,
			email,
			role: auth.role.clone(),
		},
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
				html: crate::graphql::markdown_html(&d.markdown),
			})
			.collect(),
	}))
}

/// What the studio loads first: the schema and its settings.
pub fn api_routes() -> Router<AppState> {
	Router::new().route("/schema", get(schema_route))
}
