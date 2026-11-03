//! Folders of assets, like Craft's: nested, an asset in one or on the top.
//! Only the database has them, the files stay where they are, so moving is
//! no copy and urls don't change.

use std::collections::HashMap;

use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

use crate::db::now;
use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Folder {
	pub id: i64,
	/// `None` on the top
	pub parent_id: Option<i64>,
	pub name: String,
	/// the assets right in it
	pub assets: i64,
}

/// All folders, by name; the studio makes the tree.
pub(crate) fn folders(conn: &Connection) -> Result<Vec<Folder>> {
	let mut stmt = conn.prepare(
		"SELECT id, parent_id, name,
			(SELECT COUNT(*) FROM assets a
				WHERE a.folder_id = f.id AND a.deleted_at IS NULL)
		FROM asset_folders f ORDER BY name COLLATE NOCASE",
	)?;
	let all = stmt
		.query_map([], from_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(all)
}

/// How many assets are in no folder, on the top.
pub(crate) fn top_count(conn: &Connection) -> Result<i64> {
	let n = conn.query_row(
		"SELECT COUNT(*) FROM assets
		WHERE folder_id IS NULL AND deleted_at IS NULL",
		[],
		|r| r.get(0),
	)?;
	Ok(n)
}

pub(crate) fn folder(conn: &Connection, id: i64) -> Result<Folder> {
	conn.query_row(
		"SELECT id, parent_id, name,
			(SELECT COUNT(*) FROM assets a
				WHERE a.folder_id = f.id AND a.deleted_at IS NULL)
		FROM asset_folders f WHERE id = ?1",
		[id],
		from_row,
	)
	.optional()?
	.ok_or(Error::NotFound)
}

fn from_row(r: &rusqlite::Row) -> rusqlite::Result<Folder> {
	Ok(Folder {
		id: r.get(0)?,
		parent_id: r.get(1)?,
		name: r.get(2)?,
		assets: r.get(3)?,
	})
}

/// A name for a folder in `parent`: not empty, not the one of another
/// folder there (`id` is the folder's own).
fn check_name(
	conn: &Connection,
	parent: Option<i64>,
	name: &str,
	id: Option<i64>,
) -> Result<String> {
	let name = name.trim();
	// `.` and `..` would be paths in a zip, `\` a folder on Windows
	if name.is_empty()
		|| name.contains(['/', '\\'])
		|| name == "."
		|| name == ".."
	{
		return Err(Error::Request(
			"a name, without / or \\ and not . or ..".into(),
		));
	}
	let taken: Option<i64> = conn
		.query_row(
			"SELECT id FROM asset_folders WHERE parent_id IS ?1
			AND name = ?2 COLLATE NOCASE AND id IS NOT ?3",
			params![parent, name, id],
			|r| r.get(0),
		)
		.optional()?;
	if taken.is_some() {
		return Err(Error::Request(format!("there is a folder {name} here")));
	}
	Ok(name.into())
}

/// That the folder exists, `None` is the top.
pub(crate) fn check_folder(
	conn: &Connection,
	parent: Option<i64>,
) -> Result<()> {
	match parent {
		Some(p) => folder(conn, p).map(|_| ()),
		None => Ok(()),
	}
}

pub(crate) fn create_folder(
	conn: &Connection,
	parent: Option<i64>,
	name: &str,
) -> Result<Folder> {
	check_folder(conn, parent)?;
	let name = check_name(conn, parent, name, None)?;
	conn.execute(
		"INSERT INTO asset_folders (parent_id, name, created_at)
		VALUES (?1, ?2, ?3)",
		params![parent, name, now()],
	)?;
	folder(conn, conn.last_insert_rowid())
}

/// The folder of this name in `parent`, made when there is none; names
/// match like [`check_name`] compares them.
pub(crate) fn folder_named(
	conn: &Connection,
	parent: Option<i64>,
	name: &str,
) -> Result<i64> {
	let found: Option<i64> = conn
		.query_row(
			"SELECT id FROM asset_folders WHERE parent_id IS ?1
			AND name = ?2 COLLATE NOCASE",
			params![parent, name.trim()],
			|r| r.get(0),
		)
		.optional()?;
	match found {
		Some(id) => Ok(id),
		None => Ok(create_folder(conn, parent, name)?.id),
	}
}

/// A new name or place; not into itself or a folder in it.
pub(crate) fn update_folder(
	conn: &Connection,
	id: i64,
	parent: Option<i64>,
	name: &str,
) -> Result<Folder> {
	folder(conn, id)?;
	check_folder(conn, parent)?;
	let all = folders(conn)?;
	let mut at = parent;
	while let Some(p) = at {
		if p == id {
			return Err(Error::Request("not into itself".into()));
		}
		at = all.iter().find(|f| f.id == p).and_then(|f| f.parent_id);
	}
	let name = check_name(conn, parent, name, Some(id))?;
	conn.execute(
		"UPDATE asset_folders SET parent_id = ?1, name = ?2 WHERE id = ?3",
		params![parent, name, id],
	)?;
	folder(conn, id)
}

/// Deletes an empty folder: one with files or folders stays, so nothing
/// goes by mistake.
pub(crate) fn delete_folder(conn: &Connection, id: i64) -> Result<()> {
	let f = folder(conn, id)?;
	let folders_in: i64 = conn.query_row(
		"SELECT COUNT(*) FROM asset_folders WHERE parent_id = ?1",
		[id],
		|r| r.get(0),
	)?;
	if f.assets > 0 || folders_in > 0 {
		return Err(Error::Conflict(format!("{} isn't empty", f.name)));
	}
	// assets of it in the trash come back on the top
	conn.execute(
		"UPDATE assets SET folder_id = NULL WHERE folder_id = ?1",
		[id],
	)?;
	conn.execute("DELETE FROM asset_folders WHERE id = ?1", [id])?;
	Ok(())
}

/// Puts assets into a folder, `None` on the top.
pub(crate) fn move_assets(
	conn: &Connection,
	ids: &[i64],
	folder_id: Option<i64>,
) -> Result<()> {
	check_folder(conn, folder_id)?;
	for id in ids {
		let n = conn.execute(
			"UPDATE assets SET folder_id = ?1
			WHERE id = ?2 AND deleted_at IS NULL",
			params![folder_id, id],
		)?;
		if n == 0 {
			return Err(Error::NotFound);
		}
	}
	Ok(())
}

/// The path of every folder by id, `Clubs/Bern`.
pub(crate) fn folder_paths(conn: &Connection) -> Result<HashMap<i64, String>> {
	let all = folders(conn)?;
	let by_id: HashMap<i64, &Folder> = all.iter().map(|f| (f.id, f)).collect();
	let mut paths = HashMap::new();
	for f in &all {
		let mut names = vec![f.name.as_str()];
		let mut at = f.parent_id;
		// a parent is never in itself, see update_folder
		while let Some(p) = at.and_then(|p| by_id.get(&p)) {
			names.push(&p.name);
			at = p.parent_id;
		}
		names.reverse();
		paths.insert(f.id, names.join("/"));
	}
	Ok(paths)
}

/// The folder of a path and every folder in it, like a volume of Craft
/// with its folders; names match like [`check_name`] compares them.
pub(crate) fn folders_under(
	paths: &HashMap<i64, String>,
	path: &str,
) -> Vec<i64> {
	let path = path.trim_matches('/').to_lowercase();
	let inner = format!("{path}/");
	let mut ids: Vec<i64> = paths
		.iter()
		.filter(|(_, p)| {
			let p = p.to_lowercase();
			p == path || p.starts_with(&inner)
		})
		.map(|(id, _)| *id)
		.collect();
	ids.sort();
	ids
}
