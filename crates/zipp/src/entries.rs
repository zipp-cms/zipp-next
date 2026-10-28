//! Entries: the content of a site. Their fields are stored as JSON, the
//! indexed fields and all relations are copied to `entry_values` and
//! `entry_relations` on every save, so they can be queried.

use std::collections::BTreeMap;

use axum::extract::{Path, Query as QueryParams, State};
use axum::http::HeaderMap;
use axum::routing::get;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, Row, params, params_from_iter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AppState;
use crate::auth::{Authed, session};
use crate::content::{FieldError, index_values, relations, validate};
use crate::db::now;
use crate::error::{Error, Result};
use crate::migrate::Migration;
use crate::query::Query;
use crate::schema::Schema;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
	pub id: i64,
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	pub slug: String,
	pub published: bool,
	pub fields: Value,
	pub created_at: i64,
	pub updated_at: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EntryInput {
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	#[serde(default)]
	pub slug: String,
	#[serde(default)]
	pub published: bool,
	#[serde(default)]
	pub fields: Value,
}

/// What a relation id points to, sent next to the entries.
#[derive(Debug, Clone, Serialize)]
pub struct EntryRef {
	pub id: i64,
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	pub slug: String,
}

const COLUMNS: &str = "e.id, e.type, e.title, e.slug, e.published, e.fields, \
	e.created_at, e.updated_at";

fn from_row(row: &Row) -> rusqlite::Result<Entry> {
	let fields: String = row.get(5)?;
	Ok(Entry {
		id: row.get(0)?,
		ty: row.get(1)?,
		title: row.get(2)?,
		slug: row.get(3)?,
		published: row.get(4)?,
		fields: serde_json::from_str(&fields).unwrap_or_default(),
		created_at: row.get(6)?,
		updated_at: row.get(7)?,
	})
}

pub fn get_entry(conn: &Connection, id: i64) -> Result<Entry> {
	let sql = format!("SELECT {COLUMNS} FROM entries e WHERE e.id = ?1");
	Ok(conn.query_row(&sql, [id], from_row)?)
}

pub fn list(conn: &Connection, q: &Query) -> Result<(Vec<Entry>, i64)> {
	let (from, params) = q.from_where();

	let total = conn.query_row(
		&format!("SELECT COUNT(*) {from}"),
		params_from_iter(&params),
		|r| r.get(0),
	)?;

	let sql = format!(
		"SELECT {COLUMNS} {from} {} LIMIT {} OFFSET {}",
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

/// Creates an entry, or updates it when `id` is given. The type of an
/// entry can't change.
pub fn save(
	conn: &Connection,
	schema: &Schema,
	id: Option<i64>,
	input: EntryInput,
) -> Result<Entry> {
	let comp = schema
		.entry_type(&input.ty)
		.ok_or_else(|| Error::Request(format!("unknown type {}", input.ty)))?;
	if let Some(id) = id
		&& get_entry(conn, id)?.ty != input.ty
	{
		return Err(Error::Request("the type of an entry can't change".into()));
	}

	let mut errors = vec![];
	let title = input.title.trim();
	if title.is_empty() {
		errors.push(field_error("title", "is required"));
	}
	let slug = match input.slug.trim() {
		"" => slugify(title),
		s => slugify(s),
	};
	let taken: Option<i64> = conn
		.query_row(
			"SELECT id FROM entries WHERE type = ?1 AND slug = ?2",
			params![input.ty, slug],
			|r| r.get(0),
		)
		.optional()?;
	if taken.is_some() && taken != id {
		errors.push(field_error("slug", "is already used"));
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
				"UPDATE entries SET title = ?1, slug = ?2, published = ?3,
				fields = ?4, updated_at = ?5 WHERE id = ?6",
				params![
					title,
					slug,
					input.published,
					fields.to_string(),
					now,
					id
				],
			)?;
			id
		}
		None => {
			tx.execute(
				"INSERT INTO entries
				(type, title, slug, published, fields, created_at, updated_at)
				VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?6)",
				params![
					input.ty,
					title,
					slug,
					input.published,
					fields.to_string(),
					now
				],
			)?;
			tx.last_insert_rowid()
		}
	};
	reindex(&tx, schema, id, &input.ty, &fields)?;
	tx.commit()?;

	get_entry(conn, id)
}

