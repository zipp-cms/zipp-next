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
fn index_values_of_entry() {
	let s = site();
	let fields = json!({ "city": "Bern", "price": 12.5 });
	let vals = index_values(s.get("event").unwrap(), &fields);
	assert_eq!(vals[0].text.as_deref(), Some("Bern"));
	assert_eq!(vals[1].num, Some(12.5));
}
