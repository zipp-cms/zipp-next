use serde_json::json;

use super::*;
use crate::db::Db;
use crate::entries::query::Query;
use crate::entries::{get_entry, list, save};
use crate::fixtures::site;
use crate::migrate::{Rename, diff};
use crate::schema::Field;
use crate::testing::input;

/// Applies with the last schema as the code's.
fn apply(conn: &Connection, migrations: &[Migration]) -> Result<Vec<String>> {
	let code = &migrations.last().unwrap().schema;
	apply_migrations(conn, migrations, code)
}

fn migration(
	name: &str,
	old: &Schema,
	new: &Schema,
	renames: &[Rename],
) -> Migration {
	Migration {
		name: name.into(),
		ops: diff(old, new, renames).unwrap(),
		schema: new.clone(),
		..Default::default()
	}
}

#[test]
fn nothing_is_applied_in_an_empty_database() {
	let empty = Connection::open_in_memory().unwrap();
	assert!(!is_applied(&empty, "0001").unwrap());

	let db = Db::memory().unwrap();
	let conn = db.conn();
	let first = migration("0001", &Schema::new(), &site(), &[]);
	apply(&conn, &[first]).unwrap();
	assert!(is_applied(&conn, "0001").unwrap());
	assert!(!is_applied(&conn, "0002").unwrap());
}

#[test]
fn first_migration_creates_singles() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s, &[]);
	assert_eq!(
		apply(&conn, std::slice::from_ref(&first)).unwrap(),
		["0001"]
	);
	assert!(apply(&conn, &[first]).unwrap().is_empty());

	let params = vec![("uri".to_string(), String::new())];
	let (home, _) = list(&conn, &Query::parse(&s, &params).unwrap()).unwrap();
	assert_eq!(home[0].section, "home");
	assert_eq!(home[0].title, "Home");
}

#[test]
fn migrations_change_content_and_index() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();

	let fields = json!({ "city": "Bern", "price": 10 });
	let e = save(&conn, &s, None, input("events", "event", "E", fields), None)
		.unwrap();

	let sd = crate::drafts::SiteDoc {
		ty: "event".into(),
		dates: Default::default(),
		title: "E".into(),
		slug: "e".into(),
		published: true,
		fields: json!({ "city": "Bern" }),
	};
	crate::drafts::put_draft(&conn, &s, e.id, "de", sd, None, None).unwrap();

	// rename city to town
	let mut s2 = s.clone();
	s2.get_mut("event").unwrap().fields[0].handle = "town".into();
	let renames = [Rename::parse("event.city=town").unwrap()];
	let second = migration("0002", &s, &s2, &renames);
	let migrations = [first, second];
	assert_eq!(apply(&conn, &migrations).unwrap(), ["0002"]);

	let entry = get_entry(&conn, e.id, "en").unwrap();
	assert_eq!(entry.fields["town"], "Bern");
	let draft = crate::drafts::get_draft(&conn, e.id).unwrap().unwrap();
	assert_eq!(draft.doc["de"].fields["town"], "Bern");
	let params = vec![
		("type".to_string(), "event".to_string()),
		("f.town".to_string(), "Bern".to_string()),
	];
	let q = Query::parse(&s2, &params).unwrap();
	assert_eq!(list(&conn, &q).unwrap().1, 1);
}

#[test]
fn migrations_change_asset_fields() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let s =
		site().asset_fields([Field::text("credit"), Field::text("licence")]);
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();
	let a = crate::assets::store(&conn, dir, "a.pdf", b"%PDF", None).unwrap();
	let change = crate::assets::AssetChange {
		fields: Some(json!({ "credit": "Anna", "licence": "CC" })),
		..Default::default()
	};
	crate::assets::update(&conn, dir, &s, a.id, change, None).unwrap();
	let fields = || crate::assets::get_asset(&conn, a.id).unwrap().fields;

	// credit is renamed to photographer, licence goes
	let s2 = site().asset_fields([Field::text("photographer")]);
	let renames = [Rename::parse("asset.credit=photographer").unwrap()];
	let second = migration("0002", &s, &s2, &renames);
	let migrations = [first, second];
	apply(&conn, &migrations).unwrap();
	assert_eq!(fields(), json!({ "photographer": "Anna" }));

	// without asset fields the values go
	let third = migration("0003", &s2, &site(), &[]);
	apply(
		&conn,
		&[migrations[0].clone(), migrations[1].clone(), third],
	)
	.unwrap();
	assert_eq!(fields(), json!({}));
}

