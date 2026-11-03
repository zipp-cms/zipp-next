//! Backups of a site: a snapshot of the database and its files in a
//! folder per backup, `<dir>/2026-11-01T08-00-00/` with `zipp.db` and
//! `files/`. Files that didn't change since the last backup are hard
//! links to it, so a big asset library takes its space once.
//!
//! That layout is the format of a backup, the version of its content is
//! the one of the database (`PRAGMA user_version`, see `db.rs`): a
//! backup of an older zipp is migrated on start, one of a newer is
//! refused.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use rusqlite::Connection;

use crate::db::{MIGRATIONS, iso, newer, parse_iso, version_of};

#[cfg(test)]
mod tests;

/// What [`backup`] wrote.
#[derive(Debug)]
pub struct Written {
	pub dir: PathBuf,
	/// Files copied, new or changed since the last backup
	pub copied: usize,
	/// Files linked to the last backup
	pub linked: usize,
}

/// A backup of the database and the files into a new folder of `to`,
/// named after `now` (milliseconds). The server may run meanwhile:
/// `VACUUM INTO` reads one consistent state, a copy of a WAL database
/// would not.
pub fn backup(
	db: &Path,
	files: &Path,
	to: &Path,
	now: i64,
) -> Result<Written, String> {
	let err = |p: &Path, e: io::Error| format!("{}: {e}", p.display());
	if !db.is_file() {
		return Err(format!("{} is no database", db.display()));
	}
	fs::create_dir_all(to).map_err(|e| err(to, e))?;
	let name = name_of(now);
	let dir = to.join(&name);
	if dir.exists() {
		return Err(format!("{} exists already", dir.display()));
	}
	let last = list(to).map_err(|e| err(to, e))?.pop().map(|(_, p)| p);

	// written under a hidden name: a folder that has the name is
	// complete, one that broke off is removed by the next backup
	remove_partial(to).map_err(|e| err(to, e))?;
	let tmp = to.join(format!(".{name}.partial"));
	fs::create_dir(&tmp).map_err(|e| err(&tmp, e))?;

	// the database first: files uploaded meanwhile are only extra
	let conn = Connection::open(db).map_err(|e| format!("{e}"))?;
	conn.execute("VACUUM INTO ?1", [tmp.join(DB).to_string_lossy()])
		.map_err(|e| format!("the database: {e}"))?;
	drop(conn);

	let mut written = Written {
		dir: dir.clone(),
		copied: 0,
		linked: 0,
	};
	if files.exists() {
		let last = last.map(|l| l.join(FILES));
		copy_files(files, &tmp.join(FILES), last.as_deref(), &mut written)
			.map_err(|e| err(files, e))?;
	} else {
		fs::create_dir(tmp.join(FILES)).map_err(|e| err(&tmp, e))?;
	}

	fs::rename(&tmp, &dir).map_err(|e| err(&dir, e))?;
	Ok(written)
}

const DB: &str = "zipp.db";
const FILES: &str = "files";

/// Copies a folder, a file that is in `last` with the same size and
/// time becomes a hard link to it. Uploads are never changed in place
/// (a replaced file is a new one), so a link can't change later.
/// Hidden entries are left out: the transforms (`.t`), which are made
/// again, and temporary files.
fn copy_files(
	from: &Path,
	to: &Path,
	last: Option<&Path>,
	written: &mut Written,
) -> io::Result<()> {
	fs::create_dir(to)?;
	for e in fs::read_dir(from)? {
		let e = e?;
		if e.file_name().to_string_lossy().starts_with('.') {
			continue;
		}
		match copy_entry(&e, to, last, written) {
			// deleted or renamed while the backup runs, the database of
			// the backup is from before
			Err(err) if err.kind() == io::ErrorKind::NotFound => {
				let path = e.path();
				tracing::warn!("backup: {} is gone, left out", path.display());
			}
			r => r?,
		}
	}
	Ok(())
}

