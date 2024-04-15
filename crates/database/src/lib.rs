//! Database layer
//!
//! This crate provides an abstraction around tokio postgres and some
//! helper functions to implement a memory database.
//!
//! Maybe the tree should be
//! DatabasePool
//! > Database
//! > .transaction
//! > > Transaction
//! > > .connection
//! > > > Connection
//! > .connection
//! > > Connection (MemoryConnection, PostgresConnection)

use postgres::{connection::ConnectionOwned, migrations::Migrations};

pub use postgres::connection::Error;
pub use postgres::database::DatabaseError;
use serde::Deserialize;

pub mod id;
pub mod macros;
pub mod memory;

mod types;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	user: String,
	password: String,
	database: String,
	host: Option<String>,
	port: Option<u16>,
}

#[derive(Debug, Clone)]
enum Inner {
	Memory,
	Postgres(postgres::Database),
}

// Maybe call this DatabasePools?
#[derive(Debug, Clone)]
pub struct DatabasePool {
	inner: Inner,
}

impl DatabasePool {
	/// Create a new memory database pool
	pub fn new_memory() -> Self {
		Self {
			inner: Inner::Memory,
		}
	}

	/// Create a new postgres database pool
	pub async fn new_postgres(cfg: Config) -> Result<Self, DatabaseError> {
		let config = deadpool_postgres::Config {
			user: Some(cfg.user),
			password: Some(cfg.password),
			dbname: Some(cfg.database),
			host: cfg.host,
			port: cfg.port,
			..Default::default()
		};

		Ok(Self {
			inner: Inner::Postgres(postgres::Database::with_cfg(config).await?),
		})
	}

	/// Get a database from the pool
	pub async fn get(&self) -> Result<Database, DatabaseError> {
		match &self.inner {
			Inner::Memory => Ok(Database {
				inner: DatabaseInner::Memory,
			}),
			Inner::Postgres(pg) => Ok(Database {
				inner: DatabaseInner::Postgres {
					conn: pg.get().await?,
					migrations: pg.migrations(),
				},
			}),
		}
	}
}

enum DatabaseInner {
	Memory,
	Postgres {
		conn: ConnectionOwned,
		migrations: Migrations,
	},
}

/// A Database from the pool
// needs to provide transaction and commit
pub struct Database {
	inner: DatabaseInner,
}

#[derive(Debug, Clone, Copy)]
pub enum DatabaseKind {
	Memory,
	Postgres,
}

impl Database {
	/// Get the kind of the database
	pub fn kind(&self) -> DatabaseKind {
		match self.inner {
			DatabaseInner::Memory => DatabaseKind::Memory,
			DatabaseInner::Postgres { .. } => DatabaseKind::Postgres,
		}
	}

	/// Get the migrations
	pub fn migrations(&self) -> Option<Migrations> {
		match &self.inner {
			DatabaseInner::Memory => None,
			DatabaseInner::Postgres { migrations, .. } => {
				Some(migrations.clone())
			}
		}
	}

	/// This will panic if not called when the connection is a postgres
	pub fn connection_owned(&mut self) -> &mut ConnectionOwned {
		match &mut self.inner {
			DatabaseInner::Memory => panic!("memory connection"),
			DatabaseInner::Postgres { conn, .. } => conn,
		}
	}

	pub fn connection(&self) -> Connection {
		match &self.inner {
			DatabaseInner::Memory => Connection {
				inner: ConnectionInner::Memory(memory::Connection::new()),
			},
			DatabaseInner::Postgres { conn, .. } => Connection {
				inner: ConnectionInner::Postgres(conn.connection()),
			},
		}
	}
}

/// A database connection
#[derive(Debug, Clone, Copy)]
pub struct Connection<'a> {
	inner: ConnectionInner<'a>,
}

impl<'a> Connection<'a> {
	pub fn get<T: FromConnection<'a>>(&self) -> T {
		T::from_connection(*self)
	}

	pub fn into_memory(self) -> memory::Connection<'a> {
		match self.inner {
			ConnectionInner::Memory(mem) => mem,
			ConnectionInner::Postgres(_) => unreachable!("memory expected"),
		}
	}

	pub fn into_postgres(self) -> postgres::Connection<'a> {
		match self.inner {
			ConnectionInner::Memory(_) => unreachable!("postgres expected"),
			ConnectionInner::Postgres(pg) => pg,
		}
	}

	pub fn try_into_postgres(self) -> Option<postgres::Connection<'a>> {
		match self.inner {
			ConnectionInner::Memory(_) => None,
			ConnectionInner::Postgres(pg) => Some(pg),
		}
	}
}

#[derive(Debug, Clone, Copy)]
enum ConnectionInner<'a> {
	Memory(memory::Connection<'a>),
	Postgres(postgres::Connection<'a>),
}

impl<'a> From<Connection<'a>> for memory::Connection<'a> {
	fn from(conn: Connection<'a>) -> Self {
		match conn.inner {
			ConnectionInner::Memory(mem) => mem,
			ConnectionInner::Postgres(_) => unreachable!("memory expected"),
		}
	}
}

pub trait FromConnection<'a>: private::Sealed {
	fn from_connection(conn: Connection<'a>) -> Self;
}

impl<'a> FromConnection<'a> for memory::Connection<'a> {
	fn from_connection(conn: Connection<'a>) -> Self {
		match conn.inner {
			ConnectionInner::Memory(mem) => mem,
			ConnectionInner::Postgres(_) => unreachable!("memory expected"),
		}
	}
}

impl private::Sealed for memory::Connection<'_> {}

impl<'a> FromConnection<'a> for postgres::Connection<'a> {
	fn from_connection(conn: Connection<'a>) -> Self {
		match conn.inner {
			ConnectionInner::Memory(_) => unreachable!("postgres expected"),
			ConnectionInner::Postgres(pg) => pg,
		}
	}
}

impl private::Sealed for postgres::Connection<'_> {}

mod private {
	pub trait Sealed {}
}
