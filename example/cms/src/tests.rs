//! The example's content model and roles, through the api like the
//! studio uses it.

use serde_json::{Value, json};
use zipp::drafts::SiteDoc;
use zipp::drafts::api::{PublishReq, PutReq};
use zipp::entries::api::{CreateReq, UpdateReq};
use zipp::error::Error;
use zipp::testing::{Client, TestServer, input};

use crate::content::schema;
use crate::routes::routes;

fn create(section: &str, ty: &str, title: &str, fields: Value) -> CreateReq {
	CreateReq(input(section, ty, title, fields))
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
	let mut live = SiteDoc::new(&jazz.ty, &jazz.title, &jazz.slug, jazz.fields);
	live.published = true;
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
	let ts = TestServer::new(schema()).routes(routes());
	let admin = ts.admin();
	admin.call(&event("Gestern", "2000-01-01")).await.unwrap();
	admin.call(&event("Bald", "2099-01-01")).await.unwrap();

	let resp = ts.anonymous().get("/events.json").await;
	let events: Value = Client::json(resp).await.unwrap();
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
	let vars = json!({ "categories": concerts.id.to_string(), "siteId": 1 });
	let data = ts.graphql(query, vars).await;

	let titles = |list: &str| -> Vec<&str> {
		let list = data["data"][list].as_array();
		let list = list.unwrap_or_else(|| panic!("{data}"));
		list.iter().map(|e| e["title"].as_str().unwrap()).collect()
	};
	assert_eq!(titles("articles"), ["Jazz im Park"]);
	assert_eq!(titles("categories"), ["Konzerte", "Verein"]);
}

#[tokio::test]
async fn tickets_are_a_group_of_the_event() {
	let ts = TestServer::new(schema());
	let mut jazz = event("Jazz", "2099-01-01");
	jazz.0.fields["tickets"] = json!({ "price": 25, "soldOut": true });
	ts.admin().call(&jazz).await.unwrap();
	ts.admin()
		.call(&event("Lesung", "2099-02-01"))
		.await
		.unwrap();

	// the query of the events block, the group is the block eventTickets
	let query = include_str!("../../crelte/svelte/src/queries/events.graphql");
	let data = ts.graphql(query, json!({ "siteId": 1 })).await;
	let events = &data["data"]["events"];
	assert_eq!(events[0]["tickets"], json!({ "soldOut": true }), "{data}");
	// without a value the group is there, like Craft's content block
	assert_eq!(events[1]["tickets"], json!({ "soldOut": null }));
}

/// A small png, 4 x 2.
const PNG: &[u8] = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x04\x00\x00\x00\x02\x08\x02\x00\x00\x00\xf0\xca\xea4\x00\x00\x00\x0eIDATx\x9cch@\x02\x0c\xc8\x1c\x00\x9c\x1a\x0c\x01<\x82sD\x00\x00\x00\x00IEND\xaeB`\x82";

#[tokio::test]
async fn pages_load_in_a_few_statements() {
	let ts = TestServer::new(schema());
	let admin = ts.admin();
	let resp = admin.upload("/assets", &vec![("a.png", PNG); 20]).await;
	let assets: Vec<zipp::assets::Asset> = Client::json(resp).await.unwrap();
	let image = |i: usize| json!([assets[i % assets.len()].id]);

	// a page with children, images, buttons and links in its text
	let about = create("pages", "page", "About", json!({}));
	let about = admin.call(&about).await.unwrap();
	let mut content = vec![];
	for i in 0..5 {
		let mut child =
			create("pages", "page", &format!("Child {i}"), json!({}));
		child.0.parent_id = Some(about.id);
		let child = admin.call(&child).await.unwrap();
		let link = json!({ "type": "entry", "value": child.id });
		content.push(json!({ "type": "button", "fields": { "link": link } }));
		let images: Vec<i64> = (0..4).map(|j| assets[i * 4 + j].id).collect();
		content
			.push(json!({ "type": "images", "fields": { "images": images } }));
		let text = json!({ "type": "doc", "content": [
			{ "type": "paragraph", "content": [{ "type": "text", "text": "more",
				"marks": [{ "type": "link", "attrs": { "link": link } }] }] },
			{ "type": "button", "attrs": { "link": link } },
		] });
		content.push(json!({ "type": "text", "fields": { "text": text } }));
	}
	let update = UpdateReq {
		id: about.id,
		input: input(
			"pages",
			"page",
			"About",
			json!({ "content": content, "hero": image(0) }),
		),
	};
	admin.call(&update).await.unwrap();

	// articles with categories, events with images and a page
	let category = |title| create("categories", "category", title, json!({}));
	let mut categories = vec![];
	for t in ["Konzerte", "Verein", "Kultur"] {
		categories.push(admin.call(&category(t)).await.unwrap().id);
	}
	for i in 0..30 {
		let fields = json!({ "categories": [categories[i % 3], categories[(i + 1) % 3]],
			"hero": image(i) });
		admin
			.call(&create("news", "article", &format!("Article {i}"), fields))
			.await
			.unwrap();
		let mut e = event(&format!("Event {i}"), "2099-01-01");
		e.0.fields["image"] = image(i);
		e.0.fields["page"] = json!([about.id]);
		admin.call(&e).await.unwrap();
	}

	let count = |query: &'static str, vars: Value| {
		let ts = &ts;
		async move {
			let (data, n) = ts.statements(ts.graphql(query, vars)).await;
			assert!(data["errors"].is_null(), "{data}");
			(data["data"].clone(), n)
		}
	};
	let entry = include_str!("../../crelte/svelte/src/queries/entry.graphql");
	let events = include_str!("../../crelte/svelte/src/queries/events.graphql");
	let articles =
		include_str!("../../crelte/svelte/src/queries/articles.graphql");
	let (data, page) =
		count(entry, json!({ "uri": "about", "siteId": 1 })).await;
	let about = &data["entry"];
	assert_eq!(about["children"].as_array().unwrap().len(), 5);
	assert_eq!(about["content"][0]["link"]["url"], "/about/child-0");
	let images = &about["content"][4]["images"];
	assert_eq!(
		images[3]["url"],
		format!("/assets/{}/medium/a.png", assets[7].id)
	);
	let text = &about["content"][5]["text"]["chunks"];
	assert_eq!(text[1]["buttons"][0]["url"], "/about/child-1");
	assert_eq!(about["hero"][0]["width"], 4);

	let (data, events) = count(events, json!({ "siteId": 1 })).await;
	assert_eq!(data["events"].as_array().unwrap().len(), 30);
	assert!(data["events"][29]["image"][0]["url"].is_string());
	let (data, articles) = count(articles, json!({ "siteId": 1 })).await;
	let last = &data["articles"][29];
	assert_eq!(last["title"], "Article 0");
	assert_eq!(last["categories"][1]["title"], "Verein");
	// 66, 32 and 94 when each relation, asset and link had its query
	assert!(page <= 30, "entry.graphql runs {page} statements");
	assert!(events <= 10, "events.graphql runs {events} statements");
	assert!(
		articles <= 10,
		"articles.graphql runs {articles} statements"
	);
}
