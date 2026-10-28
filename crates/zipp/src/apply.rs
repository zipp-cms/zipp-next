//! Applies schema migrations to the stored content. Ops on components and
//! fields change the JSON of every row, ops on sites and sections add or
//! delete rows. Afterwards the entries of new singles are created and all
//! indexes and uris are rebuilt.

use rusqlite::{Connection, params};
use serde_json::Value;

use crate::db::now;
use crate::drafts::Doc;
use crate::entries::{refresh_uris, reindex};
use crate::error::{Error, Result};
use crate::migrate::{Migration, Op};
use crate::schema::{Schema, SectionKind};

/// Applies the migrations that didn't run yet and returns their names.
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
		for op in &m.ops {
			apply_op(&tx, &schema, op)?;
			op.apply_schema(&mut schema);
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
		let tx = conn.unchecked_transaction()?;
		create_singles(&tx, &schema)?;
		rebuild(&tx, &schema)?;
		tx.commit()?;
	}

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
			change_docs(conn, &["drafts"], |doc, _| {
				if let Some(sd) = doc.get(&primary.handle).cloned() {
					doc.insert(site.handle.clone(), sd);
				}
			})?;
		}
		Op::RemoveSite { site } => {
			for table in ["entry_sites", "entry_values", "entry_relations"] {
				conn.execute(
					&format!("DELETE FROM {table} WHERE site = ?1"),
					[site],
				)?;
			}
			change_docs(conn, &["drafts", "revisions"], |doc, _| {
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
			let marks = vec!["?"; section.types.len()].join(", ");
			let mut values = vec![section.handle.clone()];
			values.extend(section.types.iter().cloned());
			conn.execute(
				&format!(
					"DELETE FROM entries WHERE section = ? \
					AND type NOT IN ({marks})"
				),
				rusqlite::params_from_iter(values),
			)?;
		}
		_ => {
			change_content(conn, schema, op)?;
			change_docs(conn, &["drafts", "revisions"], |doc, ty| {
				for sd in doc.values_mut() {
					op.apply_entry(schema, &mut ty.to_string(), &mut sd.fields);
				}
			})?;
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
		let mut fields: Value = serde_json::from_str(&fields)
			.map_err(|e| Error::Internal(e.to_string()))?;
		let old_ty = ty.clone();
		if !op.apply_entry(schema, &mut ty, &mut fields) {
			conn.execute("DELETE FROM entries WHERE id = ?1", [id])?;
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

/// Changes the content of drafts and revisions, `f` gets the type of the
/// entry.
fn change_docs(
	conn: &Connection,
	tables: &[&str],
	mut f: impl FnMut(&mut Doc, &str),
) -> Result<()> {
	for table in tables {
		let key = if *table == "drafts" { "entry_id" } else { "id" };
		let mut stmt = conn.prepare(&format!(
			"SELECT d.{key}, d.doc, e.type FROM {table} d
			JOIN entries e ON e.id = d.entry_id"
		))?;
		let rows = stmt
			.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
			.collect::<rusqlite::Result<Vec<(i64, String, String)>>>()?;
		drop(stmt);

		for (id, doc, ty) in rows {
			let mut doc: Doc = serde_json::from_str(&doc).unwrap_or_default();
			f(&mut doc, &ty);
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
		let fields: Value = serde_json::from_str(&fields).unwrap_or_default();
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

/// Every single has its entry, titled like the section.
fn create_singles(conn: &Connection, schema: &Schema) -> Result<()> {
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
				VALUES (?1, ?2, ?3, ?4, NULL, 1, '{}', ?5)",
				params![id, site.handle, section.name, section.handle, now],
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
		let fields = serde_json::from_str(&fields).unwrap_or_default();
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
	use crate::schema::tests::site;

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
			apply_migrations(&conn, std::slice::from_ref(&first)).unwrap(),
			["0001"]
		);
		assert!(apply_migrations(&conn, &[first]).unwrap().is_empty());

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
		apply_migrations(&conn, std::slice::from_ref(&first)).unwrap();

		let fields = json!({ "city": "Bern", "price": 10 });
		let e = save(&conn, &s, None, input("events", "event", "E", fields))
			.unwrap();

		let sd = crate::drafts::SiteDoc {
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
		assert_eq!(apply_migrations(&conn, &migrations).unwrap(), ["0002"]);

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
	fn sites_and_sections_change_rows() {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let s = site();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply_migrations(&conn, std::slice::from_ref(&first)).unwrap();

		let mut page =
			input("pages", "page", "Hallo", json!({ "intro": "Hoi" }));
		page.slug = "hallo".into();
		let p = save(&conn, &s, None, page).unwrap();
		let e = save(&conn, &s, None, input("events", "event", "E", json!({})))
			.unwrap();

		// fr starts as a copy of de, events now live under agenda/, intro
		// is no longer translatable
		let mut s2 = s.clone().site("fr", "Français", "fr");
		s2.sections[2].uri = Some("agenda/{slug}".into());
		s2.get_mut("page").unwrap().fields[0].translatable = false;
		let mut en = input("pages", "page", "Hello", json!({ "intro": "Hi" }));
		en.site = Some("en".into());
		save(&conn, &s, Some(p.id), en).unwrap();

		apply_migrations(
			&conn,
			&[first.clone(), migration("0002", &s, &s2, &[])],
		)
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
		apply_migrations(&conn, &[first, migration("0002", &s, &s2, &[]), m3])
			.unwrap();
		assert!(get_entry(&conn, e.id, "de").is_err());
	}
}
