//! The site's own code next to zipp's: a route and a hook.

use zipp::axum::Router;
use zipp::axum::extract::State;
use zipp::axum::http::header;
use zipp::axum::response::IntoResponse;
use zipp::axum::routing::get;
use zipp::entries::query::Query;
use zipp::error::Result;
use zipp::{AppState, Change};

/// The routes of the site, they get zipp's AppState.
pub fn routes() -> Router<AppState> {
	Router::new().route("/events.json", get(events))
}

/// After every change of an entry, e.g. to clear a cache.
pub fn on_change(c: &Change) {
	println!("{:?} {} in {}", c.kind, c.entry_id, c.section);
}

/// A route of the site next to zipp's: the upcoming events as JSON, for a
/// widget on another site.
async fn events(State(state): State<AppState>) -> Result<impl IntoResponse> {
	let conn = state.db.conn();
	// a date is indexed as text, `2026-11-14` sorts and compares as one
	let today = &zipp::db::iso(zipp::db::now())[..10];
	let params = [
		("site", "de"),
		("type", "event"),
		("order", "date"),
		("f.date.gte", today),
	];
	let q = Query::published(&state.schema, &params).unwrap();
	let (entries, _) = zipp::entries::list(&conn, &q)?;
	let events: Vec<_> = entries
		.into_iter()
		.map(|e| {
			let url = e
				.uri
				.as_deref()
				.and_then(|u| zipp::page_url(&state.site_urls, &e.site, u));
			serde_json::json!({
				"title": e.title,
				"date": e.fields["date"],
				"city": e.fields["city"],
				"url": url,
			})
		})
		.collect();
	Ok((
		[(header::CONTENT_TYPE, "application/json")],
		serde_json::to_string(&events).unwrap(),
	))
}
