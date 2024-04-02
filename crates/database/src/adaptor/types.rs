use crate::{
	types::{id::Id, schema},
	Error,
};

use serde_json::{Number, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BasicValue {
	Null,
	Bool(bool),
	Number(Number),
	String(String),
}

pub fn validate_schema_value(
	value: Value,
	field: &schema::Field,
) -> Result<BasicValue, Error> {
	use schema::FieldKind;

	if value.is_null() && field.nullable {
		return Ok(BasicValue::Null);
	}

	match (&field.kind, value) {
		(FieldKind::Id | FieldKind::ComponentId, Value::String(id)) => {
			// make sure the value is a valid id
			let _id: Id = id.parse().map_err(|_| Error::IncorrectDataType {
				expected: "id".into(),
				got: id.clone().into(),
			})?;

			Ok(BasicValue::String(id))
		}
		(FieldKind::Boolean, Value::Bool(b)) => Ok(BasicValue::Bool(b)),
		(FieldKind::Int, Value::Number(n)) => {
			let _n = n.as_i64().ok_or_else(|| Error::IncorrectDataType {
				expected: "int".into(),
				got: format!("{n:?}").into(),
			})?;

			Ok(BasicValue::Number(n))
		}
		(FieldKind::Float, Value::Number(n)) => {
			let _n = n.as_f64().ok_or_else(|| Error::IncorrectDataType {
				expected: "float".into(),
				got: format!("{n:?}").into(),
			})?;

			Ok(BasicValue::Number(n))
		}
		(FieldKind::Text, Value::String(s)) => Ok(BasicValue::String(s)),
		_ => todo!(),
	}
}
