//! Content is plain JSON: an entry's fields are an object, a blocks field
//! is a list of `{ "id", "type", "fields" }` and a relation a list of ids.

use rand::Rng;
use serde::Serialize;
use serde_json::{Map, Value};

use crate::richtext;
use crate::schema::{Component, Field, FieldKind, LinkKind, Schema};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FieldError {
	/// `body.0.label`: field handles and block positions
	pub path: String,
	pub message: String,
}

/// Checks the fields of a component and returns them cleaned up: nulls are
/// dropped and new blocks get an id.
pub fn validate(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
) -> Result<Value, Vec<FieldError>> {
	let mut errors = vec![];
	let out = validate_fields(schema, comp, fields, "", &mut errors);
	if errors.is_empty() {
		Ok(out)
	} else {
		Err(errors)
	}
}

pub(crate) fn validate_fields(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
	prefix: &str,
	errors: &mut Vec<FieldError>,
) -> Value {
	let mut out = Map::new();
	let empty = Map::new();
	let obj = match fields {
		Value::Object(o) => o,
		Value::Null => &empty,
		_ => {
			error(errors, prefix.trim_end_matches('.'), "must be an object");
			return Value::Object(out);
		}
	};

	for key in obj.keys() {
		if !comp.fields.iter().any(|f| &f.handle == key) {
			error(errors, &format!("{prefix}{key}"), "unknown field");
		}
	}

	for field in &comp.fields {
		let path = format!("{prefix}{}", field.handle);
		let value = obj.get(&field.handle).unwrap_or(&Value::Null);
		if let Some(v) = validate_value(schema, field, value, &path, errors) {
			out.insert(field.handle.clone(), v);
		}
	}

	Value::Object(out)
}

fn validate_value(
	schema: &Schema,
	field: &Field,
	value: &Value,
	path: &str,
	errors: &mut Vec<FieldError>,
) -> Option<Value> {
	let empty = match value {
		Value::Null => true,
		Value::String(s) => s.is_empty(),
		Value::Array(a) => a.is_empty(),
		// a link without a target
		Value::Object(o) if matches!(field.kind, FieldKind::Link { .. }) => {
			o.get("value").is_none_or(|v| v.is_null() || v == "")
		}
		_ => false,
	};
	if empty {
		if field.required {
			error(errors, path, "is required");
		}
		return None;
	}

	match (&field.kind, value) {
		(FieldKind::Text { max, .. }, Value::String(s)) => {
			if let Some(max) = max
				&& s.chars().count() > *max as usize
			{
				error(errors, path, &format!("at most {max} characters"));
			}
		}
		(FieldKind::Number { min, max }, Value::Number(n)) => {
			let n = n.as_f64().unwrap_or_default();
			if min.is_some_and(|min| n < min) || max.is_some_and(|max| n > max)
			{
				error(errors, path, "out of range");
			}
		}
		(FieldKind::Boolean, Value::Bool(_)) => {}
		(FieldKind::Markdown, Value::String(_)) => {}
		(FieldKind::Group { component }, Value::Object(_)) => {
			let comp = schema.get(component).unwrap();
			let prefix = format!("{path}.");
			let out = validate_fields(schema, comp, value, &prefix, errors);
			if out.as_object().is_some_and(|o| o.is_empty()) {
				if field.required {
					error(errors, path, "is required");
				}
				return None;
			}
			return Some(out);
		}
		(kind @ FieldKind::RichText { .. }, Value::Object(_)) => {
			let allowed = richtext::Allowed::of(kind).unwrap();
			let doc = richtext::validate(schema, &allowed, value, path, errors);
			if richtext::is_empty(&doc) {
				if field.required {
					error(errors, path, "is required");
				}
				return None;
			}
			return Some(doc);
		}
		(FieldKind::Link { to, labels, .. }, Value::Object(link)) => {
			if let Err(e) = check_link(to, *labels, link) {
				error(errors, path, &e);
			}
		}
		(FieldKind::Relation { max, .. }, Value::Array(ids))
		| (FieldKind::Assets { max, .. }, Value::Array(ids)) => {
			if ids.iter().any(|id| !id.is_i64()) {
				error(errors, path, "must be a list of ids");
			}
			check_count(errors, path, ids.len(), None, *max);
		}
		(FieldKind::Blocks { types, min, max }, Value::Array(blocks)) => {
			check_count(errors, path, blocks.len(), *min, *max);
			let blocks = blocks
				.iter()
				.enumerate()
				.map(|(i, b)| {
					validate_block(
						schema,
						types,
						b,
						&format!("{path}.{i}"),
						errors,
					)
				})
				.collect();
			return Some(Value::Array(blocks));
		}
		(FieldKind::Custom { ty, options }, value) => {
			let t = schema.field_types.get(ty).unwrap();
			match t.validate(options, value) {
				Ok(v) => return Some(v),
				Err(e) => error(errors, path, &e),
			}
		}
		_ => error(errors, path, "wrong type"),
	}

	Some(value.clone())
}

