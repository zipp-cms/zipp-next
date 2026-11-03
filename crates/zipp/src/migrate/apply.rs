//! Applies schema migrations to the stored content. Ops on components and
//! fields change the JSON of every row, ops on sites and sections add or
//! delete rows. The step of a data migration runs after its ops.
//! Afterwards the entries of new singles are created and all indexes and
//! uris are rebuilt.
//!
//! Deleting an entry here takes its children to its parent, they may be of
//! a type that stays.

use rusqlite::{Connection, OptionalExtension, params};
use serde_json::Value;

use crate::content::{convert, defaults};
use crate::db::{marks, now};
use crate::drafts::Doc;
use crate::entries::{INDEX_TABLES, next_position, refresh_uris, reindex_all};
use crate::error::{Error, Result};
use crate::migrate::{DataTx, Migration, Op, unmigrated};
use crate::schema::{ASSET, Schema, SectionKind};

/// Brings the database up to the code before the server or a command
/// uses it: refuses when the code changed since the last migration,
/// applies the migrations that didn't run yet and returns their names.
pub(crate) fn up_to_date(
	conn: &Connection,
	migrations: &[Migration],
	code: &Schema,
) -> std::result::Result<Vec<String>, String> {
	if !unmigrated(migrations, code)?.is_empty() {
		return Err("The schema changed since the last migration, run \
			`makemigration` first"
			.into());
	}
	apply_migrations(conn, migrations, code).map_err(|e| e.to_string())
}

/// Applies the migrations that didn't run yet and returns their names.
/// All of them and the rebuild of the indexes are one transaction: when
/// anything fails nothing is recorded, the next start tries again.
/// `code` is the schema of the code, the same as the last migration's
/// (see [`unmigrated`]): the singles and the
/// indexes are made with it, a migration file has no field types of
/// plugins and their relations would go.
pub(crate) fn apply_migrations(
	conn: &Connection,
	migrations: &[Migration],
	code: &Schema,
) -> Result<Vec<String>> {
	let mut applied = vec![];
	let mut schema = Schema::new();
	let tx = conn.unchecked_transaction()?;

	for m in migrations {
		if is_applied(&tx, &m.name)? {
			schema = m.schema.clone();
			continue;
		}

		for op in &m.ops {
			apply_op(&tx, &schema, op)?;
			op.apply_schema(&mut schema);
		}
		if let Some(step) = &m.step {
			let data = DataTx {
				conn: &tx,
				schema: &m.schema,
			};
			(step.0)(&data).map_err(|e| {
				Error::Internal(format!("data migration {}: {e}", m.name))
			})?;
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

/// Whether a migration ran already; in an empty database (`check` on a
/// new file) none did.
pub(crate) fn is_applied(conn: &Connection, name: &str) -> Result<bool> {
	let migrated: bool = conn.query_row(
		"SELECT COUNT(*) FROM sqlite_master WHERE name = 'schema_migrations'",
		[],
		|r| r.get(0),
	)?;
	if !migrated {
		return Ok(false);
	}
	Ok(conn.query_row(
		"SELECT COUNT(*) FROM schema_migrations WHERE name = ?1",
		[name],
		|r| r.get(0),
	)?)
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
			for table in ["entry_sites"].iter().chain(&INDEX_TABLES) {
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
	let end = next_position(conn, &section, parent)?;
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
	each_doc(conn, &["drafts", "revisions"], |of, doc| {
		// an empty type is the entry's
		for sd in doc.values_mut() {
			if sd.ty.is_empty() || keep(&of.section, &sd.ty) {
				continue;
			}
			sd.fields = match (schema.get(&sd.ty), schema.get(&of.ty)) {
				(Some(from), Some(to)) => convert(from, to, &sd.fields),
				_ => Value::Object(Default::default()),
			};
			sd.ty = of.ty.clone();
		}
		Ok(())
	})
}

/// Changes the content of drafts and revisions.
fn change_docs(
	conn: &Connection,
	tables: &[&str],
	mut f: impl FnMut(&mut Doc),
) -> Result<()> {
	each_doc(conn, tables, |_, doc| {
		f(doc);
		Ok(())
	})
}

/// The entry of a draft or revision.
pub(super) struct DocOf {
	pub entry_id: i64,
	pub section: String,
	/// The entry's type, a draft or revision may have another.
	pub ty: String,
}

/// Calls `f` with every draft or revision of `tables` and writes the
/// ones it changed.
pub(super) fn each_doc<E: From<Error>>(
	conn: &Connection,
	tables: &[&str],
	mut f: impl FnMut(&DocOf, &mut Doc) -> std::result::Result<(), E>,
) -> std::result::Result<(), E> {
	for table in tables {
		let key = if *table == "drafts" { "entry_id" } else { "id" };
		let rows = doc_rows(conn, table, key)?;

		for (id, raw, of) in rows {
			let mut doc: Doc = parse(&raw)?;
			let before = doc.clone();
			f(&of, &mut doc)?;
			// unchanged rows stay as they are
			if doc == before {
				continue;
			}
			let doc = serde_json::to_string(&doc)
				.map_err(|e| Error::Internal(format!("stored content: {e}")))?;
			conn.execute(
				&format!("UPDATE {table} SET doc = ?1 WHERE {key} = ?2"),
				params![doc, id],
			)
			.map_err(Error::from)?;
		}
	}
	Ok(())
}

/// The key, the doc and the entry of each row of a table of docs.
fn doc_rows(
	conn: &Connection,
	table: &str,
	key: &str,
) -> Result<Vec<(i64, String, DocOf)>> {
	let mut stmt = conn.prepare(&format!(
		"SELECT d.{key}, d.doc, e.id, e.section, e.type FROM {table} d
		JOIN entries e ON e.id = d.entry_id"
	))?;
	let rows = stmt
		.query_map([], |r| {
			let of = DocOf {
				entry_id: r.get(2)?,
				section: r.get(3)?,
				ty: r.get(4)?,
			};
			Ok((r.get(0)?, r.get(1)?, of))
		})?
		.collect::<rusqlite::Result<_>>()?;
	Ok(rows)
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
pub(crate) fn create_singles(conn: &Connection, schema: &Schema) -> Result<()> {
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
	reindex_all(conn, schema)?;
	for section in &schema.sections {
		refresh_uris(conn, schema, section, None)?;
	}
	Ok(())
}

#[cfg(test)]
#[path = "apply_tests.rs"]
mod tests;
