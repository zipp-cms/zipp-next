use crate::{components::schemas::schema::ComponentSchema, fields::Fields};

use super::{Persistent, SchemaError};

#[derive(Debug)]
pub struct Memory {
	inner: Vec<ComponentSchema>,
}

impl Memory {
	pub fn new() -> Self {
		Self { inner: Vec::new() }
	}
}

#[async_trait::async_trait]
impl Persistent for Memory {
	async fn load(
		&mut self,
		_fields: &Fields,
	) -> Result<Vec<ComponentSchema>, SchemaError> {
		Ok(self.inner.clone())
	}

	async fn save(
		&mut self,
		contents: &[ComponentSchema],
	) -> Result<(), SchemaError> {
		self.inner = contents.to_vec();
		Ok(())
	}
}
