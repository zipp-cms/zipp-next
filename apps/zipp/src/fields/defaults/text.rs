use serde::Deserialize;
use serde_json::Value;

use crate::fields::{
	ErasedFieldSchema, Field, FieldKind, FieldSchema, ParseFieldError,
	PersistentKind, Settings, ValidateError,
};

#[derive(Debug, Clone)]
pub struct TextFieldKind;
const TEXT_FIELD_NAME: &str = "text";

impl FieldKind for TextFieldKind {
	type Field = TextFieldSchema;

	fn name() -> String {
		TEXT_FIELD_NAME.to_string()
	}

	fn parse(
		&self,
		settings: Settings,
	) -> Result<TextFieldSchema, ParseFieldError> {
		let settings = Value::Object(settings.into_iter().collect());

		let field: TextFieldSchema =
			serde_json::from_value(settings).expect("todo");

		Ok(field)
	}
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default = "TextFieldSchema::default")]
pub struct TextFieldSchema {
	max_length: usize,
}

impl Default for TextFieldSchema {
	fn default() -> Self {
		Self { max_length: 255 }
	}
}

impl FieldSchema for TextFieldSchema {
	type Field = TextField;

	fn kind(&self) -> String {
		TextFieldKind::name()
	}

	fn settings(&self) -> Settings {
		let mut settings = Settings::new();
		if self.max_length != TextFieldSchema::default().max_length {
			settings.insert(
				"max_length".to_string(),
				Value::Number(self.max_length.into()),
			);
		}
		settings
	}

	fn from_api(&self, value: Value) -> Result<Self::Field, ValidateError> {
		let s = match value {
			Value::String(s) => s,
			_ => return Err(ValidateError::ValidationFailed),
		};

		if s.len() > self.max_length as usize {
			return Err(ValidateError::ValidationFailed);
		}

		Ok(TextField(s))
	}

	fn persistent_kind(&self) -> PersistentKind {
		PersistentKind::Text
	}

	fn clone_box(&self) -> Box<dyn ErasedFieldSchema> {
		Box::new(self.clone())
	}
}

#[derive(Debug, Clone)]
pub struct TextField(pub String);

impl Field for TextField {
	fn into_persistent(self) -> Value {
		self.0.into()
	}
}
