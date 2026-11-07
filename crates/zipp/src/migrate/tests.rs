use serde_json::json;

use super::*;
use crate::fixtures::site;
use crate::schema::{Column, LinkKind};

fn apply_all(old: &Schema, ops: &[Op]) -> Schema {
	let mut s = old.clone();
	for op in ops {
		op.apply_schema(&mut s);
	}
	s
}

#[test]
fn no_changes() {
	let s = site();
	assert_eq!(diff(&s, &s, &[]).unwrap(), []);

	// labels and order are free
	let mut s2 = s.clone();
	s2.components[0].fields.reverse();
	s2.components[0].fields[0].label = "Content".into();
	assert_eq!(diff(&s, &s2, &[]).unwrap(), []);
}

#[test]
fn initial_migration_adds_everything() {
	let ops = diff(&Schema::new(), &site(), &[]).unwrap();
	assert_eq!(ops.len(), 10);
	assert!(
		diff(&apply_all(&Schema::new(), &ops), &site(), &[])
			.unwrap()
			.is_empty()
	);
}

fn changed_site() -> Schema {
	let mut s = site();
	// page.intro renamed to lead, button.label removed, event.city
	// changed to a number, text renamed to paragraph
	let page = s.get_mut("page").unwrap();
	page.fields[0].handle = "lead".into();
	let button = s.get_mut("button").unwrap();
	button.fields.remove(0);
	let event = s.get_mut("event").unwrap();
	event.fields[0] = Field::number("city").indexed();
	let text = s.get_mut("text").unwrap();
	text.handle = "paragraph".into();
	let page = s.get_mut("page").unwrap();
	page.fields[1] = Field::blocks("body", ["paragraph", "button"]);
	s
}

#[test]
fn diff_with_renames() {
	let renames = [
		Rename::parse("text=paragraph").unwrap(),
		Rename::parse("page.intro=lead").unwrap(),
	];
	let ops = diff(&site(), &changed_site(), &renames).unwrap();
	let kinds: Vec<_> = ops.iter().map(|o| o.describe()).collect();
	assert_eq!(
		kinds,
		[
			"  rename component text to paragraph",
			"  rename field page.intro to lead",
			"  change field page.body (content starts over if the kind changed, but values it takes as they are)",
			"  change field event.city (content starts over if the kind changed, but values it takes as they are)",
			"! remove field button.label, its content is deleted",
		]
	);
	let after = apply_all(&site(), &ops);
	assert!(diff(&after, &changed_site(), &[]).unwrap().is_empty());
}

#[test]
fn without_hint_rename_is_remove_and_add() {
	let ops = diff(&site(), &changed_site(), &[]).unwrap();
	assert!(ops.contains(&Op::RemoveComponent {
		component: "text".into()
	}));
	assert!(ops.contains(&Op::RemoveField {
		component: "page".into(),
		field: "intro".into()
	}));
}

#[test]
fn applies_to_content() {
	let renames = [
		Rename::parse("text=paragraph").unwrap(),
		Rename::parse("page.intro=lead").unwrap(),
	];
	let ops = diff(&site(), &changed_site(), &renames).unwrap();

	let mut ty = "page".to_string();
	let mut fields = json!({
		"intro": "Hi",
		"body": [
			{ "id": "a", "type": "text", "fields": { "content": "x" } },
			{ "id": "b", "type": "button",
				"fields": { "label": "Go", "link": [1] } },
		]
	});
	let mut schema = site();
	for op in &ops {
		assert!(op.apply_entry(&schema, &mut ty, &mut fields));
		op.apply_schema(&mut schema);
	}

	assert_eq!(
		fields,
		json!({
			"body": [
				{ "id": "a", "type": "paragraph",
					"fields": { "content": "x" } },
				{ "id": "b", "type": "button", "fields": { "link": [1] } },
			],
			"lead": "Hi",
		})
	);

	let mut ty = "event".to_string();
	let mut fields = json!({ "city": "Bern", "price": 3 });
	let mut schema = site();
	for op in &ops {
		op.apply_entry(&schema, &mut ty, &mut fields);
		op.apply_schema(&mut schema);
	}
	assert_eq!(fields, json!({ "price": 3 }));
}

