//! Content is plain JSON: an entry's fields are an object, a blocks field
//! is a list of `{ "id", "type", "fields" }` and a relation a list of ids.

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::schema::{
	ColumnKind, Component, Field, FieldKind, LinkKind, Schema,
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
pub(crate) use validate::validate_fields;

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

/// The fields of `into` with the shared (not translatable) ones taken
/// from `from`, so every site has the same shared values.
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
