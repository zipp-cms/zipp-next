use std::path::Path;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::Connection;

/// Tables of zipp itself, every entry is applied once, tracked with
/// `PRAGMA user_version`. Only ever append to this list. Changes of the
/// content model are schema migrations (`migrate.rs`), not these.
const MIGRATIONS: &[&str] = &[
	r#"
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
		-- empty: a title from the filename
		title TEXT NOT NULL DEFAULT '',
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
		-- a role of the schema, NULL is an admin
		role TEXT,
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
"#,
	// the image editor, who uploaded, where assets are used
	r#"
	ALTER TABLE assets ADD COLUMN edits TEXT;
	ALTER TABLE assets ADD COLUMN uploaded_by INTEGER
		REFERENCES users(id) ON DELETE SET NULL;
	CREATE TABLE asset_relations (
		entry_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
		site TEXT NOT NULL,
		path TEXT NOT NULL,
		asset_id INTEGER NOT NULL
	);
	CREATE INDEX asset_relations_asset ON asset_relations(asset_id);
	CREATE INDEX asset_relations_entry ON asset_relations(entry_id);
"#,
	// sessions end at a time that using the studio pushes back
	r#"
	ALTER TABLE sessions ADD COLUMN expires_at INTEGER NOT NULL DEFAULT 0;
	UPDATE sessions SET expires_at = created_at + 1000 * 60 * 60 * 24 * 30;
"#,
];

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

/// Milliseconds as `2026-10-28T08:00:00+00:00`.
pub fn iso(ms: i64) -> String {
	let secs = ms.div_euclid(1000);
	let days = secs.div_euclid(86_400);
	let rest = secs.rem_euclid(86_400);
	// civil from days, Howard Hinnant's algorithm
	let z = days + 719_468;
	let era = z.div_euclid(146_097);
	let doe = z - era * 146_097;
	let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
	let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
	let mp = (5 * doy + 2) / 153;
	let d = doy - (153 * mp + 2) / 5 + 1;
	let m = if mp < 10 { mp + 3 } else { mp - 9 };
	let y = yoe + era * 400 + i64::from(m <= 2);
	format!(
		"{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}+00:00",
		rest / 3600,
		rest % 3600 / 60,
		rest % 60
	)
}

/// `2026-10-28T08:00:00+02:00` (or `…Z`, or without seconds' offset
/// `…T08:00:00`, read as UTC) as milliseconds, the inverse of [`iso`].
pub fn parse_iso(s: &str) -> Option<i64> {
	let num = |r: std::ops::Range<usize>| s.get(r)?.parse::<i64>().ok();
	let (y, m, d) = (num(0..4)?, num(5..7)?, num(8..10)?);
	let (h, min, sec) = (num(11..13)?, num(14..16)?, num(17..19)?);
	let offset = match s.get(19..) {
		None | Some("") | Some("Z") => 0,
		Some(o) => {
			let sign = if o.starts_with('-') { -1 } else { 1 };
			let h = o.get(1..3)?.parse::<i64>().ok()?;
			let m = o.get(4..6)?.parse::<i64>().ok()?;
			sign * (h * 3600 + m * 60)
		}
	};
	// days from civil, Howard Hinnant's algorithm
	let y = if m <= 2 { y - 1 } else { y };
	let era = y.div_euclid(400);
	let yoe = y - era * 400;
	let mp = (m + 9) % 12;
	let doy = (153 * mp + 2) / 5 + d - 1;
	let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
	let days = era * 146_097 + doe - 719_468;
	let secs = days * 86_400 + h * 3600 + min * 60 + sec - offset;
	Some(secs * 1000)
}

pub fn now() -> i64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis() as i64
}
