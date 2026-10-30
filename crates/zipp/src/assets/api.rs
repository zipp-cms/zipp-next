//! The JSON requests of assets. Uploads are multipart: `POST /assets`
//! with the files, `POST /assets/{id}/file` with one, both answer with
//! the assets.

use serde::{Deserialize, Serialize};

use super::{Asset, AssetChange, Usage};
use crate::request;

/// The newest assets, or the ones of `ids`.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListReq {
	/// part of the filename
	pub search: Option<String>,
	/// `1,2,3`, in this order
	pub ids: Option<String>,
	/// `image` or `file`
	pub kind: Option<String>,
}

request!(ListReq, GET "/assets" => Vec<Asset>);

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReq {
	pub id: i64,
	#[serde(flatten)]
	pub change: AssetChange,
}

request!(UpdateReq, PUT "/assets/{id}" => Asset);

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteReq {
	pub id: i64,
}

request!(DeleteReq, DELETE "/assets/{id}" => ());

/// The entries that use an asset, the ones the user may see.
#[derive(Debug, Serialize, Deserialize)]
pub struct UsageReq {
	pub id: i64,
}

request!(UsageReq, GET "/assets/{id}/usage" => Vec<Usage>);
