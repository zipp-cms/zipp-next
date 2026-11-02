//! Applies schema migrations to the stored content. Ops on components and
//! fields change the JSON of every row, ops on sites and sections add or
//! delete rows. Afterwards the entries of new singles are created and all
//! indexes and uris are rebuilt.
//!
//! Deleting an entry here takes its children to its parent, they may be of
//! a type that stays.

use rusqlite::{Connection, OptionalExtension, params};
use serde_json::Value;

use crate::content::{convert, defaults};
use crate::db::{marks, now};
use crate::drafts::Doc;
use crate::entries::{refresh_uris, reindex};
use crate::error::{Error, Result};
use crate::migrate::{Migration, Op};
use crate::schema::{ASSET, Schema, SectionKind};

/// Applies the migrations that didn't run yet and returns their names.
/// All of them and the rebuild of the indexes are one transaction: when
/// anything fails nothing is recorded, the next start tries again.
/// `code` is the schema of the code, the same as the last migration's
/// (see [`unmigrated`](crate::migrate::unmigrated)): the singles and the
/// indexes are made with it, a migration file has no field types of
/// plugins and their relations would go.
pub fn apply_migrations(
	conn: &Connection,
	migrations: &[Migration],
	code: &Schema,
) -> Result<Vec<String>> {
	let mut applied = vec![];
	let mut schema = Schema::new();
	let tx = conn.unchecked_transaction()?;

	for m in migrations {
		let done: bool = tx.query_row(
			"SELECT COUNT(*) FROM schema_migrations WHERE name = ?1",
			[&m.name],
			|r| r.get(0),
		)?;
		if done {
			schema = m.schema.clone();
			continue;
		}

		for op in &m.ops {
			apply_op(&tx, &schema, op)?;
			op.apply_schema(&mut schema);
		}
		tx.execute(
			"INSERT INTO schema_migrations (name, applied_at) VALUES (?1, ?2)",
			params![m.name, now()],
		)?;

		schema = m.schema.clone();
		applied.push(m.name.clone());
	}

	if !applied.is_empty() {
		create_singles(&tx, code)?;
		rebuild(&tx, code)?;
	}
	tx.commit()?;

	Ok(applied)
}

/// One op, `schema` is the schema before it.
fn apply_op(conn: &Connection, schema: &Schema, op: &Op) -> Result<()> {
	match op {
		Op::AddSite { site } => {
			// the first migration has no content to copy
			let Some(primary) = schema.sites.first() else {
				return Ok(());
			};
			conn.execute(
				"INSERT INTO entry_sites (entry_id, site, title, slug, uri,
				published, fields, updated_at)
				SELECT entry_id, ?1, title, slug, uri, published, fields,
				updated_at FROM entry_sites WHERE site = ?2",
				params![site.handle, primary.handle],
			)?;
			// a draft gets the new site too, revisions stay as they were
			change_docs(conn, &["drafts"], |doc| {
				if let Some(sd) = doc.get(&primary.handle).cloned() {
					doc.insert(site.handle.clone(), sd);
				}
			})?;
		}
		Op::RemoveSite { site } => {
			for table in [
				"entry_sites",
				"entry_values",
				"entry_relations",
				"asset_relations",
				"entry_texts",
			] {
				conn.execute(
					&format!("DELETE FROM {table} WHERE site = ?1"),
					[site],
				)?;
			}
			change_docs(conn, &["drafts", "revisions"], |doc| {
				doc.remove(site);
			})?;
		}
		Op::AddSection { .. } => {}
		Op::RemoveSection { section } => {
			conn.execute("DELETE FROM entries WHERE section = ?1", [section])?;
		}
		Op::ChangeSection { section } => {
			if section.kind != SectionKind::Structure {
				conn.execute(
					"UPDATE entries SET parent_id = NULL WHERE section = ?1",
					[&section.handle],
				)?;
			}
			let marks = marks(section.types.len());
			let mut values = vec![section.handle.clone()];
			values.extend(section.types.iter().cloned());
			let mut stmt = conn.prepare(&format!(
				"SELECT id FROM entries WHERE section = ? \
				AND type NOT IN ({marks})"
			))?;
			let gone = stmt
				.query_map(rusqlite::params_from_iter(values), |r| r.get(0))?
				.collect::<rusqlite::Result<Vec<i64>>>()?;
			for id in gone {
				delete_entry(conn, id)?;
			}
			// drafts and revisions of the entries that stay
			retype_docs(conn, schema, |s, ty| {
				s != section.handle || section.types.iter().any(|t| t == ty)
			})?;
		}
		_ => {
			change_content(conn, schema, op)?;
			change_assets(conn, schema, op)?;
			change_docs(conn, &["drafts", "revisions"], |doc| {
				// each with its own type, a draft or revision may have
				// another than the entry
				for sd in doc.values_mut() {
					op.apply_entry(schema, &mut sd.ty, &mut sd.fields);
				}
			})?;
			if let Op::RemoveComponent { component } = op {
				retype_docs(conn, schema, |_, ty| ty != component)?;
			}
			if let Op::ChangeField { component, field } = op {
				let was = schema
					.get(component)
					.and_then(|c| {
						c.fields.iter().find(|f| f.handle == field.handle)
					})
					.is_some_and(|f| f.translatable);
				if was && !field.translatable {
					share_primary(conn, schema, component, &field.handle)?;
				}
			}
		}
	}

	Ok(())
}

