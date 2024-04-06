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

use postgres::{
	error::{CreateError, GetError},
	Config, PostgresPool,
};

pub mod id;
pub mod memory;
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
}

impl DatabasePool {
	pub fn new_memory() -> Self {
		Self {
			inner: Inner::Memory,
		}
	}

	pub async fn new_postgres(cfg: Config) -> Result<Self, CreateError> {
		Ok(Self {
			inner: Inner::Postgres(PostgresPool::new(cfg).await?),
		})
	}

	pub async fn get(&self) -> Result<Database, GetError> {
		match &self.inner {
			Inner::Memory => Ok(Database {
				inner: DatabaseInner::Memory,
			}),
			Inner::Postgres(pg) => Ok(Database {
				inner: DatabaseInner::Postgres(pg.get().await?),
			}),
		}
	}
}

// #[derive(Debug, Clone)]
// pub enum Accessor {
// 	Memory(MemoryAccessor),
// 	Postgres(PostgresAccessor),
// }

// #[derive(Debug, Clone)]
// pub struct MemoryAccessor {}

// impl MemoryAccessor {
// 	pub fn get(&self, conn: Connection) -> () {
// 		match conn.inner {
// 			ConnectionInner::Memory(_) => (),
// 			ConnectionInner::Postgres(_) => unreachable!("memory expected"),
// 		}
// 	}
// }

// #[derive(Debug, Clone)]
// pub struct PostgresAccessor {}

// impl PostgresAccessor {
// 	pub fn get<'a>(&self, conn: Connection<'a>) -> postgres::Connection<'a> {
// 		match conn.inner {
// 			ConnectionInner::Memory(_) => unreachable!("postgres expected"),
// 			ConnectionInner::Postgres(pg) => pg,
// 		}
// 	}
// }

enum DatabaseInner {
	Memory,
	Postgres(postgres::Postgres),
}

// needs to provide transaction and commit, as well as normal queries
// how should it be called, maybe something like GenericlCient?
// or Access??
pub struct Database {
	inner: DatabaseInner,
}

impl Database {
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

	pub fn kind(&self) -> ConnectionKind {
		match self.inner {
			ConnectionInner::Memory(_) => ConnectionKind::Memory,
			ConnectionInner::Postgres(_) => ConnectionKind::Postgres,
		}
	}

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
pub enum ConnectionKind {
	Memory,
	Postgres,
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
