//! Schemas
//!
//! Schemas manage the structure of all components you can query
//! schemas to know which fields exist on that component
//!
//! Fields are managed by the fields
//!

mod error;
mod persistent;
mod schema;

pub use error::SchemaError;

use persistent::Persistent;
pub use schema::ComponentSchema;

use indexmap::IndexMap;

use crate::fields::Fields;

// component schemas
#[derive(Debug)]
pub struct ComponentSchemas {
	inner: IndexMap<String, ComponentSchema>,
	// a reference to the global fields
	fields: Fields,
	persistent: Box<dyn Persistent>,
}

impl ComponentSchemas {
	/// Creates a new schema store
	pub fn new(fields: Fields, persistent: impl Persistent) -> Self {
		Self {
			inner: IndexMap::new(),
			fields,
			persistent: Box::new(persistent),
		}
	}

	/// Creates a new schema store with a memory persistent storage
	pub fn new_memory(fields: Fields) -> Self {
		Self::new(fields, persistent::new_memory())
	}

	/// Creates a new schema store with a file persistent storage
	/// and loads the schemas from the file
	pub async fn load_file(
		fields: Fields,
		file_name: &str,
	) -> Result<Self, SchemaError> {
		let mut me = Self::new(fields, persistent::new_file(file_name));

		me.load().await?;

		Ok(me)
	}

	/// Load the schemas from the persistent storage
	///
	/// Replaces the current schemas
	pub async fn load(&mut self) -> Result<(), SchemaError> {
		let schemas = self.persistent.load(&self.fields).await?;
		self.inner =
			schemas.into_iter().map(|s| (s.handle.clone(), s)).collect();
		Ok(())
	}

	/// Returns all the schemas
	pub fn get_all(&self) -> impl ExactSizeIterator<Item = &ComponentSchema> {
		self.inner.values()
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	/// Returns a schema by its handle
	pub fn get_by_handle(&self, handle: &str) -> Option<&ComponentSchema> {
		self.inner.get(handle)
	}

	/// Removes a schema by its handle
	pub fn remove_by_handle(&mut self, handle: &str) {
		self.inner.swap_remove(handle);
	}

	/// Inserts a schema
	pub fn insert(&mut self, component: ComponentSchema) {
		self.inner.insert(component.handle.clone(), component);
	}

	pub async fn save(&mut self) -> Result<(), SchemaError> {
		let schemas = self.get_all().cloned().collect::<Vec<_>>();
		self.persistent.save(&schemas).await
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

	use crate::fields::{
		defaults::{NumberFieldSchema, TextFieldSchema},
		BoxedFieldSchema,
	};

	use super::*;

	async fn load_with_defaults(file: &str) -> ComponentSchemas {
		let fields = Fields::default();
		ComponentSchemas::load_file(fields, file).await.unwrap()
	}

	#[tokio::test]
	async fn test_load_from_disk() {
		let components =
			load_with_defaults("testfiles/components/test_load.json").await;

		let mut all = components.get_all();
		let comp = all.next().unwrap();
		assert_eq!(comp.name, "Button");
		assert_eq!(comp.handle, "button");
		assert_eq!(comp.fields.len(), 1);
		let (field_name, field) = comp.fields.iter().next().unwrap();
		assert_eq!(field_name, "label");
		eprintln!("{:?}", field);
		let _field: &TextFieldSchema = field.downcast_ref().unwrap();

		let comp = all.next().unwrap();
		assert_eq!(comp.name, "counter");
		assert_eq!(comp.handle, "counter");
		assert_eq!(comp.fields.len(), 1);
		let (field_name, field) = comp.fields.iter().next().unwrap();
		assert_eq!(field_name, "count");
		let field: &NumberFieldSchema = field.downcast_ref().unwrap();
		assert_eq!(field.min, 10);
		assert_eq!(field.max, 20);
	}

	#[tokio::test]
	async fn test_add_component() {
		let mut components =
			load_with_defaults("testfiles/components/minimal.json").await;

		let component = ComponentSchema::new("Test", "test");
		components.insert(component);
		assert_eq!(3, components.len());
	}
	// todo: test a component that has fields

	// #[tokio::test]
	// async fn test_save_to_disk() {
	// 	let mut components =
	// 		load_with_defaults("testfiles/components/minimal.json").await;

	// 	let component = ComponentSchema::new("test", "test");
	// 	components.insert(component);

	// todo use crate tempfile

	// 	components
	// 		.save(Some("testfiles/components/minimal2.json"))
	// 		.await;

	// 	let mut components2 =
	// 		load_with_defaults("testfiles/components/minimal2.json").await;

	// 	assert_eq!(components, components2);
	// }

	#[tokio::test]
	async fn test_get_all() {
		let components =
			load_with_defaults("testfiles/components/minimal.json").await;

		let all = components.get_all();

		assert_eq!(2, all.len());
	}

	#[tokio::test]
	async fn test_get_by_handle() {
		let components =
			load_with_defaults("testfiles/components/minimal.json").await;

		let component = components.get_by_handle("button");

		assert!(component.is_some());
	}

	#[tokio::test]
	async fn test_remove_by_handle() {
		let mut components =
			load_with_defaults("testfiles/components/minimal.json").await;

		components.remove_by_handle("button");

		let all = components.get_all();

		assert_eq!(1, all.len());
	}

	#[tokio::test]
	async fn test_update() {
		let mut components =
			load_with_defaults("testfiles/components/minimal.json").await;

		let mut component = components.get_by_handle("button").unwrap().clone();
		component.name = "new name".to_string();

		component.fields.insert(
			"new field".to_string(),
			BoxedFieldSchema::new(TextFieldSchema::default()),
		);

		components.insert(component);

		let updated = components.get_by_handle("button").unwrap();

		assert_eq!("new name", updated.name);
		assert!(updated.fields.get("new field").is_some());
	}
}
