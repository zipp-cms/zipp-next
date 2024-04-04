use crate::{
	types::{id::Id, schema},
	Error,
};

use serde_json::Value;

pub fn validate_schema_value(
	value: &Value,
	field: &schema::Field,
) -> Result<(), Error> {
	use schema::FieldKind;

	if value.is_null() && field.nullable {
		return Ok(());
	}

	match (&field.kind, value) {
		(FieldKind::Id | FieldKind::ComponentId, Value::String(id)) => {
			// make sure the value is a valid id
			let _id: Id = id.parse().map_err(|_| Error::IncorrectDataType {
				expected: "id".into(),
				got: id.clone().into(),
			})?;

			Ok(())
		}
		(FieldKind::Boolean, Value::Bool(_)) => Ok(()),
		(FieldKind::Int, Value::Number(n)) => {
			let _n = n.as_i64().ok_or_else(|| Error::IncorrectDataType {
				expected: "int".into(),
				got: format!("{n:?}").into(),
			})?;

			Ok(())
		}
		(FieldKind::Float, Value::Number(n)) => {
			let _n = n.as_f64().ok_or_else(|| Error::IncorrectDataType {
				expected: "float".into(),
				got: format!("{n:?}").into(),
			})?;

			Ok(())
		}
		(FieldKind::Text, Value::String(_)) => Ok(()),
		_ => todo!(),
	}
}
