use axum::body::{Body, to_bytes};
use axum::http::{Request, header};
use serde_json::{Value, json};

use super::api::CreateTokenReq;
use crate::SiteUrls;
use crate::entries::api::CreateReq;
use crate::fixtures::site_with_roles;
use crate::schema::{Field, UserPages};
use crate::testing::{TestServer, input};
use crate::users::profile::set_fields;

/// The test site with cooks: users with a live event have a page.
fn server() -> TestServer {
	let schema = site_with_roles()
		.user_fields([Field::text("bio").multiline(), Field::images("photo")])
		.user_pages(
			UserPages::new("cooks/{slug}")
				.site_uri("en", "chefs/{slug}")
				.with_entries_in("events"),
		);
	let urls = SiteUrls::from([("de".into(), "https://x.ch".into())]);
	TestServer::new(schema).site_urls(urls)
}

/// A user of a role with a name and a bio, and their entry: a live one
/// or not.
async fn cook(
	ts: &TestServer,
	role: Option<&str>,
	name: &str,
	section: &str,
	live: bool,
) -> i64 {
	let client = ts.user(role);
	let ty = if section == "events" { "event" } else { "page" };
	let mut entry = input(section, ty, name, json!({}));
	entry.published = live;
	let e = client.call(&CreateReq(entry)).await.unwrap();
	let id = e.author_id.unwrap();
	let conn = ts.state.db.conn();
	crate::users::set_name(&conn, id, name).unwrap();
	set_fields(&conn, id, &json!({ "bio": format!("{name} cooks") })).unwrap();
	id
}

/// A query with a token of a frontend, the whole answer.
async fn as_token(ts: &TestServer, token: &str, query: &str) -> Value {
	let body = json!({ "query": query });
	let req = Request::post("/api/graphql")
		.header(header::CONTENT_TYPE, "application/json")
		.header(header::AUTHORIZATION, format!("Bearer {token}"))
		.body(Body::from(body.to_string()))
		.unwrap();
	let resp = ts.anonymous().send(req).await;
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	serde_json::from_slice(&body).unwrap()
}

async fn token(ts: &TestServer, schema: &str) -> String {
	let req = CreateTokenReq {
		name: "Website".into(),
		schema: schema.into(),
		expires_at: None,
	};
	ts.admin().call(&req).await.unwrap().token
}

#[tokio::test]
async fn users_with_live_entries_are_public() {
	let ts = server();
	let mia = cook(&ts, Some("editor"), "Mia", "events", true).await;
	cook(&ts, Some("editor"), "Sam", "events", false).await;
	let admin = cook(&ts, None, "Ada", "pages", true).await;

	// only Mia has a live event; no email, role or logins
	let query = "{ users(hasEntries: { section: \"events\" }) {
		id name slug uri url bio } userCount }";
	let body = ts.graphql(query, json!({})).await;
	let users = json!([{
		"id": mia.to_string(),
		"name": "Mia",
		"slug": "mia",
		"uri": "cooks/mia",
		"url": "https://x.ch/cooks/mia",
		"bio": "Mia cooks",
	}]);
	assert_eq!(body["data"]["users"], users, "{body}");
	assert_eq!(body["data"]["userCount"], 1);
	// a page of another section doesn't make Ada public, it isn't the
	// section of the user pages
	let body = ts.graphql("{ users { name } }", json!({})).await;
	assert_eq!(body["data"]["users"], json!([{ "name": "Mia" }]), "{body}");
	for private in ["email", "role", "lastLoginAt"] {
		let query = format!("{{ users {{ {private} }} }}");
		let body = ts.graphql(&query, json!({})).await;
		assert!(body["errors"][0]["message"].is_string(), "{body}");
	}

	// one by slug or id
	let query = "{ user(slug: \"mia\") { name url } }";
	let body = ts.graphql(query, json!({})).await;
	assert_eq!(
		body["data"]["user"],
		json!({ "name": "Mia", "url": "https://x.ch/cooks/mia" })
	);
	for query in [
		"{ user(slug: \"sam\") { name } }".to_string(),
		format!("{{ user(id: {admin}) {{ name }} }}"),
	] {
		let body = ts.graphql(&query, json!({})).await;
		assert_eq!(body["data"]["user"], Value::Null, "{body}");
	}
	let query = format!("{{ user(id: {mia}) {{ name }} }}");
	let body = ts.graphql(&query, json!({})).await;
	assert_eq!(body["data"]["user"]["name"], "Mia");

	// the author of an entry is the same user
	let query = "{ entries(section: \"events\") {
		... on event_Entry { author { name slug uri bio } } } }";
	let body = ts.graphql(query, json!({})).await;
	let author = json!({
		"name": "Mia", "slug": "mia", "uri": "cooks/mia", "bio": "Mia cooks"
	});
	assert_eq!(body["data"]["entries"], json!([{ "author": author }]));
	// Ada's page names her, she has no profile there
	let query = "{ entries(section: \"pages\") {
		... on page_Entry { author { name slug url bio } } } }";
	let body = ts.graphql(query, json!({})).await;
	let author =
		json!({ "name": "Ada", "slug": null, "url": null, "bio": null });
	assert_eq!(body["data"]["entries"], json!([{ "author": author }]));
}

#[tokio::test]
async fn tokens_need_the_users_scope() {
	let ts = server();
	cook(&ts, Some("editor"), "Mia", "events", true).await;
	let without = token(&ts, "live").await;

	// no users, the author only by name
	let query = "{ users { name } userCount user(slug: \"mia\") { name }
		entries(section: \"events\") {
			... on event_Entry { author { name slug bio } } } }";
	let body = as_token(&ts, &without, query).await;
	assert_eq!(
		body["data"],
		json!({
			"users": [],
			"userCount": 0,
			"user": null,
			"entries": [{ "author": { "name": "Mia", "slug": null, "bio": null } }],
		}),
		"{body}"
	);

	// with the scope, of the sections it reads
	let with = token(&ts, "website").await;
	let body = as_token(&ts, &with, "{ users { name } }").await;
	assert_eq!(body["data"]["users"], json!([{ "name": "Mia" }]));
	let pages = token(&ts, "pages").await;
	let body = as_token(&ts, &pages, "{ users { name } }").await;
	assert_eq!(body["data"]["users"], json!([]));
}

#[tokio::test]
async fn users_are_ordered_and_paged() {
	let ts = server();
	let zoe = cook(&ts, Some("editor"), "Zoe", "events", true).await;
	let ann = cook(&ts, Some("editor"), "Ann", "events", true).await;

	let names = |body: &Value| -> Vec<String> {
		let users = body["data"]["users"].as_array().unwrap();
		users.iter().map(|u| u["name"].to_string()).collect()
	};
	let body = ts.graphql("{ users { name } }", json!({})).await;
	assert_eq!(names(&body), ["\"Ann\"", "\"Zoe\""]);
	let query = "{ users(orderBy: \"name DESC\", limit: 1) { name } }";
	let body = ts.graphql(query, json!({})).await;
	assert_eq!(names(&body), ["\"Zoe\""]);
	let query = "{ users(orderBy: \"id\", offset: 1) { id } }";
	let body = ts.graphql(query, json!({})).await;
	assert_eq!(body["data"]["users"], json!([{ "id": ann.to_string() }]));
	assert!(zoe < ann);
	let body = ts
		.graphql("{ users(orderBy: \"email\") { id } }", json!({}))
		.await;
	assert!(body["errors"][0]["message"].is_string(), "{body}");
}
