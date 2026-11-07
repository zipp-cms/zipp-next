use super::*;

/// Checks the fields of a component and returns them cleaned up: nulls are
/// dropped and new blocks get an id.
pub fn validate(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
) -> Result<Value, Vec<FieldError>> {
	let mut errors = vec![];
	let out = validate_fields(schema, comp, fields, "", false, &mut errors);
	if errors.is_empty() {
		Ok(out)
	} else {
		Err(errors)
	}
}

/// `hidden`: the fields are inside a field hidden by its condition, they
/// keep their values but needn't be filled.
pub(crate) fn validate_fields(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
	prefix: &str,
	hidden: bool,
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
		// a hidden field keeps its value but needn't be filled, also
		// not the fields inside it
		let hidden = hidden || !comp.is_shown(&field.handle, obj);
		if let Some(v) =
			validate_value(schema, field, value, &path, hidden, errors)
		{
			out.insert(field.handle.clone(), v);
		}
	}

	Value::Object(out)
}

/// Whether a value counts as nothing before it is checked: null, an
/// empty text or list, a link without a target.
fn is_blank(field: &Field, value: &Value) -> bool {
	match value {
		Value::Null => true,
		Value::String(s) => s.is_empty(),
		Value::Array(a) => a.is_empty(),
		Value::Object(_) if matches!(field.kind, FieldKind::Link { .. }) => {
			link_is_empty(value)
		}
		_ => false,
	}
}

/// Whether a field has no value, like its required check says; for the
/// conditions of fields.
pub(crate) fn is_empty(field: &Field, value: &Value) -> bool {
	is_blank(field, value)
		|| (matches!(field.kind, FieldKind::RichText { .. })
			&& richtext::is_empty(value))
}

/// Whether a field takes a value as it is, nothing empty.
pub(crate) fn takes(schema: &Schema, field: &Field, value: &Value) -> bool {
	let mut errors = vec![];
	let out = validate_value(schema, field, value, "", false, &mut errors);
	errors.is_empty() && out.as_ref() == Some(value)
}

/// Whether a field's default is a value it takes. Defaults are for plain
/// values, not for ids of the database or content with blocks.
pub(crate) fn check_default(
	schema: &Schema,
	field: &Field,
) -> Result<(), String> {
	let Some(default) = &field.default else {
		return Ok(());
	};
	let plain = matches!(
		field.kind,
		FieldKind::Text { .. }
			| FieldKind::Email
			| FieldKind::Number { .. }
			| FieldKind::Boolean
			| FieldKind::Date { .. }
			| FieldKind::Markdown
			| FieldKind::Dropdown { .. }
			| FieldKind::ButtonGroup { .. }
			| FieldKind::Checkboxes { .. }
	) || matches!(&field.kind, FieldKind::Custom { blocks, .. } if blocks.is_empty());
	if !plain {
		return Err("only fields of plain values have one".into());
	}

	let mut errors = vec![];
	let value = validate_value(schema, field, default, "", false, &mut errors);
	match (errors.first(), value) {
		(Some(e), _) => Err(e.message.clone()),
		(None, None) => Err("is empty".into()),
		(None, Some(_)) => Ok(()),
	}
}

