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

#[derive(Debug)]
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
}

// impl Field {
// 	pub fn new(kind: String) -> Self {
// 		Self {
// 			kind,
// 			settings: Vec::new(),
// 		}
// 	}
// }

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
