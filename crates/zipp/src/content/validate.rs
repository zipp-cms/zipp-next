use super::*;

use serde_json::json;

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

pub(super) fn validate_value(
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
		(FieldKind::Checkboxes { options }, Value::Array(values)) => {
			let mut out: Vec<Value> = vec![];
			for v in values {
				match v.as_str() {
					Some(s) if options.iter().any(|o| o.value == s) => {
						if !out.contains(v) {
							out.push(v.clone());
						}
					}
					_ => error(errors, path, "not one of the options"),
				}
			}
			return (!out.is_empty()).then_some(Value::Array(out));
		}
		(FieldKind::Table { columns }, Value::Array(rows)) => {
			let mut out = vec![];
			for (i, row) in rows.iter().enumerate() {
				let Some(row) = row.as_object() else {
					error(errors, &format!("{path}.{i}"), "must be a row");
					continue;
				};
				let mut cells = Map::new();
				for c in columns {
					let text = row.get(&c.handle).and_then(Value::as_str);
					let Some(text) =
						text.map(str::trim).filter(|t| !t.is_empty())
					else {
						continue;
					};
					let ok = match c.kind {
						ColumnKind::Text => true,
						ColumnKind::Url => safe_url(text),
						ColumnKind::Email => text.contains('@'),
					};
					if !ok {
						error(
							errors,
							&format!("{path}.{i}.{}", c.handle),
							"not valid",
						);
					}
					cells.insert(c.handle.clone(), Value::from(text));
				}
				// an empty row is no row
				if !cells.is_empty() {
					out.push(Value::Object(cells));
				}
			}
			return (!out.is_empty()).then_some(Value::Array(out));
		}
		(FieldKind::Navigation { to, depth }, Value::Array(items)) => {
			let out = validate_navigation(to, *depth, items, path, errors);
			return (!out.is_empty()).then_some(Value::Array(out));
		}
		(FieldKind::Seo, Value::Object(seo)) => {
			let mut out = Map::new();
			for (key, v) in seo {
				let p = format!("{path}.{key}");
				let empty = v.is_null() || v == "" || v == &json!([]);
				match key.as_str() {
					_ if empty => {}
					"title" | "description" | "socialTitle"
					| "socialDescription"
						if v.is_string() => {}
					"image" | "socialImage"
						if v.as_array().is_some_and(|a| {
							a.len() <= 1 && a.iter().all(Value::is_i64)
						}) => {}
					"robots"
						if crate::schema::seo::ROBOTS
							.iter()
							.any(|(r, _)| v == *r) => {}
					"canonicalUrl" if v.as_str().is_some_and(safe_url) => {}
					_ => {
						error(errors, &p, "not a valid value");
						continue;
					}
				}
				if !empty {
					out.insert(key.clone(), v.clone());
				}
			}
			return (!out.is_empty()).then_some(Value::Object(out));
		}
		(FieldKind::Texts { keys }, Value::Object(texts)) => {
			// only the keys of the code, empty ones fall back to its text
			let mut out = Map::new();
			for (key, text) in texts {
				match text {
					Value::String(t) if t.is_empty() => {}
					Value::String(_) if keys.iter().any(|(k, _)| k == key) => {
						out.insert(key.clone(), text.clone());
					}
					Value::String(_) => {}
					_ => {
						error(errors, &format!("{path}.{key}"), "must be text")
					}
				}
			}
			return (!out.is_empty()).then_some(Value::Object(out));
		}
		(FieldKind::Dropdown { options }, Value::String(s)) => {
			if !options.iter().any(|o| &o.value == s) {
				error(errors, path, "not one of the options");
			}
		}
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

pub(super) fn validate_block(
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

/// A url a link may have: the web, or a path or anchor of the site. No
/// `javascript:` and the like, they run as the website.
pub(crate) fn safe_url(u: &str) -> bool {
	u.starts_with("https://")
		|| u.starts_with("http://")
		|| (u.starts_with('/') && !u.starts_with("//"))
		|| u.starts_with('#')
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
		LinkKind::Url => value.as_str().is_some_and(safe_url),
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
			// a file to download instead of opening it
			"download" => kind == LinkKind::Asset,
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

pub(super) fn check_count(
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

/// The items of a navigation: a link with a label and the items under it.
fn validate_navigation(
	to: &[LinkKind],
	depth: u32,
	items: &[Value],
	path: &str,
	errors: &mut Vec<FieldError>,
) -> Vec<Value> {
	let mut out = vec![];
	for (i, item) in items.iter().enumerate() {
		let p = format!("{path}.{i}");
		let Some(obj) = item.as_object() else {
			error(errors, &p, "must be a link");
			continue;
		};
		let mut link = obj.clone();
		link.remove("children");
		if let Err(e) = check_link(to, true, &link) {
			error(errors, &p, &e);
		}
		let mut item = obj.clone();
		let children = obj.get("children").and_then(Value::as_array);
		match children.filter(|c| !c.is_empty()) {
			Some(_) if depth <= 1 => {
				error(errors, &p, "too deep");
			}
			Some(children) => {
				let children = validate_navigation(
					to,
					depth - 1,
					children,
					&format!("{p}.children"),
					errors,
				);
				item.insert("children".into(), Value::Array(children));
			}
			None => {
				item.remove("children");
			}
		}
		out.push(Value::Object(item));
	}
	out
}
