pub mod memory;

use std::{fmt, future::Future, pin::Pin};

use database::{id::Id, Connection, FromConnection};
use serde::Deserialize;

use super::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct RawUser {
	pub id: Id,
	pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InsertRawUser {
	pub email: String,
}

#[async_trait::async_trait]
pub trait UsersPersistent: fmt::Debug + Send + Sync {
	async fn insert(
		&self,
		conn: Connection<'_>,
		user: InsertRawUser,
	) -> Result<RawUser, Error>;

	async fn by_email(
		&self,
		conn: Connection<'_>,
		email: &str,
	) -> Result<Option<RawUser>, Error>;

	async fn by_id(
		&self,
		conn: Connection<'_>,
		id: &Id,
	) -> Result<Option<RawUser>, Error>;

	fn clone_box(&self) -> Box<dyn UsersPersistent>;
}
