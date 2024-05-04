//! Components
//!
//! This module contains everything about components. Which components
//! exists, what fields they haven or what data or instances exist.
//!
//! After creating components the state of schemas and instaces might
//! be out of sync.
//! After calling migrate it will be in sync for the remainder of the
//! application.

mod error;
mod instances;
mod schemas;

use std::sync::{
	atomic::{AtomicBool, Ordering},
	Arc,
};

pub use error::Error;

use database::{Connection, Database, DatabaseKind};
use tokio::sync::RwLock;

use crate::fields::Fields;

use self::schemas::{ComponentSchema, ComponentSchemas};

const DEFAULT_SCHEMA_FILE: &str = "./components.json";

// todo i think we need to store the schemas in the database as well
// so we know if the data is out of sync and are able to execute commands
// before the migration starts

#[derive(Debug, Clone)]
pub struct Components {
	out_of_sync: Arc<AtomicBool>,
	schemas: Arc<RwLock<ComponentSchemas>>,
	instances: instances::Components,
}

impl Components {
	pub async fn new(
		conn: &mut Database,
		fields: &Fields,
	) -> Result<Self, Error> {
		let schemas = match conn.kind() {
			DatabaseKind::Memory => {
				ComponentSchemas::new_memory(fields.clone())
			}
			DatabaseKind::Postgres => {
				let file_name = DEFAULT_SCHEMA_FILE;
				ComponentSchemas::load_file(fields.clone(), file_name).await?
			}
		};

		Ok(Self {
			out_of_sync: Arc::new(AtomicBool::new(true)),
			schemas: Arc::new(RwLock::new(schemas)),
			instances: instances::Components::new(conn)?,
		})
	}

	pub async fn with_conn<'a>(
		&'a self,
		conn: Connection<'a>,
	) -> ComponentsWithConn<'a> {
		ComponentsWithConn {
			out_of_sync: &self.out_of_sync,
			schemas: &self.schemas,
			instances: self.instances.with_conn(conn),
		}
	}
}

pub struct ComponentsWithConn<'a> {
	out_of_sync: &'a AtomicBool,
	schemas: &'a RwLock<ComponentSchemas>,
	instances: instances::ComponentsWithConn<'a>,
}

impl<'a> ComponentsWithConn<'a> {
	pub async fn migrate(&self) -> Result<(), Error> {
		let schemas = self.schemas.read().await;

		for component in schemas.get_all() {
			self.instances.set_schema(component).await?;
		}

		// self.instances.migrate(&mut *schemas).await?;

		self.out_of_sync.store(false, Ordering::Relaxed);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::Components;

	fn is_send<T: Send>() {}

	#[test]
	fn test_send_sync() {
		is_send::<Components>();
		is_send::<&Components>();
	}
}