pub(super) fn validate_value(
	schema: &Schema,
	field: &Field,
	value: &Value,
	path: &str,
	hidden: bool,
	errors: &mut Vec<FieldError>,
) -> Option<Value> {
	// hidden by its condition it needn't be filled, its value is checked
	let required = field.required && !hidden;
	if is_blank(field, value) {
		// a group without a value yet (a new entry) still needs the
		// fields it requires
		if let FieldKind::Group { component } = &field.kind {
			let comp = schema.get(component).unwrap();
			let none = Value::Object(Default::default());
			let prefix = format!("{path}.");
			validate_fields(schema, comp, &none, &prefix, hidden, errors);
		}
		return filled(Value::Null, true, required, path, errors);
	}

	match (&field.kind, value) {
		(FieldKind::Text { max, .. }, Value::String(s)) => {
			if let Some(max) = max
				&& s.chars().count() > *max as usize
			{
				error(errors, path, &format!("at most {max} characters"));
			}
		}
		(
			FieldKind::Number {
				min, max, decimals, ..
			},
			Value::Number(n),
		) => {
			let n = n.as_f64().unwrap_or_default();
			if let Err(e) = check_number(n, *min, *max, *decimals) {
				error(errors, path, &e);
			}
		}
		(FieldKind::Boolean, Value::Bool(_)) => {}
		(FieldKind::Email, Value::String(s)) => {
			if !valid_email(s) {
				error(errors, path, "an email address like name@example.com");
			}
		}
		(FieldKind::Markdown, Value::String(_)) => {}
		(FieldKind::Date { time }, Value::String(s)) => {
			if !valid_date(s, *time) {
				let want = if *time {
					"a date and time like 2026-11-14T20:00"
				} else {
					"a date like 2026-11-14"
				};
				error(errors, path, want);
			}
		}
		(FieldKind::Checkboxes { options }, Value::Array(values)) => {
			let out = validate_checkboxes(options, values, path, errors);
			let empty = out.is_empty();
			return filled(Value::Array(out), empty, required, path, errors);
		}
		(FieldKind::Table { columns }, Value::Array(rows)) => {
			let out =
				validate_table(schema, columns, rows, path, hidden, errors);
			let empty = out.is_empty();
			return filled(Value::Array(out), empty, required, path, errors);
		}
		(FieldKind::Navigation { to, depth, .. }, Value::Array(items)) => {
			let out = validate_navigation(to, *depth, items, path, errors);
			let empty = out.is_empty();
			return filled(Value::Array(out), empty, required, path, errors);
		}
		(FieldKind::Texts { keys }, Value::Object(texts)) => {
			let out = validate_texts(keys, texts, path, errors);
			let empty = out.is_empty();
			return filled(Value::Object(out), empty, required, path, errors);
		}
		(FieldKind::Dropdown { options }, Value::String(s)) => {
			if !options.iter().any(|o| &o.value == s) {
				error(errors, path, "not one of the options");
			}
		}
		(FieldKind::ButtonGroup { options }, Value::String(s)) => {
			if !options.iter().any(|o| &o.value == s) {
				error(errors, path, "not one of the options");
			}
		}
		(FieldKind::Group { component }, Value::Object(_)) => {
			let comp = schema.get(component).unwrap();
			let prefix = format!("{path}.");
			let out =
				validate_fields(schema, comp, value, &prefix, hidden, errors);
			let empty = out.as_object().is_some_and(|o| o.is_empty());
			return filled(out, empty, required, path, errors);
		}
		(kind @ FieldKind::RichText { .. }, Value::Object(_)) => {
			let allowed = richtext::Allowed::of(kind).unwrap();
			let doc = richtext::validate(
				schema, &allowed, value, path, hidden, errors,
			);
			let empty = richtext::is_empty(&doc);
			return filled(doc, empty, required, path, errors);
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
			let min = min.filter(|_| !hidden);
			check_count(errors, path, blocks.len(), min, *max);
			let blocks =
				validate_blocks(schema, types, blocks, path, hidden, errors);
			return Some(Value::Array(blocks));
		}
		(FieldKind::Note { .. }, _) => {
			error(errors, path, "a note holds no value")
		}
		(
			FieldKind::Custom {
				ty,
				options,
				blocks,
			},
			value,
		) => {
			// zipp checks the blocks of the items, the type the rest
			let items = match blocks.is_empty() {
				true => Some(value.clone()),
				false => {
					item_blocks(schema, blocks, value, path, hidden, errors)
				}
			};
			let Some(items) = items else {
				return Some(value.clone());
			};
			let t = schema.field_types.get(ty).unwrap();
			match t.validate(options, &items) {
				// null: nothing left, like an empty text
				Ok(v) => return (!v.is_null()).then_some(v),
				Err(e) => error(errors, path, &e),
			}
		}
		_ => error(errors, path, "wrong type"),
	}

	Some(value.clone())
}

/// A number within the field's range with at most its decimal places.
fn check_number(
	n: f64,
	min: Option<f64>,
	max: Option<f64>,
	decimals: Option<u32>,
) -> Result<(), String> {
	if min.is_some_and(|min| n < min) || max.is_some_and(|max| n > max) {
		return Err("out of range".into());
	}
	match decimals {
		Some(d) if !has_decimals(n, d) => Err(match d {
			0 => "a whole number".into(),
			1 => "at most 1 decimal place".into(),
			d => format!("at most {d} decimal places"),
		}),
		_ => Ok(()),
	}
}

