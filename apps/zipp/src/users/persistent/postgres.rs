use std::marker::PhantomData;

use database::{
	id::Id,
	memory::{self, ReadWrite},
	migration_files, Connection, Database,
};
use postgres::table::{table::TableWithConn, Table};

use crate::users::KIND;

use super::{
	Error, InsertRawUser, RawUser, UsersPersistent, UsersPersistentBuilder,
};

const MIGRATIONS: &[(&str, &str)] = migration_files!["users-00-create"];

#[derive(Debug, Clone)]
pub struct PostgresBuilder {
	table: Table,
}

impl PostgresBuilder {
	pub async fn new(db: &mut Database) -> Result<Self, Error> {
		let migrations = db.migrations().unwrap();

		for (name, sql) in MIGRATIONS {
			migrations.add(db.connection_owned(), *name, *sql).await?;
		}

		Ok(Self {
			table: Table::new("users"),
		})
	}
}

impl UsersPersistentBuilder for PostgresBuilder {
	fn with_conn<'a>(
		&'a self,
		conn: Connection<'a>,
	) -> Box<dyn UsersPersistent + 'a> {
		Box::new(Postgres {
			table: self.table.with_conn(conn.into_postgres()),
		})
	}

	fn clone_box(&self) -> Box<dyn UsersPersistentBuilder> {
		Box::new(Self {
			table: self.table.clone(),
		})
	}
}

#[derive(Debug, Clone)]
pub struct Postgres<'a> {
	table: TableWithConn<'a>,
}

#[async_trait::async_trait]
impl UsersPersistent for Postgres<'_> {
	async fn insert(&self, user: InsertRawUser) -> Result<RawUser, Error> {
		todo!()
	}

	async fn by_email(&self, email: &str) -> Result<Option<RawUser>, Error> {
		todo!()
	}

	async fn by_id(&self, id: &Id) -> Result<Option<RawUser>, Error> {
		todo!()
	}
}
