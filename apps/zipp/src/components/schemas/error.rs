use std::io;

use crate::fields::ParseFieldError;

#[derive(Debug, thiserror::Error)]
pub enum SchemaError {
	#[error("IO error: {error} for file: {file_name}")]
	Io { error: io::Error, file_name: String },

	#[error("JSON error: {error} for file: {file_name}")]
	Json {
		error: serde_json::Error,
		file_name: String,
	},

	#[error("Parse error: {error} for file: {file_name}")]
	Parse {
		error: ParseFieldError,
		file_name: String,
	},
}

impl SchemaError {
	pub fn io(error: io::Error, file_name: impl Into<String>) -> Self {
		Self::Io {
			error,
			file_name: file_name.into(),
		}
	}

	pub fn json(
		error: serde_json::Error,
		file_name: impl Into<String>,
	) -> Self {
		Self::Json {
			error,
			file_name: file_name.into(),
		}
	}

	pub fn parse(error: ParseFieldError, file_name: impl Into<String>) -> Self {
		Self::Parse {
			error,
			file_name: file_name.into(),
		}
	}
}
