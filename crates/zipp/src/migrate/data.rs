//! Data migrations: a step in Rust for what the ops can't express, like
//! filling a new field from old ones. A migration file with `"data":
//! true` stands for it in the order of the migrations, the site registers
//! its step under the file's name with
//! [`Cms::data_migration`](crate::Cms::data_migration). The step runs
//! once, in the transaction of the other migrations, with the schema of
//! its file.
//!
//! [`DataTx::each`] changes every stored copy of an entry or block, like
//! the ops do: the live content of each site, drafts and revisions, also
//! of entries in the trash, so publishing a draft or restoring a revision
//! later brings the new shape, and the fields of assets. The indexes, the search and the uris are
//! rebuilt after the migrations, as after the ops.

use std::fmt;
use std::sync::Arc;

use rusqlite::{Connection, params};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::content::walk_mut;
use crate::migrate::apply::each_doc;
use crate::schema::{ASSET, Schema, USER};

/// What a step returns: `?` works on any error, it stops the migration
/// and rolls back all of them.
pub type DataResult =
	std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>;

/// A registered step, see [`Cms::data_migration`](crate::Cms::data_migration).
#[derive(Clone)]
pub(crate) struct DataStep(
	pub(crate) Arc<dyn Fn(&DataTx) -> DataResult + Send + Sync>,
);

impl fmt::Debug for DataStep {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("DataStep")
	}
}

/// What a step gets: its migration's transaction and schema.
pub struct DataTx<'a> {
	pub(crate) conn: &'a Connection,
	pub(crate) schema: &'a Schema,
}

/// The fields of one entry or block in one site, of the live content, a
/// draft or a revision, or of an asset or a user.
pub struct Content<'a> {
	/// The entry's id, of an asset or a user theirs.
	pub entry_id: i64,
	/// The entry's type, `asset` or `user` for their fields.
	pub ty: &'a str,
	/// Empty for an asset or a user, they have no sites.
	pub site: &'a str,
	/// The title of the entry in the site, of a block too.
	pub title: &'a str,
	pub fields: &'a mut Map<String, Value>,
}

impl Content<'_> {
	/// A field's value, `None` when it is missing or has another shape.
	pub fn get<T: DeserializeOwned>(&self, field: &str) -> Option<T> {
		let value = self.fields.get(field)?;
		serde_json::from_value(value.clone()).ok()
	}

	/// Sets a field; a value that isn't JSON (a map with keys that
	/// aren't strings) stops the step instead of writing null.
	pub fn set(&mut self, field: &str, value: impl Serialize) -> DataResult {
		let value = serde_json::to_value(value)?;
		self.fields.insert(field.into(), value);
		Ok(())
	}

	pub fn remove(&mut self, field: &str) -> Option<Value> {
		self.fields.remove(field)
	}
}

impl DataTx<'_> {
	/// The transaction, for what `each` can't do. The indexes are
	/// rebuilt afterwards, only the content needs to be written.
	pub fn conn(&self) -> &Connection {
		self.conn
	}

	/// The schema of the migration file, it still has the fields that a
	/// later migration removes.
	pub fn schema(&self) -> &Schema {
		self.schema
	}

	/// Calls `f` with every entry of an entry type or every block (or
	/// group) of a component, in every site, in the live content, drafts
	/// and revisions, and in the fields of assets (`"asset"`) and users
	/// (`"user"`). What `f` changes is written, an error stops.
	pub fn each(
		&self,
		component: &str,
		mut f: impl FnMut(&mut Content) -> DataResult,
	) -> DataResult {
		if self.schema.get(component).is_none() {
			return Err(format!("no component {component}").into());
		}

		let mut stmt = self.conn.prepare(
			"SELECT e.id, s.site, e.type, s.title, s.fields FROM entries e
			JOIN entry_sites s ON s.entry_id = e.id",
		)?;
		let rows = stmt
			.query_map([], |r| {
				Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
			})?
			.collect::<rusqlite::Result<Vec<(i64, String, String, String, String)>>>(
			)?;
		drop(stmt);

		for (id, site, ty, title, fields) in rows {
			let mut fields: Value = serde_json::from_str(&fields)?;
			let at = (id, site.as_str(), title.as_str());
			if self.run(component, &ty, &mut fields, at, &mut f)? {
				self.conn.execute(
					"UPDATE entry_sites SET fields = ?1
					WHERE entry_id = ?2 AND site = ?3",
					params![fields.to_string(), id, site],
				)?;
			}
		}

		each_doc(
			self.conn,
			&["drafts", "revisions"],
			|of, doc| -> DataResult {
				for (site, sd) in doc.iter_mut() {
					// an empty type is the entry's
					let ty = if sd.ty.is_empty() { &of.ty } else { &sd.ty };
					let at = (of.entry_id, site.as_str(), sd.title.as_str());
					self.run(component, ty, &mut sd.fields, at, &mut f)?;
				}
				Ok(())
			},
		)?;

		// assets have no sites and their title is their own
		let mut stmt = self.conn.prepare(
			"SELECT id, CASE title WHEN '' THEN filename ELSE title END,
			fields FROM assets",
		)?;
		let rows = stmt
			.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
			.collect::<rusqlite::Result<Vec<(i64, String, String)>>>()?;
		drop(stmt);

		for (id, title, fields) in rows {
			let mut fields: Value = serde_json::from_str(&fields)?;
			let at = (id, "", title.as_str());
			if self.run(component, ASSET, &mut fields, at, &mut f)? {
				self.conn.execute(
					"UPDATE assets SET fields = ?1 WHERE id = ?2",
					params![fields.to_string(), id],
				)?;
			}
		}

		// users neither, their title is their name
		let mut stmt =
			self.conn.prepare("SELECT id, name, fields FROM users")?;
		let rows = stmt
			.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
			.collect::<rusqlite::Result<Vec<(i64, String, String)>>>()?;
		drop(stmt);

		for (id, name, fields) in rows {
			let mut fields: Value = serde_json::from_str(&fields)?;
			let at = (id, "", name.as_str());
			if self.run(component, USER, &mut fields, at, &mut f)? {
				self.conn.execute(
					"UPDATE users SET fields = ?1 WHERE id = ?2",
					params![fields.to_string(), id],
				)?;
			}
		}

		Ok(())
	}

	/// Runs `f` over the objects of `component` in the fields of an entry
	/// of type `ty`, returns whether they changed.
	fn run(
		&self,
		component: &str,
		ty: &str,
		fields: &mut Value,
		(entry_id, site, title): (i64, &str, &str),
		f: &mut impl FnMut(&mut Content) -> DataResult,
	) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
		// a type the schema doesn't know has nothing to change
		let Some(comp) = self.schema.get(ty) else {
			return Ok(false);
		};
		let before = fields.clone();
		let mut res = Ok(());
		walk_mut(self.schema, comp, fields, &mut |c, obj| {
			if c.handle != component || res.is_err() {
				return;
			}
			res = f(&mut Content {
				entry_id,
				ty,
				site,
				title,
				fields: obj,
			});
		});
		res?;
		Ok(*fields != before)
	}
}

#[cfg(test)]
#[path = "data_tests.rs"]
mod tests;
