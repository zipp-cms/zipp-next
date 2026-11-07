//! Entries: the content of a site. An entry lives in a section and has a
//! row per site with its title, slug, uri and fields as JSON. On every
//! save the indexed fields and all relations are copied to `entry_values`
//! and `entry_relations`, so they can be queried.

use std::collections::{BTreeMap, HashMap, HashSet};

use axum::extract::State;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::auth::Authed;
use crate::content::{
	FieldError, Target, drop_links, follow, index_values, relations, validate,
	with_defaults,
};
use crate::db::{marks, now};
use crate::drafts::{keep_live, record_revision};
use crate::entries::query::{OwnOnly, Query};
use crate::error::{Error, Result};
use crate::schema::Access;
use crate::schema::{Component, LinkKind, Schema, Section, SectionKind};
use crate::{AppState, ChangeKind};

pub mod api;
#[cfg(test)]
mod api_tests;
mod as_user;
pub mod query;
mod routes;
mod save;
mod structure;
#[cfg(test)]
pub(crate) mod tests;

pub(crate) use as_user::restrict;
pub use as_user::{
	Saved, WithDraft, create_as, delete_as, get_as, list_as, update_as,
};
pub(crate) use routes::routes;
pub(crate) use routes::viewable_refs;
pub(crate) use save::check;
pub(crate) use save::reindex;
pub(crate) use save::reindex_all;
pub use save::save;
pub(crate) use save::write;
pub(crate) use save::{check_relations, links_of};
pub(crate) use structure::move_entry;
pub(crate) use structure::refresh_uris;
use structure::*;
pub(crate) use structure::{fits_below, free_uri, levels_of, next_position};

/// The tables that are built from an entry's fields, per entry and site:
/// whatever is removed or rewritten has to clear them all.
pub(crate) const INDEX_TABLES: [&str; 4] = [
	"entry_values",
	"entry_relations",
	"asset_relations",
	"entry_texts",
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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
	#[serde(flatten)]
	pub dates: Dates,
	/// the user who made it, like Craft's author; `None`: the system
	/// (the command line, an import) or a deleted user
	pub author_id: Option<i64>,
}

/// When an entry is live, in all its sites, like Craft's post and expiry
/// date: from `post_date` (set when it is first published) until
/// `expiry_date`, milliseconds. Published in a site and in these dates
/// is live there, see `LIVE_DATES`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Dates {
	#[serde(default)]
	pub post_date: Option<i64>,
	#[serde(default)]
	pub expiry_date: Option<i64>,
}

/// The SQL of an entry `e` whose dates make it live now.
/// Now in whole milliseconds like `db::now` (`strftime('%s')` has
/// seconds, julianday's float rounds below the millisecond it is).
pub(crate) const LIVE_DATES: &str = "(e.post_date IS NULL \
	OR e.post_date <= ROUND((julianday('now') - 2440587.5) * 86400000)) \
	AND (e.expiry_date IS NULL \
	OR e.expiry_date > ROUND((julianday('now') - 2440587.5) * 86400000))";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
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
	/// no post date: the one it has, or now when it is first published
	#[serde(flatten, default)]
	pub dates: Dates,
	/// only when creating an entry of a structure, later it moves
	#[serde(default)]
	pub parent_id: Option<i64>,
	#[serde(default)]
	pub fields: Value,
}

impl EntryInput {
	/// An unpublished entry of the primary site, the slug from the
	/// title.
	pub fn new(section: &str, ty: &str, title: &str, fields: Value) -> Self {
		Self {
			section: section.into(),
			ty: ty.into(),
			site: None,
			title: title.into(),
			slug: String::new(),
			published: false,
			dates: Dates::default(),
			parent_id: None,
			fields,
		}
	}
}

/// What a relation id points to, sent next to the entries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
	/// whether it is live now, for its status
	#[serde(flatten)]
	pub dates: Dates,
	/// who made it, see [`Entry::author_id`]
	pub author_id: Option<i64>,
}

