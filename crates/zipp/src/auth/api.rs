use serde::{Deserialize, Serialize};

use super::links::LinkKind;
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
	/// the email of the admin who logged in as the user
	pub impersonated_by: Option<String>,
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

/// Mails a password reset link if there is a user with the email; the
/// answer is the same either way.
#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotReq {
	pub email: String,
}

request!(ForgotReq, POST "/auth/forgot" => ());

/// What a link of `/admin/password/<token>` is for, before it is used.
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkReq {
	pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkInfo {
	pub kind: LinkKind,
	pub email: String,
	pub name: String,
	pub expires_at: i64,
}

request!(LinkReq, GET "/auth/links/{token}" => LinkInfo);

/// Sets the password with an invite or reset link and logs in; an
/// invited user may give their name too.
#[derive(Debug, Serialize, Deserialize)]
pub struct SetPasswordReq {
	pub token: String,
	pub password: String,
	pub name: Option<String>,
}

request!(SetPasswordReq, POST "/auth/links/{token}/password" => Authenticated);

/// Logs in as the user of a link an admin made, see `users`.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAsReq {
	pub token: String,
}

request!(LoginAsReq, POST "/auth/links/{token}/login-as" => Authenticated);
