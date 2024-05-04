mod persistent;
use database::{Connection, Database, DatabaseKind};

use self::persistent::{
	memory::Memory, ComponentsPersistent, ComponentsPersistentBuilder,
	FieldKind, SetFieldColumn,
};

use super::{schemas::ComponentSchema, Error};

#[derive(Debug)]
pub struct Components {
	persistent: Box<dyn ComponentsPersistentBuilder>,
}

impl Components {
	pub fn new(conn: &mut Database) -> Result<Self, Error> {
		let persistent: Box<dyn ComponentsPersistentBuilder> = match conn.kind()
		{
			DatabaseKind::Memory => Box::new(Memory::new()),
			DatabaseKind::Postgres => {
				todo!()
				// Box::new(PostgresBuilder::new(conn).await?)
			}
		};

		Ok(Self { persistent })
	}

	pub fn with_conn<'a>(
		&'a self,
		conn: Connection<'a>,
	) -> ComponentsWithConn<'a> {
		ComponentsWithConn {
			inner: self.persistent.with_conn(conn),
		}
	}
}

impl Clone for Components {
	fn clone(&self) -> Self {
		Self {
			persistent: self.persistent.clone_box(),
		}
	}
}

pub struct ComponentsWithConn<'a> {
	inner: Box<dyn ComponentsPersistent + 'a>,
}

impl ComponentsWithConn<'_> {
	pub async fn set_schema(
		&self,
		schema: &ComponentSchema,
	) -> Result<(), Error> {
		let mut columns = vec![];

		for (name, field) in &schema.fields {
			let Some(kind) = FieldKind::from_kind(field.persistent_kind())
			else {
				continue;
			};

			columns.push(SetFieldColumn { name, kind });
		}

		self.inner.update_schema(&schema.handle, columns).await
	}
}