/// An [`EntryRef`] of `e.id, e.section, e.type, s.title, s.slug, s.uri,
/// s.published, e.post_date, e.expiry_date, e.author_id`.
fn entry_ref(r: &rusqlite::Row) -> rusqlite::Result<EntryRef> {
	Ok(EntryRef {
		id: r.get(0)?,
		section: r.get(1)?,
		ty: r.get(2)?,
		title: r.get(3)?,
		slug: r.get(4)?,
		uri: r.get(5)?,
		published: r.get(6)?,
		dates: Dates {
			post_date: r.get(7)?,
			expiry_date: r.get(8)?,
		},
		author_id: r.get(9)?,
	})
}

pub(crate) const COLUMNS: &str = "e.id, e.section, e.type, s.site, s.title, s.slug, \
	s.uri, s.published, e.parent_id, s.fields, e.created_at, s.updated_at, \
	e.post_date, e.expiry_date, e.author_id";

pub(crate) fn from_row(row: &Row) -> rusqlite::Result<Entry> {
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
		dates: Dates {
			post_date: row.get(12)?,
			expiry_date: row.get(13)?,
		},
		author_id: row.get(14)?,
	})
}

/// An entry in a picker: an [`EntryRef`] with its parent for the tree,
/// its dates and whether it has unpublished changes, for its status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EntryChoice {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	pub slug: String,
	pub uri: Option<String>,
	/// in the site
	pub published: bool,
	pub parent_id: Option<i64>,
	#[serde(flatten)]
	pub dates: Dates,
	/// it has a draft
	pub draft: bool,
}

/// The most entries of a section a picker gets at once, and of a
/// search; the tests page with a few.
pub(crate) const CHOICES_LIMIT: i64 =
	if cfg!(test) { 3 } else { query::MAX_LIMIT };

/// How many entries of a section there are, sent or not: `entries` in
/// its order (a structure with its other types, as parents), `pickable`
/// of the types.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChoiceCount {
	pub entries: i64,
	pub pickable: i64,
}

/// The entries a picker shows and how many there are of each section.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Choices {
	pub entries: Vec<EntryChoice>,
	pub totals: BTreeMap<String, ChoiceCount>,
	/// in a search the parents of the found ones, for their paths
	pub parents: Vec<EntryChoice>,
}

/// The entries of some sections to choose from, each in its order: a
/// structure as its tree, so its other types are there too as the
/// parents; a channel newest first, only the types. At most
/// [`CHOICES_LIMIT`] per section from `offset`, a picker loads more.
/// Of the sections of `own_only` only the user's entries.
pub(crate) fn choices(
	conn: &Connection,
	schema: &Schema,
	types: &[&str],
	sections: &[&str],
	site: &str,
	offset: i64,
	own_only: Option<OwnOnly>,
) -> Result<Choices> {
	if schema.get_site(site).is_none() {
		return Err(Error::Request(format!("unknown site {site}")));
	}
	let mut out = Choices {
		entries: vec![],
		totals: BTreeMap::new(),
		parents: vec![],
	};
	for section in sections {
		let mut of_types = Query {
			site: site.into(),
			section: Some(section.to_string()),
			types: types.iter().map(|t| t.to_string()).collect(),
			limit: CHOICES_LIMIT,
			offset,
			own_only: own_only.clone(),
			..Query::new(schema)
		};
		of_types.section_order(schema);
		// a tree needs the other types as parents
		let mut q = of_types.clone();
		if schema
			.get_section(section)
			.is_some_and(|s| s.kind == SectionKind::Structure)
		{
			q.types = vec![];
		}

		let count = ChoiceCount {
			entries: count(conn, &q)?,
			pickable: count(conn, &of_types)?,
		};
		out.totals.insert(section.to_string(), count);
		out.entries.extend(to_choices(conn, select(conn, &q)?)?);
	}
	Ok(out)
}

