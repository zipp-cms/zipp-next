use std::borrow::Cow;

// this error
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("field {field} is missing from schema {schema}")]
	MissingField {
		schema: Cow<'static, str>,
		field: Cow<'static, str>,
	},

	#[error("field {field} already exists in schema {schema}")]
	DuplicateField {
		schema: Cow<'static, str>,
		field: Cow<'static, str>,
	},

	#[error("field {field} is a primary field in schema {schema} and is not allowed to be set")]
	PrimaryFieldSet {
		schema: Cow<'static, str>,
		field: Cow<'static, str>,
	},

	#[error("incorrect data type: expected {expected} got {got}")]
	IncorrectDataType {
		expected: Cow<'static, str>,
		got: Cow<'static, str>,
	},

	#[error("schema {schema} does not have a field {field}")]
	UnknownFieldToSchema {
		schema: Cow<'static, str>,
		field: Cow<'static, str>,
	},

	#[error("schema {schema} expects an object found {found}")]
	SchemaExpectsAnObject {
		schema: Cow<'static, str>,
		found: Cow<'static, str>,
	},

	#[error("schema {schema} expects an array found {found}")]
	SchemaExpectsAnArray {
		schema: Cow<'static, str>,
		found: Cow<'static, str>,
	},

	#[error("schema not found: {0}")]
	SchemaNotFound(Cow<'static, str>),

	#[error("unknown error: {0}")]
	Unknown(Cow<'static, str>),
}
