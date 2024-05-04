use std::{fmt::Debug, ops::Deref};

use serde_json::Value;

pub trait Field: Debug + Send + Sync {
	fn into_persistent(self) -> Value;

	// // todo this should accept some arguments about which arguments where selected
	// fn into_presentation(self) -> serde_json::Value;
}

#[derive(Debug)]
pub struct BoxedField {
	inner: Box<dyn Field>,
}

impl BoxedField {
	pub fn new<T>(field: T) -> Self
	where
		T: Field + 'static,
	{
		Self {
			inner: Box::new(field),
		}
	}
}

impl Deref for BoxedField {
	type Target = dyn Field;

	fn deref(&self) -> &Self::Target {
		&*self.inner
	}
}

impl From<Box<dyn Field>> for BoxedField {
	fn from(inner: Box<dyn Field>) -> Self {
		Self { inner }
	}
}
