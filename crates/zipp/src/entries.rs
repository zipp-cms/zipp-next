//! Entries: the content of a site. An entry lives in a section and has a
//! row per site with its title, slug, uri and fields as JSON. On every
//! save the indexed fields and all relations are copied to `entry_values`
//! and `entry_relations`, so they can be queried.

use std::collections::{BTreeMap, HashMap};

use axum::extract::{Path, Query as QueryParams, State};
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AppState;
use crate::auth::{Authed, session};
use crate::content::{FieldError, index_values, relations, share, validate};
use crate::db::now;
use crate::error::{Error, Result};
use crate::query::Query;
use crate::schema::{Schema, Section, SectionKind};

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, Serialize)]
pub struct EntryRef {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	pub slug: String,
	pub uri: Option<String>,
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

/// Creates an entry in all sites, or updates it in one site when `id` is
/// given. Shared fields are copied to the other sites. Section and type
/// of an entry can't change.
pub fn save(
	conn: &Connection,
	schema: &Schema,
	id: Option<i64>,
	input: EntryInput,
) -> Result<Entry> {
	let site = input
		.site
		.clone()
		.unwrap_or_else(|| schema.primary_site().handle.clone());
	if schema.get_site(&site).is_none() {
		return Err(Error::Request(format!("unknown site {site}")));
	}
	let section = schema
		.get_section(&input.section)
		.filter(|s| s.types.contains(&input.ty))
		.ok_or_else(|| {
			Error::Request(format!(
				"no type {} in section {}",
				input.ty, input.section
			))
		})?;
	let comp = schema.entry_type(&input.ty).unwrap();

	let parent_id = match id {
		Some(id) => {
			let e = get_entry(conn, id, &site)?;
			if e.section != input.section || e.ty != input.ty {
				return Err(Error::Request(
					"section and type of an entry can't change".into(),
				));
			}
			e.parent_id
		}
		None if section.kind == SectionKind::Single => {
			return Err(Error::Request(
				"a single has its entry already".into(),
			));
		}
		None => {
			if let Some(p) = input.parent_id {
				check_parent(conn, section, p)?;
			}
			input.parent_id
		}
	};

	let mut errors = vec![];
	let title = input.title.trim();
	if title.is_empty() {
		errors.push(field_error("title", "is required"));
	}
	let slug = match input.slug.trim() {
		"" => slugify(title),
		s => slugify(s),
	};

	// the uri in every site, a new entry starts with this slug everywhere
	let mut uris = HashMap::new();
	for s in &schema.sites {
		if id.is_some() && s.handle != site {
			continue;
		}
		let uri = uri_for(conn, section, &slug, parent_id, &s.handle)?;
		if let Some(uri) = &uri {
			let taken: Option<i64> = conn
				.query_row(
					"SELECT entry_id FROM entry_sites WHERE site = ?1 AND uri = ?2",
					params![s.handle, uri],
					|r| r.get(0),
				)
				.optional()?;
			if taken.is_some() && taken != id {
				let msg = format!("the uri /{uri} is already used");
				errors.push(field_error("slug", &msg));
				break;
			}
		}
		uris.insert(s.handle.clone(), uri);
	}

	let fields = match validate(schema, comp, &input.fields) {
		Ok(f) => f,
		Err(e) => {
			errors.extend(e);
			Value::Null
		}
	};
	if errors.is_empty() {
		errors.extend(check_relations(conn, schema, &input.ty, &fields)?);
	}
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}

