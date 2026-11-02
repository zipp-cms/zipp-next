use serde_json::json;

use super::*;
use crate::db::{DAY_MS as DAY, Db};
use crate::fixtures::site;
use crate::schema::Field;
use crate::testing::input;

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
		None,
	)
	.unwrap();
	assert_eq!(about.slug, "über-uns");
	assert_eq!(about.uri.as_deref(), Some("über-uns"));

	for (city, price) in [("Bern", 10), ("Zürich", 30), ("Bern", 25)] {
		let fields =
			json!({ "city": city, "price": price, "page": [about.id] });
		let title = format!("{city} {price}");
		let e = input("events", "event", &title, fields);
		save(&conn, &s, None, e, None).unwrap();
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
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "Konzert", fields),
		None,
	)
	.unwrap();

	let mut en =
		input("events", "event", "Concert", json!({ "city": "Berne" }));
	en.site = Some("en".into());
	save(&conn, &s, Some(e.id), en, None).unwrap();

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
	assert_eq!(team.uri.as_deref(), Some("about/team"));

	// renaming the parent moves the child
	let mut rename = input("pages", "page", "About", json!({}));
	rename.slug = "company".into();
	save(&conn, &s, Some(about.id), rename, None).unwrap();
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
fn the_tree_is_the_order_of_the_section_without_the_trash() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();

	let about = page(&conn, &s, "About", None);
	let team = page(&conn, &s, "Team", Some(about));
	page(&conn, &s, "Jobs", Some(about));
	let old = page(&conn, &s, "Old", None);
	page(&conn, &s, "Older", Some(old));
	move_entry(&conn, &s, team, Some(about), 1).unwrap();
	crate::trash::trash_entry(&conn, &s, old, None).unwrap();

	let tree = [("section", "pages"), ("order", "structure")];
	assert_eq!(titles(&conn, &tree), ["About", "Jobs", "Team"]);
	// the children in their order, without the tree
	let children = [("parent", &*about.to_string()), ("order", "structure")];
	assert_eq!(titles(&conn, &children), ["Jobs", "Team"]);
	let children = [("parent", &*about.to_string()), ("order", "-structure")];
	assert_eq!(titles(&conn, &children), ["Team", "Jobs"]);
	// the trees of all sections
	let all = [("type", "page"), ("order", "structure")];
	assert_eq!(titles(&conn, &all), ["About", "Jobs", "Team"]);
}

/// [`site`] with pages `max` levels deep.
pub fn site_with_levels(max: u32) -> Schema {
	let mut s = site();
	s.sections
		.iter_mut()
		.find(|x| x.handle == "pages")
		.unwrap()
		.max_level = Some(max);
	s
}

fn page(
	conn: &Connection,
	s: &Schema,
	title: &str,
	parent: Option<i64>,
) -> i64 {
	let mut p = input("pages", "page", title, json!({}));
	p.parent_id = parent;
	save(conn, s, None, p, None).unwrap().id
}

#[test]
fn structures_stay_within_their_max_level() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site_with_levels(2);

	let about = page(&conn, &s, "About", None);
	let team = page(&conn, &s, "Team", Some(about));
	let mut deep = input("pages", "page", "Deep", json!({}));
	deep.parent_id = Some(team);
	let err = save(&conn, &s, None, deep, None).unwrap_err();
	assert_eq!(
		err,
		Error::Request(
			"the entry goes too deep, pages has entries down to level 2".into()
		)
	);

	// an entry with a child doesn't fit below another
	let jobs = page(&conn, &s, "Jobs", None);
	let open = page(&conn, &s, "Open", Some(jobs));
	let err = move_entry(&conn, &s, jobs, Some(about), 0).unwrap_err();
	assert!(
		matches!(err, Error::Request(m) if m.contains("with its children"))
	);
	let err = move_entry(&conn, &s, open, Some(team), 0).unwrap_err();
	assert_eq!(
		err,
		Error::Request(
			"the entry goes too deep, pages has entries down to level 2".into()
		)
	);

	// without its child it does
	move_entry(&conn, &s, open, Some(about), 1).unwrap();
	move_entry(&conn, &s, jobs, Some(about), 2).unwrap();
	assert_eq!(get_entry(&conn, jobs, "de").unwrap().parent_id, Some(about));
}

#[test]
fn rejects_invalid_content() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();

	let event = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	)
	.unwrap();
	let fields = json!({ "body": [
		{ "type": "button", "fields": { "label": "Go", "link": [event.id] } }
	]});
	let Err(Error::Invalid(errs)) =
		save(&conn, &s, None, input("pages", "page", "E", fields), None)
	else {
		panic!("should fail");
	};
	assert_eq!(errs[0].path, "body.0.link");

	let Err(Error::Invalid(errs)) = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	) else {
		panic!("should fail");
	};
	assert_eq!(errs[0].message, "the uri /events/e is already used");

	let wrong = input("events", "page", "P", json!({}));
	assert!(save(&conn, &s, None, wrong, None).is_err());
	let single = input("home", "page", "Home", json!({}));
	assert!(save(&conn, &s, None, single, None).is_err());
}

