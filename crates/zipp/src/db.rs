use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

/// Tables of zipp itself, every entry is applied once, tracked with
/// `PRAGMA user_version`. Only ever append to this list. Changes of the
/// content model are schema migrations (`migrate.rs`), not these.
const MIGRATIONS: &[&str] = &[r#"
	CREATE TABLE entries (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		section TEXT NOT NULL,
		type TEXT NOT NULL,
		-- the tree of a structure, deleting a parent deletes its children
		parent_id INTEGER REFERENCES entries(id) ON DELETE CASCADE,
		position INTEGER NOT NULL DEFAULT 0,
		created_at INTEGER NOT NULL,
		updated_at INTEGER NOT NULL
	);
	CREATE INDEX entries_section ON entries(section);
	CREATE INDEX entries_parent ON entries(parent_id);

	-- every entry has a row per site
	CREATE TABLE entry_sites (
		entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
		site TEXT NOT NULL,
		title TEXT NOT NULL,
		slug TEXT NOT NULL,
		uri TEXT,
		published INTEGER NOT NULL DEFAULT 0,
		fields TEXT NOT NULL,
		updated_at INTEGER NOT NULL,
		PRIMARY KEY (entry_id, site)
	);
	CREATE UNIQUE INDEX entry_sites_uri ON entry_sites(site, uri);

	-- indexed fields, to filter and sort entries by them
	CREATE TABLE entry_values (
		entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
		site TEXT NOT NULL,
		field TEXT NOT NULL,
		text TEXT,
		num REAL
	);
	CREATE INDEX entry_values_entry ON entry_values(entry_id);
	CREATE INDEX entry_values_text ON entry_values(site, field, text);
	CREATE INDEX entry_values_num ON entry_values(site, field, num);

	-- every relation, also inside blocks, to find what links to an entry
	CREATE TABLE entry_relations (
		entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
		site TEXT NOT NULL,
		path TEXT NOT NULL,
		target_id INTEGER NOT NULL,
		position INTEGER NOT NULL
	);
	CREATE INDEX entry_relations_entry ON entry_relations(entry_id);
	CREATE INDEX entry_relations_target ON entry_relations(target_id);

	-- unpublished changes of an entry, all sites as JSON, one per entry
	CREATE TABLE drafts (
		entry_id INTEGER PRIMARY KEY REFERENCES entries(id) ON DELETE CASCADE,
		doc TEXT NOT NULL,
		user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
		updated_at INTEGER NOT NULL
	);

	-- the published state after each save, to look back and restore
	CREATE TABLE revisions (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
		doc TEXT NOT NULL,
		user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
		created_at INTEGER NOT NULL
	);
	CREATE INDEX revisions_entry ON revisions(entry_id);

	-- short lived tokens that show the draft of an entry on the website
	CREATE TABLE preview_tokens (
		token TEXT PRIMARY KEY,
		-- NULL: every draft, like the AI chat of the studio shows them
		entry_id INTEGER REFERENCES entries(id) ON DELETE CASCADE,
		expires_at INTEGER NOT NULL
	);

	-- uploaded files, the file is files/<id>/<filename>
	CREATE TABLE assets (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		filename TEXT NOT NULL,
		-- image or file
		kind TEXT NOT NULL,
		mime TEXT NOT NULL,
		size INTEGER NOT NULL,
		width INTEGER,
		height INTEGER,
		alt TEXT NOT NULL DEFAULT '',
		-- 0 to 1 from the left and the top
		focal_x REAL NOT NULL DEFAULT 0.5,
		focal_y REAL NOT NULL DEFAULT 0.5,
		created_at INTEGER NOT NULL
	);

	CREATE TABLE users (
		id INTEGER PRIMARY KEY AUTOINCREMENT,
		email TEXT NOT NULL UNIQUE,
		password_hash TEXT NOT NULL,
		created_at INTEGER NOT NULL
	);

	CREATE TABLE sessions (
		token TEXT PRIMARY KEY,
		user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
		created_at INTEGER NOT NULL
	);

	CREATE TABLE schema_migrations (
		name TEXT PRIMARY KEY,
		applied_at INTEGER NOT NULL
	);
"#];

#[derive(Clone)]
pub struct Db {
	conn: Arc<Mutex<Connection>>,
}

impl Db {
	pub fn open(path: &Path) -> rusqlite::Result<Self> {
		Self::new(Connection::open(path)?)
	}

	pub fn memory() -> rusqlite::Result<Self> {
		Self::new(Connection::open_in_memory()?)
	}

	fn new(conn: Connection) -> rusqlite::Result<Self> {
		conn.execute_batch(
			"PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON;",
		)?;

		let version: usize =
			conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
		for (i, sql) in MIGRATIONS.iter().enumerate().skip(version) {
			conn.execute_batch(sql)?;
			conn.pragma_update(None, "user_version", i + 1)?;
		}

		Ok(Self {
			conn: Arc::new(Mutex::new(conn)),
		})
	}

	pub fn conn(&self) -> MutexGuard<'_, Connection> {
		self.conn.lock().unwrap()
	}
}

/// 128 random bits as hex, for sessions and preview links.
pub fn random_token() -> String {
	use rand::Rng;
	let n: u128 = rand::rng().random();
	format!("{n:032x}")
}

pub fn now() -> i64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis() as i64
}
