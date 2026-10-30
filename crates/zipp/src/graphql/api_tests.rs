use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::{Value, json};

use crate::drafts::SiteDoc;
use crate::drafts::api::{PreviewReq, PutReq};
use crate::entries::api::CreateReq;
use crate::entries::tests::input;
use crate::schema::tests::site_with_roles;
use crate::testing::{Client, TestServer};

const TITLES: &str =
	r#"{"query": "{ entries(section: \"events\") { title } }"}"#;

/// The answer of GraphQL, with the endpoint token or without.
async fn graphql(
	client: &Client,
	path: &str,
	body: &str,
	bearer: bool,
) -> (StatusCode, Value) {
	let mut req = Request::post(path)
		.header(header::CONTENT_TYPE, "application/json")
		.body(Body::from(body.to_string()))
		.unwrap();
	if bearer {
		let token = "Bearer test-token".parse().unwrap();
		req.headers_mut().insert(header::AUTHORIZATION, token);
	}
	let resp = client.send(req).await;
	let status = resp.status();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	(status, serde_json::from_slice(&body).unwrap())
}

fn titles(data: &Value) -> Vec<&str> {
	data["data"]["entries"]
		.as_array()
		.unwrap()
		.iter()
		.map(|e| e["title"].as_str().unwrap())
		.collect()
}

#[tokio::test]
async fn private_like_crafts() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let mut draft = input("events", "event", "Hidden", json!({}));
	draft.published = false;
	admin.call(&CreateReq(draft)).await.unwrap();
	let live = input("events", "event", "Jazz", json!({}));
	admin.call(&CreateReq(live)).await.unwrap();
	let anon = ts.anonymous();

	let (status, body) = graphql(&anon, "/api/graphql", TITLES, false).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	assert_eq!(body["error"]["type"], "MISSING_ENDPOINT_TOKEN");

	// a frontend sees what is published, an admin everything
	let (status, body) = graphql(&anon, "/api/graphql", TITLES, true).await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(titles(&body), ["Jazz"]);
	let (_, body) = graphql(&admin, "/api/graphql", TITLES, false).await;
	assert_eq!(titles(&body).len(), 2);

	// a role reads it like a frontend
	let author = ts.user(Some("author"));
	let (status, _) = graphql(&author, "/api/graphql", TITLES, false).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	let (_, body) = graphql(&author, "/api/graphql", TITLES, true).await;
	assert_eq!(titles(&body), ["Jazz"]);

	// the playground only in development
	let req = Request::get("/api/graphql").body(Body::empty()).unwrap();
	assert_eq!(anon.send(req).await.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn preview_tokens_show_the_draft() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let live = input("events", "event", "Jazz", json!({}));
	let id = admin.call(&CreateReq(live)).await.unwrap().id;
	let put = PutReq {
		id,
		site: "de".into(),
		base: None,
		content: SiteDoc {
			title: "Jazz am Abend".into(),
			slug: "jazz".into(),
			published: true,
			fields: json!({}),
		},
	};
	admin.call(&put).await.unwrap();
	let token = admin.call(&PreviewReq { id }).await.unwrap().token;

	let anon = ts.anonymous();
	let (_, body) = graphql(&anon, "/api/graphql", TITLES, true).await;
	assert_eq!(titles(&body), ["Jazz"]);
	let path = format!("/api/graphql?token={token}");
	let (_, body) = graphql(&anon, &path, TITLES, true).await;
	assert_eq!(titles(&body), ["Jazz am Abend"]);

	// an unknown or expired token shows what is live
	let wrong = "/api/graphql?token=wrong";
	let (_, body) = graphql(&anon, wrong, TITLES, true).await;
	assert_eq!(titles(&body), ["Jazz"]);
}
