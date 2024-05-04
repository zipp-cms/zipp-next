//! Fields holds a list of all field kinds
//!
//! fields kinds can be used in components
//! entities and other places
//!
//! From a field kind you can create a field

pub mod defaults;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{self, Debug};
use std::sync::{Arc, RwLock};

use crate::utils::AsAny;

pub type Settings = BTreeMap<String, serde_json::Value>;

#[derive(Debug, thiserror::Error)]
pub enum ValidateError {
	#[error("Field validation failed")]
	ValidationFailed,
}

// todo this should probably be called FieldSchema?
pub trait Field: AsAny + Debug + Send + Sync {
	/// returns the kind of this field
	fn kind(&self) -> String;

	/// returns the current settings
	fn settings(&self) -> Settings;

	/// validates field data
	fn validate(&self, value: &serde_json::Value) -> Result<(), ValidateError>;

	/// makes a clone of the field
	fn clone_box(&self) -> Box<dyn Field>;
}

#[derive(thiserror::Error)]
pub enum ParseFieldError {
	#[error("Field has unknown kind: {0}")]
	KindNotFound(String),

	#[error("Invalid settings: {settings:?}")]
	InvalidSettings { settings: Vec<String> },
}

impl fmt::Debug for ParseFieldError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self)
	}
}

pub trait FieldKind: Debug + Send + Sync {
	type Field: Field;

	fn name() -> String;

	fn parse(&self, settings: Settings)
		-> Result<Self::Field, ParseFieldError>;
}

pub trait ErasedFieldKind: Debug + Send + Sync {
	fn parse(
		&self,
		settings: Settings,
	) -> Result<Box<dyn Field>, ParseFieldError>;
}

impl<T> ErasedFieldKind for T
where
	T: FieldKind + 'static,
{
	fn parse(
		&self,
		settings: Settings,
	) -> Result<Box<dyn Field>, ParseFieldError> {
		let field = self.parse(settings)?;
		Ok(Box::new(field))
	}
}

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
	) -> Result<Box<dyn Field>, ParseFieldError> {
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
	) -> Result<Box<dyn Field>, ParseFieldError> {
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
