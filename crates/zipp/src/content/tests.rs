use serde_json::json;

use super::*;
use crate::schema::Section;
use crate::schema::tests::site;

fn page_fields() -> Value {
	json!({
		"intro": "Hi",
		"body": [
			{ "type": "text", "fields": { "content": "Hello" } },
			{ "id": "b1", "type": "button",
				"fields": { "label": "Go", "link": [3] } },
		]
	})
}

#[test]
fn validates_and_adds_block_ids() {
	let s = site();
	let out = validate(&s, s.get("page").unwrap(), &page_fields()).unwrap();
	assert_eq!(out["body"][0]["id"].as_str().unwrap().len(), 16);
	assert_eq!(out["body"][1]["id"], "b1");
	assert_eq!(out["body"][1]["fields"]["link"], json!([3]));
}

#[test]
fn reports_paths() {
	let s = site();
	let fields = json!({
		"intro": 5,
		"other": 1,
		"body": [
			{ "type": "button", "fields": { "label": "a".repeat(21) } },
			{ "type": "page", "fields": {} },
		]
	});
	let errs = validate(&s, s.get("page").unwrap(), &fields).unwrap_err();
	let paths: Vec<_> = errs.iter().map(|e| e.path.as_str()).collect();
	assert_eq!(paths, ["other", "intro", "body.0.label", "body.1"]);
}

#[test]
fn required_and_empty() {
	let s = site();
	let fields = json!({ "body": [{ "type": "text", "fields": {} }] });
	let errs = validate(&s, s.get("page").unwrap(), &fields).unwrap_err();
	assert_eq!(errs[0].path, "body.0.content");

	let out = validate(&s, s.get("page").unwrap(), &json!({ "intro": "" }));
	assert_eq!(out.unwrap(), json!({}));
}

#[test]
fn checks_links() {
	let to = [LinkKind::Entry, LinkKind::Url];
	let link = |v: Value| v.as_object().unwrap().clone();
	let ok =
		link(json!({ "type": "url", "value": "https://x.ch", "label": "X" }));
	assert!(check_link(&to, true, &ok).is_ok());
	let bad = link(json!({ "type": "url", "value": "x.ch" }));
	assert_eq!(check_link(&to, true, &bad).unwrap_err(), "not a valid url");
	let mail = link(json!({ "type": "email", "value": "a@b.ch" }));
	assert_eq!(
		check_link(&to, true, &mail).unwrap_err(),
		"can't link to email"
	);
	assert!(check_link(&to, false, &ok).unwrap_err().contains("label"));

	// a section of the page, not of an email
	let anchor =
		link(json!({ "type": "entry", "value": 1, "anchor": "anfahrt" }));
	assert!(check_link(&to, true, &anchor).is_ok());
	let hash = link(json!({ "type": "entry", "value": 1, "anchor": "#a b" }));
	assert!(check_link(&to, true, &hash).unwrap_err().contains("anchor"));
	let to = [LinkKind::Email];
	let mail =
		link(json!({ "type": "email", "value": "a@b.ch", "anchor": "x" }));
	assert!(check_link(&to, true, &mail).unwrap_err().contains("anchor"));
}

#[test]
fn finds_nested_relations() {
	let s = site();
	let rels = relations(&s, s.get("page").unwrap(), &page_fields());
	assert_eq!(rels.len(), 1);
	assert_eq!(rels[0].path, "body.1.link");
	assert_eq!(rels[0].ids, [3]);
}

#[test]
fn walks_all_components() {
	let s = site();
	let mut fields = page_fields();
	let mut seen = vec![];
	walk_mut(&s, s.get("page").unwrap(), &mut fields, &mut |c, _| {
		seen.push(c.handle.clone())
	});
	assert_eq!(seen, ["page", "text", "button"]);
}

#[test]
fn shares_fields_that_are_not_translatable() {
	let s = site();
	let de = json!({ "city": "Bern", "price": 10 });
	let en = json!({ "city": "Berne", "price": 8, "page": [1] });
	let out = share(s.get("event").unwrap(), &de, &en);
	assert_eq!(out, json!({ "city": "Bern", "price": 10 }));

	let de = json!({ "intro": "Hallo" });
	let en = json!({ "intro": "Hello" });
	let out = share(s.get("page").unwrap(), &de, &en);
	assert_eq!(out, json!({ "intro": "Hello" }));
}

