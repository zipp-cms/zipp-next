use axum::body::Body;
use axum::http::{Request, StatusCode, header};

use super::api::*;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::TestServer;

#[tokio::test]
async fn the_schema_for_the_user() {
	let ts = TestServer::new(site_with_roles());
	let schema = ts
		.user(Some("author"))
		.call_json(&SchemaReq {})
		.await
		.unwrap();
	// who is logged in comes with the session, see auth
	assert!(schema.get("me").is_none());
	assert_eq!(schema["roles"].as_array().unwrap().len(), 2);
	assert_eq!(schema["sites"][0]["handle"], "de");
	// no key, no AI
	assert!(schema["aiModels"].as_array().unwrap().is_empty());

	let err = ts.anonymous().call_json(&SchemaReq {}).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn serves_the_studio() {
	let ts = TestServer::new(site_with_roles());
	let get = |path: &str| Request::get(path).body(Body::empty()).unwrap();
	let anon = ts.anonymous();

	// every page of the studio is its index.html, nobody else frames it
	let resp = anon.send(get("/admin/entries/pages")).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let csp = resp.headers()[header::CONTENT_SECURITY_POLICY].to_str();
	assert_eq!(csp.unwrap().trim(), "frame-ancestors 'self'");

	let resp = anon.send(get("/")).await;
	assert_eq!(resp.status(), StatusCode::SEE_OTHER);
	let resp = anon.send(get("/nothing")).await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
	let resp = anon.send(get("/_fields/nothing.js")).await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
