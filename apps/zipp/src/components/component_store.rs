use core::fmt;

use tokio::io;

use super::{
	field_kinds::{FieldKind, FieldKinds},
	json_storage::{self, JsonStorage},
	Component, ComponentDto, Field,
};

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
	#[error("IO error: {error} for file: {file_name}")]
	IO { error: io::Error, file_name: String },
	#[error("JSON error: {error} for file: {file_name}")]
	JSON {
		error: serde_json::Error,
		file_name: String,
	},
}

#[async_trait::async_trait]
pub trait Persistent<T> {
	async fn load(&self) -> Result<T, LoadError>;
	async fn save(&self, contents: &T);
}

pub struct ComponentStore {
	components: Vec<Component>,
	field_kinds: FieldKinds,
	persistent: Box<dyn Persistent<Vec<ComponentDto>>>,
}

impl ComponentStore {
	pub async fn new_json_storage(file_name: &str) -> Self {
		let persistent = Box::new(JsonStorage::new(file_name));
		let components = persistent.load().await.unwrap();
		let field_kinds = FieldKinds::default();

		// todo: this should be done in the json storage
		// turn ComponentDto into Component
		let components: Vec<Component> = components
			.into_iter()
			.map(|c| Component::from_dto(c, &field_kinds))
			.collect();

		Self {
			components,
			persistent,
			field_kinds,
		}
	}

	pub fn add(&mut self, component: Component) {
		self.components.push(component);
		// todo: maybe save to persistent storage
	}

	// /// Add a field kind to the store
	// /// this is useful for plugins to register their field kinds
	// pub fn add_field_kind(&mut self, kind: FieldKind) {
	// 	self.field_kinds.push(kind);
	// }
}

impl fmt::Debug for ComponentStore {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("ComponentStore")
			.field("components", &self.components)
			.finish()
	}
}

#[cfg(test)]
mod tests {

	use crate::components::component_store::ComponentStore;

	use super::*;

	#[tokio::test]
	async fn test_save() {
		let components = ComponentStore::new_json_storage(
			"testfiles/components/minimal.json",
		)
		.await;

		println!("{:?}", components);
	}
}
