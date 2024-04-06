use std::{
	collections::BTreeMap,
	fmt::{self, Debug},
};

use serde_json::Value;
use tracing::error;

use super::{
	default_field_kinds::{NumberFieldKind, TextFieldKind},
	Field, FieldDto,
};

pub type Settings = BTreeMap<String, serde_json::Value>;

#[derive(Debug, thiserror::Error)]
pub enum ValidateError {
	#[error("Field validation failed")]
	ValidationFailed,
}

pub trait FieldTrait: Debug {
	fn name(&self) -> String;
	fn settings(&self) -> Settings;
	fn clone_box(&self) -> Box<dyn FieldTrait>;
	fn validate(&self, value: &serde_json::Value) -> Result<(), ValidateError>;
}

pub trait FieldKind {
	type Field: FieldTrait;

	fn name() -> String;

	fn build(&self, settings: Settings)
		-> Result<Self::Field, ParseFieldError>;
}

pub trait ErasedFieldKind {
	fn build(
		&self,
		settings: Settings,
	) -> Result<Box<dyn FieldTrait>, ParseFieldError>;
}

impl<T> ErasedFieldKind for T
where
	T: FieldKind + 'static,
{
	fn build(
		&self,
		settings: Settings,
	) -> Result<Box<dyn FieldTrait>, ParseFieldError> {
		let field = self.build(settings)?;
		Ok(Box::new(field))
	}
}

pub struct FieldKinds(BTreeMap<String, Box<dyn ErasedFieldKind>>);

#[derive(thiserror::Error)]
pub enum ParseFieldError {
	#[error("Field has unknown kind: {kind}")]
	KindNotFound { kind: String },
	#[error("Invalid settings: {settings:?}")]
	InvalidSettings { settings: Vec<String> },
}

impl fmt::Debug for ParseFieldError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self)
	}
}

impl FieldKinds {
	pub fn push<T>(&mut self, kind: T)
	where
		T: FieldKind + 'static,
	{
		self.0.insert(T::name(), Box::new(kind));
	}

	pub fn field_from_dto(
		&self,
		dto: FieldDto,
	) -> Result<Field, ParseFieldError> {
		// check if kind exists
		self.0
			.get(&dto.kind)
			.ok_or(ParseFieldError::KindNotFound { kind: dto.kind })?
			.build(dto.settings)
			.map(Field::new)
	}
}

impl Default for FieldKinds {
	fn default() -> Self {
		let mut this = Self(BTreeMap::new());

		this.push(NumberFieldKind);
		this.push(TextFieldKind);

		// boolean, number, text, relation, media, component, richtext

		this
	}
}
