use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum ValidateError {
	#[error("Field validation failed")]
	ValidationFailed,
}

/*
1. api
2. validate
3. persistent
4. schema
5. component

*/

#[derive(thiserror::Error)]
pub enum ParseFieldError {
	#[error("Field has unknown kind: {0}")]
	KindNotFound(String),

	#[error("Invalid settings: {settings:?}")]
	InvalidSettings { settings: Vec<String> },
}

impl fmt::Debug for ParseFieldError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self)
	}
}
