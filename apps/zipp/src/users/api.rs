use core::fmt;

use database::DatabasePool;
use email_address::EmailAddress;
use fire_http::{header::StatusCode, FireBuilder};
use fire_http_api::{api, ApiError, Method, Request};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::users::Users;

use super::User;

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
	// server internal
	fn internal<E: fire_http_api::error::Error>(error: E) -> Self {
		Self::Internal(error.to_string())
	}

	// an error with the request
	fn request<E: fire_http_api::error::Error>(error: E) -> Self {
		Self::Request(error.to_string())
	}

	fn status_code(&self) -> StatusCode {
		match self {
			Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Request(_) => StatusCode::BAD_REQUEST,
		}
	}
}

// todo
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
	token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReq {
	pub email: EmailAddress,
	pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
	pub user: User,
	pub session: Session,
}

impl Request for LoginReq {
	type Response = Login;
	type Error = Error;

	const PATH: &'static str = "/users/login";
	const METHOD: Method = Method::POST;
}

#[api(LoginReq)]
pub async fn login(
	req: LoginReq,
	users: &Users,
	db: &DatabasePool,
) -> Result<Login, Error> {
	let db = db.get().await.map_err(Error::string_internal)?;
	let users = users.with_conn(db.connection());

	let user = users
		.by_email(req.email.as_ref())
		.await
		.map_err(Error::string_internal)?;

	info!("login user: {:?}", user);

	todo!()
}

pub fn register(fire: &mut FireBuilder) {
	fire.add_route(login);
}
