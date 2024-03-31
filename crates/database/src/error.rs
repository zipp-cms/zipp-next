use std::borrow::Cow;

// this error
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("field {field} is missing from schema {schema}")]
	MissingField {
		schema: Cow<'static, str>,
		field: Cow<'static, str>,
	},

	#[error("incorrect data type: expected {expected} got {got}")]
	IncorrectDataType {
		expected: Cow<'static, str>,
		got: Cow<'static, str>,
	},

	#[error("schema {schema} does not have a field {field}")]
	SetUnknownFieldToSchema {
		schema: Cow<'static, str>,
		field: Cow<'static, str>,
	},

	#[error("schema {schema} expects an object found {found}")]
	SchemaExpectsAnObject {
		schema: Cow<'static, str>,
		found: Cow<'static, str>,
	},

	#[error("schema not found: {0}")]
	SchemaNotFound(Cow<'static, str>),

	#[error("unknown error: {0}")]
	Unknown(Cow<'static, str>),
}
