use std::collections::BTreeMap;

use self::field_kinds::{FieldKind, FieldKinds, FieldTrait};

pub mod component_store;
pub mod default_field_kinds;
pub mod field_kinds;
pub mod json_storage;

#[derive(Debug)]
pub struct Field {
	pub inner: Box<dyn FieldTrait>,
}

impl Field {
	pub fn new(inner: Box<dyn FieldTrait>) -> Self {
		Self { inner }
	}
}

impl PartialEq for Field {
	fn eq(&self, other: &Self) -> bool {
		todo!("Field::eq")
	}
}

impl Clone for Field {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone_box(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Component {
	pub name: String,
	pub handle: String,
	pub fields: BTreeMap<String, Field>,
}

impl Component {
	pub fn new(name: String, handle: String) -> Self {
		Self {
			name,
			handle,
			fields: BTreeMap::new(),
		}
	}

	pub fn from_dto(dto: ComponentDto, field_kinds: &FieldKinds) -> Self {
		Self {
			name: dto.name,
			handle: dto.handle,
			fields: dto
				.fields
				.into_iter()
				.map(|(name, field_dto)| {
					(name, field_kinds.field_from_dto(field_dto).unwrap())
				})
				.collect(),
		}
	}

	pub fn to_dto(&self) -> ComponentDto {
		ComponentDto {
			name: self.name.clone(),
			handle: self.handle.clone(),
			fields: self
				.fields
				.iter()
				.map(|(name, field)| {
					(
						name.clone(),
						FieldDto {
							kind: field.inner.name(),
							settings: field.inner.settings(),
						},
					)
				})
				.collect(),
		}
	}
}

impl PartialEq for Component {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.handle == other.handle
			&& self.fields == other.fields
	}
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FieldDto {
	pub kind: String,
	#[serde(default)]
	pub settings: BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ComponentDto {
	pub name: String,
	pub handle: String,
	pub fields: BTreeMap<String, FieldDto>,
}
