//! What the studio's SEO field shows as placeholders.

use zipp::AppState;
use zipp::api::{ApiResult, ApiRouter, Req};
use zipp::assets::get_asset;
use zipp::auth::{EntryAccess, View};
use zipp::axum::extract::State;
use zipp::axum::{Json, Router};
use zipp::drafts::draft_or_live;
use zipp::entries::get_entry;
use zipp::error::Error;

use crate::api::DefaultsReq;
use crate::resolve::{EntryPage, Page, defaults_for_entry};
use crate::sitemap::load_globals;

async fn defaults_route(
	State(state): State<AppState>,
	_access: EntryAccess<View>,
	Req(req): Req<DefaultsReq>,
) -> ApiResult<DefaultsReq> {
	let (schema, id) = (&state.schema, req.id);
	let site = req
		.site
		.unwrap_or_else(|| schema.primary_site().handle.clone());
	let site_def = schema
		.get_site(&site)
		.ok_or_else(|| Error::Request(format!("unknown site {site}")))?;
	let conn = state.db.conn();
	let mut entry = get_entry(&conn, id, &site)?;
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
		.and_then(|u| zipp::page_url(&state.site_urls, &site, u));
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
	Router::new().api::<DefaultsReq, _>(defaults_route)
}
