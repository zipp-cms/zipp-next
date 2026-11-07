use axum::http::{StatusCode, header};

use super::api::*;
use crate::error::Error;
use crate::fixtures::site_with_roles;
use crate::schema::{ButtonOption, Condition, Field, Section};
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
	let anon = ts.anonymous();

	// every page of the studio is its index.html, scripts only of the
	// cms, nobody else frames it
	let resp = anon.get("/admin/entries/pages").await;
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

	let resp = anon.get("/").await;
	assert_eq!(resp.status(), StatusCode::SEE_OTHER);
	let resp = anon.get("/nothing").await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
	let resp = anon.get("/_fields/nothing.js").await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

/// A plugin with a module in the studio, `chat` one with data only.
struct Studio(&'static str);

impl crate::Plugin for Studio {
	fn name(&self) -> &'static str {
		self.0
	}

	fn studio(&self) -> Option<crate::PluginStudio> {
		Some(match self.0 {
			"chat" => crate::PluginStudio::default()
				.data(serde_json::json!({ "chat": true })),
			_ => crate::PluginStudio::new("export {}"),
		})
	}
}

#[tokio::test]
async fn plugins_with_and_without_a_module() {
	let schema = site_with_roles()
		.plugin(Studio("cards"))
		.plugin(Studio("chat"));
	let ts = TestServer::new(schema);
	let schema = ts.admin().call_json(&SchemaReq {}).await.unwrap();
	assert_eq!(
		schema["plugins"],
		serde_json::json!([
			{ "name": "cards", "module": true, "data": null },
			{ "name": "chat", "module": false, "data": { "chat": true } },
		])
	);

	let anon = ts.anonymous();
	let resp = anon.get("/_plugins/cards.js").await;
	assert_eq!(resp.status(), StatusCode::OK);
	let resp = anon.get("/_plugins/chat.js").await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

/// A field type of another crate with its editor.
struct Stars;

impl crate::FieldType for Stars {
	fn name(&self) -> &'static str {
		"stars"
	}

	fn validate(
		&self,
		_options: &serde_json::Value,
		value: &serde_json::Value,
	) -> Result<serde_json::Value, String> {
		Ok(value.clone())
	}

	fn graphql(&self) -> crate::GraphqlType {
		crate::GraphqlType::Scalar(crate::Scalar::Int)
	}

	fn studio(&self) -> crate::StudioModule {
		crate::StudioModule::new("export const mount = 1;")
	}
}

#[tokio::test]
async fn field_types_bring_their_editor() {
	let schema = site_with_roles().field_type(Stars).block(
		"review",
		"Review",
		[
			Field::custom("stars", "stars", serde_json::Value::Null),
			Field::button_group(
				"tone",
				[
					ButtonOption::from(("warm", "Warm", "#c05621")),
					ButtonOption::new("cold", "Cold").icon("snowflake"),
				],
			),
		],
	);
	let ts = TestServer::new(schema);
	let anon = ts.anonymous();
	let resp = anon.get("/_fields/stars.js").await;
	assert_eq!(resp.status(), StatusCode::OK);
	let js = crate::testing::Client::bytes(resp).await;
	assert_eq!(js, b"export const mount = 1;");

	// a button group is zipp's own, it has no module
	let resp = anon.get("/_fields/buttonGroup.js").await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
	let schema = ts.admin().call_json(&SchemaReq {}).await.unwrap();
	let review = schema["components"]
		.as_array()
		.unwrap()
		.iter()
		.find(|c| c["handle"] == "review")
		.unwrap();
	assert_eq!(
		review["fields"][1],
		serde_json::json!({
			"handle": "tone", "label": "Tone", "required": false,
			"indexed": false, "translatable": false, "kind": "buttonGroup",
			"options": [
				{ "value": "warm", "label": "Warm", "color": "#c05621" },
				{ "value": "cold", "label": "Cold",
					"icon": { "lucide": "snowflake" } },
			],
		})
	);
}

