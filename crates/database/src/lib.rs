//! Database layer
//!
//! This crate provides various database implementations
//! like a postgres connector, or a memory database.
//!
//! The database has two types of data Schemas and Components.
//!
//! Schemas roughfly correspond to tables in a relational database.
//! Components are a more abstract type container.
//!
//! Components should be used when their data model might change often
//! and that model can be changed by a user.
//!
//! A schema is often accompanied by a component.

pub mod error;
pub mod types;

mod adaptor;

use std::collections::BTreeMap;

use adaptor::Adaptor;
use error::Error;
use serde_json::{Map, Value};
use types::{
	guards::Valid,
	schema::{self, Schema},
};

use crate::adaptor::{types::validate_schema_value, CreateSchemaData};

#[derive(Debug)]
pub struct Database {
	adaptor: Box<dyn Adaptor>,
}

impl Database {
	#[cfg(feature = "memory")]
	pub fn new_memory() -> Self {
		Self {
			adaptor: Box::new(adaptor::memory::MemoryDatabase::new()),
		}
	}

	/// Updates or creates a schema
	///
	/// If a schema get's updated their data representation is updated as well
	///
	///
	/// ## Note
	/// The schema name needs to be globally unique
	pub async fn set_schema(&self, schema: Schema) -> Result<(), Error> {
		// todo validate and make sure the schema is correct
		// we need atleast one primary, no duplicate keys
		// related values need to match another field

		if let Some(_existing_schema) =
			self.adaptor.get_schema(&schema.name).await?
		{
			todo!("update schema")
		}

		self.adaptor
			.create_schema(Valid::assume_valid(schema))
			.await
	}

	/// Create a new schema data this might update multiple schemas
	///
	/// schema data needs to be structured as
	/// ```json
	/// {
	///		"schema": {
	/// 		"field": "value"
	/// 	},
	/// 	"other_schema": [
	/// 		{ "field": "value" },
	/// 		{ "field": "value" }
	/// 	]
	/// }
	/// ```
	///
	/// ## Note
	/// Related fields will be check to contain the correct data
	pub async fn create_schema_data(
		&self,
		data: schema::Data,
	) -> Result<(), Error> {
		// validate the data

		let mut queue = Vec::with_capacity(data.len());

		self.validate_and_transform_schema_data(data, &mut queue)
			.await?;

		self.adaptor.create_schema_data(queue).await
	}

	async fn validate_and_transform_schema_data(
		&self,
		data: schema::Data,
		queue: &mut Vec<CreateSchemaData>,
	) -> Result<(), Error> {
		for (name, value) in data {
			let schema =
				self.adaptor.get_schema(&name).await?.ok_or_else(|| {
					Error::SchemaNotFound(name.clone().into())
				})?;

			// each schema should contain an object
			let Value::Object(fields) = value else {
				return Err(Error::SchemaExpectsAnObject {
					schema: name.clone().into(),
					found: value.to_string().into(),
				});
			};

			// convert the schemas fields into a map so we can remove them while we validate
			let mut schema_fields = schema
				.fields
				.into_iter()
				.map(|f| (f.name.clone(), f))
				.collect::<BTreeMap<_, _>>();

			let mut n_fields = BTreeMap::new();

			// validate fields
			for (field, value) in fields {
				// todo this might contain another schema

				let Some(schema_field) = schema_fields.remove(&field) else {
					// either the field is a schema and both schemas are related
					// or the field does not exists and we need to return an error

					// todo validate that they are related
					if let Some(_schema) =
						self.adaptor.get_schema(&field).await?
					{
						let mut map = Map::new();
						map.insert(field, value);

						Box::pin(
							self.validate_and_transform_schema_data(map, queue),
						)
						.await?;

						continue;
					}

					return Err(Error::SetUnknownFieldToSchema {
						schema: name.clone().into(),
						field: field.clone().into(),
					});
				};

				let value = validate_schema_value(value, &schema_field)?;

				n_fields.insert(field, value);
			}

			// make sure all fields are set
			for missing_field in schema_fields {
				return Err(Error::MissingField {
					schema: name.clone().into(),
					field: missing_field.0.into(),
				});
			}

			queue.push(CreateSchemaData {
				schema: name,
				data: n_fields,
			});
		}

		Ok(())
	}
}