	let tx = conn.unchecked_transaction()?;
	let now = now();
	let id = match id {
		Some(id) => {
			tx.execute(
				"UPDATE entry_sites SET title = ?1, slug = ?2, uri = ?3,
				published = ?4, fields = ?5, updated_at = ?6
				WHERE entry_id = ?7 AND site = ?8",
				params![
					title,
					slug,
					uris[&site],
					input.published,
					fields.to_string(),
					now,
					id,
					site
				],
			)?;
			reindex(&tx, schema, id, &site, &input.ty, &fields)?;

			for other in schema.sites.iter().filter(|s| s.handle != site) {
				let theirs: String = tx.query_row(
					"SELECT fields FROM entry_sites
					WHERE entry_id = ?1 AND site = ?2",
					params![id, other.handle],
					|r| r.get(0),
				)?;
				let theirs = serde_json::from_str(&theirs).unwrap_or_default();
				let theirs = share(comp, &fields, &theirs);
				tx.execute(
					"UPDATE entry_sites SET fields = ?1
					WHERE entry_id = ?2 AND site = ?3",
					params![theirs.to_string(), id, other.handle],
				)?;
				reindex(&tx, schema, id, &other.handle, &input.ty, &theirs)?;
			}

			tx.execute(
				"UPDATE entries SET updated_at = ?1 WHERE id = ?2",
				params![now, id],
			)?;
			// children follow the uri of their parent
			if section.kind == SectionKind::Structure {
				refresh_uris(&tx, schema, section)?;
			}
			id
		}
		None => {
			let position: i64 = tx.query_row(
				"SELECT IFNULL(MAX(position) + 1, 0) FROM entries
				WHERE section = ?1 AND parent_id IS ?2",
				params![input.section, parent_id],
				|r| r.get(0),
			)?;
			tx.execute(
				"INSERT INTO entries
				(section, type, parent_id, position, created_at, updated_at)
				VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
				params![input.section, input.ty, parent_id, position, now],
			)?;
			let id = tx.last_insert_rowid();

			for s in &schema.sites {
				tx.execute(
					"INSERT INTO entry_sites (entry_id, site, title, slug, uri,
					published, fields, updated_at)
					VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
					params![
						id,
						s.handle,
						title,
						slug,
						uris[&s.handle],
						input.published,
						fields.to_string(),
						now
					],
				)?;
				reindex(&tx, schema, id, &s.handle, &input.ty, &fields)?;
			}
			id
		}
	};
	tx.commit()?;

	get_entry(conn, id, &site)
}

fn field_error(path: &str, message: &str) -> FieldError {
	FieldError {
		path: path.into(),
		message: message.into(),
	}
}

fn check_parent(
	conn: &Connection,
	section: &Section,
	parent: i64,
) -> Result<()> {
	if section.kind != SectionKind::Structure {
		return Err(Error::Request(
			"only entries of a structure have parents".into(),
		));
	}
	let parent_section: Option<String> = conn
		.query_row("SELECT section FROM entries WHERE id = ?1", [parent], |r| {
			r.get(0)
		})
		.optional()?;
	if parent_section.as_ref() != Some(&section.handle) {
		return Err(Error::Request(format!(
			"the parent {parent} is not in {}",
			section.handle
		)));
	}
	Ok(())
}

/// The uri of an entry in a site, after the uri of its parent.
fn uri_for(
	conn: &Connection,
	section: &Section,
	slug: &str,
	parent_id: Option<i64>,
	site: &str,
) -> Result<Option<String>> {
	let parent_uri: Option<String> = match parent_id {
		Some(p) => conn
			.query_row(
				"SELECT uri FROM entry_sites WHERE entry_id = ?1 AND site = ?2",
				params![p, site],
				|r| r.get(0),
			)
			.optional()?
			.flatten(),
		None => None,
	};
	Ok(section.render_uri(slug, parent_uri.as_deref()))
}

/// Recomputes the uris of all entries of a section in all sites, parents
/// before their children.
pub fn refresh_uris(
	conn: &Connection,
	schema: &Schema,
	section: &Section,
) -> Result<()> {
	let mut stmt = conn.prepare(
		"WITH RECURSIVE tree(id, depth) AS (
			SELECT id, 0 FROM entries
			WHERE section = ?1 AND parent_id IS NULL
			UNION ALL
			SELECT e.id, t.depth + 1 FROM entries e JOIN tree t
			ON e.parent_id = t.id
		)
		SELECT t.id, e.parent_id, s.slug, s.uri FROM tree t
		JOIN entries e ON e.id = t.id
		JOIN entry_sites s ON s.entry_id = t.id AND s.site = ?2
		ORDER BY t.depth",
	)?;

	for site in &schema.sites {
		let rows = stmt
			.query_map(params![section.handle, site.handle], |r| {
				Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
			})?
			.collect::<rusqlite::Result<
				Vec<(i64, Option<i64>, String, Option<String>)>,
			>>()?;

		let mut uris: HashMap<i64, Option<String>> = HashMap::new();
		for (id, parent, slug, old) in rows {
			let parent_uri =
				parent.and_then(|p| uris.get(&p).cloned().flatten());
			let uri = section.render_uri(&slug, parent_uri.as_deref());
			if uri != old {
				conn.execute(
					"UPDATE entry_sites SET uri = ?1
					WHERE entry_id = ?2 AND site = ?3",
					params![uri, id, site.handle],
				)
				.map_err(|_| {
					Error::Request(format!(
						"the uri /{} is already used",
						uri.clone().unwrap_or_default()
					))
				})?;
			}
			uris.insert(id, uri);
		}
	}

	Ok(())
}