fn field_error(path: &str, message: &str) -> FieldError {
	FieldError {
		path: path.into(),
		message: message.into(),
	}
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

/// Rewrites the index rows of an entry from its fields.
fn reindex(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	ty: &str,
	fields: &Value,
) -> Result<()> {
	let comp = schema.get(ty).unwrap();
	conn.execute("DELETE FROM entry_values WHERE entry_id = ?1", [id])?;
	conn.execute("DELETE FROM entry_relations WHERE entry_id = ?1", [id])?;

	for v in index_values(comp, fields) {
		conn.execute(
			"INSERT INTO entry_values (entry_id, field, text, num)
			VALUES (?1, ?2, ?3, ?4)",
			params![id, v.field, v.text, v.num],
		)?;
	}
	for rel in relations(schema, comp, fields) {
		for (pos, target) in rel.ids.iter().enumerate() {
			conn.execute(
				"INSERT INTO entry_relations
				(entry_id, path, target_id, position)
				VALUES (?1, ?2, ?3, ?4)",
				params![id, rel.path, target, pos],
			)?;
		}
	}

	Ok(())
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
	match conn.execute("DELETE FROM entries WHERE id = ?1", [id])? {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

/// The entries the given ones link to.
pub fn refs(
	conn: &Connection,
	ids: &[i64],
	published_only: bool,
) -> Result<BTreeMap<i64, EntryRef>> {
	let marks = vec!["?"; ids.len()].join(", ");
	let sql =
		format!(
		"SELECT DISTINCT e.id, e.type, e.title, e.slug FROM entry_relations r
		JOIN entries e ON e.id = r.target_id
		WHERE r.entry_id IN ({marks}) {}",
		if published_only { "AND e.published = 1" } else { "" }
	);
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query_map(params_from_iter(ids), |r| {
		Ok(EntryRef {
			id: r.get(0)?,
			ty: r.get(1)?,
			title: r.get(2)?,
			slug: r.get(3)?,
		})
	})?;

	let mut out = BTreeMap::new();
	for r in rows {
		let r = r?;
		out.insert(r.id, r);
	}
	Ok(out)
}

/// Applies the migrations that didn't run yet to all entries and returns
/// their names. Afterwards every entry is reindexed, the indexed fields
/// may have changed.
pub fn apply_migrations(
	conn: &Connection,
	migrations: &[Migration],
) -> Result<Vec<String>> {
	let mut applied = vec![];
	let mut schema = Schema::new();

	for m in migrations {
		let done: bool = conn.query_row(
			"SELECT COUNT(*) FROM schema_migrations WHERE name = ?1",
			[&m.name],
			|r| r.get(0),
		)?;
		if done {
			schema = m.schema.clone();
			continue;
		}

		let tx = conn.unchecked_transaction()?;
		let mut stmt = tx.prepare("SELECT id, type, fields FROM entries")?;
		let rows = stmt
			.query_map([], |r| {
				Ok((r.get::<_, i64>(0)?, r.get(1)?, r.get::<_, String>(2)?))
			})?
			.collect::<rusqlite::Result<Vec<(i64, String, String)>>>()?;
		drop(stmt);

		let mut schemas = vec![schema.clone()];
		for op in &m.ops {
			let mut next = schemas.last().unwrap().clone();
			op.apply_schema(&mut next);
			schemas.push(next);
		}

		for (id, mut ty, fields) in rows {
			let mut fields: Value = serde_json::from_str(&fields)
				.map_err(|e| Error::Internal(e.to_string()))?;
			let keep = m
				.ops
				.iter()
				.zip(&schemas)
				.all(|(op, s)| op.apply_entry(s, &mut ty, &mut fields));

			if keep {
				tx.execute(
					"UPDATE entries SET type = ?1, fields = ?2 WHERE id = ?3",
					params![ty, fields.to_string(), id],
				)?;
			} else {
				tx.execute("DELETE FROM entries WHERE id = ?1", [id])?;
			}
		}

		tx.execute(
			"INSERT INTO schema_migrations (name, applied_at) VALUES (?1, ?2)",
			params![m.name, now()],
		)?;
		tx.commit()?;
		schema = m.schema.clone();
		applied.push(m.name.clone());
	}

	if !applied.is_empty() {
		let mut stmt = conn.prepare("SELECT id, type, fields FROM entries")?;
		let rows = stmt
			.query_map([], |r| {
				Ok((r.get::<_, i64>(0)?, r.get(1)?, r.get::<_, String>(2)?))
			})?
			.collect::<rusqlite::Result<Vec<(i64, String, String)>>>()?;
		let tx = conn.unchecked_transaction()?;
		for (id, ty, fields) in rows {
			let fields = serde_json::from_str(&fields).unwrap_or_default();
			reindex(&tx, &schema, id, &ty, &fields)?;
		}
		tx.commit()?;
	}

	Ok(applied)
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
	let relations = refs(&conn, &ids, q.published_only)?;

	Ok(Json(ListResp {
		entries,
		total,
		relations,
	}))
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
) -> Result<Json<EntryResp>> {
	let conn = state.db.conn();
	let published_only = session(&conn, &headers)?.is_none();

	let entry = get_entry(&conn, id)?;
	if published_only && !entry.published {
		return Err(Error::NotFound);
	}
	let relations = refs(&conn, &[id], published_only)?;

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

async fn delete_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	delete(&state.db.conn(), id)
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
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;
	use crate::db::Db;
	use crate::migrate::{Rename, diff};
	use crate::schema::tests::site;

	fn input(ty: &str, title: &str, fields: Value) -> EntryInput {
		EntryInput {
			ty: ty.into(),
			title: title.into(),
			slug: String::new(),
			published: true,
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

		let home = save(&conn, &s, None, input("page", "Über uns!", json!({})))
			.unwrap();
		assert_eq!(home.slug, "über-uns");

		for (city, price) in [("Bern", 10), ("Zürich", 30), ("Bern", 25)] {
			let fields =
				json!({ "city": city, "price": price, "page": [home.id] });
			let title = format!("{city} {price}");
			save(&conn, &s, None, input("event", &title, fields)).unwrap();
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
					("order", "-price")
				]
			),
			["Zürich 30", "Bern 25"]
		);

		// reverse relations
		let id = home.id.to_string();
		assert_eq!(titles(&conn, &[("related", &id)]).len(), 3);
		let (events, _) = list(&conn, &query(&[("type", "event")])).unwrap();
		let r = refs(&conn, &[events[0].id], true).unwrap();
		assert_eq!(r[&home.id].slug, "über-uns");
	}

	#[test]
	fn rejects_invalid_content() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();

		let event =
			save(&conn, &s, None, input("event", "E", json!({}))).unwrap();
		let fields = json!({ "body": [
			{ "type": "button", "fields": { "label": "Go", "link": [event.id] } }
		]});
		let Err(Error::Invalid(errs)) =
			save(&conn, &s, None, input("page", "E", fields))
		else {
			panic!("should fail");
		};
		assert_eq!(errs[0].path, "body.0.link");

		let Err(Error::Invalid(errs)) =
			save(&conn, &s, None, input("event", "E", json!({})))
		else {
			panic!("should fail");
		};
		assert_eq!(errs[0].path, "slug");
	}

	#[test]
	fn unpublished_are_hidden() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let mut draft = input("page", "Draft", json!({}));
		draft.published = false;
		save(&conn, &site(), None, draft).unwrap();

		let mut q = query(&[]);
		assert_eq!(list(&conn, &q).unwrap().1, 1);
		q.published_only = true;
		assert_eq!(list(&conn, &q).unwrap().1, 0);
	}

	#[test]
	fn migrations_change_content_and_index() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = Migration {
			name: "0001".into(),
			ops: diff(&Schema::new(), &s, &[]).unwrap(),
			schema: s.clone(),
		};
		assert_eq!(
			apply_migrations(&conn, std::slice::from_ref(&first)).unwrap(),
			["0001"]
		);

		let fields = json!({ "city": "Bern", "price": 10 });
		let e = save(&conn, &s, None, input("event", "E", fields)).unwrap();

		// rename city to town
		let mut s2 = s.clone();
		s2.get_mut("event").unwrap().fields[0].handle = "town".into();
		let renames = [Rename::parse("event.city=town").unwrap()];
		let second = Migration {
			name: "0002".into(),
			ops: diff(&s, &s2, &renames).unwrap(),
			schema: s2.clone(),
		};
		let migrations = [first, second];
		assert_eq!(apply_migrations(&conn, &migrations).unwrap(), ["0002"]);
		assert!(apply_migrations(&conn, &migrations).unwrap().is_empty());

		assert_eq!(get_entry(&conn, e.id).unwrap().fields["town"], "Bern");
		let params = vec![
			("type".to_string(), "event".to_string()),
			("f.town".to_string(), "Bern".to_string()),
		];
		let q = Query::parse(&s2, &params).unwrap();
		assert_eq!(list(&conn, &q).unwrap().1, 1);
	}

	#[test]
	fn slugs() {
		assert_eq!(slugify("  Hello, World -- 2026 "), "hello-world-2026");
	}
}
