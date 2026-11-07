use axum::http::StatusCode;
use serde_json::json;

use super::api::*;
use crate::assets::Asset;
use crate::assets::api::{DeleteReq as DeleteAsset, ListReq as ListAssets};
use crate::entries::api::{CreateReq, DeleteReq, GetReq};
use crate::error::Error;
use crate::fixtures::site_with_roles;
use crate::testing::input;
use crate::testing::{Client, TestServer};

#[tokio::test]
async fn who_may_delete_restores() {
	let ts = TestServer::new(site_with_roles());
	let (author, editor) = (ts.user(Some("author")), ts.user(Some("editor")));
	let live = input("events", "event", "Jazz", json!({}));
	let id = ts.admin().call(&CreateReq(live)).await.unwrap().id;

	editor.call(&DeleteReq { id }).await.unwrap();
	let get = GetReq { id, site: None };
	assert_eq!(editor.call(&get).await.unwrap_err(), Error::NotFound);

	// an author can't delete events, the trash shows them nothing
	let trash = author.call(&TrashReq {}).await.unwrap();
	assert!(trash.entries.is_empty() && trash.assets.is_empty());
	let restore = RestoreEntryReq { id };
	assert_eq!(author.call(&restore).await.unwrap_err(), Error::Forbidden);

	let trash = editor.call(&TrashReq {}).await.unwrap();
	let by = trash.entries[0].deleted_by.clone().unwrap_or_default();
	assert!(by.starts_with("editor"), "{by}");
	editor.call(&restore).await.unwrap();
	assert_eq!(editor.call(&get).await.unwrap().entry.title, "Jazz");
	// only what is in the trash
	let purge = PurgeEntryReq { id };
	assert_eq!(editor.call(&purge).await.unwrap_err(), Error::NotFound);
}

async fn file(client: &Client, path: &str) -> StatusCode {
	client.get(path).await.status()
}

#[tokio::test]
async fn assets_go_to_the_trash() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let resp = editor.upload("/assets", &[("a.txt", b"a")]).await;
	let a: Asset = Client::json::<Vec<Asset>>(resp).await.unwrap().remove(0);
	let url = a.url(None);

	editor.call(&DeleteAsset { id: a.id }).await.unwrap();
	assert!(
		editor
			.call(&ListAssets::default())
			.await
			.unwrap()
			.is_empty()
	);
	assert_eq!(file(&ts.anonymous(), &url).await, StatusCode::NOT_FOUND);
	let trash = editor.call(&TrashReq {}).await.unwrap();
	assert_eq!(trash.assets[0].asset.id, a.id);

	editor.call(&RestoreAssetReq { id: a.id }).await.unwrap();
	assert_eq!(file(&ts.anonymous(), &url).await, StatusCode::OK);

	// emptied: gone for good
	editor.call(&DeleteAsset { id: a.id }).await.unwrap();
	editor.call(&EmptyReq {}).await.unwrap();
	let restore = RestoreAssetReq { id: a.id };
	assert_eq!(editor.call(&restore).await.unwrap_err(), Error::NotFound);
	let trash = editor.call(&TrashReq {}).await.unwrap();
	assert!(trash.assets.is_empty());
}

#[tokio::test]
async fn own_access_restores_only_own_entries() {
	let schema = site_with_roles().role(
		crate::schema::Role::new("member", "Member")
			.own("events", crate::schema::Access::Delete),
	);
	let ts = TestServer::new(schema);
	let (admin, member) = (ts.admin(), ts.user(Some("member")));
	let theirs = input("events", "event", "Jazz", json!({}));
	let theirs = admin.call(&CreateReq(theirs)).await.unwrap().id;
	let mine = input("events", "event", "Rock", json!({}));
	let mine = member.call(&CreateReq(mine)).await.unwrap().id;
	admin.call(&DeleteReq { id: theirs }).await.unwrap();
	member.call(&DeleteReq { id: mine }).await.unwrap();

	let trash = member.call(&TrashReq {}).await.unwrap();
	let ids: Vec<i64> = trash.entries.iter().map(|e| e.id).collect();
	assert_eq!(ids, [mine]);
	let restore = RestoreEntryReq { id: theirs };
	assert_eq!(member.call(&restore).await.unwrap_err(), Error::Forbidden);
	member.call(&RestoreEntryReq { id: mine }).await.unwrap();
}
