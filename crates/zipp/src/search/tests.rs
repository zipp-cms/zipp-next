use serde_json::json;

use super::*;
use crate::db::Db;
use crate::entries::save;
use crate::entries::tests::input;
use crate::schema::Role;
use crate::schema::tests::site_with_roles;

fn user(role: Option<&str>) -> Authed {
	Authed {
		user_id: 1,
		role: role.map(String::from),
		token: String::new(),
	}
}

/// The titles an admin finds in a site.
fn titles(conn: &Connection, site: &str, q: &str) -> Vec<String> {
	let found = search(conn, &site_with_roles(), &user(None), site, q);
	found
		.unwrap()
		.entries
		.into_iter()
		.map(|e| e.title)
		.collect()
}

#[test]
fn collects_the_texts_of_blocks_groups_and_rich_text() {
	let s = site_with_roles();
	let story = json!({ "type": "doc", "content": [
		{ "type": "paragraph", "content": [
			{ "type": "text", "text": "Rich" },
			{ "type": "text", "text": "ly told", "marks": [{ "type": "bold" }] },
		] },
		{ "type": "embed", "attrs": {
			"id": "b", "type": "text", "fields": { "content": "Embedded" }
		} },
		{ "type": "paragraph", "content": [{ "type": "text", "text": "End" }] },
	] });
	let fields = json!({
		"intro": "Intro",
		"body": [{ "id": "a", "type": "text", "fields": { "content": "Block" } }],
		"seo": { "title": "Seo title" },
		"story": story,
	});
	let comp = s.entry_type("page").unwrap();
	assert_eq!(
		texts(&s, comp, &fields),
		"Intro\nBlock\nSeo title\nRichly told\nEnd\nEmbedded"
	);
}

#[test]
fn finds_saved_entries_and_follows_updates() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site_with_roles();

	let fields = json!({ "intro": "Wir backen Brötchen", "body": [
		{ "id": "a", "type": "text", "fields": { "content": "Mit Sauerteig" } }
	] });
	let bakery = save(
		&conn,
		&s,
		None,
		input("pages", "page", "Bäckerei", fields),
		None,
	)
	.unwrap();
	let notes = json!({ "notes": "Bring a **jacket**" });
	save(
		&conn,
		&s,
		None,
		input("events", "event", "Jazz", notes),
		None,
	)
	.unwrap();

	// the start of a word, without accents, in blocks and markdown
	assert_eq!(titles(&conn, "de", "brotch"), ["Bäckerei"]);
	assert_eq!(titles(&conn, "en", "sauer"), ["Bäckerei"]);
	assert_eq!(titles(&conn, "de", "baeck"), Vec::<String>::new());
	assert_eq!(titles(&conn, "de", "bäck"), ["Bäckerei"]);
	assert_eq!(titles(&conn, "de", "jacket"), ["Jazz"]);
	let found = search(&conn, &s, &user(None), "de", "jacket").unwrap();
	let snippet = found.entries[0].snippet.as_deref();
	assert_eq!(snippet, Some("Bring a jacket"));
	// all words, the slug too
	assert_eq!(titles(&conn, "de", "jazz jacket"), ["Jazz"]);
	assert!(titles(&conn, "de", "jazz sauerteig").is_empty());
	assert!(titles(&conn, "de", " - ").is_empty());
	assert!(titles(&conn, "de", "\"").is_empty());

	let found = search(&conn, &s, &user(None), "de", "sauerteig").unwrap();
	let hit = &found.entries[0];
	assert_eq!(hit.path, format!("/admin/entries/pages/{}", bakery.id));
	assert_eq!(
		hit.snippet.as_deref(),
		Some("Wir backen Brötchen\nMit Sauerteig")
	);
	// only the title: no snippet
	let found = search(&conn, &s, &user(None), "de", "bäckerei").unwrap();
	assert_eq!(found.entries[0].snippet, None);

	// a save in one site changes only its texts
	let mut change = input("pages", "page", "Konditorei", json!({}));
	change.site = Some("de".into());
	change.slug = "konditorei".into();
	save(&conn, &s, Some(bakery.id), change, None).unwrap();
	assert_eq!(titles(&conn, "de", "konditorei"), ["Konditorei"]);
	assert!(titles(&conn, "de", "brötchen").is_empty());
	assert_eq!(titles(&conn, "en", "brötchen"), ["Bäckerei"]);

	// unpublished ones are found too, they say so
	let mut off = input("pages", "page", "Konditorei", json!({}));
	(off.site, off.published) = (Some("de".into()), false);
	save(&conn, &s, Some(bakery.id), off, None).unwrap();
	let found = search(&conn, &s, &user(None), "de", "konditorei").unwrap();
	assert!(!found.entries[0].published);
}

#[test]
fn trashed_entries_are_gone_until_restored() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site_with_roles();
	let jazz = input("events", "event", "Jazz", json!({ "notes": "Swing" }));
	let jazz = save(&conn, &s, None, jazz, None).unwrap();

	crate::trash::trash_entry(&conn, &s, jazz.id, None).unwrap();
	assert!(titles(&conn, "de", "swing").is_empty());
	crate::trash::restore_entry(&conn, &s, jazz.id).unwrap();
	assert_eq!(titles(&conn, "de", "swing"), ["Jazz"]);

	// deleted for good, the index rows go with it
	crate::trash::trash_entry(&conn, &s, jazz.id, None).unwrap();
	crate::trash::purge_entry(&conn, jazz.id).unwrap();
	let left: i64 = conn
		.query_row("SELECT COUNT(*) FROM entry_search", [], |r| r.get(0))
		.unwrap();
	assert_eq!(left, 0);
}

#[test]
fn finds_only_what_the_role_may_view() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	// a guest sees only events and no assets
	let s = site_with_roles()
		.role(Role::new("guest", "Guest").section("events", Access::View));
	for (section, ty) in [("pages", "page"), ("events", "event")] {
		let entry = input(section, ty, &format!("Summer {section}"), json!({}));
		save(&conn, &s, None, entry, None).unwrap();
	}
	conn.execute(
		"INSERT INTO assets (filename, kind, mime, size, created_at, alt)
		VALUES ('beach.jpg', 'image', 'image/jpeg', 1, 0, 'Summer beach')",
		[],
	)
	.unwrap();

	let sections = |role| {
		let found = search(&conn, &s, &user(role), "de", "summer").unwrap();
		let mut out: Vec<String> =
			found.entries.into_iter().map(|e| e.section).collect();
		out.sort();
		(out, found.assets.len())
	};
	let all = vec!["events".to_string(), "pages".to_string()];
	assert_eq!(sections(None), (all, 1));
	assert_eq!(sections(Some("guest")), (vec!["events".into()], 0));
	// a role the schema doesn't know any more sees nothing
	assert_eq!(sections(Some("gone")), (vec![], 0));

	let found = search(&conn, &s, &user(None), "de", "beach").unwrap();
	assert_eq!(found.assets[0].filename, "beach.jpg");
	let found = search(&conn, &s, &user(None), "de", "100%").unwrap();
	assert!(found.assets.is_empty());
}
