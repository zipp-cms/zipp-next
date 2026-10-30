use axum::body::Body;
use axum::http::{Request, header};

use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::{Client, TestServer};

async fn chat(client: &Client) -> Result<(), Error> {
	let req = Request::post("/api/ai/chat")
		.header(header::CONTENT_TYPE, "application/json")
		.body(Body::from(r#"{"messages": []}"#))
		.unwrap();
	let resp = client.send(req).await;
	Client::json::<serde_json::Value>(resp).await.map(|_| ())
}

#[tokio::test]
async fn only_roles_with_ai_chat() {
	let ts = TestServer::new(site_with_roles());
	let err = chat(&ts.user(Some("author"))).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	let err = chat(&ts.anonymous()).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);

	// the editor may, but the test server has no api key
	let err = chat(&ts.user(Some("editor"))).await.unwrap_err();
	assert!(matches!(err, Error::Request(m) if m.contains("no api key")));
}
