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

	async fn save(
		&self,
		file_name: Option<&str>,
		components: &Vec<ComponentDto>,
	) {
		// Convert the components to a JSON string
		let json = match serde_json::to_string(components) {
			Ok(json) => json,
			Err(err) => {
				error!("Failed to serialize components: {:?}", err);
				return;
			}
		};

		let file_name = file_name.unwrap_or(&self.file_name);

		// Write the JSON string to a file
		if let Err(err) = tokio::fs::write(file_name, json).await {
			error!("Failed to write to file {}: {:?}", file_name, err);
		}
	}
}
