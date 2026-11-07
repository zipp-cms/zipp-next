//! Assets into the folders of the fields that use them, like the studio
//! files uploads: for the assets of an import or of a field that got a
//! folder later.

use std::collections::{BTreeMap, BTreeSet};

use rusqlite::{Connection, params};
use serde_json::Value;

use super::folder_named;
use crate::content::asset_folders;
use crate::error::{Error, Result};
use crate::schema::Schema;

/// What [`sort_assets`] did.
#[derive(Debug, PartialEq)]
pub(crate) struct Sorted {
	/// put into another folder
	pub moved: usize,
	/// used by several entries or in fields with other folders, they
	/// stay
	pub shared: usize,
}

/// Puts each asset that the live content of one entry uses in asset
/// fields with a folder into that folder, with the entry's `{slug}`,
/// `{section}` and `{type}` like an upload in the studio (a missing one
/// ends the path). Assets of several entries or folders stay where they
/// are, so do the ones in no such field and in the trash.
pub(crate) fn sort_assets(
	conn: &Connection,
	schema: &Schema,
) -> Result<Sorted> {
	let mut wanted: BTreeMap<i64, BTreeSet<String>> = BTreeMap::new();
	let mut stmt = conn.prepare(
		"SELECT e.section, e.type, s.slug, s.fields
		FROM entries e JOIN entry_sites s ON s.entry_id = e.id
		WHERE e.deleted_at IS NULL",
	)?;
	let rows = stmt.query_map([], |r| {
		Ok((
			r.get::<_, String>(0)?,
			r.get::<_, String>(1)?,
			r.get::<_, String>(2)?,
			r.get::<_, String>(3)?,
		))
	})?;
	for row in rows {
		let (section, ty, slug, fields) = row?;
		let Some(comp) = schema.get(&ty) else {
			continue;
		};
		let fields: Value = serde_json::from_str(&fields)
			.map_err(|e| Error::Internal(format!("stored content: {e}")))?;
		for (folder, ids) in asset_folders(schema, comp, &fields) {
			let path = fill(folder, &slug, &section, &ty);
			for id in ids {
				wanted.entry(id).or_default().insert(path.clone());
			}
		}
	}

	let mut sorted = Sorted {
		moved: 0,
		shared: 0,
	};
	for (id, paths) in wanted {
		let mut paths = paths.into_iter();
		let (Some(path), None) = (paths.next(), paths.next()) else {
			sorted.shared += 1;
			continue;
		};
		if path.is_empty() {
			continue;
		}
		let mut folder = None;
		for name in path.split('/') {
			folder = Some(folder_named(conn, folder, name)?);
		}
		sorted.moved += conn.execute(
			"UPDATE assets SET folder_id = ?1
			WHERE id = ?2 AND deleted_at IS NULL AND folder_id IS NOT ?1",
			params![folder, id],
		)?;
	}
	Ok(sorted)
}

/// A field's folder with the entry's values, like `fieldFolder` of the
/// studio: a name with a missing one ends the path.
fn fill(folder: &str, slug: &str, section: &str, ty: &str) -> String {
	let mut names = vec![];
	for name in folder.split('/') {
		let missing = name.contains("{slug}") && slug.is_empty();
		if missing {
			break;
		}
		names.push(
			name.replace("{slug}", slug)
				.replace("{section}", section)
				.replace("{type}", ty),
		);
	}
	names.join("/")
}
