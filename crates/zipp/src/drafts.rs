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
//! passes `?token=` on to GraphQL, which then answers with the draft.

use std::collections::BTreeMap;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use rand::Rng;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AppState;
use crate::auth::Authed;
use crate::content::{FieldError, share};
use crate::db::now;
use crate::entries::{EntryInput, check, get_entry, write};
use crate::error::{Error, Result};
use crate::schema::Schema;

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
}

/// The content of an entry in every site, by site handle.
pub type Doc = BTreeMap<String, SiteDoc>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Draft {
	pub doc: Doc,
	#[serde(skip)]
	pub user_id: Option<i64>,
	/// email of who changed it last
	pub user: Option<String>,
	pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
	pub id: i64,
	pub user: Option<String>,
	pub created_at: i64,
	/// what changed since the revision before, empty for the first
	pub changes: Vec<Change>,
}

/// A field (or title, slug, published) that changed, and in which sites.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Change {
	pub field: String,
	pub sites: Vec<String>,
}

/// What differs between two versions of an entry, in the order of the
/// fields of the newer one.
pub fn diff_docs(old: &Doc, new: &Doc) -> Vec<Change> {
	let mut changes: Vec<Change> = vec![];
	let mut add = |field: &str, site: &str| match changes
		.iter_mut()
		.find(|c| c.field == field)
	{
		Some(c) => c.sites.push(site.into()),
		None => changes.push(Change {
			field: field.into(),
			sites: vec![site.into()],
		}),
	};

	for (site, n) in new {
		let Some(o) = old.get(site) else { continue };
		if o.title != n.title {
			add("title", site);
		}
		if o.slug != n.slug {
			add("slug", site);
		}
		if o.published != n.published {
			add("published", site);
		}
		let empty = serde_json::Map::new();
		let of = o.fields.as_object().unwrap_or(&empty);
		let nf = n.fields.as_object().unwrap_or(&empty);
		let keys = nf.keys().chain(of.keys().filter(|k| !nf.contains_key(*k)));
		for key in keys {
			if of.get(key) != nf.get(key) {
				add(key, site);
			}
		}
	}
	changes
}

/// The live content of an entry.
pub fn live_doc(conn: &Connection, id: i64) -> Result<Doc> {
	let mut stmt = conn.prepare(
		"SELECT site, title, slug, published, fields FROM entry_sites
		WHERE entry_id = ?1",
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
			},
		))
	})?;
	let doc: Doc = rows.collect::<rusqlite::Result<_>>()?;
	if doc.is_empty() {
		return Err(Error::NotFound);
	}
	Ok(doc)
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
	let comp = schema.entry_type(&entry.ty).unwrap();

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

/// Keeps the live state of an entry as a revision, called after every
/// save of the live entry.
pub fn record_revision(
	conn: &Connection,
	id: i64,
	user_id: Option<i64>,
) -> Result<()> {
	let doc = live_doc(conn, id)?;
	conn.execute(
		"INSERT INTO revisions (entry_id, doc, user_id, created_at)
		VALUES (?1, ?2, ?3, ?4)",
		params![id, serde_json::to_string(&doc).unwrap(), user_id, now()],
	)?;
	conn.execute(
		"DELETE FROM revisions WHERE entry_id = ?1 AND id NOT IN (
			SELECT id FROM revisions WHERE entry_id = ?1
			ORDER BY id DESC LIMIT ?2
		)",
		params![id, MAX_REVISIONS],
	)?;
	Ok(())
}

