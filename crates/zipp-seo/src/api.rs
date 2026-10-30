//! The api of the plugin, for the studio.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use zipp::request;

/// The defaults of an entry, what its SEO field shows as placeholders:
/// as the editor sees the entry, its draft first.
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultsReq {
	pub id: i64,
	/// default the primary site
	pub site: Option<String>,
}

request!(DefaultsReq, GET "/entries/{id}/seo" => Value);
