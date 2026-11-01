use axum::body::Body;
use axum::http::{Request, StatusCode, header};

use super::api::*;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::schema::{Field, Section};
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
	// no icons in this schema
	assert_eq!(schema["icons"], serde_json::json!({}));
	// the studio lays out a note with its text and width
	let event = schema["components"]
		.as_array()
		.unwrap()
		.iter()
		.find(|c| c["handle"] == "event")
		.unwrap();
	let note = &event["fields"].as_array().unwrap().last().unwrap();
	assert_eq!(note["kind"], "note");
	assert_eq!(note["text"], "Images in **landscape**.");
	assert_eq!(note["width"], 50);

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

#[tokio::test]
async fn the_schema_brings_its_lucide_icons() {
	let schema = site_with_roles()
		.icon("event", "calendar")
		.icon("page", "calendar")
		.icon("text", crate::Icon::svg("<svg/>"))
		.icon("button", "no-such-icon");
	let ts = TestServer::new(schema);
	let resp = ts.user(None).call_json(&SchemaReq {}).await.unwrap();
	let icons = resp["icons"].as_object().unwrap();
	// once each, own svgs and unknown names aren't there
	assert_eq!(icons.keys().collect::<Vec<_>>(), ["calendar"]);
	let svg = icons["calendar"].as_str().unwrap();
	assert!(svg.starts_with("<svg") && svg.ends_with("</svg>"), "{svg}");
}

#[tokio::test]
async fn the_schema_says_which_field_names_a_block() {
	let schema = site_with_roles().block_title("button", "label");
	let ts = TestServer::new(schema);
	let resp = ts.user(Some("author")).call_json(&SchemaReq {}).await;
	let comps = resp.unwrap()["components"].as_array().unwrap().clone();
	let title = |h: &str| {
		comps.iter().find(|c| c["handle"] == h).unwrap()["title_field"].clone()
	};
	assert_eq!(title("button"), "label");
	// without one the block shows its type
	assert!(title("text").is_null());
}

#[tokio::test]
async fn the_schema_says_which_tabs_are_cards() {
	let schema = site_with_roles()
		.section(Section::channel("posts", ["post"]).uri("posts/{slug}"))
		.entry_tabs(
			"post",
			"Post",
			[
				("Intro", vec![Field::text("lead")]),
				("Content", vec![Field::markdown("body")]),
			],
		)
		.tab_card("post", "Intro")
		.entry_card("event");
	let ts = TestServer::new(schema);
	let resp = ts.user(Some("author")).call_json(&SchemaReq {}).await;
	let comps = resp.unwrap()["components"].as_array().unwrap().clone();
	let comp = |h: &str| comps.iter().find(|c| c["handle"] == h).unwrap();
	assert_eq!(comp("post")["tabs"][0]["card"], true);
	assert!(comp("post")["tabs"][1]["card"].is_null());
	assert!(comp("post")["card"].is_null());
	assert_eq!(comp("event")["card"], true);
}
