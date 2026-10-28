//! Texts of the frontend, like `menu.open`. The keys and the text of the
//! primary site come from the schema, editors translate them per site.
//! crelte's translations plugin reads them as the global `translations`.

use std::collections::BTreeMap;

use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::auth::Authed;
use crate::error::{Error, Result};
use crate::schema::Schema;

/// namespace → key → text
pub type Texts = BTreeMap<String, BTreeMap<String, String>>;

/// The texts of a site, the default of the schema where nothing is saved.
pub fn texts(conn: &Connection, schema: &Schema, site: &str) -> Result<Texts> {
	let mut saved: BTreeMap<(String, String), String> = BTreeMap::new();
	let mut stmt = conn.prepare(
		"SELECT namespace, key, value FROM translations WHERE site = ?1",
	)?;
	let rows =
		stmt.query_map([site], |r| Ok(((r.get(0)?, r.get(1)?), r.get(2)?)))?;
	for r in rows {
		let (k, v) = r?;
		saved.insert(k, v);
	}

	let mut out = Texts::new();
	for g in &schema.translations {
		let ns = out.entry(g.namespace.clone()).or_default();
		for (key, default) in &g.keys {
			let value = saved
				.get(&(g.namespace.clone(), key.clone()))
				.filter(|v| !v.is_empty())
				.unwrap_or(default);
			ns.insert(key.clone(), value.clone());
		}
	}
	Ok(out)
}

/// Saves one text, an empty one goes back to the default.
pub fn set(
	conn: &Connection,
	schema: &Schema,
	site: &str,
	namespace: &str,
	key: &str,
	value: &str,
) -> Result<()> {
	let known = schema.translations.iter().any(|g| {
		g.namespace == namespace && g.keys.iter().any(|(k, _)| k == key)
	});
	if !known || schema.get_site(site).is_none() {
		return Err(Error::Request(format!("unknown text {namespace}.{key}")));
	}
	conn.execute(
		"INSERT INTO translations (namespace, key, site, value)
		VALUES (?1, ?2, ?3, ?4)
		ON CONFLICT (namespace, key, site) DO UPDATE SET value = ?4",
		params![namespace, key, site, value],
	)?;
	Ok(())
}

#[derive(Debug, Serialize)]
struct AllResp {
	/// site → namespace → key → saved text (without defaults)
	saved: BTreeMap<String, Texts>,
}

/// The saved texts of every site, the studio shows the defaults itself.
async fn all(
	State(state): State<AppState>,
	_auth: Authed,
) -> Result<Json<AllResp>> {
	let conn = state.db.conn();
	let mut stmt =
		conn.prepare("SELECT site, namespace, key, value FROM translations")?;
	let rows = stmt.query_map([], |r| {
		Ok((r.get::<_, String>(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
	})?;
	let mut saved: BTreeMap<String, Texts> = BTreeMap::new();
	for r in rows {
		let (site, ns, key, value): (String, String, String, String) = r?;
		saved
			.entry(site)
			.or_default()
			.entry(ns)
			.or_default()
			.insert(key, value);
	}
	Ok(Json(AllResp { saved }))
}

#[derive(Debug, Deserialize)]
struct SetReq {
	site: String,
	namespace: String,
	key: String,
	value: String,
}

async fn put(
	State(state): State<AppState>,
	_auth: Authed,
	Json(req): Json<SetReq>,
) -> Result<()> {
	let conn = state.db.conn();
	set(
		&conn,
		&state.schema,
		&req.site,
		&req.namespace,
		&req.key,
		&req.value,
	)
}

#[derive(Debug, Deserialize)]
struct SiteParam {
	site: Option<String>,
}

/// The texts of a site with defaults, like the frontend gets them.
async fn site_texts(
	State(state): State<AppState>,
	_auth: Authed,
	Query(p): Query<SiteParam>,
) -> Result<Json<Texts>> {
	let site = p
		.site
		.unwrap_or_else(|| state.schema.primary_site().handle.clone());
	Ok(Json(texts(&state.db.conn(), &state.schema, &site)?))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/translations", get(all).put(put))
		.route("/translations/site", get(site_texts))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::db::Db;
	use crate::schema::tests::site;

	#[test]
	fn defaults_until_translated() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site().translations("common", [("menu.open", "Menü öffnen")]);

		assert_eq!(
			texts(&conn, &s, "en").unwrap()["common"]["menu.open"],
			"Menü öffnen"
		);
		set(&conn, &s, "en", "common", "menu.open", "Open menu").unwrap();
		assert_eq!(
			texts(&conn, &s, "en").unwrap()["common"]["menu.open"],
			"Open menu"
		);
		assert_eq!(
			texts(&conn, &s, "de").unwrap()["common"]["menu.open"],
			"Menü öffnen"
		);

		// empty is the default again, unknown keys are refused
		set(&conn, &s, "en", "common", "menu.open", "").unwrap();
		assert_eq!(
			texts(&conn, &s, "en").unwrap()["common"]["menu.open"],
			"Menü öffnen"
		);
		assert!(set(&conn, &s, "en", "common", "menu.close", "x").is_err());
	}
}
