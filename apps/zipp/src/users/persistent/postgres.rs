use std::marker::PhantomData;

use database::{
	id::Id,
	memory::{self, ReadWrite, Table},
	migration_files,
	migrations::MigrationError,
	Connection, Database,
};

use crate::users::KIND;

use super::{Error, InsertRawUser, RawUser, UsersPersistent};

const MIGRATIONS: &[(&str, &str)] = migration_files!["users-00-create"];

#[derive(Debug, Clone)]
pub struct Postgres {}

impl Postgres {
	pub async fn new(db: &Database) -> Result<Self, MigrationError> {
		let conn = db.connection();
		let migrations = db.migrations();

		for (name, sql) in MIGRATIONS {
			migrations.add(&conn, *name, *sql).await?;
		}

		Ok(Self {})
	}
}

#[async_trait::async_trait]
impl UsersPersistent for Postgres {
	async fn insert(
		&self,
		conn: Connection<'_>,
		user: InsertRawUser,
	) -> Result<RawUser, Error> {
		todo!()
	}

	async fn by_email(
		&self,
		conn: Connection<'_>,
		email: &str,
	) -> Result<Option<RawUser>, Error> {
		todo!()
	}

	async fn by_id(
		&self,
		conn: Connection<'_>,
		id: &Id,
	) -> Result<Option<RawUser>, Error> {
		todo!()
	}

	fn clone_box(&self) -> Box<dyn UsersPersistent> {
		Box::new(Self {})
	}
}
