mod error;
#[cfg(feature = "memory")]
pub mod memory;
pub mod types;

pub use error::Error;
use types::{
	component::Component,
	query::Query,
	schema::{self, Schema},
};

/// Create a new memory database
#[cfg(feature = "memory")]
pub fn new_memory() -> Box<dyn Database> {
	Box::new(memory::MemoryDatabase::new())
}

#[async_trait::async_trait]
pub trait Database {
	/// Set a schema
	async fn set_schema(&self, schema: Schema) -> Result<(), Error>;
	/// Get a schema
	async fn get_schema(&self, name: &str) -> Result<Option<Schema>, Error>;
	/// Delete a schema
	async fn delete_schema(&self, name: &str) -> Result<(), Error>;

	async fn query_schema_data(
		&self,
		query: Query,
	) -> Result<schema::Data, Error>;

	/// Set a component
	async fn set_component(&self, component: Component) -> Result<(), Error>;
	/// Get a component
	async fn get_component(
		&self,
		name: &str,
	) -> Result<Option<Component>, Error>;
	/// Delete a component
	async fn delete_component(&self, name: &str) -> Result<(), Error>;
}
