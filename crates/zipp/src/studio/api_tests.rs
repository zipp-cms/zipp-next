use axum::body::Body;
use axum::http::{Request, StatusCode, header};

use super::api::*;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::schema::{Condition, Field, Section};
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

	// every page of the studio is its index.html, scripts only of the
	// cms, nobody else frames it
	let resp = anon.send(get("/admin/entries/pages")).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let csp = resp.headers()[header::CONTENT_SECURITY_POLICY].to_str();
	let csp = csp.unwrap();
	assert!(
		csp.starts_with("default-src 'self'; script-src 'self';"),
		"{csp}"
	);
	assert!(csp.contains("object-src 'none'; base-uri 'none';"), "{csp}");
	assert!(csp.contains("frame-src 'self';"), "{csp}");
	assert!(csp.ends_with("frame-ancestors 'self'"), "{csp}");

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

#[tokio::test]
async fn the_schema_has_the_tabs_of_blocks() {
	let schema = site_with_roles()
		.block_tabs(
			"teaser",
			"Teaser",
			[
				("Content", vec![Field::text("heading")]),
				("Settings", vec![Field::boolean("wide")]),
			],
		)
		.tab_when("teaser", "Settings", Condition::filled("heading"));
	let ts = TestServer::new(schema);
	let resp = ts.user(Some("author")).call_json(&SchemaReq {}).await;
	let comps = resp.unwrap()["components"].as_array().unwrap().clone();
	let teaser = comps.iter().find(|c| c["handle"] == "teaser").unwrap();
	assert_eq!(teaser["entry"], false);
	assert_eq!(teaser["tabs"][0]["name"], "Content");
	assert_eq!(teaser["tabs"][0]["fields"], serde_json::json!(["heading"]));
	assert_eq!(teaser["tabs"][1]["fields"], serde_json::json!(["wide"]));
	assert_eq!(teaser["tabs"][1]["when"]["field"], "heading");
}

#[tokio::test]
async fn the_studio_frames_the_sites_and_its_ancestors_frame_it() {
	let mut ts = TestServer::new(site_with_roles());
	ts.state.site_urls = std::sync::Arc::new(
		[
			("de".to_string(), "https://example.ch/de".to_string()),
			("en".to_string(), "/en".to_string()),
		]
		.into(),
	);
	ts.state.frame_ancestors =
		std::sync::Arc::new(vec!["https://dash.example.com".into()]);
	// builds the app again with the state
	let ts = ts.dev();
	let req = Request::get("/admin").body(Body::empty()).unwrap();
	let resp = ts.anonymous().send(req).await;
	let csp = resp.headers()[header::CONTENT_SECURITY_POLICY].to_str();
	let csp = csp.unwrap();
	// a relative url is the cms itself
	assert!(
		csp.contains("frame-src 'self' https://example.ch;"),
		"{csp}"
	);
	assert!(
		csp.ends_with("frame-ancestors 'self' https://dash.example.com"),
		"{csp}"
	);
}

#[test]
fn origins_of_urls() {
	use crate::origin;
	assert_eq!(origin("https://example.ch/de"), Some("https://example.ch"));
	assert_eq!(
		origin("http://localhost:5173"),
		Some("http://localhost:5173")
	);
	assert_eq!(origin("https://*.example.ch"), Some("https://*.example.ch"));
	assert_eq!(origin("/de"), None);
	assert_eq!(origin("javascript://x"), None);
	// nothing that would end the header's directive
	assert_eq!(origin("https://a.ch; script-src *"), None);
	assert_eq!(origin("https://a.ch'"), None);
}