/// The ticked options of a checkboxes field, each once.
fn validate_checkboxes(
	options: &[Choice],
	values: &[Value],
	path: &str,
	errors: &mut Vec<FieldError>,
) -> Vec<Value> {
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
	out
}

/// The blocks of a blocks field, each checked like its component.
fn validate_blocks(
	schema: &Schema,
	types: &[String],
	blocks: &[Value],
	path: &str,
	hidden: bool,
	errors: &mut Vec<FieldError>,
) -> Vec<Value> {
	blocks
		.iter()
		.enumerate()
		.map(|(i, b)| {
			let p = format!("{path}.{i}");
			validate_block(schema, types, b, &p, hidden, errors)
		})
		.collect()
}

/// A value after its check: nothing left of it is no value, and an error
/// when the field must be filled.
fn filled(
	value: Value,
	empty: bool,
	required: bool,
	path: &str,
	errors: &mut Vec<FieldError>,
) -> Option<Value> {
	if empty {
		if required {
			error(errors, path, "is required");
		}
		return None;
	}
	Some(value)
}

/// The rows of a table, each cell checked like a field of its column.
fn validate_table(
	schema: &Schema,
	columns: &[Column],
	rows: &[Value],
	path: &str,
	hidden: bool,
	errors: &mut Vec<FieldError>,
) -> Vec<Value> {
	let mut out = vec![];
	for (i, row) in rows.iter().enumerate() {
		let Some(row) = row.as_object() else {
			error(errors, &format!("{path}.{i}"), "must be a row");
			continue;
		};
		let mut cells = Map::new();
		for c in columns {
			let cell = row.get(&c.handle).unwrap_or(&Value::Null);
			// texts without the spaces around them
			let cell = match cell {
				Value::String(t) => Value::from(t.trim()),
				v => v.clone(),
			};
			let p = format!("{path}.{i}.{}", c.handle);
			let field = c.as_field();
			let v = validate_value(schema, &field, &cell, &p, hidden, errors);
			match v {
				// no is nothing, a row of only noes is empty
				None | Some(Value::Bool(false)) => continue,
				Some(Value::String(u))
					if c.kind == ColumnKind::Url && !safe_url(&u) =>
				{
					error(errors, &p, "a url like https://… or /path")
				}
				Some(v) => {
					cells.insert(c.handle.clone(), v);
				}
			}
		}
		// an empty row is no row
		if !cells.is_empty() {
			out.push(Value::Object(cells));
		}
	}
	out
}

/// The texts of a texts field: only the keys of the code, empty ones fall
/// back to its text.
fn validate_texts(
	keys: &[(String, String)],
	texts: &Map<String, Value>,
	path: &str,
	errors: &mut Vec<FieldError>,
) -> Map<String, Value> {
	let mut out = Map::new();
	for (key, text) in texts {
		match text {
			Value::String(t) if t.is_empty() => {}
			Value::String(_) if keys.iter().any(|(k, _)| k == key) => {
				out.insert(key.clone(), text.clone());
			}
			Value::String(_) => {}
			_ => error(errors, &format!("{path}.{key}"), "must be text"),
		}
	}
	out
}

/// The items of a custom type with their blocks checked, `{ "block" }`
/// each; none when it isn't a list of objects.
fn item_blocks(
	schema: &Schema,
	types: &[String],
	value: &Value,
	path: &str,
	hidden: bool,
	errors: &mut Vec<FieldError>,
) -> Option<Value> {
	let Some(items) = value.as_array() else {
		error(errors, path, "must be a list");
		return None;
	};
	if items.iter().any(|i| !i.is_object()) {
		error(errors, path, "must be a list of objects");
		return None;
	}

	// the studio's editor finds a block by its id: a copied one gets its
	// own
	let mut ids = std::collections::HashSet::new();
	let items = items
		.iter()
		.enumerate()
		.map(|(i, item)| {
			let mut item = item.clone();
			let mut block = validate_block(
				schema,
				types,
				&item["block"],
				&format!("{path}.{i}.block"),
				hidden,
				errors,
			);
			if let Some(id) = block["id"].as_str()
				&& !ids.insert(id.to_string())
			{
				block["id"] = new_block_id().into();
			}
			item["block"] = block;
			item
		})
		.collect();
	Some(Value::Array(items))
}