#[test]
fn dropdowns_take_one_of_their_options() {
	let s = site()
		.section(Section::channel("talks", ["talk"]).uri("{slug}"))
		.entry(
			"talk",
			"Talk",
			[Field::dropdown("size", [("s", "Small"), ("l", "Large")])
				.indexed()],
		);
	s.check().unwrap();
	let comp = s.get("talk").unwrap();
	let ok = validate(&s, comp, &json!({ "size": "l" })).unwrap();
	assert_eq!(ok, json!({ "size": "l" }));
	let err = validate(&s, comp, &json!({ "size": "m" })).unwrap_err();
	assert_eq!(err[0].message, "not one of the options");
	let vals = index_values(comp, &ok);
	assert_eq!(vals[0].text.as_deref(), Some("l"));

	let twice = site()
		.section(Section::channel("talks", ["talk"]).uri("{slug}"))
		.entry(
			"talk",
			"Talk",
			[Field::dropdown("size", [("s", "Small"), ("s", "Large")])],
		);
	assert!(twice.check().unwrap_err().contains("exists twice"));
}

#[test]
fn emails_are_addresses() {
	let s = site();
	let comp = s.get("event").unwrap();
	let ok = json!({ "contact": "info@textrem.ch" });
	assert_eq!(validate(&s, comp, &ok).unwrap(), ok);
	for bad in ["info", "info@textrem", "in fo@textrem.ch", "@textrem.ch"] {
		let err = validate(&s, comp, &json!({ "contact": bad })).unwrap_err();
		assert_eq!(err[0].message, "an email address like name@example.com");
	}

	assert!(valid_email("a.b+c@mail.example.org"));
	assert!(!valid_email("a@b@c.ch"));
	assert!(!valid_email("a@.ch"));
	assert!(!valid_email("<a@b.ch>"));
}

#[test]
fn notes_hold_no_value() {
	let s = site();
	let comp = s.get("event").unwrap();
	let empty = json!({ "howto": null });
	assert_eq!(validate(&s, comp, &empty).unwrap(), json!({}));
	let err = validate(&s, comp, &json!({ "howto": "x" })).unwrap_err();
	assert_eq!(err[0].path, "howto");
	assert_eq!(err[0].message, "a note holds no value");
}

#[test]
fn defaults_fit_their_field() {
	let talk = |f: Field| {
		site()
			.section(Section::channel("talks", ["talk"]).uri("{slug}"))
			.entry("talk", "Talk", [f])
	};
	let size = || Field::dropdown("size", [("s", "Small"), ("l", "Large")]);
	talk(size().default("l")).check().unwrap();
	talk(Field::boolean("on").default(true)).check().unwrap();
	talk(Field::number("n").decimals(0).default(3))
		.check()
		.unwrap();
	talk(Field::text("cta").default("Mehr")).check().unwrap();

	let err = |f: Field| talk(f).check().unwrap_err();
	assert_eq!(
		err(size().default("m")),
		"talk.size: default: not one of the options"
	);
	assert_eq!(
		err(Field::boolean("on").default("yes")),
		"talk.on: default: wrong type"
	);
	assert_eq!(
		err(Field::email("to").default("info")),
		"talk.to: default: an email address like name@example.com"
	);
	assert_eq!(
		err(Field::text("cta").default("")),
		"talk.cta: default: is empty"
	);
	assert_eq!(
		err(Field::relation("page", ["page"]).default(json!([1]))),
		"talk.page: default: only fields of plain values have one"
	);
}

#[test]
fn defaults_fill_what_is_missing() {
	let s = site()
		.section(Section::channel("talks", ["talk"]).uri("{slug}"))
		.entry(
			"talk",
			"Talk",
			[
				Field::boolean("on").default(true),
				Field::text("cta").default("Mehr"),
				Field::group("seo", "teaser"),
				Field::text("city"),
			],
		)
		.block("teaser", "Teaser", [Field::text("more").default("Weiter")]);
	s.check().unwrap();
	let comp = s.get("talk").unwrap();
	let all = json!({ "on": true, "cta": "Mehr", "seo": { "more": "Weiter" } });
	assert_eq!(with_defaults(&s, comp, &Value::Null), all);
	// what is there stays, also off or empty
	let given = json!({ "on": false, "cta": "", "city": "Bern" });
	assert_eq!(
		with_defaults(&s, comp, &given),
		json!({ "on": false, "cta": "", "city": "Bern",
			"seo": { "more": "Weiter" } })
	);
}

