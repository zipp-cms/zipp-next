use std::collections::{BTreeMap, HashMap};

use crate::{
	adaptor::{
		types::BasicValue, CreateSchemaData, ReadSchemaData,
		ReadSchemaDataFilter,
	},
	types::{guards::Valid, schema::Schema},
	Error,
};

#[derive(Debug)]
pub struct SchemaRepository {
	schemas: HashMap<String, Schema>,
	data: HashMap<String, Vec<BTreeMap<String, BasicValue>>>,
}

impl SchemaRepository {
	pub fn new() -> Self {
		Self {
			schemas: HashMap::new(),
			data: HashMap::new(),
		}
	}

	pub fn create_schema(
		&mut self,
		schema: Valid<Schema>,
	) -> Result<(), Error> {
		let schema = schema.into_inner();

		let name = schema.name.clone();
		self.schemas.insert(schema.name.clone(), schema);
		self.data.insert(name, Vec::new());

		Ok(())
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
	pub fn create_schema_data(
		&mut self,
		data: Vec<CreateSchemaData>,
	) -> Result<(), Error> {
		for data in data {
			// todo make sure the same row does not exists and unique ness constraints are met

			let entries = self.data.get_mut(&data.schema).unwrap();

			entries.push(data.data);
		}

		Ok(())
	}

	pub fn read_schema_data(
		&self,
		queries: Vec<ReadSchemaData>,
	) -> Result<Vec<Vec<BTreeMap<String, BasicValue>>>, Error> {
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
	entry: &BTreeMap<String, BasicValue>,
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
	use crate::{
		adaptor::types::BasicValue,
		types::schema::{Field, FieldKind},
	};

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

		let data = vec![CreateSchemaData::builder("test")
			.data("id", BasicValue::String("123".into()))
			.data("name", BasicValue::String("1".into()))
			.build()];

		repo.create_schema_data(data).unwrap();
	}
}
