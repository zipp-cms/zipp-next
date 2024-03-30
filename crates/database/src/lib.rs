mod error;
#[cfg(feature = "memory")]
pub mod memory;
pub mod types;

pub use error::Error;
use types::{component::Component, schema::Schema};

#[async_trait::async_trait]
pub trait Database {
	async fn set_schema(&self, schema: Schema) -> Result<(), Error>;
	async fn get_schema(&self, name: &str) -> Result<Option<Schema>, Error>;
	async fn delete_schema(&self, name: &str) -> Result<(), Error>;

	async fn set_component(&self, component: Component) -> Result<(), Error>;
	async fn get_component(
		&self,
		name: &str,
	) -> Result<Option<Component>, Error>;
	async fn delete_component(&self, name: &str) -> Result<(), Error>;
}
