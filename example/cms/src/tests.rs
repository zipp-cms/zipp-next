//! The example's content model and roles, through the api like the
//! studio uses it.

use serde_json::{Value, json};
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::http::Request;
use zipp::drafts::SiteDoc;
use zipp::drafts::api::{PublishReq, PutReq};
use zipp::entries::EntryInput;
use zipp::entries::api::CreateReq;
use zipp::error::Error;
use zipp::testing::TestServer;

use super::*;

fn event(title: &str, date: &str) -> CreateReq {
	CreateReq(EntryInput {
		section: "events".into(),
		ty: "event".into(),
		site: None,
		title: title.into(),
		slug: String::new(),
		published: true,
		parent_id: None,
		fields: json!({ "date": date, "city": "Bern", "genre": "concert" }),
	})
}

#[tokio::test]
async fn authors_write_editors_publish() {
	let ts = TestServer::new(schema());
	let author = ts.user(Some("author"));

	// an author's event waits as a draft for someone who may publish
	let jazz = author.call(&event("Jazz", "2099-01-01")).await.unwrap();
	assert!(!jazz.published);
	let live = SiteDoc {
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
