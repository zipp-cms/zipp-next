use serde_json::json;

use super::*;
use crate::schema::tests::site;

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
			"  change field page.body (content is dropped if the kind changed)",
			"  change field event.city (content is dropped if the kind changed)",
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
			 are deleted)",
		]
	);
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
	};
	let s = serde_json::to_string_pretty(&m).unwrap();
	let back: Migration = serde_json::from_str(&s).unwrap();
	assert_eq!(back.ops, m.ops);
	assert_eq!(next_name(&[back], Some("init")), "0002_init");
}
