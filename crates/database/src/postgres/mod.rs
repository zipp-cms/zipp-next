pub mod error;

use deadpool::managed::PoolError;
use deadpool_postgres::{
	ClientWrapper, CreatePoolError, Object, Pool, Runtime,
};
use serde::Deserialize;
use tokio_postgres::{NoTls, Statement};

use self::error::{CreateError, Error, GetError, TransactionError};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
	user: String,
	password: String,
	database: String,
	host: Option<String>,
	port: Option<u16>,
}

#[derive(Debug, Clone)]
pub struct PostgresPool {
	inner: Pool,
}

impl PostgresPool {
	pub async fn new(cfg: Config) -> Result<Self, CreateError> {
		let config = deadpool_postgres::Config {
			user: Some(cfg.user),
			password: Some(cfg.password),
			dbname: Some(cfg.database),
			host: cfg.host,
			port: cfg.port,
			..Default::default()
		};

		let pool =
			config
				.create_pool(Some(Runtime::Tokio1), NoTls)
				.map_err(|e| match e {
					CreatePoolError::Config(e) => {
						CreateError::Config(e.to_string())
					}
					CreatePoolError::Build(_) => unreachable!(
						"since we provide a runtime this should never happen"
					),
				})?;

		let this = Self { inner: pool };

		this.get().await.map_err(CreateError::Get)?;

		Ok(this)
	}

	pub async fn get(&self) -> Result<Postgres, GetError> {
		self.inner
			.get()
			.await
			.map_err(|e| match e {
				PoolError::Timeout(tim) => GetError::Timeout(tim),
				PoolError::Backend(e) => GetError::from_pg(e),
				PoolError::Closed => todo!("when can a pool be closed?"),
				PoolError::NoRuntimeSpecified => unreachable!(),
				PoolError::PostCreateHook(e) => {
					todo!("what is this error {e:?}?")
				}
			})
			.map(|inner| Postgres { inner })
	}
}

#[derive(Debug)]
pub struct Postgres {
	// is a holder of ClientWrapper
	inner: Object,
}

impl Postgres {
	pub fn connection(&self) -> Connection {
		Connection {
			inner: ConnectionInner::Client(&self.inner),
		}
	}

	pub async fn transaction(
		&mut self,
	) -> Result<Transaction, TransactionError> {
		Ok(Transaction {
			inner: self
				.inner
				.transaction()
				.await
				.map_err(|e| TransactionError::Unknown(e.to_string()))?,
		})
	}
}

pub struct Transaction<'a> {
	inner: deadpool_postgres::Transaction<'a>,
}

impl<'a> Transaction<'a> {
	pub fn connection(&self) -> Connection {
		Connection {
			inner: ConnectionInner::Transaction(&self.inner),
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Connection<'a> {
	inner: ConnectionInner<'a>,
}

#[derive(Debug, Clone, Copy)]
enum ConnectionInner<'a> {
	Client(&'a ClientWrapper),
	Transaction(&'a deadpool_postgres::Transaction<'a>),
}

impl Connection<'_> {
	pub async fn prepare(&self, query: &str) -> Result<Statement, Error> {
		match &self.inner {
			ConnectionInner::Client(client) => {
				client.prepare_cached(query).await.map_err(Error::from_pg)
			}
			ConnectionInner::Transaction(tr) => {
				tr.prepare_cached(query).await.map_err(Error::from_pg)
			}
		}
	}
}

// impl AsMut<ClientWrapper> for Connection {
// 	fn as_mut(&mut self) -> &mut ClientWrapper {
// 		self.inner.as_mut()
// 	}
// }

// impl AsRef<ClientWrapper> for Connection {
// 	fn as_ref(&self) -> &ClientWrapper {
// 		self.inner.as_ref()
// 	}
// }

// impl Deref for Connection {
// 	type Target = ClientWrapper;

// 	fn deref(&self) -> &Self::Target {
// 		self.inner.deref()
// 	}
// }

// impl DerefMut for Connection {
// 	fn deref_mut(&mut self) -> &mut Self::Target {
// 		self.inner.deref_mut()
// 	}
// }