#[tokio::test]
async fn the_schema_brings_its_lucide_icons() {
	let schema = site_with_roles()
		.icon("event", "calendar")
		.icon("page", "calendar")
		.icon("text", crate::Icon::svg("<svg/>"))
		.icon("button", "no-such-icon")
		.block(
			"format",
			"Format",
			[Field::button_group(
				"format",
				[ButtonOption::new("pdf", "PDF").icon("file-text")],
			)],
		);
	let ts = TestServer::new(schema);
	let resp = ts.user(None).call_json(&SchemaReq {}).await.unwrap();
	let icons = resp["icons"].as_object().unwrap();
	// once each, own svgs and unknown names aren't there; those of
	// button groups' options too
	assert_eq!(icons.keys().collect::<Vec<_>>(), ["calendar", "file-text"]);
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
	let resp = ts.anonymous().get("/admin").await;
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

/// A website on a free port of 127.0.0.1: its page and icons, and how
/// often it was asked.
async fn website(
	page: String,
) -> (String, std::sync::Arc<std::sync::atomic::AtomicUsize>) {
	use axum::routing::get;
	use std::sync::atomic::{AtomicUsize, Ordering};

	let asked = std::sync::Arc::new(AtomicUsize::new(0));
	let count = asked.clone();
	let app = axum::Router::new()
		.route(
			"/",
			get(move || std::future::ready(axum::response::Html(page.clone()))),
		)
		.route(
			"/icon.png",
			get(|| async { ([(header::CONTENT_TYPE, "image/png")], "png") }),
		)
		.route(
			"/big.png",
			get(|| async {
				([(header::CONTENT_TYPE, "image/png")], vec![0; 300 * 1024])
			}),
		)
		// a website that answers every path with its page
		.route(
			"/favicon.ico",
			get(|| async { axum::response::Html("<p>page</p>") }),
		)
		.layer(axum::middleware::from_fn(
			move |req, next: axum::middleware::Next| {
				count.fetch_add(1, Ordering::SeqCst);
				next.run(req)
			},
		));
	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
	let port = listener.local_addr().unwrap().port();
	tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
	(format!("http://127.0.0.1:{port}/"), asked)
}

fn favicon_site(url: &str) -> TestServer {
	let urls = [("de".to_string(), url.to_string())].into_iter().collect();
	TestServer::new(site_with_roles()).site_urls(urls)
}

#[tokio::test]
async fn the_favicon_of_a_site() {
	use std::sync::atomic::Ordering;

	let (url, asked) =
		website(r#"<link rel="icon" href="icon.png" sizes="32x32">"#.into())
			.await;
	let ts = favicon_site(&url);

	// any user, the icon as the website answers it
	let resp = ts.user(Some("author")).get("/api/sites/de/favicon").await;
	assert_eq!(resp.status(), StatusCode::OK);
	assert_eq!(resp.headers()[header::CONTENT_TYPE], "image/png");
	assert_eq!(crate::testing::Client::bytes(resp).await, b"png");
	assert_eq!(asked.load(Ordering::SeqCst), 2);

	// kept: the website isn't asked again
	let resp = ts.admin().get("/api/sites/de/favicon").await;
	assert_eq!(resp.status(), StatusCode::OK);
	assert_eq!(asked.load(Ordering::SeqCst), 2);

	let resp = ts.anonymous().get("/api/sites/de/favicon").await;
	assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
	// a site without a url
	let resp = ts.admin().get("/api/sites/en/favicon").await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn no_favicon_of_another_host() {
	use std::sync::atomic::Ordering;

	// the same server under another name, then /favicon.ico, which is
	// a page, not an image
	let (url, asked) = website(String::new()).await;
	let port = url.rsplit(':').next().unwrap().trim_end_matches('/');
	let page =
		format!(r#"<link rel="icon" href="http://localhost:{port}/icon.png">"#);
	let (url, other) = website(page).await;
	let ts = favicon_site(&url);

	let resp = ts.admin().get("/api/sites/de/favicon").await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);
	assert_eq!(other.load(Ordering::SeqCst), 2);
	assert_eq!(asked.load(Ordering::SeqCst), 0);

	// none is kept too
	let resp = ts.admin().get("/api/sites/de/favicon").await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);
	assert_eq!(other.load(Ordering::SeqCst), 2);

	// too big
	let (url, _) = website(r#"<link rel="icon" href="/big.png">"#.into()).await;
	let resp = favicon_site(&url)
		.admin()
		.get("/api/sites/de/favicon")
		.await;
	assert_eq!(resp.status(), StatusCode::NO_CONTENT);
}
