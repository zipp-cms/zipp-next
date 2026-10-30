//! Drafts, revisions and previews.
//!
//! An entry has at most one draft, shared by everyone who edits it: the
//! studio saves into it while typing, the website keeps showing the live
//! entry until the draft is published. A draft holds every site, like a
//! revision. Publishing checks all sites first and writes them in one
//! transaction, so a later release (several drafts published together)
//! only publishes more drafts in the same transaction.
//!
//! A preview token shows the draft of one entry on the website: crelte
//! passes `?token=` on to GraphQL, which then answers with the draft. A
//! token of the whole site shows every draft, also of entries that only
//! a draft publishes.

use std::collections::BTreeMap;

use axum::extract::State;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AppState;
use crate::content::{FieldError, share};
use crate::db::now;
use crate::entries::{Dates, EntryInput, check, get_entry, write};
use crate::error::{Error, Result};
use crate::schema::Schema;

pub mod api;
#[cfg(test)]
mod api_tests;
mod preview;
mod revisions;
mod routes;
#[cfg(test)]
pub(crate) mod tests;

pub use preview::Preview;
pub use preview::create_site_token;
pub use preview::create_token;
pub use preview::preview;
pub use revisions::FieldChange;
pub use revisions::Revision;
pub use revisions::diff_docs;
pub use revisions::get_revision;
pub use revisions::record_revision;
pub use revisions::restore;
pub use revisions::revisions;
pub use routes::routes;

/// How long a preview link works.
const TOKEN_DURATION_MS: i64 = 1000 * 60 * 60 * 24;

/// Older revisions are deleted.
const MAX_REVISIONS: i64 = 50;

/// The content of an entry in one site.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SiteDoc {
	pub title: String,
	pub slug: String,
	pub published: bool,
	pub fields: Value,
	/// the same in every site, like shared fields
	#[serde(flatten, default)]
	pub dates: Dates,
}

/// The content of an entry in every site, by site handle.
pub type Doc = BTreeMap<String, SiteDoc>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Draft {
	pub doc: Doc,
	#[serde(skip)]
	pub user_id: Option<i64>,
	/// email of who changed it last
	pub user: Option<String>,
	pub updated_at: i64,
}

/// The entries a draft links to in a site, also those only the draft
/// has: the live relations don't know them.
pub fn draft_refs(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	draft: &Draft,
	site: &str,
) -> Result<BTreeMap<i64, crate::entries::EntryRef>> {
	let ty: String =
		conn.query_row("SELECT type FROM entries WHERE id = ?1", [id], |r| {
			r.get(0)
		})?;
	let Some(comp) = schema.entry_type(&ty) else {
		return Ok(BTreeMap::new());
	};
	let mut ids = vec![];
	for doc in draft.doc.values() {
		for r in crate::content::relations(schema, comp, &doc.fields) {
			if let crate::content::Target::Entries(_) = r.target {
				ids.extend(r.ids);
			}
		}
	}
	ids.sort_unstable();
	ids.dedup();
	crate::entries::refs_of(conn, &ids, site)
}

/// The live content of an entry.
pub fn live_doc(conn: &Connection, id: i64) -> Result<Doc> {
	let mut stmt = conn.prepare(
		"SELECT s.site, s.title, s.slug, s.published, s.fields, e.post_date,
		e.expiry_date FROM entry_sites s JOIN entries e ON e.id = s.entry_id
		WHERE s.entry_id = ?1",
	)?;
	let rows = stmt.query_map([id], |r| {
		let fields: String = r.get(4)?;
		Ok((
			r.get::<_, String>(0)?,
			SiteDoc {
				title: r.get(1)?,
				slug: r.get(2)?,
				published: r.get(3)?,
				fields: serde_json::from_str(&fields).unwrap_or_default(),
				dates: Dates {
					post_date: r.get(5)?,
					expiry_date: r.get(6)?,
				},
			},
		))
	})?;
	let doc: Doc = rows.collect::<rusqlite::Result<_>>()?;
	if doc.is_empty() {
		return Err(Error::NotFound);
	}
	Ok(doc)
}

/// The content as the editors see it: the draft, the live content if
/// there is none, and whether there is a draft.
pub fn draft_or_live(conn: &Connection, id: i64) -> Result<(Doc, bool)> {
	Ok(match get_draft(conn, id)? {
		Some(d) => (d.doc, true),
		None => (live_doc(conn, id)?, false),
	})
}