/// The entries of the types in some sections that have all the words of
/// `search`, like the studio's search, the best first; at most
/// [`CHOICES_LIMIT`], with their parents.
pub(crate) fn search_choices(
	conn: &Connection,
	schema: &Schema,
	types: &[&str],
	sections: &[&str],
	site: &str,
	search: &str,
	own_only: Option<OwnOnly>,
) -> Result<Choices> {
	if schema.get_site(site).is_none() {
		return Err(Error::Request(format!("unknown site {site}")));
	}
	let q = Query {
		site: site.into(),
		types: types.iter().map(|t| t.to_string()).collect(),
		text: Some(search.into()),
		order: query::Order::Relevance,
		desc: false,
		limit: CHOICES_LIMIT,
		own_only: own_only.clone(),
		..Query::new(schema)
	};

	let mut totals = BTreeMap::new();
	for section in sections {
		let n = count(
			conn,
			&Query {
				section: Some(section.to_string()),
				..q.clone()
			},
		)?;
		let count = ChoiceCount {
			entries: n,
			pickable: n,
		};
		totals.insert(section.to_string(), count);
	}
	let found = select(
		conn,
		&Query {
			sections: Some(sections.iter().map(|s| s.to_string()).collect()),
			..q
		},
	)?;

	// the parents a level at a time, up to the top
	let mut parents = vec![];
	let mut known: HashSet<i64> = found.iter().map(|e| e.id).collect();
	let mut up: Vec<i64> = found.iter().filter_map(|e| e.parent_id).collect();
	loop {
		up.retain(|id| known.insert(*id));
		if up.is_empty() {
			break;
		}
		let q = Query {
			site: site.into(),
			ids: Some(std::mem::take(&mut up)),
			limit: query::MAX_LIMIT,
			own_only: own_only.clone(),
			..Query::new(schema)
		};
		let level = select(conn, &q)?;
		up = level.iter().filter_map(|e| e.parent_id).collect();
		parents.extend(level);
	}

	Ok(Choices {
		entries: to_choices(conn, found)?,
		totals,
		parents: to_choices(conn, parents)?,
	})
}

/// The entries as choices, with whether they have a draft.
fn to_choices(
	conn: &Connection,
	entries: Vec<Entry>,
) -> Result<Vec<EntryChoice>> {
	let ids: Vec<i64> = entries.iter().map(|e| e.id).collect();
	let drafts = with_drafts(conn, &ids)?;
	Ok(entries
		.into_iter()
		.map(|e| EntryChoice {
			draft: drafts.contains(&e.id),
			id: e.id,
			section: e.section,
			ty: e.ty,
			title: e.title,
			slug: e.slug,
			uri: e.uri,
			published: e.published,
			parent_id: e.parent_id,
			dates: e.dates,
		})
		.collect())
}

/// The fields of a single as published in a site, null if it isn't.
/// Plugins keep their settings in one.
pub fn single_fields(
	conn: &Connection,
	schema: &Schema,
	section: &str,
	site: &str,
) -> Result<Value> {
	let params = [("section", section), ("site", site), ("limit", "1")];
	let q = Query::published(schema, &params).map_err(Error::Request)?;
	let found = select(conn, &q)?;
	Ok(found
		.into_iter()
		.next()
		.map(|e| e.fields)
		.unwrap_or_default())
}

pub fn get_entry(conn: &Connection, id: i64, site: &str) -> Result<Entry> {
	let sql = format!(
		"SELECT {COLUMNS} FROM entries e
		JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?2
		WHERE e.id = ?1 AND e.deleted_at IS NULL"
	);
	Ok(conn.query_row(&sql, params![id, site], from_row)?)
}

/// The entry of a page of a site, `None` if no entry has the uri.
pub(crate) fn by_uri(
	conn: &Connection,
	site: &str,
	uri: &str,
) -> Result<Option<Entry>> {
	let sql = format!(
		"SELECT {COLUMNS} FROM entries e
		JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?1
		WHERE s.uri = ?2 AND e.deleted_at IS NULL"
	);
	Ok(conn
		.query_row(&sql, params![site, uri], from_row)
		.optional()?)
}

