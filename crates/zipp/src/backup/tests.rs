use std::os::unix::fs::MetadataExt;

use super::*;
use crate::auth::add_user;
use crate::db::{DAY_MS as DAY, Db};

// 2026-11-01T08:00:00
const NOW: i64 = 1_793_520_000_000;

fn users(conn: &Connection) -> Vec<String> {
	let mut stmt = conn.prepare("SELECT email FROM users ORDER BY id").unwrap();
	stmt.query_map([], |r| r.get(0))
		.unwrap()
		.collect::<rusqlite::Result<_>>()
		.unwrap()
}

/// A site with a user and an uploaded file, the server's `Db` open.
fn site(root: &Path) -> (Db, PathBuf, PathBuf) {
	let (db_path, files) = (root.join("zipp.db"), root.join("files"));
	let db = Db::open(&db_path).unwrap();
	add_user(&db.conn(), "anna@example.ch", "a long password", None).unwrap();
	fs::create_dir_all(files.join("1/.t")).unwrap();
	fs::write(files.join("1/photo.jpg"), "photo").unwrap();
	fs::write(files.join("1/.t/small-photo.jpg"), "small").unwrap();
	(db, db_path, files)
}

#[test]
fn a_backup_restores_to_the_same_content() {
	let root = tempfile::tempdir().unwrap();
	let (db, db_path, files) = site(root.path());
	let to = root.path().join("backups");

	// while the server has it open, in WAL mode
	let w = backup(&db_path, &files, &to, NOW).unwrap();
	assert_eq!(w.dir, to.join("2026-11-01T08-00-00"));
	assert_eq!((w.copied, w.linked), (1, 0));
	// transforms are made again, they stay out
	assert!(!w.dir.join("files/1/.t").exists());

	add_user(&db.conn(), "ben@example.ch", "a long password", None).unwrap();
	fs::remove_dir_all(files.join("1")).unwrap();
	fs::write(files.join("new.txt"), "new").unwrap();

	let err = restore(&w.dir, &db_path, &files).unwrap_err();
	assert!(err.contains("in use"), "{err}");
	drop(db);

	restore(&w.dir, &db_path, &files).unwrap();
	let db = Db::open(&db_path).unwrap();
	assert_eq!(users(&db.conn()), ["anna@example.ch"]);
	assert_eq!(
		fs::read_to_string(files.join("1/photo.jpg")).unwrap(),
		"photo"
	);
	assert!(!files.join("new.txt").exists());
}

#[test]
fn unchanged_files_are_links_to_the_last_backup() {
	let root = tempfile::tempdir().unwrap();
	let (_db, db_path, files) = site(root.path());
	let to = root.path().join("backups");
	let first = backup(&db_path, &files, &to, NOW).unwrap();

	fs::write(files.join("2.pdf"), "new").unwrap();
	let second = backup(&db_path, &files, &to, NOW + DAY).unwrap();
	assert_eq!((second.copied, second.linked), (1, 1));
	let ino =
		|d: &Path| fs::metadata(d.join("files/1/photo.jpg")).unwrap().ino();
	assert_eq!(ino(&first.dir), ino(&second.dir));

	// the same name again is refused, not mixed into the old one
	assert!(backup(&db_path, &files, &to, NOW + DAY).is_err());
}

#[test]
fn prune_keeps_the_last_ones() {
	let root = tempfile::tempdir().unwrap();
	let (_db, db_path, files) = site(root.path());
	let to = root.path().join("backups");
	for day in 0..4 {
		backup(&db_path, &files, &to, NOW + day * DAY).unwrap();
	}
	// a broken off one is no backup and goes with the next
	fs::create_dir(to.join(".2026-11-09T08-00-00.partial")).unwrap();
	assert_eq!(list(&to).unwrap().len(), 4);

	let removed = prune(&to, 2).unwrap();
	assert_eq!(
		removed,
		[
			to.join("2026-11-01T08-00-00"),
			to.join("2026-11-02T08-00-00")
		]
	);
	let left: Vec<_> = list(&to).unwrap().into_iter().map(|(t, _)| t).collect();
	assert_eq!(left, [NOW + 2 * DAY, NOW + 3 * DAY]);

	backup(&db_path, &files, &to, NOW + 4 * DAY).unwrap();
	assert!(!to.join(".2026-11-09T08-00-00.partial").exists());
}

#[test]
fn a_file_gone_during_the_backup_is_left_out() {
	let root = tempfile::tempdir().unwrap();
	let (_db, db_path, files) = site(root.path());
	let to = root.path().join("backups");
	// listed but gone when it is copied, like an asset deleted meanwhile
	fs::create_dir_all(files.join("2")).unwrap();
	std::os::unix::fs::symlink(files.join("2/gone.jpg"), files.join("2/a.jpg"))
		.unwrap();

	let w = backup(&db_path, &files, &to, NOW).unwrap();
	assert_eq!(w.copied, 1);
	assert!(w.dir.join("files/1/photo.jpg").exists());
	assert!(!w.dir.join("files/2/a.jpg").exists());
}
