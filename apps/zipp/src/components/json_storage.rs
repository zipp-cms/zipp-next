use std::vec;

use super::{component_store::Persistent, Component};

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
impl Persistent<Vec<Component>> for JsonStorage {
	async fn load(&self) -> Option<Vec<Component>> {
		println!("Loading components from a file");
		None
	}

	async fn save(&self, components: &Vec<Component>) {
		println!("Saving components to a file");
	}
}

#[cfg(test)]
mod tests {

	use crate::components::component_store::ComponentStore;

	use super::*;

	#[tokio::test]
	async fn test_save() {
		let components =
			ComponentStore::new_json_storage("components.json").await;
		println!("{:?}", components);
	}
}
