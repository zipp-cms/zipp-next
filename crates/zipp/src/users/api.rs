use serde::{Deserialize, Serialize};

use super::Account;
use crate::auth::links::LinkKind;
use crate::request;

/// All users, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListReq {}

request!(ListReq, GET "/users" => Vec<Account>);

/// A user, for admins and the user.
#[derive(Debug, Serialize, Deserialize)]
pub struct GetReq {
	pub id: i64,
}

request!(GetReq, GET "/users/{id}" => Account);

/// A new user, for admins; without a password invited, see
/// [`PasswordLinkReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
	pub email: String,
	pub name: String,
	#[serde(default)]
	pub password: String,
	/// `None` is an admin
	pub role: Option<String>,
}

request!(CreateReq, POST "/users" => Account);

/// The account of a user: admins change anyone's, a user their own but
/// not the role.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReq {
	pub id: i64,
	pub email: String,
	pub name: String,
	/// `None` is an admin; the own stays
	pub role: Option<String>,
	/// a new one, the user is logged out everywhere else
	pub password: Option<String>,
	/// with a new password of oneself
	pub current_password: Option<String>,
}

request!(UpdateReq, PUT "/users/{id}" => Account);

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteReq {
	pub id: i64,
}

request!(DeleteReq, DELETE "/users/{id}" => ());

/// Logs a user out everywhere, oneself except here.
#[derive(Debug, Serialize, Deserialize)]
pub struct EndSessionsReq {
	pub id: i64,
}

request!(EndSessionsReq, DELETE "/users/{id}/sessions" => ());

/// A link for the user to set a password, for admins: an invite while
/// the user has none, else a password reset. `send` mails it.
#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordLinkReq {
	pub id: i64,
	#[serde(default)]
	pub send: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkResp {
	pub kind: LinkKind,
	pub url: String,
	pub expires_at: i64,
	/// whether it was mailed
	pub sent: bool,
}

request!(PasswordLinkReq, POST "/users/{id}/password-link" => LinkResp);

/// A link that logs in as the user, for admins and users who aren't
/// admins; it works once, in any browser.
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginAsLinkReq {
	pub id: i64,
}

request!(LoginAsLinkReq, POST "/users/{id}/login-as" => LinkResp);
