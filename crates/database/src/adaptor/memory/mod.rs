mod components;
mod schema;

use std::sync::RwLock;

use components::ComponentRepository;
use schema::SchemaRepository;

use crate::{
	types::{
		guards::Valid,
		query::Query,
		schema::{CreateSchema, Schema, SchemaEntries},
	},
	Error,
};

use super::Adaptor;

#[derive(Debug)]
pub struct MemoryDatabase {
	schemas: RwLock<SchemaRepository>,
	#[allow(dead_code)]
	components: RwLock<ComponentRepository>,
}

impl MemoryDatabase {
	pub fn new() -> Self {
		Self {
			schemas: RwLock::new(SchemaRepository::new()),
			components: RwLock::new(ComponentRepository::new()),
		}
	}
}

#[async_trait::async_trait]
impl Adaptor for MemoryDatabase {
	async fn create_schema(
		&self,
		schema: Valid<CreateSchema>,
	) -> Result<Schema, Error> {
		let mut schemas = self.schemas.write().unwrap();

		schemas.create_schema(schema)
	}

	async fn get_schema(&self, name: &str) -> Result<Option<Schema>, Error> {
		let schemas = self.schemas.read().unwrap();

		schemas.get_schema(name)
	}

	// async fn delete_schema(&self, name: &str) -> Result<(), Error> {
	// 	let mut schemas = self.schemas.write().unwrap();

	// 	schemas.delete_schema(name)
	// }

	async fn create_schema_entries(
		&self,
		schema: String,
		data: SchemaEntries,
	) -> Result<SchemaEntries, Error> {
		let mut schemas = self.schemas.write().unwrap();

		schemas.create_schema_entries(schema, data.clone())?;

		Ok(data)
	}

	async fn read_schema_data(
		&self,
		query: Query,
	) -> Result<SchemaEntries, Error> {
		let schemas = self.schemas.read().unwrap();

		schemas.read_schema_data(query)
	}

	// async fn query_schema_data(
	// 	&self,
	// 	query: Query,
	// ) -> Result<super::schema::Data, Error> {
	// 	let schemas = self.schemas.read().unwrap();

	// 	schemas.query_schema_data(query)
	// }

	// async fn set_component(&self, component: Component) -> Result<(), Error> {
	// 	let mut components = self.components.write().unwrap();

	// 	components.set_component(component)
	// }

	// async fn get_component(
	// 	&self,
	// 	name: &str,
	// ) -> Result<Option<Component>, Error> {
	// 	let components = self.components.read().unwrap();

	// 	components.get_component(name)
	// }

	// async fn delete_component(&self, name: &str) -> Result<(), Error> {
	// 	let mut components = self.components.write().unwrap();

	// 	components.delete_component(name)
	// }
}
