use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{Entry, EntryInput, EntryRef};
use crate::request;

/// Entries of a [`Query`](crate::query::Query): its params as pairs,
/// like `section=events&order=title&field.city=Bern`.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(from = "Vec<(String, String)>")]
pub struct ListReq(pub Vec<(String, String)>);

impl From<Vec<(String, String)>> for ListReq {
	fn from(params: Vec<(String, String)>) -> Self {
		Self(params)
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResp {
	pub entries: Vec<Entry>,
	pub total: i64,
	pub relations: BTreeMap<i64, EntryRef>,
	/// whether each entry is published, per site
	pub published: BTreeMap<i64, BTreeMap<String, bool>>,
	/// the entries with unpublished changes
	pub drafts: Vec<i64>,
}

request!(ListReq, GET "/entries" => ListResp);

#[derive(Debug, Serialize, Deserialize)]
pub struct GetReq {
	pub id: i64,
	/// default the primary site
	pub site: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryResp {
	pub entry: Entry,
	pub relations: BTreeMap<i64, EntryRef>,
}

request!(GetReq, GET "/entries/{id}" => EntryResp);

/// A new entry, published only if the user may publish.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq(pub EntryInput);

request!(CreateReq, POST "/entries" => Entry);

/// Saves the live entry directly, without a draft (the api of scripts).
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReq {
	pub id: i64,
	#[serde(flatten)]
	pub input: EntryInput,
}

request!(UpdateReq, PUT "/entries/{id}" => Entry);

/// Moves an entry of a structure under a parent, at a position.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveReq {
	pub id: i64,
	pub parent_id: Option<i64>,
	pub index: usize,
}

request!(MoveReq, POST "/entries/{id}/move" => ());

/// Moves an entry to the trash, in a structure with its children.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteReq {
	pub id: i64,
}

request!(DeleteReq, DELETE "/entries/{id}" => ());
