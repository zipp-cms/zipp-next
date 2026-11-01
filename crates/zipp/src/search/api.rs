use serde::{Deserialize, Serialize};

use super::Found;
use crate::request;

/// Entries and assets with the words of `q`, entries in `site` (default
/// the primary site).
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchReq {
	pub q: String,
	pub site: Option<String>,
}

request!(SearchReq, GET "/search" => Found);
