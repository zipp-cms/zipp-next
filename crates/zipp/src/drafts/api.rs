use serde::{Deserialize, Serialize};

use super::{Doc, Draft, Revision, SiteDoc};
use crate::request;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetReq {
	pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DraftResp {
	pub draft: Option<Draft>,
}

request!(GetReq, GET "/entries/{id}/draft" => DraftResp);

/// The content of a site in the draft, saved while the editor types.
#[derive(Debug, Serialize, Deserialize)]
pub struct PutReq {
	pub id: i64,
	pub site: String,
	/// the version of the draft the editor knows, a newer one of someone
	/// else is a conflict
	pub base: Option<i64>,
	#[serde(flatten)]
	pub content: SiteDoc,
}

request!(PutReq, PUT "/entries/{id}/draft" => Draft);

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscardReq {
	pub id: i64,
}

request!(DiscardReq, DELETE "/entries/{id}/draft" => ());

/// The draft goes live in every site.
#[derive(Debug, Serialize, Deserialize)]
pub struct PublishReq {
	pub id: i64,
}

request!(PublishReq, POST "/entries/{id}/publish" => ());

#[derive(Debug, Serialize, Deserialize)]
pub struct RevisionsReq {
	pub id: i64,
}

request!(RevisionsReq, GET "/entries/{id}/revisions" => Vec<Revision>);

/// The content of a revision.
#[derive(Debug, Serialize, Deserialize)]
pub struct RevisionReq {
	pub id: i64,
	pub rev: i64,
}

request!(RevisionReq, GET "/entries/{id}/revisions/{rev}" => Doc);

/// A revision becomes the draft.
#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreReq {
	pub id: i64,
	pub rev: i64,
}

request!(RestoreReq, POST "/entries/{id}/revisions/{rev}/restore" => Draft);

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResp {
	pub token: String,
}

/// A token that shows the draft of the entry on the website, for a day.
#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewReq {
	pub id: i64,
}

request!(PreviewReq, POST "/entries/{id}/preview" => TokenResp);

/// A token that shows every draft, for the AI chat.
#[derive(Debug, Serialize, Deserialize)]
pub struct SitePreviewReq {}

request!(SitePreviewReq, POST "/preview" => TokenResp);
