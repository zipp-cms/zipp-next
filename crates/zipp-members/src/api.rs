//! The routes of the website, JSON with the site token of a logged in
//! user (`Authorization: Bearer`): 401 without one, 403 for what their
//! role may not do (another's entry needs the section's access, own
//! access is only for theirs). Invalid fields answer like the
//! studio's api (`INVALID` with the paths).
//!
//! - `GET /members/entries?section=recipes`: their entries, the newest
//!   first, unpublished ones too; `&all=true` all of the section, for
//!   who may view it (admins, editors)
//! - `GET /members/entries/{id}`: theirs, or another's their role may
//!   view
//! - `POST /members/entries`: a new one
//! - `PUT /members/entries/{id}`: live if their role may publish, else a
//!   draft for an editor
//! - `DELETE /members/entries/{id}`: to the trash
//! - `POST /members/assets`, multipart: an image, with
//!   [`Members::images`](crate::Members::images)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use zipp::entries::Saved;
use zipp::request;

/// An entry as its member edits it: with the content of its draft if it
/// has one (`hasDraft`), else as it is live.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberEntry {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub site: String,
	pub title: String,
	pub slug: String,
	pub uri: Option<String>,
	/// the page on the website, with `serve --site-url`
	pub url: Option<String>,
	/// live, whatever the draft says
	pub published: bool,
	pub post_date: Option<i64>,
	pub fields: Value,
	pub has_draft: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListReq {
	pub section: String,
	/// default the primary site
	pub site: Option<String>,
	#[serde(default)]
	pub offset: i64,
	/// all entries of the section, not just the user's: needs View of
	/// the section, own access isn't enough
	#[serde(default)]
	pub all: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResp {
	/// at most 100 from `offset`
	pub entries: Vec<MemberEntry>,
	pub total: i64,
}

request!(ListReq, GET "/members/entries" => ListResp);

#[derive(Debug, Serialize, Deserialize)]
pub struct GetReq {
	pub id: i64,
	pub site: Option<String>,
}

request!(GetReq, GET "/members/entries/{id}" => MemberEntry);

/// A new entry, published only if asked and the role may publish.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub site: Option<String>,
	pub title: String,
	#[serde(default)]
	pub fields: Value,
	#[serde(default)]
	pub published: bool,
}

request!(CreateReq, POST "/members/entries" => MemberEntry);

/// A change of an entry in a site: the fields it sends replace theirs,
/// the others stay; the type, slug and dates stay.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReq {
	pub id: i64,
	pub site: Option<String>,
	pub title: String,
	#[serde(default)]
	pub fields: Value,
	/// default as it is
	pub published: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResp {
	/// `live` or `draft`
	pub saved: Saved,
	pub entry: MemberEntry,
}

request!(UpdateReq, PUT "/members/entries/{id}" => UpdateResp);

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteReq {
	pub id: i64,
}

request!(DeleteReq, DELETE "/members/entries/{id}" => ());

/// The answer of `POST /members/assets`; the id goes into an image
/// field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Uploaded {
	pub id: i64,
	pub url: String,
	pub width: Option<u32>,
	pub height: Option<u32>,
}
