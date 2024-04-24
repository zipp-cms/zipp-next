use database::{id::Id, migration_files, Connection, Database};
use fire_postgres::{
	filter,
	table::{table::TableWithConn, Table},
	FromRow, ToRow,
};

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

// match this with migrations
#[derive(Debug, Clone, FromRow, ToRow)]
struct FullUserTable {
	id: Id,
	email: String,
}

#[async_trait::async_trait]
impl UsersPersistent for Postgres<'_> {
	async fn insert(&self, user: InsertRawUser<'_>) -> Result<RawUser, Error> {
		let user = FullUserTable {
			id: Id::new(KIND),
			email: user.email.to_string(),
		};

		self.table.insert(&user).await?;

		Ok(user.into())
	}

	async fn by_email(&self, email: &str) -> Result<Option<RawUser>, Error> {
		self.table
			.select_opt::<FullUserTable>(filter!(&email))
			.await
			.map(|opt| opt.map(Into::into))
			.map_err(Into::into)
	}

	async fn by_id(&self, id: &Id) -> Result<Option<RawUser>, Error> {
		self.table
			.select_opt::<FullUserTable>(filter!(id))
			.await
			.map(|opt| opt.map(Into::into))
			.map_err(Into::into)
	}
}

impl From<FullUserTable> for RawUser {
	fn from(user: FullUserTable) -> Self {
		Self {
			id: user.id,
			email: user.email,
		}
	}
}