#[test]
fn numbers_keep_their_decimals() {
	let s = site()
		.section(Section::channel("talks", ["talk"]).uri("{slug}"))
		.entry(
			"talk",
			"Talk",
			[
				Field::number("price").unit("CHF").decimals(2),
				Field::number("seats").decimals(0).range(0.0, 100.0),
			],
		);
	s.check().unwrap();
	let comp = s.get("talk").unwrap();
	let ok = json!({ "price": 12.35, "seats": 40 });
	assert_eq!(validate(&s, comp, &ok).unwrap(), ok);
	assert!(validate(&s, comp, &json!({ "price": 0.3, "seats": 4.0 })).is_ok());

	let err = validate(&s, comp, &json!({ "price": 12.355, "seats": 4.5 }))
		.unwrap_err();
	assert_eq!(err[0].message, "at most 2 decimal places");
	assert_eq!(err[1].message, "a whole number");
	let err = validate(&s, comp, &json!({ "seats": 101 })).unwrap_err();
	assert_eq!(err[0].message, "out of range");

	let s = site().block("x", "X", [Field::number("n").decimals(11)]);
	assert_eq!(s.check().unwrap_err(), "x.n: decimals go from 0 to 10");
}

#[test]
fn index_values_of_entry() {
	let s = site();
	let fields = json!({ "city": "Bern", "price": 12.5 });
	let vals = index_values(s.get("event").unwrap(), &fields);
	assert_eq!(vals[0].text.as_deref(), Some("Bern"));
	assert_eq!(vals[1].num, Some(12.5));
}

#[test]
fn dates_are_days_of_the_calendar() {
	use super::valid_date;
	assert!(valid_date("2026-11-14", false));
	assert!(valid_date("2028-02-29", false));
	assert!(!valid_date("2026-02-29", false));
	assert!(!valid_date("2026-13-01", false));
	assert!(!valid_date("2026-11-14T20:00", false));
	assert!(!valid_date("14.11.2026", false));
	assert!(valid_date("2026-11-14T20:00", true));
	assert!(!valid_date("2026-11-14T24:00", true));
	assert!(!valid_date("2026-11-14", true));

	let s = crate::schema::tests::site().block(
		"d",
		"D",
		[Field::date("day"), Field::date("at").time()],
	);
	let comp = s.get("d").unwrap();
	let ok = json!({ "day": "2026-11-14", "at": "2026-11-14T20:00" });
	assert!(validate(&s, comp, &ok).is_ok());
	let errs =
		validate(&s, comp, &json!({ "day": "2026-11-14T20:00" })).unwrap_err();
	assert_eq!(errs[0].path, "day");
}

#[test]
fn hidden_fields_needn_t_be_filled() {
	use crate::schema::Condition;
	let s = site().block(
		"x",
		"X",
		[
			Field::boolean("hasLink"),
			Field::text("url").required().when(Condition::on("hasLink")),
			Field::blocks("more", ["text"])
				.min(2)
				.when(Condition::on("hasLink")),
		],
	);
	let x = s.get("x").unwrap();

	// off: nothing missing, but a value is kept and still checked
	let out = validate(&s, x, &json!({ "url": "https://x.ch" })).unwrap();
	assert_eq!(out, json!({ "url": "https://x.ch" }));
	let fields = json!({ "more": [{ "type": "text", "fields": {} }] });
	assert_eq!(
		validate(&s, x, &fields).unwrap()["more"][0]["fields"],
		json!({})
	);
	let err = validate(&s, x, &json!({ "url": 5 })).unwrap_err();
	assert_eq!(
		(err[0].path.as_str(), err[0].message.as_str()),
		("url", "wrong type")
	);

	// on: all of it is needed
	let fields = json!({
		"hasLink": true,
		"more": [{ "type": "text", "fields": {} }],
	});
	let err = validate(&s, x, &fields).unwrap_err();
	let paths: Vec<_> = err.iter().map(|e| e.path.as_str()).collect();
	assert_eq!(paths, ["url", "more", "more.0.content"]);
}

#[test]
fn fields_in_a_hidden_group_needn_t_be_filled() {
	use crate::schema::Condition;
	let s = site()
		.block(
			"box",
			"Box",
			[Field::text("title").required(), Field::text("note")],
		)
		.block(
			"x",
			"X",
			[
				Field::boolean("hasBox"),
				Field::group("box", "box").when(Condition::on("hasBox")),
			],
		);
	let x = s.get("x").unwrap();

	let fields = json!({ "box": { "note": "Hi" } });
	assert_eq!(validate(&s, x, &fields).unwrap(), fields);
	let err = validate(&s, x, &json!({ "box": { "title": 5 } })).unwrap_err();
	assert_eq!(err[0].path, "box.title");
	assert_eq!(err[0].message, "wrong type");

	let fields = json!({ "hasBox": true, "box": { "note": "Hi" } });
	let err = validate(&s, x, &fields).unwrap_err();
	assert_eq!(err[0].path, "box.title");
	assert_eq!(err[0].message, "is required");
}
