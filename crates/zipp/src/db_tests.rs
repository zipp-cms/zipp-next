use super::*;

/// The sha256 of every released entry of `MIGRATIONS`. Sites ran them
/// already, a change would never reach their databases: add a new entry
/// instead, and its hash here once it is released.
const RELEASED: &[&str] =
	&["f83bf7d137f0989a090fcad026bfaf093dc634796afbdd9c056a3417135344ed"];

#[test]
fn released_migrations_never_change() {
	assert!(MIGRATIONS.len() >= RELEASED.len());
	for (i, hash) in RELEASED.iter().enumerate() {
		let sha = token_hash(MIGRATIONS[i]);
		assert_eq!(
			&sha,
			hash,
			"the zipp migration {} changed, add a new one instead",
			i + 1
		);
	}
}

#[test]
fn a_new_database_has_every_migration() {
	let db = Db::memory().unwrap();
	assert_eq!(version_of(&db.conn()).unwrap(), MIGRATIONS.len());
}

#[test]
fn a_database_of_a_newer_zipp_is_refused() {
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().join("zipp.db");
	drop(Db::open(&path).unwrap());

	let conn = Connection::open(&path).unwrap();
	conn.pragma_update(None, "user_version", MIGRATIONS.len() + 1)
		.unwrap();
	drop(conn);

	let err = Db::open(&path).err().unwrap().to_string();
	assert!(err.contains("newer zipp"), "{err}");
}

#[test]
fn videos_become_a_kind() {
	let dir = tempfile::tempdir().unwrap();
	let path = dir.path().join("zipp.db");
	let conn = Connection::open(&path).unwrap();
	conn.execute_batch(MIGRATIONS[0]).unwrap();
	conn.pragma_update(None, "user_version", 1).unwrap();
	conn.execute_batch(
		"INSERT INTO assets (filename, kind, mime, size, created_at) VALUES
		('a.mp4', 'file', 'video/mp4', 1, 0),
		('a.pdf', 'file', 'application/pdf', 1, 0)",
	)
	.unwrap();
	drop(conn);

	let db = Db::open(&path).unwrap();
	let kinds: Vec<String> = db
		.conn()
		.prepare("SELECT kind FROM assets ORDER BY id")
		.unwrap()
		.query_map([], |r| r.get(0))
		.unwrap()
		.collect::<rusqlite::Result<_>>()
		.unwrap();
	assert_eq!(kinds, ["video", "file"]);
}

/// A plugin with the tables of `TABLES[..n]`, like a newer version of it
/// adds more.
struct Tabled(usize);

const TABLES: &[&str] = &[
	"CREATE TABLE tabled_a (id INTEGER PRIMARY KEY);",
	"CREATE TABLE tabled_b (id INTEGER PRIMARY KEY);",
];

impl crate::Plugin for Tabled {
	fn name(&self) -> &'static str {
		"tabled"
	}

	fn tables(&self) -> &'static [&'static str] {
		&TABLES[..self.0]
	}
}

#[test]
fn plugin_tables_are_made_once_and_grow() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let schema = |n| crate::Schema::new().plugin(Tabled(n));

	crate::plugin::create_tables(&conn, &schema(1)).unwrap();
	conn.execute("INSERT INTO tabled_a (id) VALUES (1)", [])
		.unwrap();
	// again: nothing runs twice, the row stays
	crate::plugin::create_tables(&conn, &schema(1)).unwrap();
	crate::plugin::create_tables(&conn, &schema(2)).unwrap();
	let rows: i64 = conn
		.query_row("SELECT count(*) FROM tabled_a", [], |r| r.get(0))
		.unwrap();
	assert_eq!(rows, 1);
	conn.execute("INSERT INTO tabled_b (id) VALUES (1)", [])
		.unwrap();

	// an older version of the plugin doesn't run on its newer tables
	let err = crate::plugin::create_tables(&conn, &schema(1))
		.unwrap_err()
		.to_string();
	assert!(err.contains("newer version"), "{err}");
}
