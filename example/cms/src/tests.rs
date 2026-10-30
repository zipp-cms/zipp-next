//! The example's content model and roles, through the api like the
//! studio uses it.

use serde_json::{Value, json};
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::http::{Request, header};
use zipp::drafts::SiteDoc;
use zipp::drafts::api::{PublishReq, PutReq};
use zipp::entries::EntryInput;
use zipp::entries::api::CreateReq;
use zipp::error::Error;
use zipp::testing::TestServer;

use super::*;

fn create(section: &str, ty: &str, title: &str, fields: Value) -> CreateReq {
	CreateReq(EntryInput {
		dates: Default::default(),
		section: section.into(),
		ty: ty.into(),
		site: None,
		title: title.into(),
		slug: String::new(),
		published: true,
		parent_id: None,
		fields,
	})
}

fn event(title: &str, date: &str) -> CreateReq {
	let fields = json!({ "date": date, "city": "Bern", "genre": "concert" });
	create("events", "event", title, fields)
}

#[tokio::test]
async fn authors_write_editors_publish() {
	let ts = TestServer::new(schema());
	let author = ts.user(Some("author"));

	// an author's event waits as a draft for someone who may publish
	let jazz = author.call(&event("Jazz", "2099-01-01")).await.unwrap();
	assert!(!jazz.published);
	let live = SiteDoc {
		ty: jazz.ty,
		dates: Default::default(),
		title: jazz.title,
		slug: jazz.slug,
		published: true,
		fields: jazz.fields,
	};
	let draft = PutReq {
		id: jazz.id,
		site: "de".into(),
		base: None,
		content: live,
	};
	author.call(&draft).await.unwrap();
	let publish = PublishReq { id: jazz.id };
	assert_eq!(author.call(&publish).await.unwrap_err(), Error::Forbidden);
	ts.user(Some("editor")).call(&publish).await.unwrap();

	// what an event needs
	let mut bad = event("No date", "");
	bad.0.fields["genre"] = json!("opera");
	let Error::Invalid(fields) = author.call(&bad).await.unwrap_err() else {
		panic!("not invalid");
	};
	let paths: Vec<_> = fields.iter().map(|f| f.path.as_str()).collect();
	assert_eq!(paths, ["date", "genre"]);
}

#[tokio::test]
async fn upcoming_events_as_json() {
	let ts = TestServer::new(schema())
		.routes(Router::new().route("/events.json", get(events)));
	let admin = ts.admin();
	admin.call(&event("Gestern", "2000-01-01")).await.unwrap();
	admin.call(&event("Bald", "2099-01-01")).await.unwrap();

	let req = Request::get("/events.json").body(Body::empty()).unwrap();
	let resp = ts.anonymous().send(req).await;
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	let events: Value = serde_json::from_slice(&body).unwrap();
	assert_eq!(events[0]["title"], "Bald");
	assert_eq!(events.as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn the_news_page_filters_by_category() {
	let ts = TestServer::new(schema());
	let admin = ts.admin();
	let category = |title| create("categories", "category", title, json!({}));
	let concerts = admin.call(&category("Konzerte")).await.unwrap();
	let club = admin.call(&category("Verein")).await.unwrap();
	let article = |title, category: i64| {
		let fields = json!({ "categories": [category] });
		create("news", "article", title, fields)
	};
	admin
		.call(&article("Jazz im Park", concerts.id))
		.await
		.unwrap();
	admin
		.call(&article("Neuer Vorstand", club.id))
		.await
		.unwrap();

	// the query of the website's news page, `?category=<id>`
	let query =
		include_str!("../../crelte/svelte/src/queries/articles.graphql");
	let body = json!({
		"query": query,
		"variables": { "categories": concerts.id.to_string(), "siteId": 1 },
	});
	let req = Request::post("/api/graphql")
		.header(header::CONTENT_TYPE, "application/json")
		.header(header::AUTHORIZATION, "Bearer test-token")
		.body(Body::from(body.to_string()))
		.unwrap();
	let resp = ts.anonymous().send(req).await;
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	let data: Value = serde_json::from_slice(&body).unwrap();

	let titles = |list: &str| -> Vec<&str> {
		let list = data["data"][list].as_array();
		let list = list.unwrap_or_else(|| panic!("{data}"));
		list.iter().map(|e| e["title"].as_str().unwrap()).collect()
	};
	assert_eq!(titles("articles"), ["Jazz im Park"]);
	assert_eq!(titles("categories"), ["Konzerte", "Verein"]);
}
