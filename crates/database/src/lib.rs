//! Database layer
//!
//! This crate provides various database implementations
//! like a postgres connector, or a memory database.
//!
//! The database has two types of containers Schemas and Components.
//!
//! Schemas roughfly correspond to tables in a relational database.
//! Components are a more abstract type container.
//!
//! Components should be used when their data model might change often
//! and that model can be changed by a user.
//!
//! A schema is often accompanied by a component.
//!
//! ## Terminology
//! Each container consists of entries.
//! Each entry has fields.

pub mod error;
pub mod types;

mod adaptor;

use std::{
	collections::{BTreeMap, BTreeSet},
	sync::Arc,
};

use adaptor::Adaptor;
use error::Error;
use serde_json::Value;
use types::{
	guards::Valid,
	id::Id,
	query::Query,
	schema::{Schema, SchemaEntries, SchemaFieldValue, SetSchema},
};

use crate::{adaptor::types::validate_schema_value, types::query::FieldQuery};

#[derive(Debug, Clone)]
pub struct Database {
	adaptor: Arc<dyn Adaptor>,
}

impl Database {
	#[cfg(feature = "memory")]
	pub fn new_memory() -> Self {
		Self {
			adaptor: Arc::new(adaptor::memory::MemoryDatabase::new()),
		}
	}

	/// Updates or creates a schema
	///
	/// If a schema get's updated their data representation is updated as well
	///
	///
	/// ## Note
	/// The schema name needs to be globally unique
	pub async fn set_schema(&self, schema: SetSchema) -> Result<Schema, Error> {
		// todo validate and make sure the schema is correct
		// we need atleast one primary, no duplicate keys
		// related values need to match another field

		if let Some(_existing_schema) =
			self.adaptor.get_schema(&schema.name).await?
		{
			todo!("schema update if needed")
		}

		self.adaptor
			.create_schema(Valid::assume_valid(schema))
			.await
	}

	/// Create a new schema data this might update multiple schemas
	///
	/// ## Note
	/// Related fields will be check to contain the correct data
	// todo refactor types, don't take multiple schemas, if this is wanted
	// just run the command in parallel
	pub async fn create_schema_entries(
		&self,
		schema: String,
		mut entries: SchemaEntries,
	) -> Result<SchemaEntries, Error> {
		// validate the data
		self.validate_schema_entries(&schema, &mut entries).await?;

		self.adaptor.create_schema_entries(schema, entries).await
	}

	/// Query schema data
	///
	/// Multiple schemas can be queried at once
	/// ```json
	/// {
	/// 	"schema": [
	/// 		"field1",
	/// 		"field2",
	/// 	],
	/// 	"other_schema": [
	/// 		{
	/// 			"name": "schema",
	/// 			"fields": [
	/// 				"field1",
	/// 				"field2",
	/// 			]
	/// 		}
	/// 	]
	/// }
	/// ```
	pub async fn read_schema_data(
		&self,
		query: Query,
	) -> Result<SchemaEntries, Error> {
		// validate the fields
		// validate filters

		self.validate_schema_query(&query).await?;

		self.adaptor.read_schema_data(query).await
	}

	async fn validate_schema_entries(
		&self,
		schema_name: &str,
		entries: &mut SchemaEntries,
	) -> Result<(), Error> {
		let schema =
			self.adaptor.get_schema(schema_name).await?.ok_or_else(|| {
				Error::SchemaNotFound(schema_name.to_string().into())
			})?;

		// convert the schemas fields into a map so we can remove them while we validate
		let schema_fields = schema
			.fields
			.into_iter()
			.map(|f| (f.name.clone(), f))
			.collect::<BTreeMap<_, _>>();

		for entry in &mut entries.0 {
			let mut schema_fields = schema_fields.clone();

			// validate fields
			for (name, value) in &mut entry.0 {
				match value {
					SchemaFieldValue::Entries(nested_entries) => {
						// the field is a schema
						// todo validate that they are related

						let _schema =
							self.adaptor.get_schema(&name).await?.ok_or_else(
								|| Error::SchemaNotFound(name.clone().into()),
							)?;

						Box::pin(
							self.validate_schema_entries(name, nested_entries),
						)
						.await?;

						continue;
					}
					SchemaFieldValue::Value(value) => {
						let Some(schema_field) = schema_fields.remove(name)
						else {
							return Err(Error::UnknownFieldToSchema {
								schema: schema_name.to_string().into(),
								field: name.clone().into(),
							});
						};

						// if this field is a primary field it is not allowed to be set
						if schema_field.primary {
							return Err(Error::PrimaryFieldSet {
								schema: schema_name.to_string().into(),
								field: name.clone().into(),
							});
						}

						validate_schema_value(value, &schema_field)?;
					}
				}
			}

			// make sure all fields are set
			for (name, field) in schema_fields {
				if field.primary {
					entry.0.insert(
						name,
						SchemaFieldValue::Value(Value::String(
							Id::new(schema.kind).to_string(),
						)),
					);
					continue;
				}

				if field.related.is_some() {
					todo!("set related field");
				}

				if field.nullable {
					entry.0.insert(name, SchemaFieldValue::Value(Value::Null));
					continue;
				}

				return Err(Error::MissingField {
					schema: schema_name.to_string().into(),
					field: name.into(),
				});
			}
		}

		Ok(())
	}