#[test]
fn renames_blocks_in_rich_text() {
	let op = Op::RenameComponent {
		from: "text".into(),
		to: "paragraph".into(),
	};
	let mut ty = "page".to_string();
	let mut fields = json!({ "story": { "type": "doc", "content": [
		{ "type": "paragraph" },
		{ "type": "embed", "attrs": { "id": "a", "type": "text",
			"fields": { "content": "x" } } },
	] } });
	op.apply_entry(&site(), &mut ty, &mut fields);
	assert_eq!(fields["story"]["content"][1]["attrs"]["type"], "paragraph");

	let op = Op::RemoveComponent {
		component: "text".into(),
	};
	let mut fields = json!({ "story": { "type": "doc", "content": [
		{ "type": "embed", "attrs": { "id": "a", "type": "text", "fields": {} } },
	] } });
	op.apply_entry(&site(), &mut ty, &mut fields);
	assert_eq!(fields["story"]["content"], json!([]));
}

#[test]
fn removing_a_component_deletes_entries_and_blocks() {
	let op = Op::RemoveComponent {
		component: "button".into(),
	};
	let mut ty = "page".to_string();
	let mut fields = json!({ "body": [
		{ "id": "a", "type": "button", "fields": {} },
		{ "id": "b", "type": "text", "fields": { "content": "x" } },
	]});
	assert!(op.apply_entry(&site(), &mut ty, &mut fields));
	assert_eq!(fields["body"].as_array().unwrap().len(), 1);

	let op = Op::RemoveComponent {
		component: "event".into(),
	};
	let mut ty = "event".to_string();
	assert!(!op.apply_entry(&site(), &mut ty, &mut json!({})));
}

#[test]
fn narrowing_a_relation_says_what_a_save_refuses() {
	let mut new = site();
	let event = new.get_mut("event").unwrap();
	event.fields[2] =
		Field::relation("page", ["page"]).max(1).sections(["pages"]);
	let ops = diff(&site(), &new, &[]).unwrap();
	assert_eq!(ops.len(), 1);
	assert!(
		ops[0]
			.describe()
			.contains("a save refuses what it no longer allows"),
		"{}",
		ops[0].describe()
	);
}

#[test]
fn diff_sites_and_sections() {
	let mut new = site().site("fr", "Français", "fr");
	new.sites.retain(|s| s.handle != "en");
	new.sections[2].uri = Some("agenda/{slug}".into());
	new.sections.remove(1);

	let ops = diff(&site(), &new, &[]).unwrap();
	let lines: Vec<_> = ops.iter().map(|o| o.describe()).collect();
	assert_eq!(
		lines,
		[
			"  add site fr, it starts as a copy of the primary site",
			"! remove site en, its content is deleted",
			"! remove section pages, its entries are deleted",
			"  change section events (entries of types it no longer has \
			 are deleted, their children move up)",
		]
	);
	assert!(
		diff(&apply_all(&site(), &ops), &new, &[])
			.unwrap()
			.is_empty()
	);
}

#[test]
fn max_level_changes_the_section() {
	let new = crate::entries::tests::site_with_levels(2);
	let ops = diff(&site(), &new, &[]).unwrap();
	assert!(matches!(
		&ops[..],
		[Op::ChangeSection { section }] if section.max_level == Some(2)
	));
	assert!(
		diff(&apply_all(&site(), &ops), &new, &[])
			.unwrap()
			.is_empty()
	);
}

#[test]
fn migration_file_roundtrip() {
	let m = Migration {
		name: String::new(),
		ops: diff(&Schema::new(), &site(), &[]).unwrap(),
		schema: site(),
		..Default::default()
	};
	let s = serde_json::to_string_pretty(&m).unwrap();
	let back: Migration = serde_json::from_str(&s).unwrap();
	assert_eq!(back.ops, m.ops);
	assert_eq!(next_name(&[back], Some("init")), "0002_init");
}

#[test]
fn only_blocks_are_blocks() {
	// a block type named like a kind of link, next to a navigation and a
	// table whose rows have a `type` too
	let s = site().block("url", "Url", []).entry(
		"menu",
		"Menu",
		[
			Field::blocks("body", ["url"]),
			Field::navigation("nav", [LinkKind::Url]),
			Field::table("rows", [Column::text("type")]),
		],
	);
	let op = Op::RemoveComponent {
		component: "url".into(),
	};
	let mut ty = "menu".to_string();
	let link =
		json!({ "type": "url", "value": "https://x.ch", "children": [] });
	let row = json!({ "type": "url" });
	let mut fields = json!({
		"body": [{ "id": "a", "type": "url", "fields": {} }],
		"nav": [link],
		"rows": [row],
	});
	op.apply_entry(&s, &mut ty, &mut fields);
	assert_eq!(fields["body"], json!([]));
	assert_eq!(fields["nav"], json!([link]));
	assert_eq!(fields["rows"], json!([row]));
}

