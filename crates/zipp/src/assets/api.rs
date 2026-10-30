//! The JSON requests of assets. Uploads are multipart: `POST /assets`
//! with the files, `POST /assets/{id}/file` with one, both answer with
//! the assets.

use serde::{Deserialize, Serialize};

use super::{Asset, AssetChange, Folder, Usage};
use crate::request;

/// The newest assets, or the ones of `ids`; in a folder with `folder`
/// (an id, or `top`), else of all folders.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ListReq {
	/// part of the filename
	pub search: Option<String>,
	/// `1,2,3`, in this order
	pub ids: Option<String>,
	/// `image` or `file`
	pub kind: Option<String>,
	/// a folder's id, `top` for the ones in none
	pub folder: Option<String>,
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

/// All folders, by name.
#[derive(Debug, Serialize, Deserialize)]
pub struct FoldersReq {}

request!(FoldersReq, GET "/asset-folders" => Vec<Folder>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderReq {
	/// `None` on the top
	pub parent_id: Option<i64>,
	pub name: String,
}

request!(CreateFolderReq, POST "/asset-folders" => Folder);

/// A new name or place of a folder.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderReq {
	pub id: i64,
	pub parent_id: Option<i64>,
	pub name: String,
}

request!(UpdateFolderReq, PUT "/asset-folders/{id}" => Folder);

/// Deletes an empty folder.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteFolderReq {
	pub id: i64,
}

request!(DeleteFolderReq, DELETE "/asset-folders/{id}" => ());

/// Puts assets into a folder, `None` on the top.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoveReq {
	pub ids: Vec<i64>,
	pub folder_id: Option<i64>,
}

request!(MoveReq, POST "/assets/move" => ());