#[test]
fn migrations_change_user_fields() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site().user_fields([Field::text("bio"), Field::text("city")]);
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();
	let id = crate::users::create(&conn, "a@x.ch", "A", "", None, "").unwrap();
	let fields = json!({ "bio": "Cook", "city": "Bern" });
	crate::users::profile::set_fields(&conn, id, &fields).unwrap();
	let fields = || crate::users::user(&conn, id).unwrap().fields;

	// bio is renamed to about, city goes
	let s2 = site().user_fields([Field::text("about")]);
	let renames = [Rename::parse("user.bio=about").unwrap()];
	let second = migration("0002", &s, &s2, &renames);
	apply(&conn, &[first, second]).unwrap();
	assert_eq!(fields(), json!({ "about": "Cook" }));
}

#[test]
fn new_fields_bring_their_default() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	)
	.unwrap();

	// a boolean that is on, also for the event and the home of before,
	// and a text without a default
	let mut s2 = s.clone();
	for ty in ["event", "page"] {
		let c = s2.get_mut(ty).unwrap();
		c.fields.push(Field::boolean("teasers").default(true));
		c.fields.push(Field::text("cta"));
	}
	let second = migration("0002", &s, &s2, &[]);
	apply(&conn, &[first, second]).unwrap();

	let event = get_entry(&conn, e.id, "en").unwrap();
	assert_eq!(event.fields["teasers"], true);
	assert!(event.fields.get("cta").is_none());
	let params = vec![("uri".to_string(), String::new())];
	let (home, _) = list(&conn, &Query::parse(&s2, &params).unwrap()).unwrap();
	assert_eq!(home[0].fields["teasers"], true);
}

#[test]
fn singles_start_with_defaults() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let mut s = site();
	let page = s.get_mut("page").unwrap();
	page.fields.push(Field::boolean("teasers").default(true));
	create_singles(&conn, &s).unwrap();

	let home = get_entry(&conn, 1, "en").unwrap();
	assert_eq!(home.section, "home");
	assert_eq!(home.fields, json!({ "teasers": true }));
}

#[test]
fn sites_and_sections_change_rows() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();

	let mut page = input("pages", "page", "Hallo", json!({ "intro": "Hoi" }));
	page.slug = "hallo".into();
	let p = save(&conn, &s, None, page, None).unwrap();
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({})),
		None,
	)
	.unwrap();

	// fr starts as a copy of de, events now live under agenda/, intro
	// is no longer translatable
	let mut s2 = s.clone().site("fr", "Français", "fr");
	s2.sections[2].uri = Some("agenda/{slug}".into());
	s2.get_mut("page").unwrap().fields[0].translatable = false;
	let mut en = input("pages", "page", "Hello", json!({ "intro": "Hi" }));
	en.site = Some("en".into());
	save(&conn, &s, Some(p.id), en, None).unwrap();

	apply(&conn, &[first.clone(), migration("0002", &s, &s2, &[])]).unwrap();
	let fr = get_entry(&conn, p.id, "fr").unwrap();
	assert_eq!(fr.title, "Hallo");
	let en = get_entry(&conn, p.id, "en").unwrap();
	assert_eq!(en.fields["intro"], "Hoi");
	let e = get_entry(&conn, e.id, "fr").unwrap();
	assert_eq!(e.uri.as_deref(), Some("agenda/e"));

	// removing the section deletes its entries
	let mut s3 = s2.clone();
	s3.sections.remove(2);
	s3.components.retain(|c| c.handle != "event");
	let m3 = migration("0003", &s2, &s3, &[]);
	apply(&conn, &[first, migration("0002", &s, &s2, &[]), m3]).unwrap();
	assert!(get_entry(&conn, e.id, "de").is_err());
}

