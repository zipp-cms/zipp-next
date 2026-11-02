//! A field type outside of zipp, as an example: a row of buttons in
//! colors where the editor picks one, like Craft's button group. The
//! value is the value of the option; GraphQL gives value, label and color,
//! so the website can show the label in its color.
//!
//! ```ignore
//! Schema::new()
//!     .field_type(ButtonGroup)
//!     .entry("event", "Event", [
//!         button_group("genre", [
//!             ("concert", "Konzert", "#2b6cb0"),
//!             ("reading", "Lesung", "#2f855a"),
//!         ]),
//!     ])
//! ```
//!
//! The editor of the studio is `studio/`, a small Svelte project built
//! into `assets/button-group.js` (`npm run build`, committed) and compiled into
//! this crate.

use serde_json::{Value, json};
use zipp::{Field, FieldType, GraphqlType, Scalar, StudioModule};

#[cfg(test)]
mod tests;

pub struct ButtonGroup;

/// A button group field with its options: value, label and color.
pub fn button_group<'a>(
	handle: &str,
	options: impl IntoIterator<Item = (&'a str, &'a str, &'a str)>,
) -> Field {
	let options: Vec<Value> = options
		.into_iter()
		.map(|(value, label, color)| {
			json!({ "value": value, "label": label, "color": color })
		})
		.collect();
	Field::custom(handle, "buttonGroup", Value::Array(options))
}

fn find<'a>(options: &'a Value, value: &str) -> Option<&'a Value> {
	options.as_array()?.iter().find(|o| o["value"] == value)
}

fn is_color(c: &str) -> bool {
	c.len() == 7
		&& c.starts_with('#')
		&& c[1..].chars().all(|c| c.is_ascii_hexdigit())
}

impl FieldType for ButtonGroup {
	fn name(&self) -> &'static str {
		"buttonGroup"
	}

	fn check(&self, options: &Value) -> Result<(), String> {
		let list = options.as_array().filter(|l| !l.is_empty());
		let Some(list) = list else {
			return Err("a button group needs options".into());
		};
		for (i, o) in list.iter().enumerate() {
			let value = o["value"].as_str().unwrap_or_default();
			if value.is_empty() {
				return Err("an option needs a value".into());
			}
			if list[..i].iter().any(|x| x["value"] == value) {
				return Err(format!("option {value} exists twice"));
			}
			if !is_color(o["color"].as_str().unwrap_or_default()) {
				return Err(format!("option {value}: the color is #rrggbb"));
			}
		}
		Ok(())
	}

	fn validate(
		&self,
		options: &Value,
		value: &Value,
	) -> Result<Value, String> {
		match value.as_str() {
			Some(v) if find(options, v).is_some() => Ok(value.clone()),
			_ => Err("not one of the options".into()),
		}
	}

	fn graphql(&self) -> GraphqlType {
		GraphqlType::Object {
			name: "ButtonGroupOption",
			fields: vec![
				("value", Scalar::String),
				("label", Scalar::String),
				("color", Scalar::String),
			],
		}
	}

	/// The option, an option that was removed since shows as nothing.
	fn output(&self, options: &Value, value: &Value) -> Value {
		value
			.as_str()
			.and_then(|v| find(options, v))
			.cloned()
			.unwrap_or(Value::Null)
	}

	fn typescript(&self) -> String {
		"string".into()
	}

	fn studio(&self) -> StudioModule {
		StudioModule::new(include_str!("../assets/button-group.js"))
	}
}
