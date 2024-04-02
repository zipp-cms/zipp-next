use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::field_kinds::FieldKinds;

pub mod component_store;
pub mod field_kinds;
pub mod json_storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
	pub kind: String,
	#[serde(default)]
	pub settings: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
	pub name: String,
	pub handle: String,
	#[serde(default)]
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

impl Field {
	pub fn new(kind: String) -> Self {
		Self {
			kind,
			settings: BTreeMap::new(),
		}
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
