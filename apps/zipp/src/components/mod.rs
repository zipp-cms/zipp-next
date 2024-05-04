//! Components
//!
//! This module contains everything about components. Which components
//! exists, what fields they haven or what data or instances exist.

mod error;
mod instances;
mod schemas;

use std::sync::Arc;

pub use error::Error;

use database::{
	id::{Id, Kind},
	Database,
};
use tokio::sync::RwLock;

use self::schemas::ComponentSchemas;

#[derive(Debug, Clone)]
pub struct Components {
	schemas: Arc<RwLock<ComponentSchemas>>,
}

impl Components {
	pub async fn new(conn: &mut Database) -> Result<Self, Error> {
		todo!()
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
