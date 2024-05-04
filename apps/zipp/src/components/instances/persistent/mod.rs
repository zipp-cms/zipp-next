pub mod memory;

use std::fmt;

use database::Connection;

use crate::{components::Error, fields::PersistentKind};

#[derive(Debug, Clone)]
pub struct SetFieldColumn<'a> {
	pub name: &'a str,
	pub kind: FieldKind,
}

#[derive(Debug, Clone)]
pub enum FieldKind {
	Id,
	// ComponentId,
	// Component { name: &'a str },
	Boolean,
	Int,
	Float,
	Text,
	Json,
	DateTime,
}

impl FieldKind {
	pub fn from_kind(kind: PersistentKind) -> Option<Self> {
		Some(match kind {
			PersistentKind::Id => FieldKind::Id,
			PersistentKind::ComponentRelation => return None,
			PersistentKind::Boolean => FieldKind::Boolean,
			PersistentKind::Int => FieldKind::Int,
			PersistentKind::Float => FieldKind::Float,
			PersistentKind::Text => FieldKind::Text,
			PersistentKind::Json => FieldKind::Json,
			PersistentKind::DateTime => FieldKind::DateTime,
		})
	}
}

#[async_trait::async_trait]
pub trait ComponentsPersistentBuilder: fmt::Debug + Send + Sync {
	fn with_conn<'a>(
		&'a self,
		conn: Connection<'a>,
	) -> Box<dyn ComponentsPersistent + 'a>;

	fn clone_box(&self) -> Box<dyn ComponentsPersistentBuilder>;
}

#[async_trait::async_trait]
pub trait ComponentsPersistent: fmt::Debug + Send + Sync {
	async fn update_schema(
		&self,
		handle: &str,
		columns: Vec<SetFieldColumn<'_>>,
	) -> Result<(), Error>;
}