/// Runs an op over the fields of every row.
fn change_content(conn: &Connection, schema: &Schema, op: &Op) -> Result<()> {
	let mut stmt = conn.prepare(
		"SELECT e.id, s.site, e.type, s.fields FROM entries e
		JOIN entry_sites s ON s.entry_id = e.id",
	)?;
	let rows = stmt
		.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
		.collect::<rusqlite::Result<Vec<(i64, String, String, String)>>>()?;

	for (id, site, mut ty, fields) in rows {
		let mut fields: Value = parse(&fields)?;
		let old_ty = ty.clone();
		if !op.apply_entry(schema, &mut ty, &mut fields) {
			delete_entry(conn, id)?;
			continue;
		}

		conn.execute(
			"UPDATE entry_sites SET fields = ?1 WHERE entry_id = ?2 AND site = ?3",
			params![fields.to_string(), id, site],
		)?;
		if ty != old_ty {
			conn.execute(
				"UPDATE entries SET type = ?1 WHERE id = ?2",
				params![ty, id],
			)?;
		}
	}

	Ok(())
}

/// Runs an op over the values of the asset fields of every asset, like
/// over an entry of the type `asset`; removing the asset fields empties
/// them.
fn change_assets(conn: &Connection, schema: &Schema, op: &Op) -> Result<()> {
	if schema.get(ASSET).is_none() {
		return Ok(());
	}
	let mut stmt = conn.prepare("SELECT id, fields FROM assets")?;
	let rows = stmt
		.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
		.collect::<rusqlite::Result<Vec<(i64, String)>>>()?;

	for (id, fields) in rows {
		let mut fields: Value = parse(&fields)?;
		let mut ty = ASSET.to_string();
		if !op.apply_entry(schema, &mut ty, &mut fields) {
			fields = Value::Object(Default::default());
		}
		conn.execute(
			"UPDATE assets SET fields = ?1 WHERE id = ?2",
			params![fields.to_string(), id],
		)?;
	}

	Ok(())
}

/// Content as stored. What doesn't parse fails the migration instead of
/// being emptied, nothing is lost and it can be fixed by hand.
fn parse<T: serde::de::DeserializeOwned>(json: &str) -> Result<T> {
	serde_json::from_str(json)
		.map_err(|e| Error::Internal(format!("stored content: {e}")))
}

/// Deletes an entry. Its children move to its parent (or the top) at the
/// end of their new siblings, they may be of a type that stays.
fn delete_entry(conn: &Connection, id: i64) -> Result<()> {
	// gone already with the row of another site
	let Some((section, parent)) = conn
		.query_row(
			"SELECT section, parent_id FROM entries WHERE id = ?1",
			[id],
			|r| Ok((r.get::<_, String>(0)?, r.get::<_, Option<i64>>(1)?)),
		)
		.optional()?
	else {
		return Ok(());
	};
	let end: i64 = conn.query_row(
		"SELECT IFNULL(MAX(position) + 1, 0) FROM entries
		WHERE section = ?1 AND parent_id IS ?2",
		params![section, parent],
		|r| r.get(0),
	)?;
	conn.execute(
		"UPDATE entries SET parent_id = ?1, position = position + ?2
		WHERE parent_id = ?3",
		params![parent, end, id],
	)?;
	conn.execute("DELETE FROM entries WHERE id = ?1", [id])?;
	Ok(())
}

