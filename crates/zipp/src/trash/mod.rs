//! The trash, like Craft's: deleting an entry or an asset in the studio
//! moves it here. For everything else it is gone (lists, GraphQL, the
//! website, relations to it), its content stays and comes back with
//! [`restore_entry`]. After [`KEEP_DAYS`] it is deleted for good.
//!
//! An entry of a structure goes with its children and comes back with
//! them; its uris are free while it is in the trash, a restored entry
//! whose uri is taken meanwhile gets another slug (`about-2`).

use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::auth::Authed;
use crate::db::now;
use crate::entries::reindex;
use crate::error::{Error, Result};
use crate::schema::{Access, Schema, SectionKind};

pub mod api;
#[cfg(test)]
mod api_tests;
mod routes;
#[cfg(test)]
mod tests;

pub use routes::routes;

/// How long something stays in the trash.
pub const KEEP_DAYS: i64 = 30;

const DAY_MS: i64 = 24 * 60 * 60 * 1000;

/// An entry in the trash, with the children that went with it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrashedEntry {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	/// in the primary site
	pub title: String,
	/// the children that come back with it
	pub children: i64,
	pub deleted_at: i64,
	/// the email of who deleted it
	pub deleted_by: Option<String>,
}

/// An asset in the trash.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrashedAsset {
	#[serde(flatten)]
	pub asset: crate::assets::Asset,
	pub deleted_at: i64,
	pub deleted_by: Option<String>,
}

/// What is in the trash, the newest first.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trash {
	pub entries: Vec<TrashedEntry>,
	pub assets: Vec<TrashedAsset>,
	/// [`KEEP_DAYS`]
	pub keep_days: i64,
}

/// An entry and the entries below it that are in the same state: live
/// ones to trash, or the ones trashed with it (the same `deleted_at`).
/// Parents before their children.
fn with_children(
	conn: &Connection,
	id: i64,
	deleted_at: Option<i64>,
) -> Result<Vec<i64>> {
	let mut stmt = conn.prepare(
		"WITH RECURSIVE tree(id, depth) AS (
			SELECT ?1, 0
			UNION ALL
			SELECT e.id, t.depth + 1 FROM entries e JOIN tree t
			ON e.parent_id = t.id WHERE e.deleted_at IS ?2
		)
		SELECT id FROM tree ORDER BY depth",
	)?;
	let ids = stmt
		.query_map(params![id, deleted_at], |r| r.get(0))?
		.collect::<rusqlite::Result<_>>()?;
	Ok(ids)
}

/// The section of an entry in the trash.
pub fn trashed_section(conn: &Connection, id: i64) -> Result<String> {
	conn.query_row(
		"SELECT section FROM entries WHERE id = ?1
		AND deleted_at IS NOT NULL",
		[id],
		|r| r.get(0),
	)
	.optional()?
	.ok_or(Error::NotFound)
}

/// Moves an entry to the trash, in a structure with its children.
pub fn trash_entry(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	user_id: Option<i64>,
) -> Result<()> {
	let section = crate::entries::section_of(conn, id)?;
	let single = schema
		.get_section(&section)
		.is_some_and(|s| s.kind == SectionKind::Single);
	if single {
		return Err(Error::Request("the entry of a single stays".into()));
	}

	let ids = with_children(conn, id, None)?;
	let tx = conn.unchecked_transaction()?;
	let now = now();
	for e in &ids {
		tx.execute(
			"UPDATE entries SET deleted_at = ?1, deleted_by = ?2
			WHERE id = ?3",
			params![now, user_id, e],
		)?;
		// its uris are free, nothing links to it or finds it by a value
		for sql in [
			"UPDATE entry_sites SET uri = NULL WHERE entry_id = ?1",
			"DELETE FROM entry_values WHERE entry_id = ?1",
			"DELETE FROM entry_relations WHERE entry_id = ?1",
			"DELETE FROM asset_relations WHERE entry_id = ?1",
		] {
			tx.execute(sql, [e])?;
		}
	}
	tx.commit()?;
	Ok(())
}

