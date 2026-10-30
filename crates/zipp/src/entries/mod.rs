//! Entries: the content of a site. An entry lives in a section and has a
//! row per site with its title, slug, uri and fields as JSON. On every
//! save the indexed fields and all relations are copied to `entry_values`
//! and `entry_relations`, so they can be queried.

use std::collections::{BTreeMap, HashMap};

use axum::extract::State;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::auth::Authed;
use crate::content::{
	FieldError, Target, index_values, relations, share, validate,
};
use crate::db::now;
use crate::drafts::record_revision;
use crate::error::{Error, Result};
use crate::query::Query;
use crate::schema::Access;
use crate::schema::{Schema, Section, SectionKind};
use crate::{AppState, ChangeKind};

pub mod api;
#[cfg(test)]
mod api_tests;
pub mod query;
mod routes;
mod save;
mod structure;
#[cfg(test)]
pub(crate) mod tests;

pub use routes::routes;
pub use save::check;
pub use save::reindex;
pub use save::save;
pub use save::write;
pub use structure::move_entry;
pub use structure::refresh_uris;
use structure::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub site: String,
	pub title: String,
	pub slug: String,
	/// `None` if the section has no urls, `""` is the home page
	pub uri: Option<String>,
	pub published: bool,
	pub parent_id: Option<i64>,
	pub fields: Value,
	pub created_at: i64,
	pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryInput {
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	/// default the primary site
	#[serde(default)]
	pub site: Option<String>,
	pub title: String,
	#[serde(default)]
	pub slug: String,
	#[serde(default)]
	pub published: bool,
	/// only when creating an entry of a structure, later it moves
	#[serde(default)]
	pub parent_id: Option<i64>,
	#[serde(default)]
	pub fields: Value,
}

/// What a relation id points to, sent next to the entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntryRef {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	pub slug: String,
	pub uri: Option<String>,
	/// in the site
	pub published: bool,
}

/// An [`EntryRef`] of `e.id, e.section, e.type, s.title, s.slug, s.uri,
/// s.published`.
fn entry_ref(r: &rusqlite::Row) -> rusqlite::Result<EntryRef> {
	Ok(EntryRef {
		id: r.get(0)?,
		section: r.get(1)?,
		ty: r.get(2)?,
		title: r.get(3)?,
		slug: r.get(4)?,
		uri: r.get(5)?,
		published: r.get(6)?,
	})
}

const COLUMNS: &str = "e.id, e.section, e.type, s.site, s.title, s.slug, \
	s.uri, s.published, e.parent_id, s.fields, e.created_at, s.updated_at";

fn from_row(row: &Row) -> rusqlite::Result<Entry> {
	let fields: String = row.get(9)?;
	Ok(Entry {
		id: row.get(0)?,
		section: row.get(1)?,
		ty: row.get(2)?,
		site: row.get(3)?,
		title: row.get(4)?,
		slug: row.get(5)?,
		uri: row.get(6)?,
		published: row.get(7)?,
		parent_id: row.get(8)?,
		fields: serde_json::from_str(&fields).unwrap_or_default(),
		created_at: row.get(10)?,
		updated_at: row.get(11)?,
	})
}

pub fn get_entry(conn: &Connection, id: i64, site: &str) -> Result<Entry> {
	let sql = format!(
		"SELECT {COLUMNS} FROM entries e
		JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?2
		WHERE e.id = ?1"
	);
	Ok(conn.query_row(&sql, params![id, site], from_row)?)
}

pub fn list(conn: &Connection, q: &Query) -> Result<(Vec<Entry>, i64)> {
	let (from, params) = q.from_where();
	let with = q.with();

	let total = conn.query_row(
		&format!("{with}SELECT COUNT(*) {from}"),
		params_from_iter(&params),
		|r| r.get(0),
	)?;

	let sql = format!(
		"{with}SELECT {COLUMNS} {from} {} LIMIT {} OFFSET {}",
		q.order_by(),
		q.limit,
		q.offset
	);
	let mut stmt = conn.prepare(&sql)?;
	let entries = stmt
		.query_map(params_from_iter(&params), from_row)?
		.collect::<rusqlite::Result<_>>()?;

	Ok((entries, total))
}

/// A save that passed all checks, ready to be written. Publishing a draft
/// checks all its sites first and then writes them together.
pub struct Checked {
	id: Option<i64>,
	site: String,
	section: Section,
	ty: String,
	published: bool,
	parent_id: Option<i64>,
	title: String,
	slug: String,
	/// per site, only the saved site for an update
	uris: HashMap<String, Option<String>>,
	fields: Value,
}