/// The entries of a query and how many there are without its limit.
pub fn list(conn: &Connection, q: &Query) -> Result<(Vec<Entry>, i64)> {
	Ok((select(conn, q)?, count(conn, q)?))
}

/// The entries of a query, [`list`] without the count.
pub(crate) fn select(conn: &Connection, q: &Query) -> Result<Vec<Entry>> {
	let (with, mut params) = q.with();
	let (from, more) = q.sql_from();
	params.extend(more);

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
	Ok(entries)
}

/// The children of the query's parents in the query's order, its limit
/// and offset for each parent.
pub(crate) fn children(conn: &Connection, q: &Query) -> Result<Vec<Entry>> {
	let (from, params) = q.sql_from();
	let order = q.order_by();
	let (first, last) = (q.offset, q.offset.saturating_add(q.limit));
	// the columns of the entries have different names
	let sql = format!(
		"SELECT * FROM (SELECT {COLUMNS}, ROW_NUMBER() OVER (
			PARTITION BY e.parent_id {order}) AS n {from})
		WHERE n > {first} AND n <= {last} ORDER BY parent_id, n"
	);
	let mut stmt = conn.prepare(&sql)?;
	let entries = stmt
		.query_map(params_from_iter(&params), from_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(entries)
}

/// How many entries a query finds without its limit.
pub(crate) fn count(conn: &Connection, q: &Query) -> Result<i64> {
	let (with, mut params) = q.with();
	let (from, more) = q.sql_from();
	params.extend(more);

	Ok(conn.query_row(
		&format!("{with}SELECT COUNT(*) {from}"),
		params_from_iter(&params),
		|r| r.get(0),
	)?)
}

/// A save that passed all checks, ready to be written. Publishing a draft
/// checks all its sites first and then writes them together.
pub(crate) struct Checked {
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
	dates: Dates,
	/// of a new entry
	author_id: Option<i64>,
}

