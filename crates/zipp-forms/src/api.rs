//! The requests of the studio's page Submissions, for who may view
//! `forms` (deleting needs Delete there).

use serde::{Deserialize, Serialize};
use zipp::request;

use crate::submissions::Submission;

/// `GET /forms/submissions?form=5&offset=50`: the newest first, a page
/// of [`PAGE`](crate::submissions::PAGE).
#[derive(Debug, Serialize, Deserialize)]
pub struct ListReq {
	pub form: Option<i64>,
	#[serde(default)]
	pub offset: i64,
	/// whose titles name the forms, default the primary site
	pub site: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResp {
	pub submissions: Vec<Submission>,
	pub total: i64,
}

request!(ListReq, GET "/forms/submissions" => ListResp);

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteReq {
	pub id: i64,
}

request!(DeleteReq, DELETE "/forms/submissions/{id}" => ());