/// Deletes an entry in all sites, in a structure also its children.
pub fn delete(conn: &Connection, schema: &Schema, id: i64) -> Result<()> {
	let section: String = conn.query_row(
		"SELECT section FROM entries WHERE id = ?1",
		[id],
		|r| r.get(0),
	)?;
	let single = schema
		.get_section(&section)
		.is_some_and(|s| s.kind == SectionKind::Single);
	if single {
		return Err(Error::Request("the entry of a single stays".into()));
	}

	conn.execute("DELETE FROM entries WHERE id = ?1", [id])?;
	Ok(())
}

/// The entries the given ones link to, in a site.
pub fn refs(
	conn: &Connection,
	ids: &[i64],
	site: &str,
	published_only: bool,
) -> Result<BTreeMap<i64, EntryRef>> {
	let marks = vec!["?"; ids.len()].join(", ");
	let sql = format!(
		"SELECT DISTINCT e.id, e.section, e.type, s.title, s.slug, s.uri,
			s.published
		FROM entry_relations r
		JOIN entries e ON e.id = r.target_id
		JOIN entry_sites s ON s.entry_id = e.id AND s.site = r.site
		WHERE r.site = ? AND r.entry_id IN ({marks}) {}",
		if published_only {
			"AND s.published = 1"
		} else {
			""
		}
	);
	let mut stmt = conn.prepare(&sql)?;
	let mut params = vec![rusqlite::types::Value::Text(site.into())];
	params.extend(ids.iter().map(|i| rusqlite::types::Value::Integer(*i)));
	let rows = stmt.query_map(params_from_iter(params), entry_ref)?;

	let mut out = BTreeMap::new();
	for r in rows {
		let r = r?;
		out.insert(r.id, r);
	}
	Ok(out)
}

/// The entries of these ids as they show in a site, for chips.
pub fn refs_of(
	conn: &Connection,
	ids: &[i64],
	site: &str,
) -> Result<BTreeMap<i64, EntryRef>> {
	let marks = vec!["?"; ids.len()].join(", ");
	let sql = format!(
		"SELECT e.id, e.section, e.type, s.title, s.slug, s.uri, s.published
		FROM entries e JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?
		WHERE e.id IN ({marks})"
	);
	let mut stmt = conn.prepare(&sql)?;
	let mut params = vec![rusqlite::types::Value::Text(site.into())];
	params.extend(ids.iter().map(|i| rusqlite::types::Value::Integer(*i)));
	let rows = stmt.query_map(params_from_iter(params), entry_ref)?;
	let mut out = BTreeMap::new();
	for r in rows {
		let r = r?;
		out.insert(r.id, r);
	}
	Ok(out)
}

/// Which of the entries have a draft.
pub fn with_drafts(conn: &Connection, ids: &[i64]) -> Result<Vec<i64>> {
	let marks = vec!["?"; ids.len()].join(", ");
	let sql =
		format!("SELECT entry_id FROM drafts WHERE entry_id IN ({marks})");
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query_map(params_from_iter(ids), |r| r.get(0))?;
	Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Whether the entries are published, per site.
pub fn published_per_site(
	conn: &Connection,
	ids: &[i64],
) -> Result<BTreeMap<i64, BTreeMap<String, bool>>> {
	let marks = vec!["?"; ids.len()].join(", ");
	let sql = format!(
		"SELECT entry_id, site, published FROM entry_sites
		WHERE entry_id IN ({marks})"
	);
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query_map(params_from_iter(ids), |r| {
		Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get(2)?))
	})?;

	let mut out: BTreeMap<i64, BTreeMap<String, bool>> = BTreeMap::new();
	for r in rows {
		let (id, site, published) = r?;
		out.entry(id).or_default().insert(site, published);
	}
	Ok(out)
}

pub fn section_of(conn: &Connection, id: i64) -> Result<String> {
	Ok(conn.query_row(
		"SELECT section FROM entries WHERE id = ?1",
		[id],
		|r| r.get(0),
	)?)
}

/// `Über uns!` becomes `über-uns`.
pub fn slugify(s: &str) -> String {
	let mut out = String::new();
	for c in s.chars().flat_map(char::to_lowercase) {
		if c.is_alphanumeric() {
			out.push(c);
		} else if !out.is_empty() && !out.ends_with('-') {
			out.push('-');
		}
	}
	out.trim_end_matches('-').into()
}
