//! Fields holds a list of all field kinds
//!
//! fields kinds can be used in components
//! entities and other places
//!
//! From a field kind you can create a field

pub mod defaults;
mod error;
mod field;
mod kind;
mod schema;

pub use error::{ParseFieldError, ValidateError};
pub use field::{BoxedField, Field};
pub use kind::{BoxedFieldKind, ErasedFieldKind, FieldKind, Settings};
pub use schema::{
	BoxedFieldSchema, ErasedFieldSchema, FieldSchema, PersistentKind,
};

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct Fields {
	inner: Arc<RwLock<Inner>>,
}

impl Fields {
	pub fn new() -> Self {
		Self {
			inner: Arc::new(RwLock::new(Inner::new())),
		}
	}

	pub fn insert<T>(&self, kind: T)
	where
		T: FieldKind + 'static,
	{
		let mut inner = self.inner.write().unwrap();

		inner.push(kind);
	}

	pub fn exists(&self, name: &str) -> bool {
		let inner = self.inner.read().unwrap();

		inner.exists(name)
	}

	/// parse a field from the settings
	pub fn parse_field(
		&self,
		name: &str,
		settings: Settings,
	) -> Result<Box<dyn ErasedFieldSchema>, ParseFieldError> {
		let inner = self.inner.read().unwrap();

		inner.parse_field(name, settings)
	}
}

#[derive(Debug)]
struct Inner {
	kinds: BTreeMap<String, Box<dyn ErasedFieldKind>>,
}

impl Inner {
	fn new() -> Self {
		Self {
			kinds: BTreeMap::new(),
		}
	}

	pub fn push<T>(&mut self, kind: T)
	where
		T: FieldKind + 'static,
	{
		self.kinds.insert(T::name(), Box::new(kind));
	}

	pub fn exists(&self, name: &str) -> bool {
		self.kinds.contains_key(name)
	}

	/// parse a field from the settings
	pub fn parse_field(
		&self,
		name: &str,
		settings: Settings,
	) -> Result<Box<dyn ErasedFieldSchema>, ParseFieldError> {
		match self.kinds.get(name) {
			Some(kind) => kind.parse(settings),
			None => Err(ParseFieldError::KindNotFound(name.to_string())),
		}
	}
}

impl Default for Fields {
	fn default() -> Self {
		let mut this = Self::new();

		this.insert(defaults::NumberFieldKind);
		this.insert(defaults::TextFieldKind);

		// boolean, number, text, relation, media, component, richtext

		this
	}
}

#[cfg(test)]
mod tests {
	use super::Fields;

	fn is_send<T: Send>() {}

	#[test]
	fn test_send_sync() {
		is_send::<Fields>();
		is_send::<&Fields>();
	}
}
