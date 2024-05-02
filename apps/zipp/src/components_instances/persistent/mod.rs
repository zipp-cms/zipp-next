use std::fmt;

use database::{id::Id, Connection};
use serde::Deserialize;

use super::Error;

#[derive(Debug, Clone)]
pub struct ComponentSchema<'a> {
	pub name: &'a str,
	pub fields: Vec<SchemaField<'a>>,
}

#[derive(Debug, Clone)]
pub struct SchemaField<'a> {
	pub name: &'a str,
	pub kind: FieldKind<'a>,
	pub related: Option<&'a str>,
	pub primary: bool,
	pub index: bool,
}

#[derive(Debug, Clone)]
pub enum FieldKind<'a> {
	Id,
	ComponentId,
	Component { name: &'a str },
	Boolean,
	Int,
	Float,
	Text,
	Json,
	DateTime,
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
		user: ComponentSchema<'_>,
	) -> Result<(), Error>;
}
