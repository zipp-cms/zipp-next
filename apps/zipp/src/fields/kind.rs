use std::fmt::Debug;
use std::{collections::BTreeMap, ops::Deref};

use super::{ErasedFieldSchema, FieldSchema, ParseFieldError};

pub type Settings = BTreeMap<String, serde_json::Value>;

pub trait FieldKind: Debug + Send + Sync {
	type Field: FieldSchema;

	fn name() -> String;

	fn parse(&self, settings: Settings)
		-> Result<Self::Field, ParseFieldError>;
}

pub trait ErasedFieldKind: Debug + Send + Sync {
	fn parse(
		&self,
		settings: Settings,
	) -> Result<Box<dyn ErasedFieldSchema>, ParseFieldError>;
}

impl<T> ErasedFieldKind for T
where
	T: FieldKind + 'static,
{
	fn parse(
		&self,
		settings: Settings,
	) -> Result<Box<dyn ErasedFieldSchema>, ParseFieldError> {
		let field = self.parse(settings)?;
		Ok(Box::new(field))
	}
}

pub struct BoxedFieldKind {
	inner: Box<dyn ErasedFieldKind>,
}

impl BoxedFieldKind {
	pub fn new<T>(kind: T) -> Self
	where
		T: FieldKind + 'static,
	{
		Self {
			inner: Box::new(kind),
		}
	}
}

impl Deref for BoxedFieldKind {
	type Target = dyn ErasedFieldKind;

	fn deref(&self) -> &Self::Target {
		&*self.inner
	}
}
