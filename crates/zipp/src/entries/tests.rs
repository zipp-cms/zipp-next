use serde_json::json;

use super::*;
use crate::db::Db;
use crate::schema::Field;
use crate::schema::tests::site;

pub fn input(
	section: &str,
	ty: &str,
	title: &str,
	fields: Value,
) -> EntryInput {
	EntryInput {
		section: section.into(),
		ty: ty.into(),
		site: None,
		title: title.into(),
		slug: String::new(),
		published: true,
		parent_id: None,
		fields,
	}
}

fn query(params: &[(&str, &str)]) -> Query {
	let params: Vec<_> = params
		.iter()
		.map(|(k, v)| (k.to_string(), v.to_string()))
		.collect();
	Query::parse(&site(), &params).unwrap()
}

fn titles(conn: &Connection, params: &[(&str, &str)]) -> Vec<String> {
	let (entries, _) = list(conn, &query(params)).unwrap();
	entries.into_iter().map(|e| e.title).collect()
}

#[test]
fn saves_and_queries() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();

	let about = save(
		&conn,
		&s,
		None,
		input("pages", "page", "Über uns!", json!({})),
	)
	.unwrap();
	assert_eq!(about.slug, "über-uns");
	assert_eq!(about.uri.as_deref(), Some("über-uns"));

	for (city, price) in [("Bern", 10), ("Zürich", 30), ("Bern", 25)] {
		let fields =
			json!({ "city": city, "price": price, "page": [about.id] });
		let title = format!("{city} {price}");
		let e = input("events", "event", &title, fields);
		save(&conn, &s, None, e).unwrap();
	}

	assert_eq!(
		titles(
			&conn,
			&[("type", "event"), ("f.city", "Bern"), ("order", "price")]
		),
		["Bern 10", "Bern 25"]
	);
	assert_eq!(
		titles(
			&conn,
			&[
				("type", "event"),
				("f.price.gte", "25"),
				("order", "-price"),
				("site", "en"),
			]
		),
		["Zürich 30", "Bern 25"]
	);
	assert_eq!(titles(&conn, &[("uri", "events/bern-25")]), ["Bern 25"]);

	// reverse relations
	let id = about.id.to_string();
	assert_eq!(titles(&conn, &[("related", &id)]).len(), 3);
	let (events, _) = list(&conn, &query(&[("type", "event")])).unwrap();
	let r = refs(&conn, &[events[0].id], "de", true).unwrap();
	assert_eq!(r[&about.id].uri.as_deref(), Some("über-uns"));
}

#[test]
fn sites_share_fields_that_are_not_translatable() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();

	let fields = json!({ "city": "Bern", "price": 10 });
	let e = save(&conn, &s, None, input("events", "event", "Konzert", fields))
		.unwrap();

	let mut en =
		input("events", "event", "Concert", json!({ "city": "Berne" }));
	en.site = Some("en".into());
	save(&conn, &s, Some(e.id), en).unwrap();

	let de = get_entry(&conn, e.id, "de").unwrap();
	let en = get_entry(&conn, e.id, "en").unwrap();
	assert_eq!(
		(de.title.as_str(), en.title.as_str()),
		("Konzert", "Concert")
	);
	assert_eq!(en.uri.as_deref(), Some("events/concert"));
	// city and price are shared: en removed the price everywhere
	assert_eq!(de.fields, json!({ "city": "Berne" }));
	assert_eq!(
		titles(&conn, &[("type", "event"), ("f.city", "Berne")]).len(),
		1
	);
}

#[test]
fn structures_build_uris_from_parents() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();

	let about =
		save(&conn, &s, None, input("pages", "page", "About", json!({})))
			.unwrap();
	let mut team = input("pages", "page", "Team", json!({}));
	team.parent_id = Some(about.id);
	let team = save(&conn, &s, None, team).unwrap();
	assert_eq!(team.uri.as_deref(), Some("about/team"));

	// renaming the parent moves the child
	let mut rename = input("pages", "page", "About", json!({}));
	rename.slug = "company".into();
	save(&conn, &s, Some(about.id), rename).unwrap();
	let team = get_entry(&conn, team.id, "de").unwrap();
	assert_eq!(team.uri.as_deref(), Some("company/team"));

	// moving to the top and before about
	move_entry(&conn, &s, team.id, None, 0).unwrap();
	let team = get_entry(&conn, team.id, "en").unwrap();
	assert_eq!(team.uri.as_deref(), Some("team"));
	assert_eq!(
		titles(&conn, &[("section", "pages"), ("order", "structure")]),
		["Team", "About"]
	);

	let err = move_entry(&conn, &s, about.id, Some(about.id), 0);
	assert!(err.is_err());
}

#[test]
fn rejects_invalid_content() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();

	let event = save(&conn, &s, None, input("events", "event", "E", json!({})))
		.unwrap();
	let fields = json!({ "body": [
		{ "type": "button", "fields": { "label": "Go", "link": [event.id] } }
	]});
	let Err(Error::Invalid(errs)) =
		save(&conn, &s, None, input("pages", "page", "E", fields))
	else {
		panic!("should fail");
	};
	assert_eq!(errs[0].path, "body.0.link");

	let Err(Error::Invalid(errs)) =
		save(&conn, &s, None, input("events", "event", "E", json!({})))
	else {
		panic!("should fail");
	};
	assert_eq!(errs[0].message, "the uri /events/e is already used");

	let wrong = input("events", "page", "P", json!({}));
	assert!(save(&conn, &s, None, wrong).is_err());
	let single = input("home", "page", "Home", json!({}));
	assert!(save(&conn, &s, None, single).is_err());
}

#[test]
fn unpublished_are_hidden() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let mut draft = input("pages", "page", "Draft", json!({}));
	draft.published = false;
	save(&conn, &site(), None, draft).unwrap();

	let mut q = query(&[]);
	assert_eq!(list(&conn, &q).unwrap().1, 1);
	q.published_only = true;
	assert_eq!(list(&conn, &q).unwrap().1, 0);
}

#[test]
fn slugs() {
	assert_eq!(slugify("  Hello, World -- 2026 "), "hello-world-2026");
}

#[test]
fn asset_fields_take_their_kinds_of_files() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let mut s = site().block(
		"form",
		"Form",
		[Field::assets("pdf").extensions(["pdf"])],
	);
	let page = |fields| {
		let body = json!([{ "type": "form", "fields": fields }]);
		input("pages", "page", "P", json!({ "body": body }))
	};
	for (name, kind) in [("a.PDF", "file"), ("b.png", "image")] {
		conn.execute(
			"INSERT INTO assets (filename, kind, mime, size, created_at)
				VALUES (?1, ?2, '', 1, 0)",
			[name, kind],
		)
		.unwrap();
	}
	s.get_mut("page").unwrap().fields[1] =
		Field::blocks("body", ["text", "button", "form"]).translatable();

	let Err(Error::Invalid(errs)) =
		save(&conn, &s, None, page(json!({ "pdf": [1, 2] })))
	else {
		panic!("should fail");
	};
	assert_eq!(errs.len(), 1);
	assert_eq!(errs[0].path, "body.0.pdf");
	assert_eq!(errs[0].message, "asset 2 is no pdf");
	save(&conn, &s, None, page(json!({ "pdf": [1] }))).unwrap();
}
