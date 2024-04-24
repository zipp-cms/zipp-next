pub mod memory;
pub mod postgres;

use std::fmt;

use database::{id::Id, Connection};
use serde::Deserialize;

use super::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct RawUser {
	pub id: Id,
	pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InsertRawUser<'a> {
	pub email: &'a str,
}

#[async_trait::async_trait]
pub trait UsersPersistentBuilder: fmt::Debug + Send + Sync {
	fn with_conn<'a>(
		&'a self,
		conn: Connection<'a>,
	) -> Box<dyn UsersPersistent + 'a>;

	fn clone_box(&self) -> Box<dyn UsersPersistentBuilder>;
}

#[async_trait::async_trait]
pub trait UsersPersistent: fmt::Debug + Send + Sync {
	async fn insert(&self, user: InsertRawUser<'_>) -> Result<RawUser, Error>;

	async fn by_email(&self, email: &str) -> Result<Option<RawUser>, Error>;

	async fn by_id(&self, id: &Id) -> Result<Option<RawUser>, Error>;
}