#[test]
fn removed_types_leave_their_children() {
	// event leaves pages: once only from the section, once for good
	let mut s = site();
	s.sections[1].types.push("event".into());
	let mut only_pages = s.clone();
	only_pages.sections[1].types = vec!["page".into()];
	let mut no_events = only_pages.clone();
	no_events.sections.remove(2);
	no_events.components.retain(|c| c.handle != "event");

	for s2 in [only_pages, no_events] {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();
		let party = save(
			&conn,
			&s,
			None,
			input("pages", "event", "Party", json!({})),
			None,
		)
		.unwrap();
		let mut kid = input("pages", "page", "Kid", json!({}));
		kid.parent_id = Some(party.id);
		let kid = save(&conn, &s, None, kid, None).unwrap();
		assert_eq!(kid.uri.as_deref(), Some("party/kid"));

		let second = migration("0002", &s, &s2, &[]);
		assert!(second.ops[0].describe().contains("children"));
		apply(&conn, &[first, second]).unwrap();
		assert!(get_entry(&conn, party.id, "de").is_err());
		let kid = get_entry(&conn, kid.id, "de").unwrap();
		assert_eq!(kid.parent_id, None);
		assert_eq!(kid.uri.as_deref(), Some("kid"));
	}
}

/// A field type whose value is a list of pages.
struct Pages;

impl crate::FieldType for Pages {
	fn name(&self) -> &'static str {
		"pages"
	}

	fn validate(
		&self,
		_options: &Value,
		value: &Value,
	) -> std::result::Result<Value, String> {
		Ok(value.clone())
	}

	fn graphql(&self) -> crate::GraphqlType {
		crate::GraphqlType::Scalar(crate::Scalar::String)
	}

	fn relations(
		&self,
		_options: &Value,
		value: &Value,
	) -> Vec<crate::FieldRelation> {
		let ids = value.as_array().into_iter().flatten();
		let ids = ids.filter_map(Value::as_i64).collect();
		let pages = crate::Target::Entries {
			types: vec!["page".into()],
			sections: vec![],
		};
		vec![crate::FieldRelation::new(pages, ids)]
	}

	fn studio(&self) -> crate::StudioModule {
		crate::StudioModule::new("")
	}
}

#[test]
fn plugin_relations_stay_after_migrations() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let mut s = site().field_type(Pages);
	let event = s.get_mut("event").unwrap();
	event
		.fields
		.push(Field::custom("more", "pages", Value::Null));
	// a migration file knows no field types
	let from_file = |mut m: Migration| {
		let json = serde_json::to_value(&m.schema).unwrap();
		m.schema = serde_json::from_value(json).unwrap();
		m
	};
	let first = from_file(migration("0001", &Schema::new(), &s, &[]));
	apply_migrations(&conn, std::slice::from_ref(&first), &s).unwrap();
	let fields = json!({ "more": [1] });
	let e = save(&conn, &s, None, input("events", "event", "E", fields), None)
		.unwrap();

	let mut s2 = s.clone();
	s2.get_mut("event").unwrap().fields.push(Field::text("cta"));
	let second = from_file(migration("0002", &s, &s2, &[]));
	apply_migrations(&conn, &[first, second], &s2).unwrap();
	let n: i64 = conn
		.query_row(
			"SELECT COUNT(*) FROM entry_relations
			WHERE entry_id = ?1 AND path = 'more' AND target_id = 1",
			[e.id],
			|r| r.get(0),
		)
		.unwrap();
	assert_eq!(n, 2);
}