/// Relations must point to existing entries of the allowed types.
fn check_relations(
	conn: &Connection,
	schema: &Schema,
	ty: &str,
	fields: &Value,
) -> Result<Vec<FieldError>> {
	let comp = schema.get(ty).unwrap();
	let mut errors = vec![];

	for rel in relations(schema, comp, fields) {
		for id in rel.ids {
			let target: Option<String> = conn
				.query_row(
					"SELECT type FROM entries WHERE id = ?1",
					[id],
					|r| r.get(0),
				)
				.optional()?;
			match target {
				Some(t) if rel.types.contains(&t) => {}
				Some(t) => errors.push(field_error(
					&rel.path,
					&format!("entry {id} is a {t}, not allowed here"),
				)),
				None => errors.push(field_error(
					&rel.path,
					&format!("entry {id} doesn't exist"),
				)),
			}
		}
	}

	Ok(errors)
}

/// Rewrites the index rows of an entry in a site from its fields.
pub fn reindex(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	site: &str,
	ty: &str,
	fields: &Value,
) -> Result<()> {
	let Some(comp) = schema.get(ty) else {
		return Ok(());
	};
	conn.execute(
		"DELETE FROM entry_values WHERE entry_id = ?1 AND site = ?2",
		params![id, site],
	)?;
	conn.execute(
		"DELETE FROM entry_relations WHERE entry_id = ?1 AND site = ?2",
		params![id, site],
	)?;

	for v in index_values(comp, fields) {
		conn.execute(
			"INSERT INTO entry_values (entry_id, site, field, text, num)
			VALUES (?1, ?2, ?3, ?4, ?5)",
			params![id, site, v.field, v.text, v.num],
		)?;
	}
	for rel in relations(schema, comp, fields) {
		for (pos, target) in rel.ids.iter().enumerate() {
			conn.execute(
				"INSERT INTO entry_relations
				(entry_id, site, path, target_id, position)
				VALUES (?1, ?2, ?3, ?4, ?5)",
				params![id, site, rel.path, target, pos],
			)?;
		}
	}

	Ok(())
}

/// Moves an entry of a structure under `parent` (`None` is the top) at
/// `index` among its new siblings.
pub fn move_entry(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	parent: Option<i64>,
	index: usize,
) -> Result<()> {
	let section: String = conn.query_row(
		"SELECT section FROM entries WHERE id = ?1",
		[id],
		|r| r.get(0),
	)?;
	let section = schema.get_section(&section).unwrap();
	if let Some(p) = parent {
		check_parent(conn, section, p)?;

		// the new parent can't be the entry or below it
		let mut up = Some(p);
		while let Some(cur) = up {
			if cur == id {
				return Err(Error::Request(
					"can't move an entry below itself".into(),
				));
			}
			up = conn.query_row(
				"SELECT parent_id FROM entries WHERE id = ?1",
				[cur],
				|r| r.get(0),
			)?;
		}
	} else if section.kind != SectionKind::Structure {
		return Err(Error::Request("only entries of a structure move".into()));
	}

	let tx = conn.unchecked_transaction()?;
	let mut stmt = tx.prepare(
		"SELECT id FROM entries WHERE section = ?1 AND parent_id IS ?2
		AND id != ?3 ORDER BY position",
	)?;
	let mut siblings = stmt
		.query_map(params![section.handle, parent, id], |r| r.get(0))?
		.collect::<rusqlite::Result<Vec<i64>>>()?;
	drop(stmt);
	siblings.insert(index.min(siblings.len()), id);

	for (pos, sibling) in siblings.iter().enumerate() {
		tx.execute(
			"UPDATE entries SET position = ?1 WHERE id = ?2",
			params![pos, sibling],
		)?;
	}
	tx.execute(
		"UPDATE entries SET parent_id = ?1 WHERE id = ?2",
		params![parent, id],
	)?;
	refresh_uris(&tx, schema, section)?;
	tx.commit()?;

	Ok(())
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
		"SELECT DISTINCT e.id, e.section, e.type, s.title, s.slug, s.uri
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
	let rows = stmt.query_map(params_from_iter(params), |r| {
		Ok(EntryRef {
			id: r.get(0)?,
			section: r.get(1)?,
			ty: r.get(2)?,
			title: r.get(3)?,
			slug: r.get(4)?,
			uri: r.get(5)?,
		})
	})?;

	let mut out = BTreeMap::new();
	for r in rows {
		let r = r?;
		out.insert(r.id, r);
	}
	Ok(out)
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

#[derive(Debug, Serialize)]
struct ListResp {
	entries: Vec<Entry>,
	total: i64,
	relations: BTreeMap<i64, EntryRef>,
}

async fn list_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	QueryParams(params): QueryParams<Vec<(String, String)>>,
) -> Result<Json<ListResp>> {
	let conn = state.db.conn();
	let mut q = Query::parse(&state.schema, &params).map_err(Error::Request)?;
	q.published_only = session(&conn, &headers)?.is_none();

	let (entries, total) = list(&conn, &q)?;
	let ids: Vec<i64> = entries.iter().map(|e| e.id).collect();
	let relations = refs(&conn, &ids, &q.site, q.published_only)?;

	Ok(Json(ListResp {
		entries,
		total,
		relations,
	}))
}

