use serde::{Deserialize, Serialize};

use super::User;
use crate::request;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListReq {}

request!(ListReq, GET "/users" => Vec<User>);

/// A new user, or a new password and role of one with this email.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
	pub email: String,
	pub password: String,
	/// `None` is an admin
	pub role: Option<String>,
}

request!(CreateReq, POST "/users" => ());

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleReq {
	pub id: i64,
	pub role: Option<String>,
}

request!(RoleReq, PUT "/users/{id}" => ());

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteReq {
	pub id: i64,
}

request!(DeleteReq, DELETE "/users/{id}" => ());
