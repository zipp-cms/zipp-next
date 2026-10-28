//! Field types of other crates. A crate implements [`FieldType`], the
//! site registers it with `Schema::field_type` and uses it with
//! `Field::custom`. The value is stored as JSON like every other field;
//! the type checks it, says how GraphQL shows it and brings the editor of
//! the studio as an ES module.

use std::fmt;
use std::sync::Arc;

use serde_json::Value;

/// A scalar of GraphQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scalar {
	String,
	Int,
	Float,
	Boolean,
}

/// How GraphQL shows the value of a field type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphqlType {
	Scalar(Scalar),
	/// An object type of the given name with these fields, the output is
	/// a JSON object with them.
	Object {
		name: &'static str,
		fields: Vec<(&'static str, Scalar)>,
	},
}

/// The editor of a field type in the studio: an ES module that exports
/// `mount(target, props)`, props are `{ field, value, onchange }`, and
/// returns `{ update(props), destroy() }`. It brings its own framework
/// and css, the studio only mounts it.
#[derive(Debug, Clone, Copy)]
pub struct StudioModule {
	pub js: &'static str,
}

pub trait FieldType: Send + Sync + 'static {
	/// The name in the schema, like `buttonGroup`.
	fn name(&self) -> &'static str;

	/// Checks the options of a field when the schema is checked.
	fn check(&self, _options: &Value) -> Result<(), String> {
		Ok(())
	}

	/// Checks a value that is not empty, returns it cleaned up.
	fn validate(&self, options: &Value, value: &Value)
	-> Result<Value, String>;

	fn graphql(&self) -> GraphqlType;

	/// The value for GraphQL, in the shape of [`FieldType::graphql`].
	fn output(&self, _options: &Value, value: &Value) -> Value {
		value.clone()
	}

	/// The TypeScript type of a stored value.
	fn typescript(&self) -> String {
		"unknown".into()
	}

	fn studio(&self) -> StudioModule;
}

/// The registered field types, part of the schema but not of its JSON.
#[derive(Clone, Default)]
pub struct FieldTypes(pub Vec<Arc<dyn FieldType>>);

impl FieldTypes {
	pub fn get(&self, name: &str) -> Option<&Arc<dyn FieldType>> {
		self.0.iter().find(|t| t.name() == name)
	}
}

impl fmt::Debug for FieldTypes {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_list()
			.entries(self.0.iter().map(|t| t.name()))
			.finish()
	}
}

/// Code, not content: two schemas are the same whatever types they know.
impl PartialEq for FieldTypes {
	fn eq(&self, _: &Self) -> bool {
		true
	}
}