fn validate_block(
	schema: &Schema,
	types: &[String],
	block: &Value,
	path: &str,
	errors: &mut Vec<FieldError>,
) -> Value {
	let ty = block["type"].as_str().unwrap_or_default();
	let comp = match schema.get(ty) {
		Some(c) if types.iter().any(|t| t == ty) => c,
		_ => {
			error(errors, path, &format!("block type {ty:?} not allowed"));
			return Value::Null;
		}
	};

	let id = match block["id"].as_str() {
		Some(id) if !id.is_empty() => id.to_string(),
		_ => new_block_id(),
	};
	let fields = validate_fields(
		schema,
		comp,
		&block["fields"],
		&format!("{path}."),
		errors,
	);

	serde_json::json!({ "id": id, "type": ty, "fields": fields })
}

/// `{ "type": "url", "value": "https://…", "label": "…", "newTab": true }`,
/// value is the id for an entry or an asset.
pub(crate) fn check_link(
	to: &[LinkKind],
	label: bool,
	link: &Map<String, Value>,
) -> Result<(), String> {
	let ty = link.get("type").and_then(Value::as_str).unwrap_or_default();
	let kind = match ty {
		"entry" => LinkKind::Entry,
		"url" => LinkKind::Url,
		"email" => LinkKind::Email,
		"phone" => LinkKind::Phone,
		"asset" => LinkKind::Asset,
		_ => return Err(format!("unknown link type {ty:?}")),
	};
	if !to.contains(&kind) {
		return Err(format!("can't link to {ty}"));
	}

	let value = link.get("value").unwrap_or(&Value::Null);
	let ok = match kind {
		LinkKind::Entry | LinkKind::Asset => value.is_i64(),
		LinkKind::Url => value.as_str().is_some_and(|u| {
			u.starts_with("https://")
				|| u.starts_with("http://")
				|| u.starts_with('/')
				|| u.starts_with('#')
		}),
		LinkKind::Email => value.as_str().is_some_and(|e| e.contains('@')),
		LinkKind::Phone => value.as_str().is_some_and(|p| {
			!p.is_empty()
				&& p.chars()
					.all(|c| c.is_ascii_digit() || " +()-/".contains(c))
		}),
	};
	if !ok {
		return Err(format!("not a valid {ty}"));
	}

	for key in link.keys() {
		let known = match key.as_str() {
			"type" | "value" | "newTab" => true,
			"label" => label,
			_ => false,
		};
		if !known {
			return Err(format!("unknown key {key} in the link"));
		}
	}
	if link.get("label").is_some_and(|l| !l.is_string()) {
		return Err("the label must be text".into());
	}
	if link.get("newTab").is_some_and(|t| !t.is_boolean()) {
		return Err("newTab must be true or false".into());
	}
	Ok(())
}

fn check_count(
	errors: &mut Vec<FieldError>,
	path: &str,
	len: usize,
	min: Option<u32>,
	max: Option<u32>,
) {
	if min.is_some_and(|min| len < min as usize) {
		error(errors, path, &format!("at least {} needed", min.unwrap()));
	}
	if max.is_some_and(|max| len > max as usize) {
		error(errors, path, &format!("at most {} allowed", max.unwrap()));
	}
}

fn error(errors: &mut Vec<FieldError>, path: &str, message: &str) {
	errors.push(FieldError {
		path: path.into(),
		message: message.into(),
	});
}

pub(crate) fn new_block_id() -> String {
	let n: u64 = rand::rng().random();
	format!("{n:016x}")
}

/// Calls `f` with the fields of every component in the content: first the
/// entry's own, then those of each block, depth first. Blocks of unknown
/// types are skipped.
pub fn walk_mut(
	schema: &Schema,
	comp: &Component,
	fields: &mut Value,
	f: &mut impl FnMut(&Component, &mut Map<String, Value>),
) {
	let Value::Object(obj) = fields else { return };
	f(comp, obj);

	for field in &comp.fields {
		let blocks: Vec<&mut Value> =
			match (&field.kind, obj.get_mut(&field.handle)) {
				(FieldKind::Blocks { .. }, Some(Value::Array(blocks))) => {
					blocks.iter_mut().collect()
				}
				// blocks in a rich text, their attrs have the same shape
				(FieldKind::RichText { .. }, Some(doc)) => {
					richtext::blocks_mut(doc)
				}
				(FieldKind::Group { component }, Some(group)) => {
					if let Some(c) = schema.get(component) {
						walk_mut(schema, c, group, f);
					}
					continue;
				}
				_ => continue,
			};
		for block in blocks {
			let ty = block["type"].as_str().unwrap_or_default();
			if let Some(c) = schema.get(ty) {
				walk_mut(schema, c, &mut block["fields"], f);
			}
		}
	}
}

