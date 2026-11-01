use serde_json::json;

use super::api::*;
use crate::entries::api::{CreateReq, DeleteReq};
use crate::entries::tests::input;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::TestServer;

#[tokio::test]
async fn finds_what_each_role_may_view() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let story = json!({ "intro": "The garden opens in spring" });
	let page = admin
		.call(&CreateReq(input("pages", "page", "Garden", story)))
		.await
		.unwrap();
	let event = input("events", "event", "Garden party", json!({}));
	let event = admin.call(&CreateReq(event)).await.unwrap();

	let req = |q: &str| SearchReq {
		q: q.into(),
		site: Some("en".into()),
	};
	let found = admin.call(&req("gard")).await.unwrap();
	let ids: Vec<i64> = found.entries.iter().map(|e| e.id).collect();
	assert_eq!(ids.len(), 2);
	assert!(ids.contains(&page.id) && ids.contains(&event.id));
	let found = admin.call(&req("spring")).await.unwrap();
	assert_eq!(
		found.entries[0].path,
		format!("/admin/entries/pages/{}", page.id)
	);
	assert_eq!(
		found.entries[0].snippet.as_deref(),
		Some("The garden opens in spring")
	);

	// an author views pages and events, a deleted one is gone
	let author = ts.user(Some("author"));
	assert_eq!(author.call(&req("garden")).await.unwrap().entries.len(), 2);
	admin.call(&DeleteReq { id: event.id }).await.unwrap();
	let found = author.call(&req("garden")).await.unwrap();
	assert_eq!(found.entries.len(), 1);

	let err = ts.anonymous().call(&req("garden")).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}