#[test]
fn a_column_of_another_kind_loses_its_cells() {
	let table =
		|columns| site().entry("menu", "Menu", [Field::table("rows", columns)]);
	let old = table(vec![Column::text("name"), Column::text("price")]);
	let new = table(vec![
		Column::text("name"),
		Column::number("price").decimals(2),
		Column::boolean("vegan"),
	]);
	let ops = diff(&old, &new, &[]).unwrap();
	assert!(matches!(&ops[..], [Op::ChangeField { .. }]));

	let mut ty = "menu".to_string();
	let mut fields = json!({ "rows": [{ "name": "Rösti", "price": "18.50" }] });
	ops[0].apply_entry(&old, &mut ty, &mut fields);
	assert_eq!(fields["rows"], json!([{ "name": "Rösti" }]));

	// a unit is no other kind, the cells stay
	let unit = table(vec![
		Column::text("name"),
		Column::number("price").decimals(2).unit("CHF"),
		Column::boolean("vegan"),
	]);
	let ops = diff(&new, &unit, &[]).unwrap();
	let mut fields = json!({ "rows": [{ "name": "Rösti", "price": 18.5 }] });
	ops[0].apply_entry(&new, &mut ty, &mut fields);
	assert_eq!(fields["rows"], json!([{ "name": "Rösti", "price": 18.5 }]));
}

#[test]
fn text_columns_stay_as_they_were_stored() {
	// the columns of before keep their JSON, no migration
	let json = serde_json::to_value(Column::url("url")).unwrap();
	assert_eq!(
		json,
		json!({ "handle": "url", "label": "Url", "kind": "url" })
	);
	let json = serde_json::to_value(Column::number("p").unit("CHF")).unwrap();
	assert_eq!(
		json,
		json!({ "handle": "p", "label": "P", "kind": "number", "unit": "CHF" })
	);
	let back: Column = serde_json::from_value(json).unwrap();
	assert_eq!(back, Column::number("p").unit("CHF"));
}

#[test]
fn instructions_need_no_migration() {
	let s = site();
	let mut s2 = s.clone();
	s2.components[0].fields[0] = s2.components[0].fields[0]
		.clone()
		.instructions("What editors should **know**.");
	assert_eq!(diff(&s, &s2, &[]).unwrap(), []);
	let json = serde_json::to_value(&s2.components[0].fields[0]).unwrap();
	assert_eq!(json["instructions"], "What editors should **know**.");
	// none stays out of the JSON
	let json = serde_json::to_value(&s.components[0].fields[0]).unwrap();
	assert!(json.get("instructions").is_none());
}

#[test]
fn widths_and_notes_need_no_migration() {
	let s = site();
	let mut s2 = s.clone();
	let event = s2.get_mut("event").unwrap();
	event.fields[0].width = 50;
	// a note changes its text, another comes, the one of before goes
	let note = event.fields.iter_mut().find(|f| f.handle == "howto");
	*note.unwrap() = Field::note("howto", "Other text");
	event.fields.push(Field::note("hint", "New"));
	assert_eq!(diff(&s, &s2, &[]).unwrap(), []);
	let mut s3 = s.clone();
	s3.get_mut("event")
		.unwrap()
		.fields
		.retain(|f| f.handle != "howto");
	assert_eq!(diff(&s, &s3, &[]).unwrap(), []);

	// a field that becomes a note loses its content
	let mut s4 = s.clone();
	let event = s4.get_mut("event").unwrap();
	event.fields[0] = Field::note("city", "No city");
	assert_eq!(
		diff(&s, &s4, &[]).unwrap(),
		[Op::RemoveField {
			component: "event".into(),
			field: "city".into(),
		}]
	);
}

#[test]
fn conditions_need_no_migration() {
	use crate::schema::{Condition, Tab};
	let s = site();
	let mut s2 = s.clone();
	let event = s2.get_mut("event").unwrap();
	event.fields[0].when = Some(Condition::on("listed"));
	event.tabs = vec![Tab {
		name: "Content".into(),
		fields: vec!["city".into()],
		when: Some(Condition::filled("price")),
		card: false,
	}];
	assert_eq!(diff(&s, &s2, &[]).unwrap(), []);
}

#[test]
fn cards_need_no_migration() {
	let s = site();
	let s2 = s.clone().entry_card("event");
	assert_eq!(diff(&s, &s2, &[]).unwrap(), []);
}