/// A relation in the content: `path` is where it is, `body.2.link`.
#[derive(Debug, Clone, PartialEq)]
pub struct RelationRef {
	pub path: String,
	pub target: Target,
	pub ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
	/// entries of these types
	Entries(Vec<String>),
	/// assets, only images if true
	Assets(bool),
}

/// All relations to entries and assets, also those inside blocks.
pub fn relations(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
) -> Vec<RelationRef> {
	let mut out = vec![];
	collect_relations(schema, comp, fields, "", &mut out);
	out
}

/// The relation of one link, to an entry or an asset.
fn link_relation(
	schema: &Schema,
	link: &Value,
	path: &str,
) -> Option<RelationRef> {
	let target = match link["type"].as_str() {
		Some("entry") => Target::Entries(
			schema
				.components
				.iter()
				.filter(|c| c.entry)
				.map(|c| c.handle.clone())
				.collect(),
		),
		Some("asset") => Target::Assets(false),
		_ => return None,
	};
	Some(RelationRef {
		path: path.into(),
		target,
		ids: vec![link["value"].as_i64()?],
	})
}

fn collect_relations(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
	prefix: &str,
	out: &mut Vec<RelationRef>,
) {
	for field in &comp.fields {
		let path = format!("{prefix}{}", field.handle);

		// links in a rich text and the relations of its blocks
		if let FieldKind::RichText { .. } = field.kind {
			let doc = &fields[&field.handle];
			for link in richtext::links(doc) {
				out.extend(link_relation(schema, link, &path));
			}
			let content = doc["content"].as_array().into_iter().flatten();
			for (i, node) in content.enumerate() {
				let bt = node["attrs"]["type"].as_str().unwrap_or_default();
				if let (true, Some(c)) =
					(node["type"] == "embed", schema.get(bt))
				{
					let prefix = format!("{path}.{i}.");
					collect_relations(
						schema,
						c,
						&node["attrs"]["fields"],
						&prefix,
						out,
					);
				}
			}
			continue;
		}

		if let FieldKind::Group { component } = &field.kind {
			if let (Some(c), Some(group)) =
				(schema.get(component), fields.get(&field.handle))
			{
				collect_relations(schema, c, group, &format!("{path}."), out);
			}
			continue;
		}

		// a link to an entry or an asset is a relation too
		if let FieldKind::Link { .. } = field.kind {
			out.extend(link_relation(schema, &fields[&field.handle], &path));
			continue;
		}

		let Some(Value::Array(items)) = fields.get(&field.handle) else {
			continue;
		};

		match &field.kind {
			FieldKind::Relation { types, .. } => out.push(RelationRef {
				path,
				target: Target::Entries(types.clone()),
				ids: items.iter().filter_map(Value::as_i64).collect(),
			}),
			FieldKind::Assets { images, .. } => out.push(RelationRef {
				path,
				target: Target::Assets(*images),
				ids: items.iter().filter_map(Value::as_i64).collect(),
			}),
			FieldKind::Blocks { .. } => {
				for (i, block) in items.iter().enumerate() {
					let ty = block["type"].as_str().unwrap_or_default();
					if let Some(c) = schema.get(ty) {
						let prefix = format!("{path}.{i}.");
						collect_relations(
							schema,
							c,
							&block["fields"],
							&prefix,
							out,
						);
					}
				}
			}
			_ => {}
		}
	}
}

/// The fields of `into` with the shared (not translatable) ones taken
/// from `from`, so every site has the same shared values.
pub fn share(comp: &Component, from: &Value, into: &Value) -> Value {
	let mut out = into.as_object().cloned().unwrap_or_default();
	for f in comp.fields.iter().filter(|f| !f.translatable) {
		match from.get(&f.handle) {
			Some(v) => out.insert(f.handle.clone(), v.clone()),
			None => out.remove(&f.handle),
		};
	}
	Value::Object(out)
}

/// A row of `entry_values`.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexValue {
	pub field: String,
	pub text: Option<String>,
	pub num: Option<f64>,
}

/// The values of the indexed fields of an entry.
pub fn index_values(comp: &Component, fields: &Value) -> Vec<IndexValue> {
	comp.fields
		.iter()
		.filter(|f| f.indexed)
		.filter_map(|f| {
			let v = fields.get(&f.handle)?;
			let (text, num) = match v {
				Value::String(s) => (Some(s.clone()), None),
				Value::Number(n) => (None, n.as_f64()),
				Value::Bool(b) => (None, Some(*b as u8 as f64)),
				_ => return None,
			};
			Some(IndexValue {
				field: f.handle.clone(),
				text,
				num,
			})
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;
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
		let ok = link(
			json!({ "type": "url", "value": "https://x.ch", "label": "X" }),
		);
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
	fn index_values_of_entry() {
		let s = site();
		let fields = json!({ "city": "Bern", "price": 12.5 });
		let vals = index_values(s.get("event").unwrap(), &fields);
		assert_eq!(vals[0].text.as_deref(), Some("Bern"));
		assert_eq!(vals[1].num, Some(12.5));
	}
}
