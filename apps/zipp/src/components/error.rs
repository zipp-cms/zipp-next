use super::schemas::SchemaError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("a connection error occured!")]
	Connection(String),

	#[error("a postgres error occured!")]
	Postgres(#[from] database::Error),

	#[error("a schema error occured!")]
	Schema(#[from] SchemaError),
}
