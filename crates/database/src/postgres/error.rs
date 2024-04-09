use tokio_postgres::{error::SqlState, Error as PgError};

pub use deadpool::managed::TimeoutType;
use tracing::error;

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
	#[error("Configuration error: {0}")]
	Config(String),

	#[error("Testing the connection failed {0}")]
	Get(GetError),

	#[error("Migration error {0}")]
	Migration(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
	#[error("Getting a connection from the pool timed out {0:?}")]
	Timeout(TimeoutType),

	#[error("Postgres error {0}")]
	Error(#[from] Error),
}

impl GetError {
	pub(super) fn from_pg(e: PgError) -> Self {
		Self::Error(Error::from_pg(e))
	}
}

// TransactionError
#[derive(Debug, thiserror::Error)]
pub enum TransactionError {
	#[error("Postgres error {0}")]
	Unknown(String),
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Unique violation {0}")]
	UniqueViolation(String),

	#[error("Unknown Postgres error {0}")]
	Unknown(String),
}

impl Error {
	pub(super) fn from_pg(e: PgError) -> Self {
		let Some(state) = e.code() else {
			return Self::Unknown(e.to_string());
		};

		match state {
			&SqlState::UNIQUE_VIOLATION => Self::UniqueViolation(e.to_string()),
			state => {
				error!("db error with state {:?}", state);
				Self::Unknown(e.to_string())
			}
		}
	}
}