/// The revisions of an entry, newest first, each with what it changed.
pub fn revisions(conn: &Connection, id: i64) -> Result<Vec<Revision>> {
	let mut stmt = conn.prepare(
		"SELECT r.id, u.email, r.created_at, r.doc FROM revisions r
		LEFT JOIN users u ON u.id = r.user_id
		WHERE r.entry_id = ?1 ORDER BY r.id",
	)?;
	let rows = stmt
		.query_map([id], |r| {
			Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get::<_, String>(3)?))
		})?
		.collect::<rusqlite::Result<Vec<(i64, Option<String>, i64, String)>>>(
		)?;

	let mut revs = vec![];
	let mut prev: Option<Doc> = None;
	for (id, user, created_at, doc) in rows {
		let doc: Doc = serde_json::from_str(&doc).unwrap_or_default();
		let changes = match &prev {
			Some(p) => diff_docs(p, &doc),
			None => vec![],
		};
		revs.push(Revision {
			id,
			user,
			created_at,
			changes,
		});
		prev = Some(doc);
	}
	revs.reverse();
	Ok(revs)
}

pub fn get_revision(conn: &Connection, id: i64, revision: i64) -> Result<Doc> {
	let doc: String = conn.query_row(
		"SELECT doc FROM revisions WHERE id = ?1 AND entry_id = ?2",
		params![revision, id],
		|r| r.get(0),
	)?;
	Ok(serde_json::from_str(&doc).unwrap_or_default())
}

/// Puts an old revision into the draft. Sites that the revision doesn't
/// know yet keep their live content.
pub fn restore(
	conn: &Connection,
	id: i64,
	revision: i64,
	user_id: Option<i64>,
) -> Result<Draft> {
	let doc: String = conn.query_row(
		"SELECT doc FROM revisions WHERE id = ?1 AND entry_id = ?2",
		params![revision, id],
		|r| r.get(0),
	)?;
	let old: Doc = serde_json::from_str(&doc).unwrap_or_default();
	let mut doc = live_doc(conn, id)?;
	for (site, sd) in old {
		if doc.contains_key(&site) {
			doc.insert(site, sd);
		}
	}

	store_draft(conn, id, &doc, user_id)?;
	Ok(get_draft(conn, id)?.unwrap())
}

pub fn create_token(conn: &Connection, id: i64) -> Result<String> {
	get_entry_exists(conn, id)?;
	let n: u128 = rand::rng().random();
	let token = format!("{n:032x}");
	conn.execute(
		"INSERT INTO preview_tokens (token, entry_id, expires_at)
		VALUES (?1, ?2, ?3)",
		params![token, id, now() + TOKEN_DURATION_MS],
	)?;
	conn.execute("DELETE FROM preview_tokens WHERE expires_at < ?1", [now()])?;
	Ok(token)
}

fn get_entry_exists(conn: &Connection, id: i64) -> Result<()> {
	conn.query_row("SELECT id FROM entries WHERE id = ?1", [id], |_| Ok(()))?;
	Ok(())
}

/// The entry a valid token shows and its draft, the live content if
/// there is none.
pub fn preview(conn: &Connection, token: &str) -> Result<Option<(i64, Doc)>> {
	let id: Option<i64> = conn
		.query_row(
			"SELECT entry_id FROM preview_tokens
			WHERE token = ?1 AND expires_at > ?2",
			params![token, now()],
			|r| r.get(0),
		)
		.optional()?;
	let Some(id) = id else { return Ok(None) };
	let doc = match get_draft(conn, id)? {
		Some(d) => d.doc,
		None => live_doc(conn, id)?,
	};
	Ok(Some((id, doc)))
}

#[derive(Debug, Serialize)]
struct DraftResp {
	draft: Option<Draft>,
}

async fn get_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<DraftResp>> {
	let draft = get_draft(&state.db.conn(), id)?;
	Ok(Json(DraftResp { draft }))
}

#[derive(Debug, Deserialize)]
struct PutReq {
	site: String,
	/// the version of the draft the editor knows
	base: Option<i64>,
	#[serde(flatten)]
	content: SiteDoc,
}

async fn put_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	Json(req): Json<PutReq>,
) -> Result<Json<Draft>> {
	let conn = state.db.conn();
	let d = put_draft(
		&conn,
		&state.schema,
		id,
		&req.site,
		req.content,
		req.base,
		Some(auth.user_id),
	)?;
	Ok(Json(d))
}