#[test]
fn unpublished_are_hidden() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let mut draft = input("pages", "page", "Draft", json!({}));
	draft.published = false;
	save(&conn, &site(), None, draft, None).unwrap();

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
		save(&conn, &s, None, page(json!({ "pdf": [1, 2] })), None)
	else {
		panic!("should fail");
	};
	assert_eq!(errs.len(), 1);
	assert_eq!(errs[0].path, "body.0.pdf");
	assert_eq!(errs[0].message, "asset 2 is no pdf");
	save(&conn, &s, None, page(json!({ "pdf": [1] })), None).unwrap();
}

#[test]
fn live_between_post_and_expiry_date() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let now = crate::db::now();
	let event = |title: &str, dates: Dates| EntryInput {
		dates,
		..input("events", "event", title, json!({}))
	};

	// published: its post date is now
	let a =
		save(&conn, &s, None, event("Jetzt", Dates::default()), None).unwrap();
	let post = a.dates.post_date.unwrap();
	assert!((post - now).abs() < 1000, "{post} {now}");
	// not published: none yet
	let mut draft = event("Entwurf", Dates::default());
	draft.published = false;
	let d = save(&conn, &s, None, draft, None).unwrap();
	assert_eq!(d.dates.post_date, None);

	let soon = Dates {
		post_date: Some(now + DAY),
		expiry_date: None,
	};
	save(&conn, &s, None, event("Bald", soon), None).unwrap();
	let gone = Dates {
		post_date: Some(now - 2 * DAY),
		expiry_date: Some(now - DAY),
	};
	let old = save(&conn, &s, None, event("Vorbei", gone), None).unwrap();

	let live = |conn: &Connection| {
		let mut q = query(&[("section", "events"), ("order", "title")]);
		q.published_only = true;
		let (entries, _) = list(conn, &q).unwrap();
		entries.into_iter().map(|e| e.title).collect::<Vec<_>>()
	};
	assert_eq!(live(&conn), ["Jetzt"]);
	// the studio sees all of them
	assert_eq!(titles(&conn, &[("section", "events")]).len(), 4);

	// saving without dates keeps the post date, the expiry date goes
	let again = save(
		&conn,
		&s,
		Some(old.id),
		event("Vorbei", Dates::default()),
		None,
	)
	.unwrap();
	assert_eq!(again.dates.post_date, gone.post_date);
	assert_eq!(again.dates.expiry_date, None);
	assert_eq!(live(&conn), ["Jetzt", "Vorbei"]);

	// an expiry before the post date
	let wrong = Dates {
		post_date: Some(now),
		expiry_date: Some(now - DAY),
	};
	let Err(Error::Invalid(errs)) =
		save(&conn, &s, None, event("Falsch", wrong), None)
	else {
		panic!("should fail");
	};
	assert_eq!(errs[0].path, "expiryDate");

	// what went live in a window, for the hooks
	let went = dated_between(&conn, now + 1000, now + 2 * DAY).unwrap();
	assert_eq!(went.len(), 1);
	assert_eq!(went[0].1, "events");
}

