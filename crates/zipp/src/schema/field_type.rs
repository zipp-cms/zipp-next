//! Field types of other crates. A crate implements [`FieldType`], the
//! site registers it with `Schema::field_type` and uses it with
//! `Field::custom`. The value is stored as JSON like every other field;
//! the type checks it, says how GraphQL shows it and brings the editor of
//! the studio as an ES module.

use std::fmt;
use std::sync::Arc;

use async_graphql::dynamic::{FieldValue, ResolverContext};
use serde_json::Value;

pub use crate::content::Target;

/// A scalar of GraphQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Scalar {
	String,
	Int,
	Float,
	Boolean,
}

/// How GraphQL shows the value of a field type.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GraphqlType {
	Scalar(Scalar),
	/// An object type of the given name with these fields, the output is
	/// a JSON object with them.
	Object {
		name: &'static str,
		fields: Vec<(&'static str, Scalar)>,
	},
	/// A type a [`Plugin`](crate::Plugin) registers, resolved by
	/// [`FieldType::resolve`].
	Named(&'static str),
}

/// What a field of a custom type relates to, so assets in use are known.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct FieldRelation {
	/// inside the value, like `image`, empty for the value itself
	pub path: String,
	pub target: Target,
	pub ids: Vec<i64>,
}

impl FieldRelation {
	/// Of the value itself, see [`FieldRelation::at`].
	pub fn new(target: Target, ids: Vec<i64>) -> Self {
		Self {
			path: String::new(),
			target,
			ids,
		}
	}

	/// Inside the value, like `image`.
	pub fn at(mut self, path: &str) -> Self {
		self.path = path.into();
		self
	}
}

/// The editor of a field type in the studio: an ES module that exports
/// `mount(target, props)`, props are `{ field, value, onchange, entryId,
/// site, host }` (see docs/extending.md), and returns
/// `{ update(props), destroy() }`. It brings its own framework
/// and css, the studio only mounts it.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub struct StudioModule {
	pub js: &'static str,
}

impl StudioModule {
	pub fn new(js: &'static str) -> Self {
		Self { js }
	}
}

pub trait FieldType: Send + Sync + 'static {
	/// The name in the schema, like `buttonGroup`.
	fn name(&self) -> &'static str;

	/// Checks the options of a field when the schema is checked.
	fn check(&self, _options: &Value) -> Result<(), String> {
		Ok(())
	}

	/// Checks a value that is not empty, returns it cleaned up; null if
	/// nothing is left.
	fn validate(&self, options: &Value, value: &Value)
	-> Result<Value, String>;

	fn graphql(&self) -> GraphqlType;

	/// The value for GraphQL, in the shape of [`FieldType::graphql`].
	fn output(&self, _options: &Value, value: &Value) -> Value {
		value.clone()
	}

	/// The field in GraphQL with the request, for output that needs more
	/// than the value: the parent entry, the database, a plugin's types.
	/// `ctx.parent_value` is the entry or the block; by default the
	/// [`FieldType::output`] of a value that isn't null.
	fn resolve<'a>(
		&self,
		_ctx: &ResolverContext<'a>,
		options: &Value,
		value: &Value,
	) -> async_graphql::Result<Option<FieldValue<'a>>> {
		if value.is_null() {
			return Ok(None);
		}
		let out = self.output(options, value);
		Ok(Some(FieldValue::value(async_graphql::Value::from_json(
			out,
		)?)))
	}

	/// The entries and assets a value relates to.
	fn relations(
		&self,
		_options: &Value,
		_value: &Value,
	) -> Vec<FieldRelation> {
		vec![]
	}

	/// The TypeScript type of a stored value.
	fn typescript(&self) -> String {
		"unknown".into()
	}

	fn studio(&self) -> StudioModule;
}

/// The registered field types, part of the schema but not of its JSON.
#[derive(Clone, Default)]
pub struct FieldTypes(pub(crate) Vec<Arc<dyn FieldType>>);

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
