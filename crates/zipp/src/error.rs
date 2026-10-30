use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};

use crate::content::FieldError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "detail", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Error {
	#[error("Email or password incorrect")]
	LoginIncorrect,
	#[error("Missing session token in request")]
	MissingSessionToken,
	#[error("Invalid session token in request")]
	InvalidSessionToken,
	/// GraphQL without the endpoint token or a session
	#[error("GraphQL needs the endpoint token: Authorization: Bearer <token>")]
	MissingEndpointToken,
	#[error("Your role may not do that")]
	Forbidden,
	#[error("Resource not found")]
	NotFound,
	#[error("Some fields are invalid")]
	Invalid(Vec<FieldError>),
	#[error("Internal server error: {0}")]
	Internal(String),
	#[error("Request error: {0}")]
	Request(String),
	/// someone else changed it meanwhile
	#[error("{0}")]
	Conflict(String),
}

impl Error {
	pub fn status_code(&self) -> StatusCode {
		match self {
			Self::LoginIncorrect
			| Self::MissingSessionToken
			| Self::MissingEndpointToken => StatusCode::UNAUTHORIZED,
			Self::InvalidSessionToken | Self::Forbidden => {
				StatusCode::FORBIDDEN
			}
			Self::NotFound => StatusCode::NOT_FOUND,
			Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Invalid(_) | Self::Request(_) => StatusCode::BAD_REQUEST,
			Self::Conflict(_) => StatusCode::CONFLICT,
		}
	}
}

#[derive(Debug, Clone, Serialize)]
struct ErrorResponse {
	pub message: String,
	pub error: Error,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let status = self.status_code();
		// what failed inside is for the log, not for whoever asked
		let this = match self {
			Self::Internal(msg) => {
				tracing::error!("{msg}");
				Self::Internal("see the log".into())
			}
			e => e,
		};
		(
			status,
			Json(ErrorResponse {
				message: this.to_string(),
				error: this,
			}),
		)
			.into_response()
	}
}

impl From<rusqlite::Error> for Error {
	fn from(e: rusqlite::Error) -> Self {
		match e {
			rusqlite::Error::QueryReturnedNoRows => Self::NotFound,
			e => Self::Internal(e.to_string()),
		}
	}
}