/// Drafts and revisions whose type `keep(section, type)` refuses get the
/// type of the entry, with the fields that fit it (see [`convert`]), so
/// they can still be published and restored.
fn retype_docs(
	conn: &Connection,
	schema: &Schema,
	keep: impl Fn(&str, &str) -> bool,
) -> Result<()> {
	for table in ["drafts", "revisions"] {
		let key = if table == "drafts" { "entry_id" } else { "id" };
		let mut stmt = conn.prepare(&format!(
			"SELECT d.{key}, d.doc, e.section, e.type FROM {table} d
			JOIN entries e ON e.id = d.entry_id"
		))?;
		let rows = stmt
			.query_map([], |r| {
				Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
			})?
			.collect::<rusqlite::Result<Vec<(i64, String, String, String)>>>(
			)?;
		drop(stmt);

		for (id, doc, section, live) in rows {
			let mut doc: Doc = parse(&doc)?;
			let mut changed = false;
			// an empty type is the entry's
			for sd in doc.values_mut() {
				if sd.ty.is_empty() || keep(&section, &sd.ty) {
					continue;
				}
				sd.fields = match (schema.get(&sd.ty), schema.get(&live)) {
					(Some(from), Some(to)) => convert(from, to, &sd.fields),
					_ => Value::Object(Default::default()),
				};
				sd.ty = live.clone();
				changed = true;
			}
			if changed {
				conn.execute(
					&format!("UPDATE {table} SET doc = ?1 WHERE {key} = ?2"),
					params![serde_json::to_string(&doc).unwrap(), id],
				)?;
			}
		}
	}
	Ok(())
}

/// Changes the content of drafts and revisions.
fn change_docs(
	conn: &Connection,
	tables: &[&str],
	mut f: impl FnMut(&mut Doc),
) -> Result<()> {
	for table in tables {
		let key = if *table == "drafts" { "entry_id" } else { "id" };
		let mut stmt =
			conn.prepare(&format!("SELECT {key}, doc FROM {table}"))?;
		let rows = stmt
			.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
			.collect::<rusqlite::Result<Vec<(i64, String)>>>()?;
		drop(stmt);

		for (id, doc) in rows {
			let mut doc: Doc = parse(&doc)?;
			f(&mut doc);
			conn.execute(
				&format!("UPDATE {table} SET doc = ?1 WHERE {key} = ?2"),
				params![serde_json::to_string(&doc).unwrap(), id],
			)?;
		}
	}
	Ok(())
}

/// A field that is no longer translatable gets the value of the primary
/// site in all sites.
fn share_primary(
	conn: &Connection,
	schema: &Schema,
	ty: &str,
	field: &str,
) -> Result<()> {
	let primary = &schema.primary_site().handle;
	let mut stmt = conn.prepare(
		"SELECT e.id, s.fields FROM entries e
		JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?2
		WHERE e.type = ?1",
	)?;
	let rows = stmt
		.query_map(params![ty, primary], |r| Ok((r.get(0)?, r.get(1)?)))?
		.collect::<rusqlite::Result<Vec<(i64, String)>>>()?;

	for (id, fields) in rows {
		let fields: Value = parse(&fields)?;
		let value = fields.get(field).cloned().unwrap_or(Value::Null);
		conn.execute(
			"UPDATE entry_sites SET fields = CASE WHEN ?1 = 'null'
				THEN json_remove(fields, '$.' || ?2)
				ELSE json_set(fields, '$.' || ?2, json(?1)) END
			WHERE entry_id = ?3 AND site != ?4",
			params![value.to_string(), field, id, primary],
		)?;
	}

	Ok(())
}