/// Brings an entry back from the trash with the children that went with
/// it, at the end of its siblings; under its parent if that isn't in the
/// trash, else on the top. Its section, for the change hooks.
pub fn restore_entry(
	conn: &Connection,
	schema: &Schema,
	id: i64,
) -> Result<String> {
	let (section, parent, deleted_at): (String, Option<i64>, i64) = conn
		.query_row(
			"SELECT section, parent_id, deleted_at FROM entries
			WHERE id = ?1 AND deleted_at IS NOT NULL",
			[id],
			|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
		)
		.optional()?
		.ok_or(Error::NotFound)?;
	let sec = schema
		.get_section(&section)
		.ok_or_else(|| Error::Request(format!("no section {section}")))?;
	let parent = match parent {
		Some(p) => conn
			.query_row(
				"SELECT id FROM entries WHERE id = ?1 AND deleted_at IS NULL",
				[p],
				|r| r.get::<_, i64>(0),
			)
			.optional()?,
		None => None,
	};

	let ids = with_children(conn, id, Some(deleted_at))?;
	let tx = conn.unchecked_transaction()?;
	let position: i64 = tx.query_row(
		"SELECT IFNULL(MAX(position) + 1, 0) FROM entries
		WHERE section = ?1 AND parent_id IS ?2 AND deleted_at IS NULL",
		params![section, parent],
		|r| r.get(0),
	)?;
	tx.execute(
		"UPDATE entries SET parent_id = ?1, position = ?2 WHERE id = ?3",
		params![parent, position, id],
	)?;
	for e in &ids {
		tx.execute(
			"UPDATE entries SET deleted_at = NULL, deleted_by = NULL
			WHERE id = ?1",
			[e],
		)?;
		// parents first: a child's uri follows the restored one
		let (ty, parent_id): (String, Option<i64>) = tx.query_row(
			"SELECT type, parent_id FROM entries WHERE id = ?1",
			[e],
			|r| Ok((r.get(0)?, r.get(1)?)),
		)?;
		for site in &schema.sites {
			let (slug, fields): (String, String) = tx.query_row(
				"SELECT slug, fields FROM entry_sites
				WHERE entry_id = ?1 AND site = ?2",
				params![e, site.handle],
				|r| Ok((r.get(0)?, r.get(1)?)),
			)?;
			let fields: Value =
				serde_json::from_str(&fields).unwrap_or_default();
			let (slug, uri) = free_uri(
				&tx,
				sec,
				&ty,
				&slug,
				parent_id,
				&site.handle,
				&fields,
			)?;
			tx.execute(
				"UPDATE entry_sites SET slug = ?1, uri = ?2
				WHERE entry_id = ?3 AND site = ?4",
				params![slug, uri, e, site.handle],
			)?;
			reindex(&tx, schema, *e, &site.handle, &ty, &fields)?;
		}
	}
	tx.commit()?;
	Ok(section)
}

/// The slug and uri of a restored entry in a site: its own, or with
/// `-2`, `-3`, … when another entry has that uri now.
fn free_uri(
	conn: &Connection,
	section: &crate::schema::Section,
	ty: &str,
	slug: &str,
	parent: Option<i64>,
	site: &str,
	fields: &Value,
) -> Result<(String, Option<String>)> {
	let parent_uri: Option<String> = match parent {
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
	for n in 1.. {
		let candidate = match n {
			1 => slug.to_string(),
			n => format!("{slug}-{n}"),
		};
		let uri = section.render_uri(
			ty,
			&candidate,
			parent_uri.as_deref(),
			site,
			fields,
		);
		let Some(u) = &uri else {
			return Ok((candidate, None));
		};
		let taken: bool = conn.query_row(
			"SELECT EXISTS (SELECT 1 FROM entry_sites
			WHERE site = ?1 AND uri = ?2)",
			params![site, u],
			|r| r.get(0),
		)?;
		if !taken {
			return Ok((candidate, uri));
		}
	}
	unreachable!()
}

