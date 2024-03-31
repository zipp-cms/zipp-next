mod components;
mod schema;

use std::sync::RwLock;

use components::ComponentRepository;
use schema::SchemaRepository;

use crate::{
	types::{component::Component, query::Query, schema::Schema},
	Database, Error,
};

#[derive(Debug)]
pub struct MemoryDatabase {
	schemas: RwLock<SchemaRepository>,
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
impl Database for MemoryDatabase {
	async fn set_schema(&self, schema: Schema) -> Result<(), Error> {
		let mut schemas = self.schemas.write().unwrap();

		schemas.set_schema(schema)
	}

	async fn get_schema(&self, name: &str) -> Result<Option<Schema>, Error> {
		let schemas = self.schemas.read().unwrap();

		schemas.get_schema(name)
	}

	async fn delete_schema(&self, name: &str) -> Result<(), Error> {
		let mut schemas = self.schemas.write().unwrap();

		schemas.delete_schema(name)
	}

	async fn query_schema_data(
		&self,
		query: Query,
	) -> Result<super::schema::Data, Error> {
		let schemas = self.schemas.read().unwrap();

		schemas.query_schema_data(query)
	}

	async fn set_component(&self, component: Component) -> Result<(), Error> {
		let mut components = self.components.write().unwrap();

		components.set_component(component)
	}

	async fn get_component(
		&self,
		name: &str,
	) -> Result<Option<Component>, Error> {
		let components = self.components.read().unwrap();

		components.get_component(name)
	}

	async fn delete_component(&self, name: &str) -> Result<(), Error> {
		let mut components = self.components.write().unwrap();

		components.delete_component(name)
	}
}
