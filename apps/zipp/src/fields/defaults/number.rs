use serde::Deserialize;
use serde_json::Value;

use crate::fields::{
	ErasedFieldSchema, Field, FieldKind, FieldSchema, ParseFieldError,
	PersistentKind, Settings, ValidateError,
};

#[derive(Debug, Clone)]
pub struct NumberFieldKind;

impl FieldKind for NumberFieldKind {
	type Field = NumberFieldSchema;

	fn name() -> String {
		"number".to_string()
	}

	fn parse(
		&self,
		settings: Settings,
	) -> Result<NumberFieldSchema, ParseFieldError> {
		let settings = Value::Object(settings.into_iter().collect());

		let field: NumberFieldSchema =
			serde_json::from_value(settings).expect("todo");

		Ok(field)
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default = "NumberFieldSchema::default")]
pub struct NumberFieldSchema {
	pub(crate) max: i64,
	pub(crate) min: i64,
}

impl Default for NumberFieldSchema {
	fn default() -> Self {
		Self {
			max: i64::MAX,
			min: i64::MIN,
		}
	}
}

impl FieldSchema for NumberFieldSchema {
	type Field = NumberField;

	fn kind(&self) -> String {
		NumberFieldKind::name()
	}

	fn settings(&self) -> Settings {
		let mut settings = Settings::new();

		if self.max != NumberFieldSchema::default().max {
			settings.insert(
				"max".to_string(),
				serde_json::to_value(&self.max).expect("todo"),
			);
		}
		if self.min != NumberFieldSchema::default().min {
			settings.insert(
				"min".to_string(),
				serde_json::to_value(&self.min).expect("todo"),
			);
		}
		settings
	}

	fn from_api(&self, value: Value) -> Result<Self::Field, ValidateError> {
		let num = value.as_i64().ok_or(ValidateError::ValidationFailed)?;

		if num < self.min || num >= self.max {
			return Err(ValidateError::ValidationFailed);
		}

		Ok(NumberField(num))
	}

	fn persistent_kind(&self) -> PersistentKind {
		PersistentKind::Int
	}

	fn clone_box(&self) -> Box<dyn ErasedFieldSchema> {
		Box::new(self.clone())
	}
}

#[derive(Debug, Clone)]
pub struct NumberField(i64);

impl Field for NumberField {
	fn into_persistent(self) -> Value {
		self.0.into()
	}
}
