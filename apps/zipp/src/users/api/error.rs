use core::fmt;

use database::DatabaseError;
use fire_http::header::StatusCode;
use fire_http_api::{error, ApiError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
pub enum Error {
	#[error("internal error {0}")]
	Internal(String),

	#[error("request error {0}")]
	Request(String),
}

impl Error {
	pub fn string_internal<E: fmt::Display>(error: E) -> Self {
		Self::Internal(error.to_string())
	}
}

impl ApiError for Error {
	fn from_error(e: error::Error) -> Self {
		use error::Error::*;

		match e {
			HeadersMissing(_) | Deserialize(_) => Self::Request(e.to_string()),
			ExtractionError(e) => {
				// we should check if the type is Error
				e.downcast()
					.map(|e| *e)
					.unwrap_or_else(|e| Self::Internal(e.to_string()))
			}
			e => Self::Internal(e.to_string()),
		}
	}

	fn status_code(&self) -> StatusCode {
		match self {
			Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Request(_) => StatusCode::BAD_REQUEST,
		}
	}
}

impl From<DatabaseError> for Error {
	fn from(e: DatabaseError) -> Self {
		Self::Internal(e.to_string())
	}
}