/// The published entries whose post or expiry date passed in `(from,
/// to]`: they went live or expired then. With their section.
pub(crate) fn dated_between(
	conn: &Connection,
	from: i64,
	to: i64,
) -> Result<Vec<(i64, String)>> {
	let mut stmt = conn.prepare(
		"SELECT e.id, e.section FROM entries e
		WHERE ((e.post_date > ?1 AND e.post_date <= ?2)
			OR (e.expiry_date > ?1 AND e.expiry_date <= ?2))
		AND e.deleted_at IS NULL AND EXISTS (SELECT 1 FROM entry_sites s
			WHERE s.entry_id = e.id AND s.published = 1)",
	)?;
	let rows =
		stmt.query_map(params![from, to], |r| Ok((r.get(0)?, r.get(1)?)))?;
	Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Deletes an entry for good in all sites, in a structure also its
/// children; the studio moves it to the trash ([`crate::trash`]).
#[cfg(feature = "craft")]
pub(crate) fn delete(
	conn: &Connection,
	schema: &Schema,
	id: i64,
) -> Result<()> {
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
pub(crate) fn refs(
	conn: &Connection,
	ids: &[i64],
	site: &str,
	published_only: bool,
) -> Result<BTreeMap<i64, EntryRef>> {
	let sql = format!(
		"SELECT DISTINCT e.id, e.section, e.type, s.title, s.slug, s.uri,
			s.published, e.post_date, e.expiry_date, e.author_id
		FROM entry_relations r
		JOIN entries e ON e.id = r.target_id
		JOIN entry_sites s ON s.entry_id = e.id AND s.site = r.site
		WHERE r.site = ? AND e.deleted_at IS NULL AND r.entry_id IN ({}) {}",
		marks(ids.len()),
		if published_only {
			format!("AND s.published = 1 AND {LIVE_DATES}")
		} else {
			String::new()
		}
	);
	refs_by(conn, &sql, site, ids)
}

/// The entries of these ids as they show in a site, for chips.
pub(crate) fn refs_of(
	conn: &Connection,
	ids: &[i64],
	site: &str,
) -> Result<BTreeMap<i64, EntryRef>> {
	let sql = format!(
		"SELECT e.id, e.section, e.type, s.title, s.slug, s.uri, s.published,
			e.post_date, e.expiry_date, e.author_id
		FROM entries e JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?
		WHERE e.deleted_at IS NULL AND e.id IN ({})",
		marks(ids.len())
	);
	refs_by(conn, &sql, site, ids)
}

/// The refs of a query whose params are the site, then the ids.
fn refs_by(
	conn: &Connection,
	sql: &str,
	site: &str,
	ids: &[i64],
) -> Result<BTreeMap<i64, EntryRef>> {
	let mut stmt = conn.prepare(sql)?;
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
pub(crate) fn with_drafts(conn: &Connection, ids: &[i64]) -> Result<Vec<i64>> {
	let sql = format!(
		"SELECT entry_id FROM drafts WHERE entry_id IN ({})",
		marks(ids.len())
	);
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query_map(params_from_iter(ids), |r| r.get(0))?;
	Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// Whether the entries are published, per site.
pub(crate) fn published_per_site(
	conn: &Connection,
	ids: &[i64],
) -> Result<BTreeMap<i64, BTreeMap<String, bool>>> {
	let sql = format!(
		"SELECT entry_id, site, published FROM entry_sites
		WHERE entry_id IN ({})",
		marks(ids.len())
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

/// The public part of a user, who made an entry: no email.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Author {
	pub id: i64,
	/// empty when it isn't set
	pub name: String,
}

/// The authors of these ids, the ones that exist.
pub(crate) fn authors(
	conn: &Connection,
	ids: &[i64],
) -> Result<HashMap<i64, Author>> {
	let sql = format!(
		"SELECT id, name FROM users WHERE id IN ({})",
		marks(ids.len())
	);
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query_map(params_from_iter(ids), |r| {
		Ok(Author {
			id: r.get(0)?,
			name: r.get(1)?,
		})
	})?;
	let mut out = HashMap::new();
	for a in rows {
		let a = a?;
		out.insert(a.id, a);
	}
	Ok(out)
}

/// Gives an entry another author, `None` for none. Not a change of its
/// content: no revision, drafts keep theirs.
pub(crate) fn set_author(
	conn: &Connection,
	id: i64,
	author: Option<i64>,
) -> Result<()> {
	if let Some(a) = author
		&& authors(conn, &[a])?.is_empty()
	{
		return Err(Error::Request(format!("unknown user {a}")));
	}
	let n = conn.execute(
		"UPDATE entries SET author_id = ?1
		WHERE id = ?2 AND deleted_at IS NULL",
		params![author, id],
	)?;
	match n {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

/// The section of an entry and who made it, what its access depends on.
pub(crate) fn owner_of(
	conn: &Connection,
	id: i64,
) -> Result<(String, Option<i64>)> {
	Ok(conn.query_row(
		"SELECT section, author_id FROM entries
		WHERE id = ?1 AND deleted_at IS NULL",
		[id],
		|r| Ok((r.get(0)?, r.get(1)?)),
	)?)
}

pub(crate) fn section_of(conn: &Connection, id: i64) -> Result<String> {
	Ok(conn.query_row(
		"SELECT section FROM entries WHERE id = ?1 AND deleted_at IS NULL",
		[id],
		|r| r.get(0),
	)?)
}

/// The slug an entry is saved with: its own, else one of its title.
pub(crate) fn slug_of(slug: &str, title: &str) -> String {
	match slug.trim() {
		"" => slugify(title),
		s => slugify(s),
	}
}

/// `Über uns!` becomes `über-uns`.
pub(crate) fn slugify(s: &str) -> String {
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
