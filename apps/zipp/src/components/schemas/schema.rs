use std::any::Any;
use std::collections::BTreeMap;

use crate::fields::Field;

// pub mod component_store;
// pub mod default_field_kinds;
// pub mod field_kinds;
// pub mod json_storage;

#[derive(Debug)]
pub struct FieldSchema {
	pub inner: Box<dyn Field>,
}

impl FieldSchema {
	pub fn new(inner: Box<dyn Field>) -> Self {
		Self { inner }
	}

	pub fn downcast_ref<T: Field>(&self) -> Option<&T> {
		self.inner.as_any().downcast_ref()
	}
}

impl PartialEq for FieldSchema {
	fn eq(&self, other: &Self) -> bool {
		todo!("Field::eq")
	}
}

impl Clone for FieldSchema {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone_box(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct ComponentSchema {
	pub name: String,
	pub handle: String,
	pub fields: BTreeMap<String, FieldSchema>,
}

impl ComponentSchema {
	pub fn new(name: impl Into<String>, handle: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			handle: handle.into(),
			fields: BTreeMap::new(),
		}
	}

	// pub fn from_dto(dto: SchemaDto, field_kinds: &FieldKinds) -> Self {
	// 	Self {
	// 		name: dto.name,
	// 		handle: dto.handle,
	// 		fields: dto
	// 			.fields
	// 			.into_iter()
	// 			.map(|(name, field_dto)| {
	// 				(name, field_kinds.field_from_dto(field_dto).unwrap())
	// 			})
	// 			.collect(),
	// 	}
	//
}

impl PartialEq for ComponentSchema {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.handle == other.handle
			&& self.fields == other.fields
	}
}
