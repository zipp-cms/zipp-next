use serde_json::Value;

use super::field_kinds::{
	FieldKind, FieldTrait, ParseFieldError, Settings, ValidateError,
};

type Error = std::io::Error;

pub struct NumberFieldKind;

impl FieldKind for NumberFieldKind {
	type Field = NumberField;

	fn name() -> String {
		"number".to_string()
	}

	fn build(
		&self,
		settings: Settings,
	) -> Result<NumberField, ParseFieldError> {
		let settings = Value::Object(settings.into_iter().collect());

		let field: NumberField =
			serde_json::from_value(settings).expect("todo");

		Ok(field)
	}
}

#[derive(Debug, serde::Deserialize, Default)]
#[serde(transparent)]
struct MaxSetting(pub i32);

impl FieldTrait for MaxSetting {
	fn validate(&self, value: &Value) -> Result<(), ValidateError> {
		value
			.as_i64()
			.filter(|v| self.0 as i64 > *v)
			.map(|_| ())
			.ok_or(ValidateError::ValidationFailed)
	}
}

#[derive(Debug, serde::Deserialize)]
pub struct NumberField {
	// min: i32,
	// min: MinSetting,
	#[serde(default)]
	max: MaxSetting,
}

impl FieldTrait for NumberField {
	fn validate(&self, value: &Value) -> Result<(), ValidateError> {
		self.max.validate(value)
	}
}

pub struct TextFieldKind;

impl FieldKind for TextFieldKind {
	type Field = TextField;

	fn name() -> String {
		"text".to_string()
	}

	fn build(&self, settings: Settings) -> Result<TextField, ParseFieldError> {
		let settings = Value::Object(settings.into_iter().collect());

		let field: TextField = serde_json::from_value(settings).expect("todo");

		Ok(field)
	}
}

#[derive(Debug, serde::Deserialize)]
pub struct TextField {
	#[serde(default)]
	max_length: i32,
}

impl FieldTrait for TextField {
	fn validate(&self, value: &Value) -> Result<(), ValidateError> {
		value
			.as_str()
			.filter(|v| self.max_length as usize > v.len())
			.map(|_| ())
			.ok_or(ValidateError::ValidationFailed)
	}
}