pub fn get_draft(conn: &Connection, id: i64) -> Result<Option<Draft>> {
	let row: Option<(String, Option<i64>, Option<String>, i64)> = conn
		.query_row(
			"SELECT d.doc, d.user_id, u.email, d.updated_at FROM drafts d
			LEFT JOIN users u ON u.id = d.user_id WHERE d.entry_id = ?1",
			[id],
			|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
		)
		.optional()?;
	Ok(row.map(|(doc, user_id, user, updated_at)| Draft {
		doc: serde_json::from_str(&doc).unwrap_or_default(),
		user_id,
		user,
		updated_at,
	}))
}

fn store_draft(
	conn: &Connection,
	id: i64,
	doc: &Doc,
	user_id: Option<i64>,
) -> Result<()> {
	conn.execute(
		"INSERT INTO drafts (entry_id, doc, user_id, updated_at)
		VALUES (?1, ?2, ?3, ?4)
		ON CONFLICT (entry_id) DO UPDATE SET doc = ?2, user_id = ?3,
		updated_at = ?4",
		params![id, serde_json::to_string(doc).unwrap(), user_id, now()],
	)?;
	Ok(())
}

/// Changes one site of the draft, it starts as a copy of the live entry.
/// Shared fields go to the other sites of the draft too. Nothing is
/// validated here, a draft may be half done.
///
/// `base` is the version of the draft the editor started from (its
/// `updated_at`, `None` without a draft). If someone else changed the
/// draft since, or published or discarded it, nothing is saved: the
/// editor sees their changes first instead of overwriting them.
pub fn put_draft(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	site: &str,
	content: SiteDoc,
	base: Option<i64>,
	user_id: Option<i64>,
) -> Result<Draft> {
	if schema.get_site(site).is_none() {
		return Err(Error::Request(format!("unknown site {site}")));
	}
	let entry = get_entry(conn, id, site)?;
	let comp = schema.entry_type(&entry.ty).ok_or_else(|| {
		Error::Internal(format!("{} is no type of the schema", entry.ty))
	})?;

	let current = get_draft(conn, id)?;
	match (&current, base) {
		(Some(d), _) if Some(d.updated_at) != base && d.user_id != user_id => {
			let who = d.user.clone().unwrap_or_else(|| "someone".into());
			return Err(Error::Conflict(format!("{who} changed this entry")));
		}
		(None, Some(_)) => {
			return Err(Error::Conflict(
				"the draft was published or discarded".into(),
			));
		}
		_ => {}
	}

	let mut doc = match current {
		Some(d) => d.doc,
		None => live_doc(conn, id)?,
	};
	for (_, sd) in doc.iter_mut().filter(|(s, _)| s.as_str() != site) {
		sd.fields = share(comp, &content.fields, &sd.fields);
		sd.dates = content.dates;
	}
	doc.insert(site.into(), content);

	store_draft(conn, id, &doc, user_id)?;
	Ok(get_draft(conn, id)?.unwrap())
}

/// Applies the draft to the live entry in every site and keeps a
/// revision. Errors of a site that isn't the primary one name the site.
pub fn publish(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	user_id: Option<i64>,
) -> Result<()> {
	let draft = get_draft(conn, id)?
		.ok_or_else(|| Error::Request("there is no draft".into()))?;
	let primary = &schema.primary_site().handle;
	let entry = get_entry(conn, id, primary)?;

	// the primary site last, its shared fields win
	let mut sites: Vec<&String> = draft.doc.keys().collect();
	sites.sort_by_key(|s| *s == primary);

	let mut checked = vec![];
	for site in sites {
		let sd = &draft.doc[site];
		let input = EntryInput {
			section: entry.section.clone(),
			ty: entry.ty.clone(),
			site: Some(site.clone()),
			title: sd.title.clone(),
			slug: sd.slug.clone(),
			published: sd.published,
			dates: sd.dates,
			parent_id: None,
			fields: sd.fields.clone(),
		};
		match check(conn, schema, Some(id), input) {
			Ok(c) => checked.push(c),
			Err(Error::Invalid(errors)) if site != primary => {
				let errors = errors
					.into_iter()
					.map(|e| FieldError {
						message: format!("{site}: {}", e.message),
						path: e.path,
					})
					.collect();
				return Err(Error::Invalid(errors));
			}
			Err(e) => return Err(e),
		}
	}

	let tx = conn.unchecked_transaction()?;
	for c in &checked {
		write(&tx, schema, c)?;
	}
	tx.execute("DELETE FROM drafts WHERE entry_id = ?1", [id])?;
	record_revision(&tx, id, user_id)?;
	tx.commit()?;
	Ok(())
}

pub fn discard(conn: &Connection, id: i64) -> Result<()> {
	conn.execute("DELETE FROM drafts WHERE entry_id = ?1", [id])?;
	Ok(())
}
