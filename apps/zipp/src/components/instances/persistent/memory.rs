use database::{
	id::Id,
	memory::{ReadWrite, Table},
	Connection,
};

use crate::components::Error;

use super::{
	ComponentsPersistent, ComponentsPersistentBuilder, SetFieldColumn,
};

#[derive(Debug, Clone)]
pub struct Memory {
	// inner: ReadWrite<Table<Id, RawUser>>,
}

impl Memory {
	pub fn new() -> Self {
		Self {
			// inner: ReadWrite::new(Table::new()),
		}
	}
}

impl ComponentsPersistentBuilder for Memory {
	fn with_conn(&self, conn: Connection<'_>) -> Box<dyn ComponentsPersistent> {
		let _conn = conn.into_memory();

		Box::new(Self {
			// inner: self.inner.clone(),
		})
	}

	fn clone_box(&self) -> Box<dyn ComponentsPersistentBuilder> {
		Box::new(Self {
			// inner: self.inner.clone(),
		})
	}
}

#[async_trait::async_trait]
impl ComponentsPersistent for Memory {
	async fn update_schema(
		&self,
		handle: &str,
		columns: Vec<SetFieldColumn<'_>>,
	) -> Result<(), Error> {
		todo!()
	}
}