#[test]
fn links_to_trashed_or_missing_entries_are_left_out() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let mut s = site();
	let page = s.get_mut("page").unwrap();
	page.fields
		.push(Field::navigation("menu", [LinkKind::Entry, LinkKind::Url]));
	page.fields.push(Field::table(
		"rows",
		[
			crate::schema::Column::text("name"),
			crate::schema::Column::link("link", [LinkKind::Entry]),
		],
	));
	let new = |title: &str, section: &str, ty: &str, fields: Value| {
		save(&conn, &s, None, input(section, ty, title, fields), None)
			.unwrap()
			.id
	};
	let gone = new("Gone", "pages", "page", json!({}));
	let kept = new("Kept", "pages", "page", json!({}));
	let to = |id: i64| json!({ "type": "entry", "value": id });

	let event_fields = json!({ "page": [gone], "ticket": to(gone) });
	let event = new("E", "events", "event", event_fields.clone());
	let mut button = to(gone);
	button["label"] = "Go".into();
	let page_fields = json!({
		"story": { "type": "doc", "content": [
			{ "type": "paragraph", "content": [{ "type": "text", "text": "see",
				"marks": [{ "type": "link", "attrs": { "link": to(gone) } }] }] },
			{ "type": "button", "attrs": { "link": button } },
		] },
		"menu": [{ "type": "entry", "value": gone, "label": "G", "children": [
			{ "type": "entry", "value": kept, "label": "K" },
		] }],
		"rows": [{ "name": "a", "link": to(gone) }],
		"body": [{ "type": "button", "fields": { "label": "Go", "link": [gone] } }],
	});
	let p = new("P", "pages", "page", page_fields.clone());

	// the target goes to the trash, saving what links to it still works
	crate::trash::trash_entry(&conn, &s, gone, None).unwrap();
	let e = input("events", "event", "E", event_fields);
	let e = save(&conn, &s, Some(event), e, None).unwrap();
	assert!(e.fields.get("page").is_none());
	assert!(e.fields.get("ticket").is_none());

	let p = save(
		&conn,
		&s,
		Some(p),
		input("pages", "page", "P", page_fields),
		None,
	)
	.unwrap();
	let story = &p.fields["story"]["content"];
	assert_eq!(story.as_array().unwrap().len(), 1);
	assert_eq!(
		story[0]["content"][0],
		json!({ "type": "text", "text": "see" })
	);
	assert_eq!(
		p.fields["menu"],
		json!([{ "type": "entry", "value": kept, "label": "K" }])
	);
	assert_eq!(p.fields["rows"], json!([{ "name": "a" }]));
	assert!(p.fields["body"][0]["fields"].get("link").is_none());

	// an entry that never was likewise
	let missing = new("M", "events", "event", json!({ "page": [999] }));
	assert!(
		get_entry(&conn, missing, "de")
			.unwrap()
			.fields
			.get("page")
			.is_none()
	);
}

#[test]
fn a_title_without_letters_needs_a_slug() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();

	// "—" would take the uri of the home page, "!!!" the one of events
	for (section, ty, title) in
		[("pages", "page", "—"), ("events", "event", "!!!")]
	{
		let Err(Error::Invalid(errs)) =
			save(&conn, &s, None, input(section, ty, title, json!({})), None)
		else {
			panic!("should fail");
		};
		assert_eq!(
			(errs[0].path.as_str(), errs[0].message.as_str()),
			("slug", "is required")
		);
		let mut with_slug = input(section, ty, title, json!({}));
		with_slug.slug = "dash".into();
		save(&conn, &s, None, with_slug, None).unwrap();
	}
}

#[test]
fn search_finds_signs_as_they_are() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	for title in ["100% sure", "1000 sure", "a_b", "axb"] {
		save(
			&conn,
			&s,
			None,
			input("pages", "page", title, json!({})),
			None,
		)
		.unwrap();
	}
	assert_eq!(titles(&conn, &[("search", "0%")]), ["100% sure"]);
	assert_eq!(titles(&conn, &[("search", "a_")]), ["a_b"]);
}

#[test]
fn searches_literally_and_caps_id_lists() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	for title in ["100% Bern", "1000 Bern", "a_b", "axb"] {
		let e = input("pages", "page", title, json!({}));
		save(&conn, &s, None, e, None).unwrap();
	}
	// % and _ are no wildcards
	assert_eq!(titles(&conn, &[("search", "0%")]), ["100% Bern"]);
	assert_eq!(titles(&conn, &[("search", "a_")]), ["a_b"]);

	let many: Vec<String> = (1..=1001).map(|i| i.to_string()).collect();
	let many = many.join(",");
	for key in ["ids", "notIds", "r.page"] {
		let params = [(key.to_string(), many.clone())];
		let err = Query::parse(&s, &params).unwrap_err();
		assert!(err.contains("more than 1000"), "{err}");
	}
}

#[test]
fn singles_give_their_published_fields() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	crate::migrate::apply::create_singles(&conn, &s).unwrap();
	assert_eq!(single_fields(&conn, &s, "home", "en").unwrap(), json!({}));

	let home = get_entry(&conn, 1, "en").unwrap();
	let mut i = input("home", "page", "Home", json!({ "intro": "Hi" }));
	i.site = Some("en".into());
	save(&conn, &s, Some(home.id), i, None).unwrap();
	let fields = single_fields(&conn, &s, "home", "en").unwrap();
	assert_eq!(fields["intro"], "Hi");

	// not published: nothing
	conn.execute("UPDATE entry_sites SET published = 0", [])
		.unwrap();
	let fields = single_fields(&conn, &s, "home", "en").unwrap();
	assert_eq!(fields, Value::Null);
}

#[test]
fn choices_are_in_the_order_of_their_section() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	for title in ["A", "B"] {
		let i = input("pages", "page", title, json!({}));
		save(&conn, &s, None, i, None).unwrap();
	}
	let all = choices(&conn, &s, &["pages"], "de").unwrap();
	let titles: Vec<&str> = all.iter().map(|c| c.title.as_str()).collect();
	assert_eq!(titles, ["A", "B"]);
	assert!(choices(&conn, &s, &["pages"], "fr").is_err());
}
