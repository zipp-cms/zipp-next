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
		type TEXT NOT NULL,
		title TEXT NOT NULL,
		slug TEXT NOT NULL,
		published INTEGER NOT NULL DEFAULT 0,
		fields TEXT NOT NULL,
		created_at INTEGER NOT NULL,
		updated_at INTEGER NOT NULL,
		UNIQUE (type, slug)
	);

	-- indexed fields, to filter and sort entries by them
	CREATE TABLE entry_values (
		entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
		field TEXT NOT NULL,
		text TEXT,
		num REAL
	);
	CREATE INDEX entry_values_entry ON entry_values(entry_id);
	CREATE INDEX entry_values_text ON entry_values(field, text);
	CREATE INDEX entry_values_num ON entry_values(field, num);

	-- every relation, also inside blocks, to find what links to an entry
	CREATE TABLE entry_relations (
		entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
		path TEXT NOT NULL,
		target_id INTEGER NOT NULL,
		position INTEGER NOT NULL
	);
	CREATE INDEX entry_relations_entry ON entry_relations(entry_id);
	CREATE INDEX entry_relations_target ON entry_relations(target_id);

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

pub fn now() -> i64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis() as i64
}
