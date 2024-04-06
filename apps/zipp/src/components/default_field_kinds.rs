use serde_json::Value;

use super::field_kinds::{
	FieldKind, FieldTrait, ParseFieldError, Settings, ValidateError,
};

type Error = std::io::Error;

pub struct NumberFieldKind;
const NUMBER_FIELD_NAME: &str = "number";

impl FieldKind for NumberFieldKind {
	type Field = NumberField;

	fn name() -> String {
		NUMBER_FIELD_NAME.to_string()
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

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default = "NumberField::default")]
pub struct NumberField {
	max: u32,
	min: u32,
}

impl NumberField {
	fn min_validate(&self, value: &Value) -> Result<(), ValidateError> {
		value
			.as_u64()
			.filter(|v| self.min as u64 <= *v)
			.map(|_| ())
			.ok_or(ValidateError::ValidationFailed)
	}

	fn max_validate(&self, value: &Value) -> Result<(), ValidateError> {
		value
			.as_u64()
			.filter(|v| self.max as u64 > *v)
			.map(|_| ())
			.ok_or(ValidateError::ValidationFailed)
	}
}

impl Default for NumberField {
	fn default() -> Self {
		Self {
			max: u32::MAX,
			min: u32::MIN,
		}
	}
}

impl FieldTrait for NumberField {
	fn name(&self) -> String {
		NUMBER_FIELD_NAME.to_string()
	}

	fn settings(&self) -> Settings {
		let mut settings = Settings::new();

		if self.max != NumberField::default().max {
			settings.insert(
				"max".to_string(),
				serde_json::to_value(&self.max).expect("todo"),
			);
		}
		if self.min != NumberField::default().min {
			settings.insert(
				"min".to_string(),
				serde_json::to_value(&self.min).expect("todo"),
			);
		}
		settings
	}

	fn clone_box(&self) -> Box<dyn FieldTrait> {
		Box::new(self.clone())
	}

	fn validate(&self, value: &Value) -> Result<(), ValidateError> {
		self.max_validate(value).and(self.min_validate(value))
	}
}

pub struct TextFieldKind;
const TEXT_FIELD_NAME: &str = "text";

impl FieldKind for TextFieldKind {
	type Field = TextField;

	fn name() -> String {
		TEXT_FIELD_NAME.to_string()
	}

	fn build(&self, settings: Settings) -> Result<TextField, ParseFieldError> {
		let settings = Value::Object(settings.into_iter().collect());

		let field: TextField = serde_json::from_value(settings).expect("todo");

		Ok(field)
	}
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default = "TextField::default")]
pub struct TextField {
	max_length: i32,
}

impl Default for TextField {
	fn default() -> Self {
		Self { max_length: 255 }
	}
}

impl FieldTrait for TextField {
	fn name(&self) -> String {
		TEXT_FIELD_NAME.to_string()
	}
	fn settings(&self) -> Settings {
		let mut settings = Settings::new();
		if self.max_length != TextField::default().max_length {
			settings.insert(
				"max_length".to_string(),
				Value::Number(self.max_length.into()),
			);
		}
		settings
	}
	fn validate(&self, value: &Value) -> Result<(), ValidateError> {
		value
			.as_str()
			.filter(|v| self.max_length as usize > v.len())
			.map(|_| ())
			.ok_or(ValidateError::ValidationFailed)
	}

	fn clone_box(&self) -> Box<dyn FieldTrait> {
		Box::new(self.clone())
	}
}
