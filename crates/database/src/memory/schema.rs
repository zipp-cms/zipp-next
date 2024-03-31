use std::collections::HashMap;

use crate::{
	types::{
		query::Query,
		schema::{Data, Schema},
	},
	Error,
};

#[derive(Debug)]
pub struct SchemaRepository {
	schemas: HashMap<String, Schema>,
}

impl SchemaRepository {
	pub fn new() -> Self {
		Self {
			schemas: HashMap::new(),
		}
	}

	pub fn set_schema(&mut self, schema: Schema) -> Result<(), Error> {
		self.schemas.insert(schema.name.clone(), schema);

		Ok(())
	}

	pub fn get_schema(&self, name: &str) -> Result<Option<Schema>, Error> {
		let schema = self.schemas.get(name).cloned();

		Ok(schema)
	}

	pub fn delete_schema(&mut self, name: &str) -> Result<(), Error> {
		self.schemas.remove(name);

		Ok(())
	}

	pub fn query_schema_data(&self, query: Query) -> Result<Data, Error> {
		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_data_selector() {}
}
