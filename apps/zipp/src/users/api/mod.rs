pub mod error;
pub mod handlers;

pub use error::Error;
pub use handlers::register;

use email_address::EmailAddress;

use fire_http_api::{Method, Request};
use serde::{Deserialize, Serialize};

use super::User;

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