#[test]
fn drafts_of_removed_types_take_the_entry_type() {
	// a talk is an event with a speaker, it leaves events: once only
	// from the section, once for good
	let mut s = site().entry(
		"talk",
		"Talk",
		[Field::text("city").indexed(), Field::text("speaker")],
	);
	s.sections[2].types.push("talk".into());
	let mut only_events = s.clone();
	only_events.sections[2].types = vec!["event".into()];
	let mut no_talks = only_events.clone();
	no_talks.components.retain(|c| c.handle != "talk");

	for s2 in [only_events, no_talks] {
		let db = Db::memory().unwrap();
		let conn = db.conn();
		let first = migration("0001", &Schema::new(), &s, &[]);
		apply(&conn, std::slice::from_ref(&first)).unwrap();
		let e = save(
			&conn,
			&s,
			None,
			input("events", "event", "E", json!({})),
			None,
		)
		.unwrap();
		let sd = crate::drafts::SiteDoc {
			ty: "talk".into(),
			dates: Default::default(),
			title: "E".into(),
			slug: "e".into(),
			published: true,
			fields: json!({ "city": "Bern", "speaker": "Anna" }),
		};
		crate::drafts::put_draft(&conn, &s, e.id, "de", sd, None, None)
			.unwrap();

		apply(&conn, &[first, migration("0002", &s, &s2, &[])]).unwrap();
		let draft = crate::drafts::get_draft(&conn, e.id).unwrap().unwrap();
		assert_eq!(draft.doc["de"].ty, "event");
		assert_eq!(draft.doc["de"].fields, json!({ "city": "Bern" }));
		crate::drafts::publish(&conn, &s2, e.id, None).unwrap();
		let e = get_entry(&conn, e.id, "de").unwrap();
		assert_eq!(e.fields["city"], "Bern");
	}
}

#[test]
fn a_new_group_brings_its_defaults() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();

	// the group's component comes after the page in the schema
	let mut s2 =
		s.clone()
			.block("hero", "Hero", [Field::boolean("big").default(true)]);
	let page = s2.get_mut("page").unwrap();
	page.fields.push(Field::group("hero", "hero"));
	apply(&conn, &[first, migration("0002", &s, &s2, &[])]).unwrap();

	let home = get_entry(&conn, 1, "de").unwrap();
	assert_eq!(home.fields["hero"], json!({ "big": true }));
}

#[test]
fn a_group_of_another_component_starts_over() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();
	let mut home = input("home", "page", "Home", json!({}));
	home.fields = json!({ "seo": { "title": "Hoi" } });
	save(&conn, &s, Some(1), home, None).unwrap();

	// seo is now a group of meta, which has no title
	let mut s2 = s.clone().block(
		"meta",
		"Meta",
		[
			Field::text("keywords"),
			Field::boolean("index").default(true),
		],
	);
	s2.get_mut("page").unwrap().fields[2] = Field::group("seo", "meta");
	apply(&conn, &[first, migration("0002", &s, &s2, &[])]).unwrap();

	let home = get_entry(&conn, 1, "de").unwrap();
	assert_eq!(home.fields["seo"], json!({ "index": true }));
	let mut again = input("home", "page", "Home", home.fields);
	again.slug = "home".into();
	save(&conn, &s2, Some(1), again, None).unwrap();
}

#[test]
fn a_dropdown_that_becomes_a_button_group_keeps_its_values() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let with = |genre: Field| {
		let mut s = site();
		s.get_mut("event").unwrap().fields.push(genre);
		s
	};
	let s = with(Field::dropdown("genre", [("talk", "Talk")]));
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();
	let fields = json!({ "genre": "talk" });
	let e = save(&conn, &s, None, input("events", "event", "E", fields), None)
		.unwrap();
	let none = input("events", "event", "N", json!({}));
	let none = save(&conn, &s, None, none, None).unwrap();

	// another kind keeps a value it takes, else brings its default; other
	// options and colors change nothing
	let buttons = |color: &str| {
		Field::button_group(
			"genre",
			[("talk", "Talk", color), ("concert", "Concert", "#2b6cb0")],
		)
		.default("concert")
	};
	let s2 = with(buttons("#2f855a"));
	let second = migration("0002", &s, &s2, &[]);
	assert!(matches!(&second.ops[..], [Op::ChangeField { .. }]));
	apply(&conn, &[first.clone(), second.clone()]).unwrap();
	assert_eq!(
		get_entry(&conn, e.id, "de").unwrap().fields["genre"],
		"talk"
	);
	assert_eq!(
		get_entry(&conn, none.id, "de").unwrap().fields["genre"],
		"concert"
	);

	let mut talk = input("events", "event", "E", json!({ "genre": "talk" }));
	talk.slug = "e".into();
	save(&conn, &s2, Some(e.id), talk, None).unwrap();
	let s3 = with(buttons("#c05621"));
	let third = migration("0003", &s2, &s3, &[]);
	assert!(matches!(&third.ops[..], [Op::ChangeField { .. }]));
	apply(&conn, &[first, second, third]).unwrap();
	assert_eq!(
		get_entry(&conn, e.id, "de").unwrap().fields["genre"],
		"talk"
	);
}