#[test]
fn tabs_of_blocks_need_no_migration() {
	let block = |s: Schema, tabbed: bool| {
		let (a, b) = (Field::text("heading"), Field::boolean("wide"));
		match tabbed {
			true => s.block_tabs(
				"teaser",
				"Teaser",
				[("Content", vec![a]), ("Layout", vec![b])],
			),
			false => s.block("teaser", "Teaser", [a, b]),
		}
	};
	let s = block(site(), false);
	assert_eq!(diff(&s, &block(site(), true), &[]).unwrap(), []);
}

#[test]
fn only_what_the_studio_shows_needs_no_migration() {
	let s = site();
	let mut s2 = s.clone();
	let f = &mut s2.get_mut("event").unwrap().fields[0];
	f.required = true;
	f.default = Some(json!("Bern"));
	s2.sections[0].name = "Start".into();
	assert_eq!(diff(&s, &s2, &[]).unwrap(), []);

	// anything else does, also what comes later
	let mut s3 = s.clone();
	s3.sections[0].site_uris.insert("en".into(), "home".into());
	s3.get_mut("event").unwrap().fields[0].indexed = false;
	let ops = diff(&s, &s3, &[]).unwrap();
	assert!(matches!(ops[0], Op::ChangeField { .. }), "{ops:?}");
	assert!(matches!(ops[1], Op::ChangeSection { .. }), "{ops:?}");
}

#[test]
fn the_sections_of_a_relation_need_a_migration() {
	let s = site();
	let mut s2 = s.clone();
	let event = s2.get_mut("event").unwrap();
	let page = event
		.fields
		.iter_mut()
		.find(|f| f.handle == "page")
		.unwrap();
	*page = page.clone().sections(["pages"]);
	let ops = diff(&s, &s2, &[]).unwrap();
	assert!(matches!(&ops[..], [Op::ChangeField { .. }]), "{ops:?}");
	assert_eq!(apply_all(&s, &ops), s2);
}

/// Files as earlier zipps wrote them: the example's first migration
/// (written before the format field), a migration with every op and a
/// data migration. They are what sites have in their `migrations/`,
/// never regenerate them.
const FIXTURES: &[(&str, &[u8])] = &[
	("0001_initial", include_bytes!("fixtures/0001_initial.json")),
	("0002_changes", include_bytes!("fixtures/0002_changes.json")),
	("0003_fill", include_bytes!("fixtures/0003_fill.json")),
];

/// Everything of `old` is in `new` unchanged; `new` may have more.
fn keeps(old: &Value, new: &Value, path: &str) {
	match (old, new) {
		(Value::Object(old), Value::Object(new)) => {
			for (k, v) in old {
				let Some(n) = new.get(k) else {
					panic!("{path}.{k} is no longer written");
				};
				keeps(v, n, &format!("{path}.{k}"));
			}
		}
		(Value::Array(old), Value::Array(new)) => {
			assert_eq!(old.len(), new.len(), "{path}");
			for (i, (o, n)) in old.iter().zip(new).enumerate() {
				keeps(o, n, &format!("{path}[{i}]"));
			}
		}
		_ => assert_eq!(old, new, "{path}"),
	}
}

/// A rename in the serde shape of a migration, its ops or the schema
/// breaks the files of every site. Fails here instead.
#[test]
fn released_migration_files_still_read() {
	for (name, data) in FIXTURES {
		let m = Migration::from_file(name, data).unwrap();
		assert_eq!(&m.name, name);

		let old: Value = serde_json::from_slice(data).unwrap();
		let new: Value = serde_json::from_str(&m.to_file()).unwrap();
		keeps(&old, &new, name);
	}

	let m = Migration::from_file(FIXTURES[1].0, FIXTURES[1].1).unwrap();
	assert_eq!(m.ops.len(), 12, "every kind of op");
	let m = Migration::from_file(FIXTURES[2].0, FIXTURES[2].1).unwrap();
	assert!(m.data && m.ops.is_empty());
}

#[test]
fn migration_files_have_a_format() {
	let m = Migration {
		schema: site(),
		..Default::default()
	};
	let file = m.to_file();
	assert!(file.starts_with("{\n  \"format\": 1,"), "{file}");

	// a file of a newer zipp says so, not what field it misses
	let newer = r#"{"format": 2, "changes": []}"#;
	let err = Migration::from_file("0004", newer.as_bytes()).unwrap_err();
	assert!(err.contains("format 2"), "{err}");
	assert!(err.contains("update zipp"), "{err}");
}