	async fn validate_schema_query(&self, query: &Query) -> Result<(), Error> {
		let name = &query.schema;

		let schema = self
			.adaptor
			.get_schema(&name)
			.await?
			.ok_or_else(|| Error::SchemaNotFound(name.clone().into()))?;

		if !query.filter.is_none() {
			todo!("validate filter")
		}

		if !query.sorting.is_none() {
			todo!("validate sorting")
		}

		self.validate_schema_query_fields(&schema, &query.fields.0)
			.await
	}

	async fn validate_schema_query_fields(
		&self,
		schema: &Schema,
		fields: &[FieldQuery],
	) -> Result<(), Error> {
		// make sure we don't have duplicated fields
		// make sure the fields are in the schema

		let mut used_fields: BTreeSet<&str> = BTreeSet::new();

		// validate fields
		for field in fields {
			let field_name = match field {
				FieldQuery::Schema { name, .. } => name,
				FieldQuery::Field(f) => f,
			};

			// check that the fields is not in the array
			if used_fields.contains(field_name.as_str()) {
				return Err(Error::DuplicateField {
					schema: schema.name.clone().into(),
					field: field_name.clone().into(),
				});
			}

			let field = match field {
				FieldQuery::Schema { name, fields } => {
					// todo validate that they are related

					let schema =
						self.adaptor.get_schema(&name).await?.ok_or_else(
							|| Error::SchemaNotFound(name.clone().into()),
						)?;

					let fut =
						self.validate_schema_query_fields(&schema, &fields.0);
					let fut = Box::pin(fut);
					fut.await?;

					name
				}
				FieldQuery::Field(name) => {
					// check that the field is in the schema
					if !schema.fields.iter().any(|f| &f.name == name) {
						return Err(Error::UnknownFieldToSchema {
							schema: schema.name.clone().into(),
							field: name.clone().into(),
						});
					}

					name
				}
			};

			used_fields.insert(field);
		}

		Ok(())
	}

	// /// the order of the queue needs to be top down
	// /// so the first schema is the first in the queue
	// /// and if a schema is nested it needs to be after the parent
	// async fn validate_and_transform_schema_query(
	// 	&self,
	// 	fields: &Map<String, Value>,
	// 	queue: &mut Vec<ReadSchemaData>,
	// ) -> Result<(), Error> {
	// 	for (name, value) in fields {
	// 		let schema =
	// 			self.adaptor.get_schema(&name).await?.ok_or_else(|| {
	// 				Error::SchemaNotFound(name.clone().into())
	// 			})?;

	// 		// each schema should contain an array of strings or objects
	// 		let Value::Array(fields) = value else {
	// 			return Err(Error::SchemaExpectsAnArray {
	// 				schema: name.clone().into(),
	// 				found: value.to_string().into(),
	// 			});
	// 		};

	// 		let mut n_fields: Vec<String> = vec![];

	// 		// validate fields
	// 		for value in fields {
	// 			let field = match value {
	// 				Value::Object(_) => todo!("nested fields"),
	// 				Value::String(s) => s,
	// 				_ => {
	// 					return Err(Error::IncorrectDataType {
	// 						expected: "string or object".into(),
	// 						got: value.to_string().into(),
	// 					})
	// 				}
	// 			};

	// 			// check that the fields is not in the array
	// 			if n_fields.contains(&field) {
	// 				return Err(Error::DuplicateField {
	// 					schema: name.clone().into(),
	// 					field: field.clone().into(),
	// 				});
	// 			}

	// 			// check that the field is in the schema
	// 			if !schema.fields.iter().any(|f| &f.name == field) {
	// 				return Err(Error::UnknownFieldToSchema {
	// 					schema: name.clone().into(),
	// 					field: field.clone().into(),
	// 				});
	// 			}

	// 			n_fields.push(field.clone());
	// 		}

	// 		queue.push(ReadSchemaData {
	// 			schema: name.clone(),
	// 			fields: n_fields,
	// 			filter: None,
	// 		});
	// 	}

	// 	Ok(())
	// }

	// // the data must match the query order
	// fn map_schema_query_data(
	// 	&self,
	// 	query: Query,
	// 	mut data: Vec<Vec<BTreeMap<String, BasicValue>>>,
	// ) -> Result<Map<String, Value>, Error> {
	// 	let mut result = Map::new();

	// 	let mut i = 0;

	// 	for (name, value) in &query.fields {
	// 		let fields = value.as_array();
	// 		let entries = mem::take(&mut data[i]);

	// 		let mut n_fields = Map::new();

	// 		for value in fields {
	// 			match value {
	// 				Value::Object(_) => todo!("obj"),
	// 				Value::String(s) => {

	// 			}
	// 		}
	// 	}

	// 	Ok(result)
	// }
}
