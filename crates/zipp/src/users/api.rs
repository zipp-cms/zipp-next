use serde::{Deserialize, Serialize};

use super::Account;
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

/// A new user, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
	pub email: String,
	pub name: String,
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