/// Moves an asset to the trash.
pub fn trash_asset(
	conn: &Connection,
	id: i64,
	user_id: Option<i64>,
) -> Result<()> {
	let n = conn.execute(
		"UPDATE assets SET deleted_at = ?1, deleted_by = ?2
		WHERE id = ?3 AND deleted_at IS NULL",
		params![now(), user_id, id],
	)?;
	if n == 0 {
		return Err(Error::NotFound);
	}
	Ok(())
}

/// Brings an asset back from the trash, into its folder.
pub fn restore_asset(conn: &Connection, id: i64) -> Result<()> {
	let n = conn.execute(
		"UPDATE assets SET deleted_at = NULL, deleted_by = NULL
		WHERE id = ?1 AND deleted_at IS NOT NULL",
		[id],
	)?;
	if n == 0 {
		return Err(Error::NotFound);
	}
	Ok(())
}

/// Deletes an entry of the trash for good, with the children that went
/// with it.
pub fn purge_entry(conn: &Connection, id: i64) -> Result<()> {
	let n = conn.execute(
		"DELETE FROM entries WHERE id = ?1 AND deleted_at IS NOT NULL",
		[id],
	)?;
	if n == 0 {
		return Err(Error::NotFound);
	}
	Ok(())
}

/// Deletes an asset of the trash for good, with its files.
pub fn purge_asset(conn: &Connection, files: &Path, id: i64) -> Result<()> {
	let trashed: bool = conn.query_row(
		"SELECT EXISTS (SELECT 1 FROM assets
		WHERE id = ?1 AND deleted_at IS NOT NULL)",
		[id],
		|r| r.get(0),
	)?;
	if !trashed {
		return Err(Error::NotFound);
	}
	crate::assets::delete(conn, files, id)
}

/// What a user sees of the trash: entries of the sections they may
/// delete in, assets if they may delete those. An entry that went with
/// its parent shows as one of its children.
pub fn list(
	conn: &Connection,
	schema: &Schema,
	auth: &Authed,
) -> Result<Trash> {
	let primary = &schema.primary_site().handle;
	let mut stmt = conn.prepare(
		"SELECT e.id, e.section, e.type, s.title, e.deleted_at,
			(SELECT email FROM users WHERE id = e.deleted_by),
			(WITH RECURSIVE tree(id) AS (
				SELECT e.id
				UNION ALL
				SELECT c.id FROM entries c JOIN tree t ON c.parent_id = t.id
				WHERE c.deleted_at = e.deleted_at
			) SELECT COUNT(*) - 1 FROM tree)
		FROM entries e
		JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?1
		LEFT JOIN entries p ON p.id = e.parent_id
		WHERE e.deleted_at IS NOT NULL
		AND p.deleted_at IS NOT e.deleted_at
		ORDER BY e.deleted_at DESC, e.id",
	)?;
	let entries = stmt
		.query_map([primary], |r| {
			Ok(TrashedEntry {
				id: r.get(0)?,
				section: r.get(1)?,
				ty: r.get(2)?,
				title: r.get(3)?,
				deleted_at: r.get(4)?,
				deleted_by: r.get(5)?,
				children: r.get(6)?,
			})
		})?
		.collect::<rusqlite::Result<Vec<_>>>()?
		.into_iter()
		.filter(|e| auth.can(schema, &e.section, Access::Delete))
		.collect();
	let assets = match auth.can_assets(schema, Access::Delete) {
		true => crate::assets::trashed(conn)?,
		false => vec![],
	};
	Ok(Trash {
		entries,
		assets,
		keep_days: KEEP_DAYS,
	})
}

/// Deletes for good what is longer in the trash than [`KEEP_DAYS`].
pub fn purge_old(conn: &Connection, files: &Path) -> Result<()> {
	let before = now() - KEEP_DAYS * DAY_MS;
	// the children go with their parent
	conn.execute("DELETE FROM entries WHERE deleted_at < ?1", [before])?;
	let mut stmt =
		conn.prepare("SELECT id FROM assets WHERE deleted_at < ?1")?;
	let ids = stmt
		.query_map([before], |r| r.get(0))?
		.collect::<rusqlite::Result<Vec<i64>>>()?;
	for id in ids {
		crate::assets::delete(conn, files, id)?;
	}
	Ok(())
}