#[derive(Debug, Deserialize)]
struct SiteParam {
	site: Option<String>,
}

#[derive(Debug, Serialize)]
struct EntryResp {
	entry: Entry,
	relations: BTreeMap<i64, EntryRef>,
}

async fn get_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	Path(id): Path<i64>,
	QueryParams(p): QueryParams<SiteParam>,
) -> Result<Json<EntryResp>> {
	let conn = state.db.conn();
	let published_only = session(&conn, &headers)?.is_none();
	let site = p
		.site
		.unwrap_or_else(|| state.schema.primary_site().handle.clone());

	let entry = get_entry(&conn, id, &site)?;
	if published_only && !entry.published {
		return Err(Error::NotFound);
	}
	let relations = refs(&conn, &[id], &site, published_only)?;

	Ok(Json(EntryResp { entry, relations }))
}

async fn create_route(
	State(state): State<AppState>,
	_auth: Authed,
	Json(input): Json<EntryInput>,
) -> Result<Json<Entry>> {
	let conn = state.db.conn();
	Ok(Json(save(&conn, &state.schema, None, input)?))
}

async fn update_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
	Json(input): Json<EntryInput>,
) -> Result<Json<Entry>> {
	let conn = state.db.conn();
	Ok(Json(save(&conn, &state.schema, Some(id), input)?))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MoveReq {
	parent_id: Option<i64>,
	index: usize,
}

async fn move_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
	Json(req): Json<MoveReq>,
) -> Result<()> {
	let conn = state.db.conn();
	move_entry(&conn, &state.schema, id, req.parent_id, req.index)
}

async fn delete_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	delete(&state.db.conn(), &state.schema, id)
}

async fn schema_route(
	State(state): State<AppState>,
	_auth: Authed,
) -> Json<Schema> {
	Json((*state.schema).clone())
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/schema", get(schema_route))
		.route("/entries", get(list_route).post(create_route))
		.route(
			"/entries/{id}",
			get(get_route).put(update_route).delete(delete_route),
		)
		.route("/entries/{id}/move", post(move_route))
}

#[cfg(test)]
pub(crate) mod tests {
	use serde_json::json;

	use super::*;
	use crate::db::Db;
	use crate::schema::tests::site;

	pub fn input(
		section: &str,
		ty: &str,
		title: &str,
		fields: Value,
	) -> EntryInput {
		EntryInput {
			section: section.into(),
			ty: ty.into(),
			site: None,
			title: title.into(),
			slug: String::new(),
			published: true,
			parent_id: None,
			fields,
		}
	}

	fn query(params: &[(&str, &str)]) -> Query {
		let params: Vec<_> = params
			.iter()
			.map(|(k, v)| (k.to_string(), v.to_string()))
			.collect();
		Query::parse(&site(), &params).unwrap()
	}

	fn titles(conn: &Connection, params: &[(&str, &str)]) -> Vec<String> {
		let (entries, _) = list(conn, &query(params)).unwrap();
		entries.into_iter().map(|e| e.title).collect()
	}

	#[test]
	fn saves_and_queries() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();

		let about = save(
			&conn,
			&s,
			None,
			input("pages", "page", "Über uns!", json!({})),
		)
		.unwrap();
		assert_eq!(about.slug, "über-uns");
		assert_eq!(about.uri.as_deref(), Some("über-uns"));

		for (city, price) in [("Bern", 10), ("Zürich", 30), ("Bern", 25)] {
			let fields =
				json!({ "city": city, "price": price, "page": [about.id] });
			let title = format!("{city} {price}");
			let e = input("events", "event", &title, fields);
			save(&conn, &s, None, e).unwrap();
		}

