#[cfg(feature = "memory")]
pub mod memory;

pub mod types;

use std::{collections::BTreeMap, fmt};

use serde_json::Value;

pub use crate::error::Error;
use crate::types::{
	guards::Valid,
	schema::{CreateSchema, Schema, SchemaEntries},
};

#[derive(Debug, Clone)]
pub struct ReadSchemaData {
	pub schema: String,
	pub fields: Vec<String>,
	// todo related schema filters
	pub filter: Option<ReadSchemaDataFilter>,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ReadSchemaDataFilter {
	// todo: contains field from previous queue
	// .
	Equal { field: String, value: Value },
	And(Vec<ReadSchemaDataFilter>),
	Or(Vec<ReadSchemaDataFilter>),
}

#[async_trait::async_trait]
pub trait Adaptor: fmt::Debug {
	/// Creates a new schema
	async fn create_schema(
		&self,
		schema: Valid<CreateSchema>,
	) -> Result<Schema, Error>;

	/// Returns a schema by its name if the schema definition exists
	async fn get_schema(&self, name: &str) -> Result<Option<Schema>, Error>;

	// /// Delete a schema
	// ///
	// /// The schema data will be deleted as well
	// ///
	// /// Relations and other stuff is already validated so just delete the schema
	// /// and it's data
	// async fn delete_schema(&self, name: &str) -> Result<(), Error>;

	/// Create a new schema data this might update multiple schemas
	///
	/// all referenced schemas and fields are already validated
	async fn create_schema_entries(
		&self,
		schema: String,
		entries: SchemaEntries,
	) -> Result<SchemaEntries, Error>;

	// /// Read schema data
	// ///
	// /// All fields need to be valid and the filter needs to be executable
	// async fn read_schema_data(
	// 	&self,
	// 	queries: Vec<ReadSchemaData>,
	// ) -> Result<Vec<Vec<BTreeMap<String, BasicValue>>>, Error>;

	// /// Query schema data
	// ///
	// /// Multiple schemas can be queried at once
	// async fn query_schema_data(
	// 	&self,
	// 	query: Query,
	// ) -> Result<schema::Data, Error>;

	// create schema_data
	// update schema_data
	// delete schema_data

	// /// Set a component
	// async fn set_component(&self, component: Component) -> Result<(), Error>;
	// /// Get a component
	// async fn get_component(
	// 	&self,
	// 	name: &str,
	// ) -> Result<Option<Component>, Error>;
	// /// Delete a component
	// async fn delete_component(&self, name: &str) -> Result<(), Error>;
}
