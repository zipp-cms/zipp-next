use serde::{Deserialize, Serialize};

use super::{SiteUser, links};
use crate::request;
use crate::users::User;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LoginReq {
	pub email: String,
	pub password: String,
}

/// A session and its user, what the studio keeps.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Authenticated {
	pub session: SessionInfo,
	pub user: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SessionInfo {
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
pub(crate) struct SessionReq {}

request!(SessionReq, GET "/auth/session" => Authenticated);

/// Ends the session of the `session-token` header, if there is one.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LogoutReq {}

request!(LogoutReq, POST "/auth/logout" => ());

/// Mails a password reset link if there is a user with the email; the
/// answer is the same either way.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ForgotReq {
	pub email: String,
}

request!(ForgotReq, POST "/auth/forgot" => ());

/// What a link of `/admin/password/<token>` is for, before it is used.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LinkReq {
	pub token: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LinkInfo {
	pub kind: links::Kind,
	pub email: String,
	pub name: String,
	pub expires_at: i64,
}

request!(LinkReq, GET "/auth/links/{token}" => LinkInfo);

/// Sets the password with an invite or reset link and logs in; an
/// invited user may give their name too.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SetPasswordReq {
	pub token: String,
	pub password: String,
	pub name: Option<String>,
}

request!(SetPasswordReq, POST "/auth/links/{token}/password" => Authenticated);

/// Logs in as the user of a link an admin made, see `users`.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct LoginAsReq {
	pub token: String,
}

request!(LoginAsReq, POST "/auth/links/{token}/login-as" => Authenticated);

/// A login on the website, sent by its frontend; see `auth/site.rs`.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SiteLoginReq {
	pub email: String,
	pub password: String,
}

/// A site session and its user, what the frontend keeps in its cookie.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct SiteAuthenticated {
	pub session: SiteSession,
	pub user: SiteUser,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SiteSession {
	pub token: String,
	/// in milliseconds; `/site-auth/me` pushes it back
	pub expires_at: i64,
}

request!(SiteLoginReq, POST "/site-auth/login" => SiteAuthenticated);

/// Ends the site session of the bearer token, if there is one.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SiteLogoutReq {}

request!(SiteLogoutReq, POST "/site-auth/logout" => ());

/// The site session of the bearer token with its user, renewed when it
/// ends soon.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SiteMeReq {}

request!(SiteMeReq, GET "/site-auth/me" => SiteAuthenticated);

/// Mails a link to the website's `/reset-password?token=…` if there is a
/// user with the email; the answer is the same either way. `site` picks
/// the site's url, else the first one's.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SiteResetReq {
	pub email: String,
	#[serde(default)]
	pub site: Option<String>,
}

request!(SiteResetReq, POST "/site-auth/reset" => ());

/// Sets the password with an invite or reset link and logs in on the
/// website.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SitePasswordReq {
	pub token: String,
	pub password: String,
}

request!(SitePasswordReq, POST "/site-auth/password" => SiteAuthenticated);