		assert_eq!(
			titles(
				&conn,
				&[("type", "event"), ("f.city", "Bern"), ("order", "price")]
			),
			["Bern 10", "Bern 25"]
		);
		assert_eq!(
			titles(
				&conn,
				&[
					("type", "event"),
					("f.price.gte", "25"),
					("order", "-price"),
					("site", "en"),
				]
			),
			["Zürich 30", "Bern 25"]
		);
		assert_eq!(titles(&conn, &[("uri", "events/bern-25")]), ["Bern 25"]);

		// reverse relations
		let id = about.id.to_string();
		assert_eq!(titles(&conn, &[("related", &id)]).len(), 3);
		let (events, _) = list(&conn, &query(&[("type", "event")])).unwrap();
		let r = refs(&conn, &[events[0].id], "de", true).unwrap();
		assert_eq!(r[&about.id].uri.as_deref(), Some("über-uns"));
	}

	#[test]
	fn sites_share_fields_that_are_not_translatable() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();

		let fields = json!({ "city": "Bern", "price": 10 });
		let e =
			save(&conn, &s, None, input("events", "event", "Konzert", fields))
				.unwrap();

		let mut en =
			input("events", "event", "Concert", json!({ "city": "Berne" }));
		en.site = Some("en".into());
		save(&conn, &s, Some(e.id), en).unwrap();

		let de = get_entry(&conn, e.id, "de").unwrap();
		let en = get_entry(&conn, e.id, "en").unwrap();
		assert_eq!(
			(de.title.as_str(), en.title.as_str()),
			("Konzert", "Concert")
		);
		assert_eq!(en.uri.as_deref(), Some("events/concert"));
		// city and price are shared: en removed the price everywhere
		assert_eq!(de.fields, json!({ "city": "Berne" }));
		assert_eq!(
			titles(&conn, &[("type", "event"), ("f.city", "Berne")]).len(),
			1
		);
	}

	#[test]
	fn structures_build_uris_from_parents() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();

		let about =
			save(&conn, &s, None, input("pages", "page", "About", json!({})))
				.unwrap();
		let mut team = input("pages", "page", "Team", json!({}));
		team.parent_id = Some(about.id);
		let team = save(&conn, &s, None, team).unwrap();
		assert_eq!(team.uri.as_deref(), Some("about/team"));

		// renaming the parent moves the child
		let mut rename = input("pages", "page", "About", json!({}));
		rename.slug = "company".into();
		save(&conn, &s, Some(about.id), rename).unwrap();
		let team = get_entry(&conn, team.id, "de").unwrap();
		assert_eq!(team.uri.as_deref(), Some("company/team"));

		// moving to the top and before about
		move_entry(&conn, &s, team.id, None, 0).unwrap();
		let team = get_entry(&conn, team.id, "en").unwrap();
		assert_eq!(team.uri.as_deref(), Some("team"));
		assert_eq!(
			titles(&conn, &[("section", "pages"), ("order", "structure")]),
			["Team", "About"]
		);

		let err = move_entry(&conn, &s, about.id, Some(about.id), 0);
		assert!(err.is_err());
	}

	#[test]
	fn rejects_invalid_content() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();

		let event =
			save(&conn, &s, None, input("events", "event", "E", json!({})))
				.unwrap();
		let fields = json!({ "body": [
			{ "type": "button", "fields": { "label": "Go", "link": [event.id] } }
		]});
		let Err(Error::Invalid(errs)) =
			save(&conn, &s, None, input("pages", "page", "E", fields))
		else {
			panic!("should fail");
		};
		assert_eq!(errs[0].path, "body.0.link");

		let Err(Error::Invalid(errs)) =
			save(&conn, &s, None, input("events", "event", "E", json!({})))
		else {
			panic!("should fail");
		};
		assert_eq!(errs[0].message, "the uri /events/e is already used");

		let wrong = input("events", "page", "P", json!({}));
		assert!(save(&conn, &s, None, wrong).is_err());
		let single = input("home", "page", "Home", json!({}));
		assert!(save(&conn, &s, None, single).is_err());
	}

	#[test]
	fn unpublished_are_hidden() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let mut draft = input("pages", "page", "Draft", json!({}));
		draft.published = false;
		save(&conn, &site(), None, draft).unwrap();

		let mut q = query(&[]);
		assert_eq!(list(&conn, &q).unwrap().1, 1);
		q.published_only = true;
		assert_eq!(list(&conn, &q).unwrap().1, 0);
	}

	#[test]
	fn slugs() {
		assert_eq!(slugify("  Hello, World -- 2026 "), "hello-world-2026");
	}
}