/// Every single has its entry, titled like the section, with the defaults
/// of its fields.
pub fn create_singles(conn: &Connection, schema: &Schema) -> Result<()> {
	let singles = schema
		.sections
		.iter()
		.filter(|s| s.kind == SectionKind::Single);
	for section in singles {
		let exists: bool = conn.query_row(
			"SELECT COUNT(*) FROM entries WHERE section = ?1",
			[&section.handle],
			|r| r.get(0),
		)?;
		if exists {
			continue;
		}

		let comp = schema.entry_type(&section.types[0]);
		let fields = comp.map(|c| defaults(schema, c)).unwrap_or_default();
		let fields = Value::Object(fields).to_string();
		let now = now();
		conn.execute(
			"INSERT INTO entries (section, type, created_at, updated_at)
			VALUES (?1, ?2, ?3, ?3)",
			params![section.handle, section.types[0], now],
		)?;
		let id = conn.last_insert_rowid();
		for site in &schema.sites {
			conn.execute(
				"INSERT INTO entry_sites (entry_id, site, title, slug, uri,
				published, fields, updated_at)
				VALUES (?1, ?2, ?3, ?4, NULL, 1, ?5, ?6)",
				params![
					id,
					site.handle,
					section.name,
					section.handle,
					fields,
					now
				],
			)?;
		}
	}

	Ok(())
}

