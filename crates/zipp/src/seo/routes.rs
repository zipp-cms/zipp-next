//! What the studio's SEO field shows as placeholders.

use axum::extract::{Path, Query as Params, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::Value;

use super::sitemap::load_globals;
use super::{EntryPage, Page, defaults_for_entry};
use crate::AppState;
use crate::assets::get_asset;
use crate::auth::Authed;
use crate::drafts::draft_or_live;
use crate::entries::get_entry;
use crate::error::{Error, Result};
use crate::schema::Access;

#[derive(Debug, Deserialize)]
struct SiteParam {
	site: Option<String>,
}

/// The defaults of an entry as the editor sees it, its draft first.
async fn defaults_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	Params(p): Params<SiteParam>,
) -> Result<Json<Value>> {
	let schema = &state.schema;
	let site = p
		.site
		.unwrap_or_else(|| schema.primary_site().handle.clone());
	let site_def = schema
		.get_site(&site)
		.ok_or_else(|| Error::Request(format!("unknown site {site}")))?;
	let conn = state.db.conn();
	let mut entry = get_entry(&conn, id, &site)?;
	auth.check(schema, &entry.section, Access::View)?;
	if let (doc, true) = draft_or_live(&conn, id)?
		&& let Some(sd) = doc.get(&site)
	{
		entry.title = sd.title.clone();
		entry.fields = sd.fields.clone();
	}
	let globals = load_globals(&conn, schema, site_def)?;

	let url = entry
		.uri
		.as_deref()
		.and_then(|u| crate::page_url(&state.site_urls, &site, u));
	let e = EntryPage {
		entry: &entry,
		url,
		alternates: vec![],
		breadcrumbs: vec![],
	};
	let asset = |id: i64| get_asset(&conn, id).ok();
	let page = Page {
		schema,
		site: site_def,
		site_url: state.site_urls.get(&site).cloned().unwrap_or_default(),
		public_url: "",
		globals,
		entry: Some(e),
		asset: &asset,
		indexing: state.indexing,
	};
	Ok(Json(defaults_for_entry(&page).unwrap_or_default()))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/entries/{id}/seo", get(defaults_route))
}
