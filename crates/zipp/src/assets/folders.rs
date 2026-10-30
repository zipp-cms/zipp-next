//! Folders of assets, like Craft's: nested, an asset in one or on the top.
//! Only the database has them, the files stay where they are, so moving is
//! no copy and urls don't change.

use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

use crate::db::now;
use crate::error::{Error, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
	pub id: i64,
	/// `None` on the top
	pub parent_id: Option<i64>,
	pub name: String,
	/// the assets right in it
	pub assets: i64,
}

/// All folders, by name; the studio makes the tree.
pub fn folders(conn: &Connection) -> Result<Vec<Folder>> {
	let mut stmt = conn.prepare(
		"SELECT id, parent_id, name,
			(SELECT COUNT(*) FROM assets a WHERE a.folder_id = f.id)
		FROM asset_folders f ORDER BY name COLLATE NOCASE",
	)?;
	let all = stmt
		.query_map([], |r| {
			Ok(Folder {
				id: r.get(0)?,
				parent_id: r.get(1)?,
				name: r.get(2)?,
				assets: r.get(3)?,
			})
		})?
		.collect::<rusqlite::Result<_>>()?;
	Ok(all)
}

fn folder(conn: &Connection, id: i64) -> Result<Folder> {
	folders(conn)?
		.into_iter()
		.find(|f| f.id == id)
		.ok_or(Error::NotFound)
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
	if name.is_empty() || name.contains('/') {
		return Err(Error::Request("a name, without /".into()));
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

fn check_parent(conn: &Connection, parent: Option<i64>) -> Result<()> {
	match parent {
		Some(p) => folder(conn, p).map(|_| ()),
		None => Ok(()),
	}
}

pub fn create_folder(
	conn: &Connection,
	parent: Option<i64>,
	name: &str,
) -> Result<Folder> {
	check_parent(conn, parent)?;
	let name = check_name(conn, parent, name, None)?;
	conn.execute(
		"INSERT INTO asset_folders (parent_id, name, created_at)
		VALUES (?1, ?2, ?3)",
		params![parent, name, now()],
	)?;
	folder(conn, conn.last_insert_rowid())
}

/// A new name or place; not into itself or a folder in it.
pub fn update_folder(
	conn: &Connection,
	id: i64,
	parent: Option<i64>,
	name: &str,
) -> Result<Folder> {
	folder(conn, id)?;
	check_parent(conn, parent)?;
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
pub fn delete_folder(conn: &Connection, id: i64) -> Result<()> {
	let f = folder(conn, id)?;
	let folders_in: i64 = conn.query_row(
		"SELECT COUNT(*) FROM asset_folders WHERE parent_id = ?1",
		[id],
		|r| r.get(0),
	)?;
	if f.assets > 0 || folders_in > 0 {
		return Err(Error::Request(format!("{} isn't empty", f.name)));
	}
	conn.execute("DELETE FROM asset_folders WHERE id = ?1", [id])?;
	Ok(())
}

/// Puts assets into a folder, `None` on the top.
pub fn move_assets(
	conn: &Connection,
	ids: &[i64],
	folder_id: Option<i64>,
) -> Result<()> {
	check_parent(conn, folder_id)?;
	for id in ids {
		let n = conn.execute(
			"UPDATE assets SET folder_id = ?1 WHERE id = ?2",
			params![folder_id, id],
		)?;
		if n == 0 {
			return Err(Error::NotFound);
		}
	}
	Ok(())
}
