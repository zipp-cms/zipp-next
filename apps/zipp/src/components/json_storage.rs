use std::{collections::BTreeMap, fs::File, vec};

use tracing::error;

use super::{
	component_store::{LoadError, Persistent},
	Component, ComponentDto,
};

pub struct JsonStorage {
	file_name: String,
}

impl JsonStorage {
	pub fn new(file_name: impl Into<String>) -> Self {
		Self {
			file_name: file_name.into(),
		}
	}
}

#[async_trait::async_trait]
impl Persistent<Vec<ComponentDto>> for JsonStorage {
	async fn load(&self) -> Result<Vec<ComponentDto>, LoadError> {
		let file_string = tokio::fs::read_to_string(&self.file_name)
			.await
			.map_err(|err| LoadError::IO {
				error: err,
				file_name: self.file_name.clone(),
			})?;

		serde_json::from_str(&file_string).map_err(|err| LoadError::JSON {
			error: err,
			file_name: self.file_name.clone(),
		})
	}

	async fn save(&self, components: &Vec<ComponentDto>) {
		for component in components {
			println!("Saving component: {:?}", component.name)
		}
	}
}