/// Rebuilds the index rows and the uris of all entries.
fn rebuild(conn: &Connection, schema: &Schema) -> Result<()> {
	let mut stmt = conn.prepare(
		"SELECT e.id, s.site, e.type, s.fields FROM entries e
		JOIN entry_sites s ON s.entry_id = e.id",
	)?;
	let rows = stmt
		.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)))?
		.collect::<rusqlite::Result<Vec<(i64, String, String, String)>>>()?;
	for (id, site, ty, fields) in rows {
		let fields = parse(&fields)?;
		reindex(conn, schema, id, &site, &ty, &fields)?;
	}

	for section in &schema.sections {
		refresh_uris(conn, schema, section)?;
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;
	use crate::db::Db;
	use crate::entries::tests::input;
	use crate::entries::{get_entry, list, save};
	use crate::migrate::{Rename, diff};
	use crate::query::Query;
	use crate::schema::Field;
	use crate::schema::tests::site;

	/// Applies with the last schema as the code's.
	fn apply(
		conn: &Connection,
		migrations: &[Migration],
	) -> Result<Vec<String>> {
		let code = &migrations.last().unwrap().schema;
		apply_migrations(conn, migrations, code)
	}

	fn migration(
		name: &str,
		old: &Schema,
		new: &Schema,
		renames: &[Rename],
	) -> Migration {
		Migration {
			name: name.into(),
			ops: diff(old, new, renames).unwrap(),
			schema: new.clone(),
		}
	}

	#[test]
	fn first_migration_creates_singles() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = migration("0001", &Schema::new(), &s, &[]);
		assert_eq!(
			apply(&conn, std::slice::from_ref(&first)).unwrap(),
			["0001"]
		);
		assert!(apply(&conn, &[first]).unwrap().is_empty());

		let params = vec![("uri".to_string(), String::new())];
		let (home, _) =
			list(&conn, &Query::parse(&s, &params).unwrap()).unwrap();
		assert_eq!(home[0].section, "home");
		assert_eq!(home[0].title, "Home");
	}

	#[test]
	fn migrations_change_content_and_index() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();

		let fields = json!({ "city": "Bern", "price": 10 });
		let e =
			save(&conn, &s, None, input("events", "event", "E", fields), None)
				.unwrap();

		let sd = crate::drafts::SiteDoc {
			ty: "event".into(),
			dates: Default::default(),
			title: "E".into(),
			slug: "e".into(),
			published: true,
			fields: json!({ "city": "Bern" }),
		};
		crate::drafts::put_draft(&conn, &s, e.id, "de", sd, None, None)
			.unwrap();

		// rename city to town
		let mut s2 = s.clone();
		s2.get_mut("event").unwrap().fields[0].handle = "town".into();
		let renames = [Rename::parse("event.city=town").unwrap()];
		let second = migration("0002", &s, &s2, &renames);
		let migrations = [first, second];
		assert_eq!(apply(&conn, &migrations).unwrap(), ["0002"]);

		let entry = get_entry(&conn, e.id, "en").unwrap();
		assert_eq!(entry.fields["town"], "Bern");
		let draft = crate::drafts::get_draft(&conn, e.id).unwrap().unwrap();
		assert_eq!(draft.doc["de"].fields["town"], "Bern");
		let params = vec![
			("type".to_string(), "event".to_string()),
			("f.town".to_string(), "Bern".to_string()),
		];
		let q = Query::parse(&s2, &params).unwrap();
		assert_eq!(list(&conn, &q).unwrap().1, 1);
	}

	#[test]
	fn migrations_change_asset_fields() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let dir = std::env::temp_dir()
			.join(format!("zipp-migrate-{}", rand::random::<u32>()));
		let s = site()
			.asset_fields([Field::text("credit"), Field::text("licence")]);
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();
		let a =
			crate::assets::store(&conn, &dir, "a.pdf", b"%PDF", None).unwrap();
		let change = crate::assets::AssetChange {
			fields: Some(json!({ "credit": "Anna", "licence": "CC" })),
			..Default::default()
		};
		crate::assets::update(&conn, &dir, &s, a.id, change).unwrap();
		let fields = || crate::assets::get_asset(&conn, a.id).unwrap().fields;

		// credit is renamed to photographer, licence goes
		let s2 = site().asset_fields([Field::text("photographer")]);
		let renames = [Rename::parse("asset.credit=photographer").unwrap()];
		let second = migration("0002", &s, &s2, &renames);
		let migrations = [first, second];
		apply(&conn, &migrations).unwrap();
		assert_eq!(fields(), json!({ "photographer": "Anna" }));

		// without asset fields the values go
		let third = migration("0003", &s2, &site(), &[]);
		apply(
			&conn,
			&[migrations[0].clone(), migrations[1].clone(), third],
		)
		.unwrap();
		assert_eq!(fields(), json!({}));
		std::fs::remove_dir_all(dir).unwrap();
	}

	#[test]
	fn new_fields_bring_their_default() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();
		let e = save(
			&conn,
			&s,
			None,
			input("events", "event", "E", json!({})),
			None,
		)
		.unwrap();

		// a boolean that is on, also for the event and the home of before,
		// and a text without a default
		let mut s2 = s.clone();
		for ty in ["event", "page"] {
			let c = s2.get_mut(ty).unwrap();
			c.fields.push(Field::boolean("teasers").default(true));
			c.fields.push(Field::text("cta"));
		}
		let second = migration("0002", &s, &s2, &[]);
		apply(&conn, &[first, second]).unwrap();

		let event = get_entry(&conn, e.id, "en").unwrap();
		assert_eq!(event.fields["teasers"], true);
		assert!(event.fields.get("cta").is_none());
		let params = vec![("uri".to_string(), String::new())];
		let (home, _) =
			list(&conn, &Query::parse(&s2, &params).unwrap()).unwrap();
		assert_eq!(home[0].fields["teasers"], true);
	}

	#[test]
	fn singles_start_with_defaults() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let mut s = site();
		let page = s.get_mut("page").unwrap();
		page.fields.push(Field::boolean("teasers").default(true));
		create_singles(&conn, &s).unwrap();

		let home = get_entry(&conn, 1, "en").unwrap();
		assert_eq!(home.section, "home");
		assert_eq!(home.fields, json!({ "teasers": true }));
	}

	#[test]
	fn sites_and_sections_change_rows() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();

		let mut page =
			input("pages", "page", "Hallo", json!({ "intro": "Hoi" }));
		page.slug = "hallo".into();
		let p = save(&conn, &s, None, page, None).unwrap();
		let e = save(
			&conn,
			&s,
			None,
			input("events", "event", "E", json!({})),
			None,
		)
		.unwrap();

		// fr starts as a copy of de, events now live under agenda/, intro
		// is no longer translatable
		let mut s2 = s.clone().site("fr", "Français", "fr");
		s2.sections[2].uri = Some("agenda/{slug}".into());
		s2.get_mut("page").unwrap().fields[0].translatable = false;
		let mut en = input("pages", "page", "Hello", json!({ "intro": "Hi" }));
		en.site = Some("en".into());
		save(&conn, &s, Some(p.id), en, None).unwrap();

		apply(&conn, &[first.clone(), migration("0002", &s, &s2, &[])])
			.unwrap();
		let fr = get_entry(&conn, p.id, "fr").unwrap();
		assert_eq!(fr.title, "Hallo");
		let en = get_entry(&conn, p.id, "en").unwrap();
		assert_eq!(en.fields["intro"], "Hoi");
		let e = get_entry(&conn, e.id, "fr").unwrap();
		assert_eq!(e.uri.as_deref(), Some("agenda/e"));

		// removing the section deletes its entries
		let mut s3 = s2.clone();
		s3.sections.remove(2);
		s3.components.retain(|c| c.handle != "event");
		let m3 = migration("0003", &s2, &s3, &[]);
		apply(&conn, &[first, migration("0002", &s, &s2, &[]), m3]).unwrap();
		assert!(get_entry(&conn, e.id, "de").is_err());
	}

	#[test]
	fn removed_types_leave_their_children() {
		// event leaves pages: once only from the section, once for good
		let mut s = site();
		s.sections[1].types.push("event".into());
		let mut only_pages = s.clone();
		only_pages.sections[1].types = vec!["page".into()];
		let mut no_events = only_pages.clone();
		no_events.sections.remove(2);
		no_events.components.retain(|c| c.handle != "event");

		for s2 in [only_pages, no_events] {
			let db = Db::memory().unwrap();
			let conn = db.conn();
			let first = migration("0001", &Schema::new(), &s, &[]);
			apply(&conn, std::slice::from_ref(&first)).unwrap();
			let party = save(
				&conn,
				&s,
				None,
				input("pages", "event", "Party", json!({})),
				None,
			)
			.unwrap();
			let mut kid = input("pages", "page", "Kid", json!({}));
			kid.parent_id = Some(party.id);
			let kid = save(&conn, &s, None, kid, None).unwrap();
			assert_eq!(kid.uri.as_deref(), Some("party/kid"));

			let second = migration("0002", &s, &s2, &[]);
			assert!(second.ops[0].describe().contains("children"));
			apply(&conn, &[first, second]).unwrap();
			assert!(get_entry(&conn, party.id, "de").is_err());
			let kid = get_entry(&conn, kid.id, "de").unwrap();
			assert_eq!(kid.parent_id, None);
			assert_eq!(kid.uri.as_deref(), Some("kid"));
		}
	}

	/// A field type whose value is a list of pages.
	struct Pages;

	impl crate::FieldType for Pages {
		fn name(&self) -> &'static str {
			"pages"
		}

		fn validate(
			&self,
			_options: &Value,
			value: &Value,
		) -> std::result::Result<Value, String> {
			Ok(value.clone())
		}

		fn graphql(&self) -> crate::GraphqlType {
			crate::GraphqlType::Scalar(crate::Scalar::String)
		}

		fn relations(
			&self,
			_options: &Value,
			value: &Value,
		) -> Vec<crate::FieldRelation> {
			let ids = value.as_array().into_iter().flatten();
			let ids = ids.filter_map(Value::as_i64).collect();
			let pages = crate::Target::Entries(vec!["page".into()]);
			vec![crate::FieldRelation::new(pages, ids)]
		}

		fn studio(&self) -> crate::StudioModule {
			crate::StudioModule::new("")
		}
	}

	#[test]
	fn plugin_relations_stay_after_migrations() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let mut s = site().field_type(Pages);
		let event = s.get_mut("event").unwrap();
		event
			.fields
			.push(Field::custom("more", "pages", Value::Null));
		// a migration file knows no field types
		let from_file = |mut m: Migration| {
			let json = serde_json::to_value(&m.schema).unwrap();
			m.schema = serde_json::from_value(json).unwrap();
			m
		};
		let first = from_file(migration("0001", &Schema::new(), &s, &[]));
		apply_migrations(&conn, std::slice::from_ref(&first), &s).unwrap();
		let fields = json!({ "more": [1] });
		let e =
			save(&conn, &s, None, input("events", "event", "E", fields), None)
				.unwrap();

		let mut s2 = s.clone();
		s2.get_mut("event").unwrap().fields.push(Field::text("cta"));
		let second = from_file(migration("0002", &s, &s2, &[]));
		apply_migrations(&conn, &[first, second], &s2).unwrap();
		let n: i64 = conn
			.query_row(
				"SELECT COUNT(*) FROM entry_relations
				WHERE entry_id = ?1 AND path = 'more' AND target_id = 1",
				[e.id],
				|r| r.get(0),
			)
			.unwrap();
		assert_eq!(n, 2);
	}

	#[test]
	fn drafts_of_removed_types_take_the_entry_type() {
		// a talk is an event with a speaker, it leaves events: once only
		// from the section, once for good
		let mut s = site().entry(
			"talk",
			"Talk",
			[Field::text("city").indexed(), Field::text("speaker")],
		);
		s.sections[2].types.push("talk".into());
		let mut only_events = s.clone();
		only_events.sections[2].types = vec!["event".into()];
		let mut no_talks = only_events.clone();
		no_talks.components.retain(|c| c.handle != "talk");

		for s2 in [only_events, no_talks] {
			let db = Db::memory().unwrap();
			let conn = db.conn();
			let first = migration("0001", &Schema::new(), &s, &[]);
			apply(&conn, std::slice::from_ref(&first)).unwrap();
			let e = save(
				&conn,
				&s,
				None,
				input("events", "event", "E", json!({})),
				None,
			)
			.unwrap();
			let sd = crate::drafts::SiteDoc {
				ty: "talk".into(),
				dates: Default::default(),
				title: "E".into(),
				slug: "e".into(),
				published: true,
				fields: json!({ "city": "Bern", "speaker": "Anna" }),
			};
			crate::drafts::put_draft(&conn, &s, e.id, "de", sd, None, None)
				.unwrap();

			apply(&conn, &[first, migration("0002", &s, &s2, &[])]).unwrap();
			let draft = crate::drafts::get_draft(&conn, e.id).unwrap().unwrap();
			assert_eq!(draft.doc["de"].ty, "event");
			assert_eq!(draft.doc["de"].fields, json!({ "city": "Bern" }));
			crate::drafts::publish(&conn, &s2, e.id, None).unwrap();
			let e = get_entry(&conn, e.id, "de").unwrap();
			assert_eq!(e.fields["city"], "Bern");
		}
	}

	#[test]
	fn a_new_group_brings_its_defaults() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();

		// the group's component comes after the page in the schema
		let mut s2 = s.clone().block(
			"hero",
			"Hero",
			[Field::boolean("big").default(true)],
		);
		let page = s2.get_mut("page").unwrap();
		page.fields.push(Field::group("hero", "hero"));
		apply(&conn, &[first, migration("0002", &s, &s2, &[])]).unwrap();

		let home = get_entry(&conn, 1, "de").unwrap();
		assert_eq!(home.fields["hero"], json!({ "big": true }));
	}

	#[test]
	fn a_group_of_another_component_starts_over() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();
		let mut home = input("home", "page", "Home", json!({}));
		home.fields = json!({ "seo": { "title": "Hoi" } });
		save(&conn, &s, Some(1), home, None).unwrap();

		// seo is now a group of meta, which has no title
		let mut s2 = s.clone().block(
			"meta",
			"Meta",
			[
				Field::text("keywords"),
				Field::boolean("index").default(true),
			],
		);
		s2.get_mut("page").unwrap().fields[2] = Field::group("seo", "meta");
		apply(&conn, &[first, migration("0002", &s, &s2, &[])]).unwrap();

		let home = get_entry(&conn, 1, "de").unwrap();
		assert_eq!(home.fields["seo"], json!({ "index": true }));
		let mut again = input("home", "page", "Home", home.fields);
		again.slug = "home".into();
		save(&conn, &s2, Some(1), again, None).unwrap();
	}

	#[test]
	fn unparsable_content_fails_the_migration() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();
		let e = save(
			&conn,
			&s,
			None,
			input("events", "event", "E", json!({ "city": "Bern" })),
			None,
		)
		.unwrap();
		let sd = crate::drafts::SiteDoc {
			ty: "event".into(),
			dates: Default::default(),
			title: "E".into(),
			slug: "e".into(),
			published: true,
			fields: json!({ "city": "Bern" }),
		};
		crate::drafts::put_draft(&conn, &s, e.id, "de", sd, None, None)
			.unwrap();
		conn.execute("UPDATE drafts SET doc = '{\"de\": '", [])
			.unwrap();

		let mut s2 = s.clone();
		s2.get_mut("event").unwrap().fields[0].handle = "town".into();
		let renames = [Rename::parse("event.city=town").unwrap()];
		let second = migration("0002", &s, &s2, &renames);
		assert!(apply(&conn, &[first, second]).is_err());
		let e = get_entry(&conn, e.id, "de").unwrap();
		assert_eq!(e.fields["city"], "Bern");
		let done: i64 = conn
			.query_row("SELECT COUNT(*) FROM schema_migrations", [], |r| {
				r.get(0)
			})
			.unwrap();
		assert_eq!(done, 1);
	}
}
