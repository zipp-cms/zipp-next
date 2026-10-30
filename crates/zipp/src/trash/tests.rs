use serde_json::json;

use super::*;
use crate::db::Db;
use crate::entries::tests::input;
use crate::entries::{get_entry, save};
use crate::query::Query;
use crate::schema::tests::site;

fn admin() -> Authed {
	Authed {
		user_id: 1,
		role: None,
		token: String::new(),
	}
}

fn titles(conn: &Connection, s: &Schema, section: &str) -> Vec<String> {
	let params = [("section".to_string(), section.to_string())];
	let q = Query::parse(s, &params).unwrap();
	let (found, _) = crate::entries::list(conn, &q).unwrap();
	found.into_iter().map(|e| e.title).collect()
}

#[test]
fn an_entry_goes_with_its_children_and_comes_back() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let about = save(
		&conn,
		&s,
		None,
		input("pages", "page", "About", json!({})),
		None,
	)
	.unwrap();
	let mut team = input("pages", "page", "Team", json!({}));
	team.parent_id = Some(about.id);
	let team = save(&conn, &s, None, team, None).unwrap();
	// an event links to the page
	let fields = json!({ "page": [about.id] });
	let event =
		save(&conn, &s, None, input("events", "event", "E", fields), None)
			.unwrap();

	trash_entry(&conn, &s, about.id, None).unwrap();
	assert!(titles(&conn, &s, "pages").is_empty());
	assert!(matches!(
		get_entry(&conn, team.id, "de"),
		Err(Error::NotFound)
	));
	let refs = crate::entries::refs(&conn, &[event.id], "de", false).unwrap();
	assert!(refs.is_empty());
	// the uri is free for another page
	let other = save(
		&conn,
		&s,
		None,
		input("pages", "page", "About", json!({})),
		None,
	)
	.unwrap();
	assert_eq!(other.uri.as_deref(), Some("about"));

	// the trash shows the parent with its child
	let trash = list(&conn, &s, &admin()).unwrap();
	assert_eq!(trash.entries.len(), 1);
	assert_eq!(
		(trash.entries[0].id, trash.entries[0].children),
		(about.id, 1)
	);

	restore_entry(&conn, &s, about.id).unwrap();
	assert_eq!(titles(&conn, &s, "pages").len(), 3);
	// its uri was taken meanwhile, the child follows
	let back = get_entry(&conn, about.id, "de").unwrap();
	assert_eq!(
		(back.slug.as_str(), back.uri.as_deref()),
		("about-2", Some("about-2"))
	);
	let child = get_entry(&conn, team.id, "de").unwrap();
	assert_eq!(child.uri.as_deref(), Some("about-2/team"));
	let refs = crate::entries::refs(&conn, &[event.id], "de", false).unwrap();
	assert!(refs.contains_key(&about.id));
	assert!(list(&conn, &s, &admin()).unwrap().entries.is_empty());
}

#[test]
fn a_child_alone_comes_back_on_the_top() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let about = save(
		&conn,
		&s,
		None,
		input("pages", "page", "About", json!({})),
		None,
	)
	.unwrap();
	let mut team = input("pages", "page", "Team", json!({}));
	team.parent_id = Some(about.id);
	let team = save(&conn, &s, None, team, None).unwrap();

	// first the child, then its parent: two items
	trash_entry(&conn, &s, team.id, None).unwrap();
	std::thread::sleep(std::time::Duration::from_millis(2));
	trash_entry(&conn, &s, about.id, None).unwrap();
	assert_eq!(list(&conn, &s, &admin()).unwrap().entries.len(), 2);

	restore_entry(&conn, &s, team.id).unwrap();
	let team = get_entry(&conn, team.id, "de").unwrap();
	assert_eq!((team.parent_id, team.uri.as_deref()), (None, Some("team")));

	// for good, and not again
	purge_entry(&conn, about.id).unwrap();
	assert!(matches!(
		restore_entry(&conn, &s, about.id),
		Err(Error::NotFound)
	));
	assert!(matches!(purge_entry(&conn, team.id), Err(Error::NotFound)));
}

#[test]
fn a_single_stays() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	crate::migrate::apply::create_singles(&conn, &s).unwrap();
	let id: i64 = conn
		.query_row("SELECT id FROM entries WHERE section = 'home'", [], |r| {
			r.get(0)
		})
		.unwrap();
	assert!(matches!(
		trash_entry(&conn, &s, id, None),
		Err(Error::Request(_))
	));
}

#[test]
fn old_ones_are_deleted() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let dir = format!("zipp-trash-{}-{}", std::process::id(), now());
	let files = std::env::temp_dir().join(dir);
	std::fs::create_dir_all(&files).unwrap();
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	)
	.unwrap();
	let a = crate::assets::store(&conn, &files, "a.txt", b"a", None).unwrap();
	trash_entry(&conn, &s, e.id, None).unwrap();
	trash_asset(&conn, a.id, None).unwrap();

	// a day: they stay
	purge_old(&conn, &files).unwrap();
	assert_eq!(list(&conn, &s, &admin()).unwrap().entries.len(), 1);
	let old = now() - (KEEP_DAYS + 1) * DAY_MS;
	conn.execute("UPDATE entries SET deleted_at = ?1", [old])
		.unwrap();
	conn.execute("UPDATE assets SET deleted_at = ?1", [old])
		.unwrap();
	purge_old(&conn, &files).unwrap();
	let trash = list(&conn, &s, &admin()).unwrap();
	assert!(trash.entries.is_empty() && trash.assets.is_empty());
	assert!(!crate::assets::asset_dir(&files, a.id).exists());
	std::fs::remove_dir_all(&files).unwrap();
}