fn copy_entry(
	e: &fs::DirEntry,
	to: &Path,
	last: Option<&Path>,
	written: &mut Written,
) -> io::Result<()> {
	let name = e.file_name();
	let (src, dst) = (e.path(), to.join(&name));
	let last = last.map(|l| l.join(&name));
	let meta = e.metadata()?;
	if meta.is_dir() {
		return copy_files(&src, &dst, last.as_deref(), written);
	}

	let modified = meta.modified()?;
	if let Some(l) = last
		&& same_file(&l, meta.len(), modified)
		&& fs::hard_link(&l, &dst).is_ok()
	{
		written.linked += 1;
		return Ok(());
	}
	fs::copy(&src, &dst)?;
	// the time of the original, so the next backup can compare
	fs::File::options()
		.write(true)
		.open(&dst)?
		.set_modified(modified)?;
	written.copied += 1;
	Ok(())
}

fn same_file(path: &Path, len: u64, modified: SystemTime) -> bool {
	fs::metadata(path)
		.is_ok_and(|m| m.len() == len && m.modified().ok() == Some(modified))
}

fn remove_partial(to: &Path) -> io::Result<()> {
	for e in fs::read_dir(to)? {
		let path = e?.path();
		let name = path.file_name().unwrap_or_default().to_string_lossy();
		if name.starts_with('.') && name.ends_with(".partial") {
			fs::remove_dir_all(&path)?;
		}
	}
	Ok(())
}

/// `2026-11-01T08-00-00`: sorts by time and works on every filesystem.
fn name_of(ms: i64) -> String {
	iso(ms)[..19].replace(':', "-")
}

fn time_of(name: &str) -> Option<i64> {
	let (day, time) = name.split_once('T')?;
	parse_iso(&format!("{day}T{}", time.replace('-', ":")))
}

/// The backups in `dir`, the oldest first, with their time.
pub fn list(dir: &Path) -> io::Result<Vec<(i64, PathBuf)>> {
	let mut found = vec![];
	for e in fs::read_dir(dir)? {
		let path = e?.path();
		let name = path.file_name().unwrap_or_default().to_string_lossy();
		if let Some(time) = time_of(&name)
			&& path.join(DB).is_file()
		{
			found.push((time, path));
		}
	}
	found.sort();
	Ok(found)
}

/// Removes all but the last `keep` backups, returns the removed.
pub fn prune(dir: &Path, keep: usize) -> io::Result<Vec<PathBuf>> {
	let found = list(dir)?;
	let old = found.len().saturating_sub(keep);
	let mut removed = vec![];
	for (_, path) in found.into_iter().take(old) {
		fs::remove_dir_all(&path)?;
		removed.push(path);
	}
	Ok(removed)
}

/// Puts a backup back: the database and the files are replaced. Refuses
/// while the database is open elsewhere, the server has to be stopped.
pub fn restore(from: &Path, db: &Path, files: &Path) -> Result<(), String> {
	let backup_db = from.join(DB);
	let backup_files = from.join(FILES);
	if !backup_db.is_file() || !backup_files.is_dir() {
		return Err(format!("{} is no backup", from.display()));
	}
	// checked before anything is replaced
	let version = Connection::open(&backup_db)
		.and_then(|c| version_of(&c))
		.map_err(|e| format!("the database: {e}"))?;
	if version > MIGRATIONS.len() {
		return Err(newer(version).to_string());
	}

	// leaving WAL mode needs the only connection: it fails while a
	// server has the database open
	let mut conn = Connection::open(db).map_err(|e| e.to_string())?;
	conn.pragma_update(None, "journal_mode", "DELETE")
		.map_err(|_| {
			format!("{} is in use, stop the server first", db.display())
		})?;
	conn.restore(rusqlite::MAIN_DB, &backup_db, None::<fn(_)>)
		.map_err(|e| format!("the database: {e}"))?;
	drop(conn);

	let err = |e: io::Error| format!("{}: {e}", files.display());
	if files.exists() {
		fs::remove_dir_all(files).map_err(err)?;
	}
	let mut written = Written {
		dir: files.to_path_buf(),
		copied: 0,
		linked: 0,
	};
	copy_files(&backup_files, files, None, &mut written).map_err(err)
}
