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
