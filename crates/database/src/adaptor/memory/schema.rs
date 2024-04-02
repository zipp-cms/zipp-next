use std::collections::{BTreeMap, HashMap};

use serde_json::Value;

use crate::{
	adaptor::{ReadSchemaData, ReadSchemaDataFilter},
	types::{
		guards::Valid,
		id::Kind,
		schema::{CreateSchema, Schema, SchemaEntries, SchemaFieldValue},
	},
	Error,
};

#[derive(Debug)]
pub struct SchemaRepository {
	kind_counter: u16,
	schemas: HashMap<String, Schema>,
	data: HashMap<String, Vec<BTreeMap<String, Value>>>,
}

impl SchemaRepository {
	pub fn new() -> Self {
		Self {
			kind_counter: 0,
			schemas: HashMap::new(),
			data: HashMap::new(),
		}
	}

	pub fn create_schema(
		&mut self,
		schema: Valid<CreateSchema>,
	) -> Result<Schema, Error> {
		let schema = schema.into_inner();
		let schema = Schema {
			name: schema.name,
			kind: Kind::new(false, self.kind_counter),
			fields: schema.fields,
		};
		self.kind_counter += 1;

		let name = schema.name.clone();
		self.schemas.insert(schema.name.clone(), schema.clone());
		self.data.insert(name, Vec::new());

		Ok(schema)
	}

	pub fn get_schema(&self, name: &str) -> Result<Option<Schema>, Error> {
		let schema = self.schemas.get(name).cloned();

		Ok(schema)
	}

	// pub fn delete_schema(&mut self, name: &str) -> Result<(), Error> {
	// 	self.schemas.remove(name);
	// 	self.data.remove(name);

	// 	Ok(())
	// }

	/// The data we received needs to be valid
	pub fn create_schema_entries(
		&mut self,
		schema: String,
		entries: SchemaEntries,
	) -> Result<(), Error> {
		// todo make sure the same row does not exists and unique ness constraints are met

		let stored_entries = self.data.get_mut(&schema).unwrap();
		let mut nested = vec![];

		for entry in entries.0 {
			let fields = entry
				.0
				.into_iter()
				.filter_map(|(name, value)| match value {
					SchemaFieldValue::Value(v) => Some((name, v)),
					SchemaFieldValue::Entries(entries) => {
						nested.push((name, entries));
						None
					}
				})
				.collect();

			stored_entries.push(fields);
		}

		for (name, entries) in nested {
			self.create_schema_entries(name, entries)?;
		}

		Ok(())
	}

	pub fn read_schema_data(
		&self,
		queries: Vec<ReadSchemaData>,
	) -> Result<Vec<Vec<BTreeMap<String, Value>>>, Error> {
		let mut result = Vec::with_capacity(queries.len());

		for query in queries {
			let entries = self.data.get(&query.schema).unwrap();

			let entries = entries
				.iter()
				// now check if we should include this entry
				.filter(|e| {
					if let Some(filter) = &query.filter {
						return filter_matches_entry(e, filter);
					}

					true
				})
				// only return the fields requested
				.map(|e| {
					let mut fields = BTreeMap::new();

					for field in &query.fields {
						fields.insert(
							field.clone(),
							e.get(field).unwrap().clone(),
						);
					}

					fields
				})
				.collect();

			result.push(entries);
		}

		Ok(result)
	}

	// pub fn query_schema_data(&self, query: Query) -> Result<Data, Error> {
	// 	todo!()
	// }
}

fn filter_matches_entry(
	entry: &BTreeMap<String, Value>,
	filter: &ReadSchemaDataFilter,
) -> bool {
	use ReadSchemaDataFilter::*;

	match filter {
		Equal { field, value } => entry.get(field).unwrap() == value,
		And(filters) => filters.iter().all(|f| filter_matches_entry(entry, f)),
		Or(filters) => filters.iter().any(|f| filter_matches_entry(entry, f)),
	}
}

#[cfg(test)]
mod tests {
	use crate::types::schema::{Field, FieldKind};

	use super::*;

	#[test]
	fn test_new_schema() {
		let mut repo = SchemaRepository::new();

		let schema = Schema::builder("test")
			.field(Field::builder("id", FieldKind::Id).primary())
			.build();

		repo.create_schema(Valid::assume_valid(schema.clone()))
			.unwrap();
	}

	#[test]
	fn test_create_data() {
		let mut repo = SchemaRepository::new();

		let schema = Schema::builder("test")
			.field(Field::builder("id", FieldKind::Id).primary())
			.field(Field::builder("name", FieldKind::Text))
			.build();

		repo.create_schema(Valid::assume_valid(schema.clone()))
			.unwrap();

		repo.create_schema_entries(
			"test".into(),
			SchemaEntries::builder()
				.entry("id", "123".to_string())
				.entry("name", "1".to_string())
				.build(),
		)
		.unwrap();
	}
}
