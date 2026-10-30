use serde::{Deserialize, Serialize};

use crate::request;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReq {
	pub email: String,
	pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResp {
	pub token: String,
}

request!(LoginReq, POST "/auth/login" => LoginResp);

/// Ends the session of the `session-token` header, if there is one.
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutReq {}

request!(LogoutReq, POST "/auth/logout" => ());

/// Whether the session is valid.
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckReq {}

request!(CheckReq, POST "/auth/check" => ());
