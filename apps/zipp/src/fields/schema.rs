use std::{fmt::Debug, ops::Deref};

use crate::utils::AsAny;

use super::{Field, Settings, ValidateError};

// todo this should probably be called FieldSchema?
pub trait FieldSchema: AsAny + Debug + Send + Sync {
	type Field: Field;

	/// returns the kind of this field
	fn kind(&self) -> String;

	/// returns the current settings
	///
	/// used to store this fields schema
	fn settings(&self) -> Settings;

	/// validates field data
	///
	/// called when data is passed via an api
	fn from_api(
		&self,
		value: serde_json::Value,
	) -> Result<Self::Field, ValidateError>;

	/// what type this field should have in the persistent layer
	///
	fn persistent_kind(&self) -> PersistentKind;

	// fn from_persistent(value: serde_json::Value) -> Self::Field;

	// /// makes a clone of the field
	fn clone_box(&self) -> Box<dyn ErasedFieldSchema>;
}

#[derive(Debug, Clone, Copy)]
pub enum PersistentKind {
	Id,
	ComponentRelation,
	Boolean,
	Int,
	Float,
	Text,
	Json,
	DateTime,
}

pub trait ErasedFieldSchema: AsAny + Debug + Send + Sync {
	fn kind(&self) -> String;

	fn settings(&self) -> Settings;

	fn from_api(
		&self,
		value: serde_json::Value,
	) -> Result<Box<dyn Field>, ValidateError>;

	fn persistent_kind(&self) -> PersistentKind;

	fn clone_box(&self) -> Box<dyn ErasedFieldSchema>;
}

impl<T> ErasedFieldSchema for T
where
	T: FieldSchema + 'static,
{
	fn kind(&self) -> String {
		self.kind()
	}

	fn settings(&self) -> Settings {
		self.settings()
	}

	fn from_api(
		&self,
		value: serde_json::Value,
	) -> Result<Box<dyn Field>, ValidateError> {
		let field = self.from_api(value)?;
		Ok(Box::new(field))
	}

	fn persistent_kind(&self) -> PersistentKind {
		self.persistent_kind()
	}

	fn clone_box(&self) -> Box<dyn ErasedFieldSchema> {
		self.clone_box()
	}
}

#[derive(Debug)]
pub struct BoxedFieldSchema {
	inner: Box<dyn ErasedFieldSchema>,
}

impl BoxedFieldSchema {
	pub fn new<T>(schema: T) -> Self
	where
		T: FieldSchema + 'static,
	{
		Self {
			inner: Box::new(schema),
		}
	}

	pub fn downcast_ref<T>(&self) -> Option<&T>
	where
		T: FieldSchema + 'static,
	{
		self.inner.as_any().downcast_ref()
	}
}

impl From<Box<dyn ErasedFieldSchema>> for BoxedFieldSchema {
	fn from(schema: Box<dyn ErasedFieldSchema>) -> Self {
		Self { inner: schema }
	}
}

impl Deref for BoxedFieldSchema {
	type Target = dyn ErasedFieldSchema;

	fn deref(&self) -> &Self::Target {
		&*self.inner
	}
}

impl Clone for BoxedFieldSchema {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone_box(),
		}
	}
}
