//! Content is plain JSON: an entry's fields are an object, a blocks field
//! is a list of `{ "id", "type", "fields" }` and a relation a list of ids.

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::schema::{
	ColumnKind, Component, Field, FieldKind, Holds, LinkKind, Schema,
};

mod relations;
pub mod richtext;
#[cfg(test)]
pub(crate) mod tests;
mod validate;

pub use relations::RelationRef;
pub use relations::Target;
pub use relations::relations;
pub(crate) use validate::check_link;
pub(crate) use validate::safe_url;
pub use validate::validate;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldError {
	/// `body.0.label`: field handles and block positions
	pub path: String,
	pub message: String,
}

fn error(errors: &mut Vec<FieldError>, path: &str, message: &str) {
	errors.push(FieldError {
		path: path.into(),
		message: message.into(),
	});
}

/// The kind of a link value, `{ "type": "entry", "value": 5, … }`.
pub(crate) fn link_kind(link: &Value) -> Option<LinkKind> {
	link.get("type").and_then(|t| LinkKind::deserialize(t).ok())
}

/// Whether a link has no target yet; the studio keeps one while an
/// editor picks it, saving leaves it out.
pub(crate) fn link_is_empty(link: &Value) -> bool {
	link.get("value").is_none_or(|v| v.is_null() || v == "")
}

pub(crate) fn new_block_id() -> String {
	let n: u64 = rand::rng().random();
	format!("{n:016x}")
}

/// A component in the value of a field: a block, a block embedded in a
/// rich text, the fields of a group. `at` is where in the value, the
/// position of a block or empty for a group.
pub struct Inner<'a> {
	pub comp: &'a Component,
	pub fields: &'a Value,
	pub at: Option<usize>,
}

impl Inner<'_> {
	/// The path of its fields: `body.2.` for a block, `seo.` for a group.
	pub fn prefix(&self, path: &str) -> String {
		match self.at {
			Some(i) => format!("{path}.{i}."),
			None => format!("{path}."),
		}
	}
}

/// The components in the value of a field, as [`FieldKind::holds`] says;
/// blocks of unknown types are left out.
pub fn inner<'a>(
	schema: &'a Schema,
	field: &'a Field,
	value: &'a Value,
) -> Vec<Inner<'a>> {
	let block = |at: usize, b: &'a Value| {
		let comp = schema.get(b["type"].as_str()?)?;
		Some(Inner {
			comp,
			fields: &b["fields"],
			at: Some(at),
		})
	};
	match field.kind.holds() {
		Holds::Nothing => vec![],
		Holds::Blocks => value
			.as_array()
			.into_iter()
			.flatten()
			.enumerate()
			.filter_map(|(i, b)| block(i, b))
			.collect(),
		// the position in the document, embeds keep the block in `attrs`
		Holds::RichText => value["content"]
			.as_array()
			.into_iter()
			.flatten()
			.enumerate()
			.filter(|(_, n)| n["type"] == "embed")
			.filter_map(|(i, n)| block(i, &n["attrs"]))
			.collect(),
		Holds::Group(component) => schema
			.get(component)
			.filter(|_| !value.is_null())
			.map(|comp| Inner {
				comp,
				fields: value,
				at: None,
			})
			.into_iter()
			.collect(),
	}
}

/// Like [`inner`], the components and their fields to change.
pub fn inner_mut<'a>(
	schema: &'a Schema,
	field: &Field,
	value: &'a mut Value,
) -> Vec<(&'a Component, &'a mut Value)> {
	let blocks: Vec<&mut Value> = match field.kind.holds() {
		Holds::Nothing => return vec![],
		Holds::Blocks => match value {
			Value::Array(list) => list.iter_mut().collect(),
			_ => return vec![],
		},
		Holds::RichText => richtext::blocks_mut(value),
		Holds::Group(component) => {
			return match schema.get(component) {
				Some(c) if !value.is_null() => vec![(c, value)],
				_ => vec![],
			};
		}
	};
	blocks
		.into_iter()
		.filter_map(|b| {
			let comp = schema.get(b["type"].as_str()?)?;
			Some((comp, &mut b["fields"]))
		})
		.collect()
}

/// Calls `f` with the fields of every component in the content: first the
/// entry's own, then those of each block and group, depth first.
pub fn walk_mut(
	schema: &Schema,
	comp: &Component,
	fields: &mut Value,
	f: &mut impl FnMut(&Component, &mut Map<String, Value>),
) {
	let Value::Object(obj) = fields else { return };
	f(comp, obj);

	for field in &comp.fields {
		let Some(value) = obj.get_mut(&field.handle) else {
			continue;
		};
		for (c, fields) in inner_mut(schema, field, value) {
			walk_mut(schema, c, fields, f);
		}
	}
}

/// The texts of a texts field with the code's text where none is saved.
pub fn texts_of(
	keys: &[(String, String)],
	value: &Value,
) -> Map<String, Value> {
	keys.iter()
		.map(|(key, default)| {
			let text = value[key].as_str().filter(|t| !t.is_empty());
			(key.clone(), Value::from(text.unwrap_or(default)))
		})
		.collect()
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
