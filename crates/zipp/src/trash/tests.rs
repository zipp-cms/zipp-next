use serde_json::json;

use super::*;
use crate::db::Db;
use crate::entries::query::Query;
use crate::entries::tests::site_with_levels;
use crate::entries::{get_entry, move_entry, save};
use crate::fixtures::site;
use crate::testing::input;

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
fn an_entry_too_deep_for_its_parent_comes_back_on_the_top() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site_with_levels(3);
	let page = |title: &str, parent: Option<i64>| {
		let mut p = input("pages", "page", title, json!({}));
		p.parent_id = parent;
		save(&conn, &s, None, p, None).unwrap().id
	};
	let about = page("About", None);
	let team = page("Team", Some(about));
	let anna = page("Anna", Some(team));
	let home = page("Home", None);

	// team goes with anna, about moves a level down meanwhile
	trash_entry(&conn, &s, team, None).unwrap();
	move_entry(&conn, &s, about, Some(home), 0).unwrap();

	restore_entry(&conn, &s, team).unwrap();
	let back = get_entry(&conn, team, "de").unwrap();
	assert_eq!((back.parent_id, back.uri.as_deref()), (None, Some("team")));
	let child = get_entry(&conn, anna, "de").unwrap();
	assert_eq!(child.uri.as_deref(), Some("team/anna"));

	// where it fits it goes back under its parent
	trash_entry(&conn, &s, anna, None).unwrap();
	restore_entry(&conn, &s, anna).unwrap();
	assert_eq!(get_entry(&conn, anna, "de").unwrap().parent_id, Some(team));
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
	let tmp = tempfile::tempdir().unwrap();
	let files = tmp.path();
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	)
	.unwrap();
	let a = crate::assets::store(&conn, files, "a.txt", b"a", None).unwrap();
	trash_entry(&conn, &s, e.id, None).unwrap();
	trash_asset(&conn, a.id, None).unwrap();

	// a day: they stay
	purge_old(&conn, files).unwrap();
	assert_eq!(list(&conn, &s, &admin()).unwrap().entries.len(), 1);
	let old = now() - (KEEP_DAYS + 1) * DAY_MS;
	conn.execute("UPDATE entries SET deleted_at = ?1", [old])
		.unwrap();
	conn.execute("UPDATE assets SET deleted_at = ?1", [old])
		.unwrap();
	purge_old(&conn, files).unwrap();
	let trash = list(&conn, &s, &admin()).unwrap();
	assert!(trash.entries.is_empty() && trash.assets.is_empty());
	assert!(!crate::assets::asset_dir(files, a.id).exists());
}

#[test]
fn a_child_trashed_on_its_own_outlives_its_parent() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let tmp = tempfile::tempdir().unwrap();
	let files = tmp.path();
	let page = |title: &str, parent: Option<i64>| {
		let mut p = input("pages", "page", title, json!({}));
		p.parent_id = parent;
		save(&conn, &s, None, p, None).unwrap().id
	};
	let about = page("About", None);
	let team = page("Team", Some(about));
	let news = page("News", None);
	let jobs = page("Jobs", Some(news));

	// first the child, then its parent; purging the parent leaves it
	trash_entry(&conn, &s, team, None).unwrap();
	std::thread::sleep(std::time::Duration::from_millis(2));
	trash_entry(&conn, &s, about, None).unwrap();
	purge_entry(&conn, about).unwrap();
	let trash = list(&conn, &s, &admin()).unwrap();
	assert_eq!(trash.entries.len(), 1);
	restore_entry(&conn, &s, team).unwrap();
	assert_eq!(get_entry(&conn, team, "de").unwrap().parent_id, None);

	// and when the parent's time is up first
	trash_entry(&conn, &s, jobs, None).unwrap();
	trash_entry(&conn, &s, news, None).unwrap();
	let old = now() - (KEEP_DAYS + 1) * DAY_MS;
	conn.execute(
		"UPDATE entries SET deleted_at = ?1 WHERE id = ?2",
		params![old, news],
	)
	.unwrap();
	purge_old(&conn, files).unwrap();
	let trash = list(&conn, &s, &admin()).unwrap();
	assert_eq!(trash.entries.len(), 1);
	assert_eq!(trash.entries[0].id, jobs);
}

#[test]
fn a_rebuild_leaves_the_trash_out_of_the_index() {
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
	let fields = json!({ "page": [about.id] });
	let event =
		save(&conn, &s, None, input("events", "event", "E", fields), None)
			.unwrap();
	trash_entry(&conn, &s, event.id, None).unwrap();

	// like after a migration
	crate::entries::reindex_all(&conn, &s).unwrap();
	for table in INDEX_TABLES {
		let n: i64 = conn
			.query_row(
				&format!("SELECT COUNT(*) FROM {table} WHERE entry_id = ?1"),
				[event.id],
				|r| r.get(0),
			)
			.unwrap();
		assert_eq!(n, 0, "{table}");
	}
	let texts: i64 = conn
		.query_row(
			"SELECT COUNT(*) FROM entry_texts WHERE entry_id = ?1",
			[about.id],
			|r| r.get(0),
		)
		.unwrap();
	assert!(texts > 0);
}

#[test]
fn a_new_entry_goes_after_the_live_ones() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let page = |title: &str| {
		let e = save(
			&conn,
			&s,
			None,
			input("pages", "page", title, json!({})),
			None,
		)
		.unwrap();
		conn.query_row(
			"SELECT position FROM entries WHERE id = ?1",
			[e.id],
			|r| r.get::<_, i64>(0),
		)
		.map(|p| (e.id, p))
		.unwrap()
	};
	assert_eq!(page("A").1, 0);
	let (b, _) = page("B");
	trash_entry(&conn, &s, b, None).unwrap();
	assert_eq!(page("C").1, 1);
}
