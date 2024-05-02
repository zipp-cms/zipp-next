mod persistent;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("a user with the email {email} already exists!")]
	AlreadyExists { email: String },

	#[error("a connection error occured!")]
	Connection(String),

	#[error("a postgres error occured!")]
	Postgres(#[from] database::Error),
}
