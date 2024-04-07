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

use migrations::Migrations;
use postgres::{
	error::{CreateError, GetError},
	Config, PostgresPool,
};

pub mod id;
pub mod memory;
pub mod migrations;
pub mod postgres;

mod types;

#[derive(Debug, Clone)]
enum Inner {
	Memory,
	Postgres(PostgresPool),
}

// Maybe call this DatabasePools?
#[derive(Debug, Clone)]
pub struct DatabasePool {
	inner: Inner,
	migrations: Migrations,
}

impl DatabasePool {
	/// Create a new memory database pool
	pub fn new_memory() -> Self {
		Self {
			inner: Inner::Memory,
			migrations: Migrations::new(),
		}
	}

	/// Create a new postgres database pool
	pub async fn new_postgres(cfg: Config) -> Result<Self, CreateError> {
		Ok(Self {
			inner: Inner::Postgres(PostgresPool::new(cfg).await?),
			migrations: Migrations::new(),
		})
	}

	/// Get a database from the pool
	pub async fn get(&self) -> Result<Database, GetError> {
		match &self.inner {
			Inner::Memory => Ok(Database {
				inner: DatabaseInner::Memory,
				migrations: self.migrations.clone(),
			}),
			Inner::Postgres(pg) => Ok(Database {
				inner: DatabaseInner::Postgres(pg.get().await?),
				migrations: self.migrations.clone(),
			}),
		}
	}
}

enum DatabaseInner {
	Memory,
	Postgres(postgres::Postgres),
}

/// A Database from the pool
// needs to provide transaction and commit
pub struct Database {
	inner: DatabaseInner,
	migrations: Migrations,
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
			DatabaseInner::Postgres(_) => DatabaseKind::Postgres,
		}
	}

	/// Get the migrations
	pub fn migrations(&self) -> Migrations {
		self.migrations.clone()
	}

	pub fn connection(&self) -> Connection {
		match &self.inner {
			DatabaseInner::Memory => Connection {
				inner: ConnectionInner::Memory(memory::Connection::new()),
			},
			DatabaseInner::Postgres(pg) => Connection {
				inner: ConnectionInner::Postgres(pg.connection()),
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
	// pub fn get_accessor(&self) -> Accessor {
	// 	match self.inner {
	// 		ConnectionInner::Memory => Accessor::Memory(MemoryAccessor {}),
	// 		ConnectionInner::Postgres(_) => {
	// 			Accessor::Postgres(PostgresAccessor {})
	// 		}
	// 	}
	// }

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

// impl FromConnection for postgres::Connection<'_> {
// 	fn from_connection<'a>(conn: Connection<'a>) -> Self {
// 		match conn.inner {
// 			ConnectionInner::Memory(_) => unreachable!("postgres expected"),
// 			ConnectionInner::Postgres(pg) => pg,
// 		}
// 	}
// }

// impl private::Sealed for postgres::Connection<'_> {}

mod private {
	pub trait Sealed {}
}
