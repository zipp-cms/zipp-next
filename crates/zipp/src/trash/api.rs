use serde::{Deserialize, Serialize};

use super::Trash;
use crate::request;

/// What the user may restore or delete for good.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrashReq {}

request!(TrashReq, GET "/trash" => Trash);

/// Brings an entry back, with the children that went with it.
#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreEntryReq {
	pub id: i64,
}

request!(RestoreEntryReq, POST "/trash/entries/{id}/restore" => ());

#[derive(Debug, Serialize, Deserialize)]
pub struct PurgeEntryReq {
	pub id: i64,
}

request!(PurgeEntryReq, DELETE "/trash/entries/{id}" => ());

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreAssetReq {
	pub id: i64,
}

request!(RestoreAssetReq, POST "/trash/assets/{id}/restore" => ());

#[derive(Debug, Serialize, Deserialize)]
pub struct PurgeAssetReq {
	pub id: i64,
}

request!(PurgeAssetReq, DELETE "/trash/assets/{id}" => ());

/// Deletes for good all the user may delete of the trash.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyReq {}

request!(EmptyReq, DELETE "/trash" => ());
