use core::fmt;
use std::{
	collections::HashMap,
	sync::{Arc, RwLock},
};

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
	components: Arc<RwLock<HashMap<String, Component>>>,
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
		let components = Arc::new(RwLock::new(
			components
				.into_iter()
				.map(|c| Component::from_dto(c, &field_kinds))
				.map(|c| (c.handle.clone(), c))
				.collect(),
		));

		Self {
			components,
			persistent,
			field_kinds,
		}
	}

	pub fn get_all(&self) -> Vec<Component> {
		let components = self.components.read().unwrap();
		components.values().cloned().collect()
	}

	pub fn get_by_handle(&self, handle: &str) -> Option<Component> {
		let components = self.components.read().unwrap();
		components.get(handle).cloned()
	}

	pub fn remove_by_handle(&self, handle: &str) {
		self.components.write().unwrap().remove(handle);
	}

	pub fn insert(&self, component: Component) {
		let mut components = self.components.write().unwrap();
		components.insert(component.handle.clone(), component);
	}

	pub async fn save(&self, file_name: Option<&str>) {
		let components: Vec<ComponentDto> =
			self.get_all().iter().map(|c| c.to_dto()).collect();
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
		todo!("ComponentStore::eq")
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
	// - [x] add a new component (insert)
	// - [x] update a component (insert)
	// - [x] delete a component
	// - [x] get a specific component by handle
	// implies:
	// - [x] load from disk
	// - [x] save to disk
	// - [x] thread safe
	// - [ ] equal comparison for testing

	use crate::components::{
		component_store::ComponentStore, default_field_kinds::TextField, Field,
	};

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
		components.insert(component);

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
		components.insert(component);

		components
			.save(Some("testfiles/components/minimal2.json"))
			.await;

		let mut components2 = ComponentStore::new_json_storage(
			"testfiles/components/minimal2.json",
		)
		.await;

		assert_eq!(components, components2);
	}

	#[tokio::test]
	async fn test_get_all() {
		let components = ComponentStore::new_json_storage(
			"testfiles/components/minimal.json",
		)
		.await;

		let all = components.get_all();

		assert_eq!(2, all.len());
	}

	#[tokio::test]
	async fn test_get_by_handle() {
		let components = ComponentStore::new_json_storage(
			"testfiles/components/minimal.json",
		)
		.await;

		let component = components.get_by_handle("button");

		assert!(component.is_some());
	}

	#[tokio::test]
	async fn test_remove_by_handle() {
		let mut components = ComponentStore::new_json_storage(
			"testfiles/components/minimal.json",
		)
		.await;

		components.remove_by_handle("button");

		let all = components.get_all();

		assert_eq!(1, all.len());
	}

	#[tokio::test]
	async fn test_update() {
		let components = ComponentStore::new_json_storage(
			"testfiles/components/minimal.json",
		)
		.await;

		let mut component = components.get_by_handle("button").unwrap().clone();
		component.name = "new name".to_string();

		component.fields.insert(
			"new field".to_string(),
			Field::new(Box::new(TextField::default())),
		);

		components.insert(component);

		let updated = components.get_by_handle("button").unwrap();

		assert_eq!("new name", updated.name);
		assert!(updated.fields.get("new field").is_some());
	}
}
