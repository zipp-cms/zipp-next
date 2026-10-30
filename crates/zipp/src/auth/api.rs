use serde::{Deserialize, Serialize};

use crate::request;
use crate::users::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginReq {
	pub email: String,
	pub password: String,
}

/// A session and its user, what the studio keeps.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Authenticated {
	pub session: SessionInfo,
	pub user: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
	pub token: String,
	/// in milliseconds; using the studio pushes it back
	pub expires_at: i64,
}

request!(LoginReq, POST "/auth/login" => Authenticated);

/// The session of the `session-token` header with its user, renewed when
/// it ends soon; the studio asks when it opens.
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionReq {}

request!(SessionReq, GET "/auth/session" => Authenticated);

/// Ends the session of the `session-token` header, if there is one.
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutReq {}

request!(LogoutReq, POST "/auth/logout" => ());