pub(super) fn validate_block(
	schema: &Schema,
	types: &[String],
	block: &Value,
	path: &str,
	hidden: bool,
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
		hidden,
		errors,
	);

	let mut out = serde_json::json!({ "id": id, "type": ty, "fields": fields });
	// switched off: kept, the website doesn't show it
	if block["enabled"] == false {
		out["enabled"] = false.into();
	}
	out
}

/// Whether `n` has at most `d` places after the point. JSON numbers are
/// the closest f64 of what was sent, rounding to `d` places and back gives
/// the same f64 only if there were no more.
fn has_decimals(n: f64, d: u32) -> bool {
	let p = 10f64.powi(d as i32);
	(n * p).round() / p == n
}

/// Whether this looks like an address mail can go to: `name@host.tld`,
/// no spaces; whether it exists only a mail tells.
pub fn valid_email(s: &str) -> bool {
	let Some((user, host)) = s.split_once('@') else {
		return false;
	};
	!user.is_empty()
		&& host.contains('.')
		&& !host.starts_with('.')
		&& !host.ends_with('.')
		&& !host.contains('@')
		&& !s.contains(|c: char| c.is_whitespace() || "<>,;\"".contains(c))
}

/// Whether a date field may hold this: `2026-11-14`, with `time`
/// `2026-11-14T20:00`, a day the calendar has.
pub(crate) fn valid_date(s: &str, time: bool) -> bool {
	let num = |r: std::ops::Range<usize>| -> Option<u32> {
		let p = s.get(r)?;
		p.bytes()
			.all(|b| b.is_ascii_digit())
			.then(|| p.parse().ok())?
	};
	let len = if time { 16 } else { 10 };
	// only ASCII, slicing other text at a byte could cut a character
	if !s.is_ascii() || s.len() != len || &s[4..5] != "-" || &s[7..8] != "-" {
		return false;
	}
	let (Some(y), Some(m), Some(d)) = (num(0..4), num(5..7), num(8..10)) else {
		return false;
	};
	let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
	let days = match m {
		2 if leap => 29,
		2 => 28,
		4 | 6 | 9 | 11 => 30,
		1..=12 => 31,
		_ => return false,
	};
	if d == 0 || d > days {
		return false;
	}
	if !time {
		return true;
	}
	&s[10..11] == "T"
		&& &s[13..14] == ":"
		&& num(11..13).is_some_and(|h| h < 24)
		&& num(14..16).is_some_and(|min| min < 60)
}

/// A url a link may have: the web, or a path or anchor of the site. No
/// `javascript:` and the like, they run as the website, and no `//` or
/// `/\\`, browsers go to another host with them.
pub(crate) fn safe_url(u: &str) -> bool {
	u.starts_with("https://")
		|| u.starts_with("http://")
		|| (u.starts_with('/') && !u.starts_with("//") && !u.starts_with("/\\"))
		|| u.starts_with('#')
}

/// `{ "type": "url", "value": "https://…", "label": "…", "newTab": true }`,
/// value is the id for an entry or an asset; entries and urls may have
/// an `anchor`, a section of the page (`kontakt` for `#kontakt`).
pub(crate) fn check_link(
	to: &[LinkKind],
	label: bool,
	link: &Map<String, Value>,
) -> Result<(), String> {
	let ty = link.get("type").and_then(Value::as_str).unwrap_or_default();
	let kind = link.get("type").and_then(|t| LinkKind::deserialize(t).ok());
	let Some(kind) = kind else {
		return Err(format!("unknown link type {ty:?}"));
	};
	if !to.contains(&kind) {
		return Err(format!("can't link to {ty}"));
	}

	let value = link.get("value").unwrap_or(&Value::Null);
	let ok = match kind {
		LinkKind::Entry | LinkKind::Asset => value.is_i64(),
		LinkKind::Url => value.as_str().is_some_and(safe_url),
		LinkKind::Email => value.as_str().is_some_and(valid_email),
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
			"anchor" => matches!(kind, LinkKind::Entry | LinkKind::Url),
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
	let anchor = link.get("anchor").map(|a| a.as_str().unwrap_or("#"));
	if anchor.is_some_and(|a| {
		!a.chars()
			.all(|c| c.is_alphanumeric() || c == '-' || c == '_')
	}) {
		return Err("an anchor is letters, digits, - and _, without #".into());
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
