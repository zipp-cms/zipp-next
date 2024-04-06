use core::fmt;

use tokio::io;

use super::{
	field_kinds::FieldKinds, json_storage::JsonStorage, Component, ComponentDto,
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
	async fn save(&self, file_name: Option<&str>, contents: &T);
}

pub struct ComponentStore {
	components: Vec<Component>,
	field_kinds: FieldKinds,
	pub persistent: Box<dyn Persistent<Vec<ComponentDto>>>,
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
	}

	pub async fn save(&self, file_name: Option<&str>) {
		let components: Vec<ComponentDto> =
			self.components.iter().map(|c| c.to_dto()).collect();
		self.persistent.save(file_name, &components).await;
	}

	// /// Add a field kind to the store
	// /// this is useful for plugins to register their field kinds
	// pub fn add_field_kind(&mut self, kind: FieldKind) {
	// 	self.field_kinds.push(kind);
	// }
}

impl PartialEq for ComponentStore {
	fn eq(&self, other: &Self) -> bool {
		self.components == other.components
	}
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

	// Component Store use cases:
	// - [x] get a list of all components
	// - [~] add a new component
	// - [ ] update a component
	// - [ ] delete a component
	// - [ ] get a specific component by handle
	// implies:
	// - [x] load from disk
	// - [x] save to disk
	// - [ ] thread safe
	// - [ ] equal comparison for testing

	use crate::components::component_store::ComponentStore;

	use super::*;

	#[tokio::test]
	async fn test_load_from_disk() {
		let components = ComponentStore::new_json_storage(
			"testfiles/components/test_load.json",
		)
		.await;

		let stringified = format!("{:?}", components);

		assert_eq!(
			"ComponentStore { components: [Component { name: \"Button\", handle: \"button\", fields: {\"label\": Field { inner: TextField { max_length: 255 } }} }, Component { name: \"counter\", handle: \"counter\", fields: {\"count\": Field { inner: NumberField { max: 10, min: 10 } }} }] }".to_string()
			,
			stringified);

		println!("{:?}", stringified);
	}

	#[tokio::test]
	async fn test_add_component() {
		let mut components = ComponentStore::new_json_storage(
			"testfiles/components/minimal.json",
		)
		.await;

		let component = Component::new("test".to_string(), "test".to_string());
		components.add(component);

		let stringified = format!("{:?}", components);

		assert!(stringified.contains(
			"Component { name: \"test\", handle: \"test\", fields: {} }"
		));
	}
	// todo: test a component that has fields

	#[tokio::test]
	async fn test_save_to_disk() {
		let mut components = ComponentStore::new_json_storage(
			"testfiles/components/minimal.json",
		)
		.await;

		let component = Component::new("test".to_string(), "test".to_string());
		components.add(component);

		components
			.save(Some("testfiles/components/minimal2.json"))
			.await;

		let mut components2 = ComponentStore::new_json_storage(
			"testfiles/components/minimal2.json",
		)
		.await;

		assert_eq!(components, components2);
	}
}
