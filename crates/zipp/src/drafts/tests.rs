use serde_json::json;

use super::*;
use crate::db::Db;
use crate::entries::save;
use crate::entries::tests::input;
use crate::schema::tests::site;

fn site_doc(title: &str, fields: Value) -> SiteDoc {
	SiteDoc {
		title: title.into(),
		slug: String::new(),
		published: true,
		fields,
	}
}

/// the version of the draft the editor knows
fn base(conn: &Connection, id: i64) -> Option<i64> {
	get_draft(conn, id).unwrap().map(|d| d.updated_at)
}

#[test]
fn conflicts_with_others() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	crate::auth::add_user(&conn, "a@x.ch", "password", None).unwrap();
	crate::auth::add_user(&conn, "b@x.ch", "password", None).unwrap();
	let e = save(
		&conn,
		&s,
		None,
		input("pages", "page", "P", json!({})),
		None,
	)
	.unwrap();

	// a starts a draft, b still has the live entry open
	let a = put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("A", json!({})),
		None,
		Some(1),
	)
	.unwrap();
	let b = put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("B", json!({})),
		None,
		Some(2),
	);
	assert!(
		matches!(b, Err(Error::Conflict(m)) if m == "a@x.ch changed this entry")
	);

	// a keeps typing, b after reloading too
	let a2 = put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("A2", json!({})),
		Some(a.updated_at),
		Some(1),
	)
	.unwrap();
	put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("B", json!({})),
		Some(a2.updated_at),
		Some(2),
	)
	.unwrap();

	// after publishing, an old version is a conflict too
	publish(&conn, &s, e.id, None).unwrap();
	let late = put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("A3", json!({})),
		Some(a2.updated_at),
		Some(1),
	);
	assert!(matches!(late, Err(Error::Conflict(_))));
}

#[test]
fn drafts_stay_until_published() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let fields = json!({ "city": "Bern", "price": 10 });
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "Konzert", fields),
		None,
	)
	.unwrap();

	// en changes its title and the shared price
	let en = site_doc("Concert", json!({ "city": "Bern", "price": 12 }));
	let d =
		put_draft(&conn, &s, e.id, "en", en, base(&conn, e.id), None).unwrap();
	assert_eq!(d.doc["de"].fields["price"], 12);
	assert_eq!(get_entry(&conn, e.id, "en").unwrap().title, "Konzert");

	publish(&conn, &s, e.id, None).unwrap();
	let live = get_entry(&conn, e.id, "en").unwrap();
	assert_eq!(live.title, "Concert");
	assert_eq!(live.uri.as_deref(), Some("events/concert"));
	assert_eq!(get_entry(&conn, e.id, "de").unwrap().fields["price"], 12);
	assert!(get_draft(&conn, e.id).unwrap().is_none());
	// the one of creating it, the one of publishing
	assert_eq!(revisions(&conn, e.id).unwrap().len(), 2);
}

#[test]
fn publish_checks_every_site_first() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	)
	.unwrap();

	put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("Neu", json!({})),
		base(&conn, e.id),
		None,
	)
	.unwrap();
	put_draft(
		&conn,
		&s,
		e.id,
		"en",
		site_doc("", json!({})),
		base(&conn, e.id),
		None,
	)
	.unwrap();
	let Err(Error::Invalid(errs)) = publish(&conn, &s, e.id, None) else {
		panic!("should fail");
	};
	assert_eq!(errs[0].message, "en: is required");
	// nothing was written
	assert_eq!(get_entry(&conn, e.id, "de").unwrap().title, "E");
}

#[test]
fn revisions_know_what_changed() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let e = save(
		&conn,
		&s,
		None,
		input("pages", "page", "P", json!({ "intro": "a" })),
		None,
	)
	.unwrap();
	record_revision(&conn, e.id, None).unwrap();
	let de = site_doc("P2", json!({ "intro": "b" }));
	put_draft(&conn, &s, e.id, "de", de, base(&conn, e.id), None).unwrap();
	publish(&conn, &s, e.id, None).unwrap();

	let revs = revisions(&conn, e.id).unwrap();
	assert!(revs[1].changes.is_empty());
	let fields: Vec<_> =
		revs[0].changes.iter().map(|c| c.field.as_str()).collect();
	// the draft has no slug, it follows the new title
	assert_eq!(fields, ["title", "slug", "intro"]);
	assert_eq!(revs[0].changes[0].sites, ["de"]);
}

#[test]
fn restores_a_revision_into_the_draft() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let e = save(
		&conn,
		&s,
		None,
		input("pages", "page", "One", json!({})),
		None,
	)
	.unwrap();
	record_revision(&conn, e.id, None).unwrap();
	put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("Two", json!({})),
		base(&conn, e.id),
		None,
	)
	.unwrap();
	publish(&conn, &s, e.id, None).unwrap();

	let first = revisions(&conn, e.id).unwrap().pop().unwrap();
	let d = restore(&conn, e.id, first.id, None).unwrap();
	assert_eq!(d.doc["de"].title, "One");
}

fn entry_doc(p: Option<Preview>) -> Doc {
	match p {
		Some(Preview::Entry(_, doc)) => doc,
		_ => panic!("not the preview of an entry"),
	}
}

#[test]
fn tokens_show_the_draft() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let e = save(
		&conn,
		&s,
		None,
		input("pages", "page", "Live", json!({})),
		None,
	)
	.unwrap();
	let token = create_token(&conn, e.id).unwrap();
	assert_eq!(
		entry_doc(preview(&conn, &token).unwrap())["de"].title,
		"Live"
	);

	put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("Draft", json!({})),
		base(&conn, e.id),
		None,
	)
	.unwrap();
	assert_eq!(
		entry_doc(preview(&conn, &token).unwrap())["de"].title,
		"Draft"
	);
	assert!(preview(&conn, "nope").unwrap().is_none());
}
