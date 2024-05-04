mod json;
mod memory;

use std::fmt::Debug;

use super::{schema::ComponentSchema, SchemaError};
use crate::fields::Fields;

#[async_trait::async_trait]
pub trait Persistent: Debug + Send + Sync + 'static {
	async fn load(
		&mut self,
		fields: &Fields,
	) -> Result<Vec<ComponentSchema>, SchemaError>;

	async fn save(
		&mut self,
		contents: &[ComponentSchema],
	) -> Result<(), SchemaError>;
}

pub fn new_memory() -> memory::Memory {
	memory::Memory::new()
}

pub fn new_file(file_name: &str) -> json::JsonStorage {
	json::JsonStorage::new(file_name)
}
