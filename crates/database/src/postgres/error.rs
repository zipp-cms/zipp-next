use tokio_postgres::Error as PgError;

pub use deadpool::managed::TimeoutType;

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
	#[error("Configuration error: {0}")]
	Config(String),

	#[error("Testing the connection failed {0}")]
	Get(GetError),
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
	#[error("Getting a connection from the pool timed out {0:?}")]
	Timeout(TimeoutType),

	#[error("Postgres error: {0}")]
	Unknown(String),
}

impl GetError {
	pub(super) fn from_pg(e: PgError) -> Self {
		todo!()
	}
}

// TransactionError
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
	#[error("Postgres error {0}")]
	Unknown(String),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

impl Error {
	pub(super) fn from_pg(e: PgError) -> Self {
		todo!()
	}
}