#[test]
fn unparsable_content_fails_the_migration() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let s = site();
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "E", json!({ "city": "Bern" })),
		None,
	)
	.unwrap();
	let sd = crate::drafts::SiteDoc {
		ty: "event".into(),
		dates: Default::default(),
		title: "E".into(),
		slug: "e".into(),
		published: true,
		fields: json!({ "city": "Bern" }),
	};
	crate::drafts::put_draft(&conn, &s, e.id, "de", sd, None, None).unwrap();
	conn.execute("UPDATE drafts SET doc = '{\"de\": '", [])
		.unwrap();

	let mut s2 = s.clone();
	s2.get_mut("event").unwrap().fields[0].handle = "town".into();
	let renames = [Rename::parse("event.city=town").unwrap()];
	let second = migration("0002", &s, &s2, &renames);
	assert!(apply(&conn, &[first, second]).is_err());
	let e = get_entry(&conn, e.id, "de").unwrap();
	assert_eq!(e.fields["city"], "Bern");
	let done: i64 = conn
		.query_row("SELECT COUNT(*) FROM schema_migrations", [], |r| r.get(0))
		.unwrap();
	assert_eq!(done, 1);
}

/// A field type whose items hold blocks.
struct Items;

impl crate::FieldType for Items {
	fn name(&self) -> &'static str {
		"items"
	}

	fn validate(
		&self,
		_options: &Value,
		value: &Value,
	) -> std::result::Result<Value, String> {
		Ok(value.clone())
	}

	fn graphql(&self) -> crate::GraphqlType {
		crate::GraphqlType::Scalar(crate::Scalar::String)
	}

	fn studio(&self) -> crate::StudioModule {
		crate::StudioModule::new("")
	}
}

#[test]
fn migrations_change_the_blocks_of_items() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let with = |types: &[&str]| {
		let mut s = site().field_type(Items).block(
			"quote",
			"Quote",
			[Field::text("said")],
		);
		s.get_mut("event").unwrap().fields.push(
			Field::custom("grid", "items", Value::Null)
				.item_blocks(types.iter().copied()),
		);
		s
	};
	let s = with(&["text", "quote"]);
	let first = migration("0001", &Schema::new(), &s, &[]);
	apply(&conn, std::slice::from_ref(&first)).unwrap();
	let item = |ty: &str, fields: Value| json!({ "span": 6, "block": { "type": ty, "fields": fields } });
	let fields = json!({ "grid": [
		item("text", json!({ "content": "A" })),
		item("quote", json!({ "said": "B" })),
	] });
	let e = save(&conn, &s, None, input("events", "event", "E", fields), None)
		.unwrap();

	// text is renamed to paragraph and its field content to body
	let mut s2 = with(&["paragraph", "quote"]);
	let text = s2.get_mut("text").unwrap();
	text.handle = "paragraph".into();
	text.fields[0].handle = "body".into();
	let renames = [
		Rename::parse("text=paragraph").unwrap(),
		Rename::parse("paragraph.content=body").unwrap(),
	];
	let second = migration("0002", &s, &s2, &renames);
	// and quote goes, with its item
	let mut s3 = with(&["paragraph"]);
	s3.components.retain(|c| c.handle != "quote");
	let p = s3.get_mut("text").unwrap();
	p.handle = "paragraph".into();
	p.fields[0].handle = "body".into();
	let third = migration("0003", &s2, &s3, &[]);
	apply(&conn, &[first, second, third]).unwrap();

	let grid = &get_entry(&conn, e.id, "en").unwrap().fields["grid"];
	assert_eq!(grid.as_array().unwrap().len(), 1);
	assert_eq!(grid[0]["span"], 6);
	assert_eq!(grid[0]["block"]["type"], "paragraph");
	assert_eq!(grid[0]["block"]["fields"], json!({ "body": "A" }));
}
