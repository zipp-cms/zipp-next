use serde_json::json;

use super::*;
use crate::db::Db;
use crate::entries::save;
use crate::entries::tests::input;
use crate::schema::Field;
use crate::schema::tests::site;

fn site_doc(ty: &str, title: &str, fields: Value) -> SiteDoc {
	SiteDoc {
		ty: ty.into(),
		dates: Default::default(),
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
		site_doc("page", "A", json!({})),
		None,
		Some(1),
	)
	.unwrap();
	let b = put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("page", "B", json!({})),
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
		site_doc("page", "A2", json!({})),
		Some(a.updated_at),
		Some(1),
	)
	.unwrap();
	put_draft(
		&conn,
		&s,
		e.id,
		"de",
		site_doc("page", "B", json!({})),
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
		site_doc("page", "A3", json!({})),
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
	let en =
		site_doc("event", "Concert", json!({ "city": "Bern", "price": 12 }));
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
		site_doc("event", "Neu", json!({})),
		base(&conn, e.id),
		None,
	)
	.unwrap();
	put_draft(
		&conn,
		&s,
		e.id,
		"en",
		site_doc("event", "", json!({})),
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
	let de = site_doc("page", "P2", json!({ "intro": "b" }));
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
		site_doc("page", "Two", json!({})),
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
		site_doc("page", "Draft", json!({})),
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

/// [`site`] with concerts among the events: the same city (translatable
/// there), the price as text, their own uris.
fn with_concerts() -> Schema {
	let mut s = site().entry(
		"concert",
		"Concert",
		[
			Field::text("city").translatable(),
			Field::text("price"),
			Field::text("band"),
		],
	);
	let events = s.sections.iter_mut().find(|s| s.handle == "events");
	let events = events.unwrap();
	events.types.push("concert".into());
	*events = events.clone().uri_of("concert", "concerts/{slug}");
	s
}

#[test]
fn a_draft_changes_the_type() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = with_concerts();
	let fields = json!({ "city": "Bern", "price": 10 });
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "Jazz", fields),
		None,
	)
	.unwrap();

	// the editor sends de converted, en follows: the city stays, the
	// price was a number
	let de = site_doc("concert", "Jazz", json!({ "city": "Bern" }));
	let d = put_draft(&conn, &s, e.id, "de", de, None, None).unwrap();
	assert_eq!(d.doc["en"].ty, "concert");
	assert_eq!(d.doc["en"].fields, json!({ "city": "Bern" }));
	// the website keeps the event until it is published
	assert_eq!(get_entry(&conn, e.id, "en").unwrap().ty, "event");

	publish(&conn, &s, e.id, None).unwrap();
	let live = get_entry(&conn, e.id, "en").unwrap();
	assert_eq!(live.ty, "concert");
	assert_eq!(live.fields, json!({ "city": "Bern" }));
	assert_eq!(live.uri.as_deref(), Some("concerts/jazz"));
	let changes = &revisions(&conn, e.id).unwrap()[0].changes;
	assert!(changes.iter().any(|c| c.field == "type"));

	// only a type of the section
	let bad = site_doc("page", "Jazz", json!({}));
	let err = put_draft(&conn, &s, e.id, "de", bad, None, None).unwrap_err();
	assert!(matches!(err, Error::Request(_)));
}

#[test]
fn a_save_changes_the_type() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = with_concerts();
	let fields = json!({ "city": "Bern", "price": 10 });
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "Jazz", fields),
		None,
	)
	.unwrap();

	let mut c = input("events", "concert", "Jazz", json!({ "band": "Trio" }));
	c.site = Some("de".into());
	save(&conn, &s, Some(e.id), c, None).unwrap();
	// the other site converted: its city kept, the price gone, the
	// shared band from de
	let en = get_entry(&conn, e.id, "en").unwrap();
	assert_eq!(en.ty, "concert");
	assert_eq!(en.fields, json!({ "city": "Bern", "band": "Trio" }));
	assert_eq!(en.uri.as_deref(), Some("concerts/jazz"));
	// the number isn't indexed any more
	let n: i64 = conn
		.query_row("SELECT COUNT(*) FROM entry_values", [], |r| r.get(0))
		.unwrap();
	assert_eq!(n, 0);
}
