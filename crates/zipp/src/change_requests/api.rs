use serde::{Deserialize, Serialize};

use super::ChangeRequest;
use crate::request;

/// All change requests, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListReq {}

request!(ListReq, GET "/change-requests" => Vec<ChangeRequest>);

/// A change of the content model someone needs, anyone may write one.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
	/// the path of the Setup page
	pub page: String,
	/// what the page shows, like `Entry types › Page`
	pub context: String,
	pub text: String,
}

request!(CreateReq, POST "/change-requests" => ChangeRequest);

/// Marks a request done or open again, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub struct DoneReq {
	pub id: i64,
	pub done: bool,
}

request!(DoneReq, PUT "/change-requests/{id}" => ());
