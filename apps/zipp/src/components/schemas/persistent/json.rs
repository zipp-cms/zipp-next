use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
	components::schemas::schema::ComponentSchema,
	fields::{Fields, ParseFieldError},
};

use super::{Persistent, SchemaError};

#[derive(Debug)]
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
impl Persistent for JsonStorage {
	async fn load(
		&mut self,
		fields: &Fields,
	) -> Result<Vec<ComponentSchema>, SchemaError> {
		let file_string = tokio::fs::read_to_string(&self.file_name)
			.await
			.map_err(|err| SchemaError::io(err, &self.file_name))?;

		// todo convert from dto

		let dtos: Vec<SchemaComponentDto> = serde_json::from_str(&file_string)
			.map_err(|err| SchemaError::json(err, &self.file_name))?;

		dtos.into_iter()
			.map(|dto| {
				component_dto_to_schema(dto, fields)
					.map_err(|e| SchemaError::parse(e, &self.file_name))
			})
			.collect()
	}

	async fn save(
		&mut self,
		components: &[ComponentSchema],
	) -> Result<(), SchemaError> {
		// todo convert to dto
		let dtos = components
			.iter()
			.cloned()
			.map(SchemaComponentDto::from)
			.collect::<Vec<_>>();

		// Convert the components to a JSON string
		let json = serde_json::to_string(&dtos)
			.map_err(|err| SchemaError::json(err, &self.file_name))?;

		// Write the JSON string to a file
		tokio::fs::write(&self.file_name, json)
			.await
			.map_err(|err| SchemaError::io(err, &self.file_name))
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaComponentDto {
	pub name: String,
	pub handle: String,
	pub fields: BTreeMap<String, FieldDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldDto {
	pub kind: String,
	#[serde(default)]
	pub settings: BTreeMap<String, serde_json::Value>,
}

impl From<ComponentSchema> for SchemaComponentDto {
	fn from(schema: ComponentSchema) -> Self {
		SchemaComponentDto {
			name: schema.name.clone(),
			handle: schema.handle.clone(),
			fields: schema
				.fields
				.iter()
				.map(|(name, field)| {
					(
						name.clone(),
						FieldDto {
							kind: field.kind(),
							settings: field.settings(),
						},
					)
				})
				.collect(),
		}
	}
}

fn component_dto_to_schema(
	dto: SchemaComponentDto,
	fields_kinds: &Fields,
) -> Result<ComponentSchema, ParseFieldError> {
	let mut fields = BTreeMap::new();

	for (name, field_dto) in dto.fields {
		let field =
			fields_kinds.parse_field(&field_dto.kind, field_dto.settings)?;

		fields.insert(name, field.into());
	}

	Ok(ComponentSchema {
		name: dto.name,
		handle: dto.handle,
		fields,
	})
}