async fn delete_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	discard(&state.db.conn(), id)
}

async fn publish_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	publish(&state.db.conn(), &state.schema, id, Some(auth.user_id))
}

async fn revisions_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<Vec<Revision>>> {
	Ok(Json(revisions(&state.db.conn(), id)?))
}

async fn revision_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path((id, rev)): Path<(i64, i64)>,
) -> Result<Json<Doc>> {
	Ok(Json(get_revision(&state.db.conn(), id, rev)?))
}

async fn restore_route(
	State(state): State<AppState>,
	auth: Authed,
	Path((id, rev)): Path<(i64, i64)>,
) -> Result<Json<Draft>> {
	let conn = state.db.conn();
	Ok(Json(restore(&conn, id, rev, Some(auth.user_id))?))
}

#[derive(Debug, Serialize)]
struct TokenResp {
	token: String,
}

async fn preview_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<TokenResp>> {
	let token = create_token(&state.db.conn(), id)?;
	Ok(Json(TokenResp { token }))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route(
			"/entries/{id}/draft",
			get(get_route).put(put_route).delete(delete_route),
		)
		.route("/entries/{id}/publish", post(publish_route))
		.route("/entries/{id}/revisions", get(revisions_route))
		.route("/entries/{id}/revisions/{rev}", get(revision_route))
		.route("/entries/{id}/revisions/{rev}/restore", post(restore_route))
		.route("/entries/{id}/preview", post(preview_route))
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;
	use crate::db::Db;
	use crate::entries::save;
	use crate::entries::tests::input;
	use crate::schema::tests::site;

	fn site_doc(title: &str, fields: Value) -> SiteDoc {
		SiteDoc {
			title: title.into(),
			slug: String::new(),
			published: true,
			fields,
		}
	}

	/// the version of the draft the editor knows
	fn base(conn: &Connection, id: i64) -> Option<i64> {
		get_draft(conn, id).unwrap().map(|d| d.updated_at)
	}

	#[test]
	fn conflicts_with_others() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		crate::auth::add_user(&conn, "a@x.ch", "password").unwrap();
		crate::auth::add_user(&conn, "b@x.ch", "password").unwrap();
		let e = save(&conn, &s, None, input("pages", "page", "P", json!({})))
			.unwrap();

		// a starts a draft, b still has the live entry open
		let a = put_draft(
			&conn,
			&s,
			e.id,
			"de",
			site_doc("A", json!({})),
			None,
			Some(1),
		)
		.unwrap();
		let b = put_draft(
			&conn,
			&s,
			e.id,
			"de",
			site_doc("B", json!({})),
			None,
			Some(2),
		);
		assert!(
			matches!(b, Err(Error::Conflict(m)) if m == "a@x.ch changed this entry")
		);

		// a keeps typing, b after reloading too
		let a2 = put_draft(
			&conn,
			&s,
			e.id,
			"de",
			site_doc("A2", json!({})),
			Some(a.updated_at),
			Some(1),
		)
		.unwrap();
		put_draft(
			&conn,
			&s,
			e.id,
			"de",
			site_doc("B", json!({})),
			Some(a2.updated_at),
			Some(2),
		)
		.unwrap();

		// after publishing, an old version is a conflict too
		publish(&conn, &s, e.id, None).unwrap();
		let late = put_draft(
			&conn,
			&s,
			e.id,
			"de",
			site_doc("A3", json!({})),
			Some(a2.updated_at),
			Some(1),
		);
		assert!(matches!(late, Err(Error::Conflict(_))));
	}

	#[test]
	fn drafts_stay_until_published() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let fields = json!({ "city": "Bern", "price": 10 });
		let e =
			save(&conn, &s, None, input("events", "event", "Konzert", fields))
				.unwrap();

		// en changes its title and the shared price
		let en = site_doc("Concert", json!({ "city": "Bern", "price": 12 }));
		let d = put_draft(&conn, &s, e.id, "en", en, base(&conn, e.id), None)
			.unwrap();
		assert_eq!(d.doc["de"].fields["price"], 12);
		assert_eq!(get_entry(&conn, e.id, "en").unwrap().title, "Konzert");

		publish(&conn, &s, e.id, None).unwrap();
		let live = get_entry(&conn, e.id, "en").unwrap();
		assert_eq!(live.title, "Concert");
		assert_eq!(live.uri.as_deref(), Some("events/concert"));
		assert_eq!(get_entry(&conn, e.id, "de").unwrap().fields["price"], 12);
		assert!(get_draft(&conn, e.id).unwrap().is_none());
		assert_eq!(revisions(&conn, e.id).unwrap().len(), 1);
	}

	#[test]
	fn publish_checks_every_site_first() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let e = save(&conn, &s, None, input("events", "event", "E", json!({})))
			.unwrap();

		put_draft(
			&conn,
			&s,
			e.id,
			"de",
			site_doc("Neu", json!({})),
			base(&conn, e.id),
			None,
		)
		.unwrap();
		put_draft(
			&conn,
			&s,
			e.id,
			"en",
			site_doc("", json!({})),
			base(&conn, e.id),
			None,
		)
		.unwrap();
		let Err(Error::Invalid(errs)) = publish(&conn, &s, e.id, None) else {
			panic!("should fail");
		};
		assert_eq!(errs[0].message, "en: is required");
		// nothing was written
		assert_eq!(get_entry(&conn, e.id, "de").unwrap().title, "E");
	}

	#[test]
	fn revisions_know_what_changed() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let e = save(
			&conn,
			&s,
			None,
			input("pages", "page", "P", json!({ "intro": "a" })),
		)
		.unwrap();
		record_revision(&conn, e.id, None).unwrap();
		let de = site_doc("P2", json!({ "intro": "b" }));
		put_draft(&conn, &s, e.id, "de", de, base(&conn, e.id), None).unwrap();
		publish(&conn, &s, e.id, None).unwrap();

		let revs = revisions(&conn, e.id).unwrap();
		assert!(revs[1].changes.is_empty());
		let fields: Vec<_> =
			revs[0].changes.iter().map(|c| c.field.as_str()).collect();
		// the draft has no slug, it follows the new title
		assert_eq!(fields, ["title", "slug", "intro"]);
		assert_eq!(revs[0].changes[0].sites, ["de"]);
	}

	#[test]
	fn restores_a_revision_into_the_draft() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let e = save(&conn, &s, None, input("pages", "page", "One", json!({})))
			.unwrap();
		record_revision(&conn, e.id, None).unwrap();
		put_draft(
			&conn,
			&s,
			e.id,
			"de",
			site_doc("Two", json!({})),
			base(&conn, e.id),
			None,
		)
		.unwrap();
		publish(&conn, &s, e.id, None).unwrap();

		let first = revisions(&conn, e.id).unwrap().pop().unwrap();
		let d = restore(&conn, e.id, first.id, None).unwrap();
		assert_eq!(d.doc["de"].title, "One");
	}

	#[test]
	fn tokens_show_the_draft() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let e =
			save(&conn, &s, None, input("pages", "page", "Live", json!({})))
				.unwrap();
		let token = create_token(&conn, e.id).unwrap();
		assert_eq!(
			preview(&conn, &token).unwrap().unwrap().1["de"].title,
			"Live"
		);

		put_draft(
			&conn,
			&s,
			e.id,
			"de",
			site_doc("Draft", json!({})),
			base(&conn, e.id),
			None,
		)
		.unwrap();
		assert_eq!(
			preview(&conn, &token).unwrap().unwrap().1["de"].title,
			"Draft"
		);
		assert!(preview(&conn, "nope").unwrap().is_none());
	}
}
