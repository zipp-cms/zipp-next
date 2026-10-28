use serde_json::json;

use super::*;
use crate::entries::save;
use crate::entries::tests::input;
use crate::schema::tests::site;

async fn run(gql: &GqlSchema, query: &str, published_only: bool) -> Value {
	let req = async_graphql::Request::new(query).data(Viewer {
		published_only,
		preview: None,
		token: None,
	});
	let resp = gql.execute(req).await;
	assert!(resp.errors.is_empty(), "{:?}", resp.errors);
	resp.data.into_json().unwrap()
}

#[tokio::test]
async fn loads_a_page_with_its_relations() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();

	let (about, event) = {
		let conn = db.conn();
		let about =
			save(&conn, &s, None, input("pages", "page", "About", json!({})))
				.unwrap();
		let fields = json!({ "city": "Bern", "price": 12, "page": [about.id] });
		let event =
			save(&conn, &s, None, input("events", "event", "Jazz", fields))
				.unwrap();
		let body = json!({ "intro": "Hi", "body": [
			{ "type": "text", "fields": { "content": "Hello" } },
			{ "type": "button", "fields": { "label": "Go", "link": [about.id] } },
		]});
		let mut team = input("pages", "page", "Team", body);
		team.parent_id = Some(about.id);
		save(&conn, &s, None, team).unwrap();
		(about, event)
	};

	let data = run(
		&gql,
		r#"{
			entry(uri: "about/team", site: "en") {
				typeHandle
				title
				parent { title }
				... on page_Entry {
					intro
					body {
						typeHandle
						... on text_Block { content }
						... on button_Block { label link { title uri } }
					}
				}
			}
		}"#,
		true,
	)
	.await;
	assert_eq!(
		data["entry"],
		json!({
			"typeHandle": "page",
			"title": "Team",
			"parent": { "title": "About" },
			"intro": "Hi",
			"body": [
				{ "typeHandle": "text", "content": "Hello" },
				{ "typeHandle": "button", "label": "Go",
					"link": [{ "title": "About", "uri": "about" }] },
			],
		})
	);

	let data = run(
		&gql,
		r#"{
			entries(type: "event", where: [{ field: "price", op: "lt", value: "20" }]) {
				title
				... on event_Entry { city page { id } }
			}
			entryCount(section: "pages")
			about: entry(id: 0) { title }
			sites { handle primary }
		}"#,
		true,
	)
	.await;
	assert_eq!(
		data["entries"],
		json!([{ "title": "Jazz", "city": "Bern", "page": [{ "id": about.id }] }])
	);
	assert_eq!(data["entryCount"], 2);
	assert_eq!(data["about"], Value::Null);
	assert_eq!(
		data["sites"][1],
		json!({ "handle": "en", "primary": false })
	);
	let _ = event;
}

#[tokio::test]
async fn assets_with_transforms() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql = build(
		s.clone(),
		db.clone(),
		"https://cms.test".into(),
		SiteUrls::new(),
	)
	.unwrap();
	let dir = std::env::temp_dir()
		.join(format!("zipp-gql-{}", rand::random::<u32>()));

	let mut png = vec![];
	image::RgbImage::new(400, 200)
		.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
		.unwrap();
	{
		let conn = db.conn();
		let a = crate::assets::store(&conn, &dir, "hero.png", &png).unwrap();
		let fields = json!({ "city": "Bern", "image": [a.id] });
		save(&conn, &s, None, input("events", "event", "Jazz", fields))
			.unwrap();
	}

	let data = run(
		&gql,
		r#"{ entry(uri: "events/jazz") { ... on event_Entry { image {
			url
			small: url(transform: "small")
			width(transform: "small")
			height(transform: "small")
			sqHeight: height(transform: "square")
			srcset(transforms: ["small", "square"])
		} } } }"#,
		true,
	)
	.await;
	let img = &data["entry"]["image"][0];
	assert_eq!(img["url"], "https://cms.test/assets/1/hero.png");
	assert_eq!(img["small"], "https://cms.test/assets/1/small/hero.png");
	assert_eq!(
		(img["width"].clone(), img["height"].clone()),
		(json!(100), json!(50))
	);
	assert_eq!(img["sqHeight"], 50);
	assert_eq!(
		img["srcset"],
		"https://cms.test/assets/1/small/hero.png 100w, \
		https://cms.test/assets/1/square/hero.png 50w"
	);
	std::fs::remove_dir_all(dir).unwrap();
}

#[tokio::test]
async fn answers_crelte_like_craft() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let urls = SiteUrls::from([
		("de".to_string(), "https://x.ch".to_string()),
		("en".to_string(), "https://x.ch/en/".to_string()),
	]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	let first = crate::migrate::Migration {
		name: "0001".into(),
		ops: crate::migrate::diff(&Schema::new(), &s, &[]).unwrap(),
		schema: (*s).clone(),
	};
	crate::apply::apply_migrations(&db.conn(), &[first]).unwrap();

	let data = run(
		&gql,
		r#"{ header: globalSet(handle: "home", siteId: 2) { title } }"#,
		true,
	)
	.await;
	assert_eq!(data["header"]["title"], "Home");

	let data = run(
		&gql,
		r#"{ crelteSites { id baseUrl language name handle primary } }"#,
		true,
	)
	.await;
	assert_eq!(
		data["crelteSites"][1],
		json!({ "id": 2, "baseUrl": "https://x.ch/en/", "language": "en",
			"name": "English", "handle": "en", "primary": false })
	);

	// crelte's entry.graphql with its variables
	let req = async_graphql::Request::new(
		r#"query ($uri: [String], $siteId: [QueryArgument]) {
			entry(uri: $uri, siteId: $siteId) {
				id siteId url sectionHandle typeHandle title
			}
		}"#,
	)
	.variables(async_graphql::Variables::from_json(
		json!({ "uri": "__home__", "siteId": 2 }),
	))
	.data(Viewer {
		published_only: true,
		preview: None,
		token: None,
	});
	let resp = gql.execute(req).await;
	assert!(resp.errors.is_empty(), "{:?}", resp.errors);
	let data = resp.data.into_json().unwrap();
	assert_eq!(
		data["entry"],
		json!({ "id": 1, "siteId": 2, "url": "https://x.ch/en/",
			"sectionHandle": "home", "typeHandle": "page", "title": "Home" })
	);
}

#[tokio::test]
async fn preview_shows_the_draft() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let doc = {
		let conn = db.conn();
		let mut e = input("pages", "page", "Live", json!({ "intro": "old" }));
		e.published = false;
		let e = save(&conn, &s, None, e).unwrap();
		let sd = crate::drafts::SiteDoc {
			title: "Draft".into(),
			slug: "live".into(),
			published: false,
			fields: json!({ "intro": "new" }),
		};
		crate::drafts::put_draft(&conn, &s, e.id, "de", sd, None, None)
			.unwrap();
		let token = crate::drafts::create_token(&conn, e.id).unwrap();
		crate::drafts::preview(&conn, &token).unwrap()
	};

	let req = async_graphql::Request::new(
		r#"{ entry(uri: "live") { title ... on page_Entry { intro } } }"#,
	)
	.data(Viewer {
		published_only: true,
		preview: doc,
		token: None,
	});
	let data = gql.execute(req).await.data.into_json().unwrap();
	assert_eq!(data["entry"], json!({ "title": "Draft", "intro": "new" }));
}

#[tokio::test]
async fn site_preview_shows_every_draft() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let preview = {
		let conn = db.conn();
		let draft = |title: &str, published: bool| crate::drafts::SiteDoc {
			title: title.into(),
			slug: title.to_lowercase(),
			published,
			fields: json!({}),
		};
		// new, only its draft publishes it
		let mut e = input("pages", "page", "New", json!({}));
		e.published = false;
		let new = save(&conn, &s, None, e).unwrap();
		crate::drafts::put_draft(
			&conn,
			&s,
			new.id,
			"de",
			draft("New", true),
			None,
			None,
		)
		.unwrap();
		// live, its draft takes it down
		let old =
			save(&conn, &s, None, input("pages", "page", "Old", json!({})))
				.unwrap();
		crate::drafts::put_draft(
			&conn,
			&s,
			old.id,
			"de",
			draft("Old", false),
			None,
			None,
		)
		.unwrap();
		save(&conn, &s, None, input("pages", "page", "Same", json!({})))
			.unwrap();
		let token = crate::drafts::create_site_token(&conn).unwrap();
		crate::drafts::preview(&conn, &token).unwrap()
	};

	let titles = |preview| {
		let req = async_graphql::Request::new(
			r#"{ entries(section: "pages", order: "title") { title } }"#,
		)
		.data(Viewer {
			published_only: true,
			preview,
			token: None,
		});
		let gql = gql.clone();
		async move { gql.execute(req).await.data.into_json().unwrap() }
	};
	assert_eq!(
		titles(None).await["entries"],
		json!([{ "title": "Old" }, { "title": "Same" }])
	);
	assert_eq!(
		titles(preview).await["entries"],
		json!([{ "title": "New" }, { "title": "Same" }])
	);
}

#[tokio::test]
async fn urls_of_a_preview_keep_the_token() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let urls = SiteUrls::from([("de".to_string(), "https://x.ch".to_string())]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	save(
		&db.conn(),
		&s,
		None,
		input("pages", "page", "Team", json!({})),
	)
	.unwrap();

	let url = |preview, token: Option<&str>| {
		let req =
			async_graphql::Request::new(r#"{ entry(uri: "team") { url } }"#)
				.data(Viewer {
					published_only: true,
					preview,
					token: token.map(Into::into),
				});
		let gql = gql.clone();
		async move {
			gql.execute(req).await.data.into_json().unwrap()["entry"]["url"]
				.clone()
		}
	};
	assert_eq!(url(None, None).await, "https://x.ch/team");
	assert_eq!(
		url(Some(Preview::All), Some("t")).await,
		"https://x.ch/team?token=t&x-craft-live-preview=1"
	);
}

#[test]
fn markdown_links_only_go_to_the_web() {
	let html = markdown_html(
		"[a](javascript:alert(1)) [b](https://x.ch) <b>c</b> ![d](data:x)",
	);
	assert!(!html.contains("javascript"), "{html}");
	assert!(!html.contains("data:"), "{html}");
	assert!(html.contains(r#"href="https://x.ch""#));
	assert!(!html.contains("<b>"));
}

#[tokio::test]
async fn resolves_links() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let urls = SiteUrls::from([("de".to_string(), "https://x.ch".to_string())]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	{
		let conn = db.conn();
		let about =
			save(&conn, &s, None, input("pages", "page", "About", json!({})))
				.unwrap();
		for (title, link) in [
			(
				"a",
				json!({ "type": "url", "value": "https://tickets.ch", "label": "Tickets" }),
			),
			("b", json!({ "type": "url", "value": "https://x.ch/about" })),
			(
				"c",
				json!({ "type": "entry", "value": about.id, "newTab": true }),
			),
			("d", json!({ "type": "email", "value": "hi@x.ch" })),
		] {
			let fields = json!({ "ticket": link });
			save(&conn, &s, None, input("events", "event", title, fields))
				.unwrap();
		}
	}

	let data = run(
		&gql,
		r#"{ entries(type: "event", order: "title") {
			... on event_Entry { ticket { type url label defaultLabel target } }
		} }"#,
		true,
	)
	.await;
	let links: Vec<_> = data["entries"]
		.as_array()
		.unwrap()
		.iter()
		.map(|e| e["ticket"].clone())
		.collect();
	assert_eq!(
		links,
		[
			json!({ "type": "url", "url": "https://tickets.ch", "label": "Tickets",
				"defaultLabel": "https://tickets.ch", "target": "_blank" }),
			// the own site isn't external
			json!({ "type": "url", "url": "https://x.ch/about", "label": null,
				"defaultLabel": "https://x.ch/about", "target": null }),
			json!({ "type": "entry", "url": "https://x.ch/about", "label": null,
				"defaultLabel": "About", "target": "_blank" }),
			json!({ "type": "email", "url": "mailto:hi@x.ch", "label": null,
				"defaultLabel": "hi@x.ch", "target": null }),
		]
	);
}

#[tokio::test]
async fn rich_text_in_chunks() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let urls = SiteUrls::from([("de".to_string(), "https://x.ch".to_string())]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	{
		let conn = db.conn();
		let about =
			save(&conn, &s, None, input("pages", "page", "About", json!({})))
				.unwrap();
		let story = json!({ "type": "doc", "content": [
			{ "type": "heading", "attrs": { "level": 2 }, "content": [
				{ "type": "text", "text": "Hi" }] },
			{ "type": "paragraph", "content": [{ "type": "text", "text": "about",
				"marks": [{ "type": "link", "attrs": { "link":
					{ "type": "entry", "value": about.id } } }] }] },
			{ "type": "buttons", "attrs": { "links": [
				{ "type": "entry", "value": about.id, "label": "More" },
				{ "type": "url", "value": "https://y.ch" },
			] } },
			{ "type": "embed", "attrs": { "type": "text",
				"fields": { "content": "boxed" } } },
			{ "type": "paragraph" },
		] });
		save(
			&conn,
			&s,
			None,
			input("pages", "page", "Story", json!({ "story": story })),
		)
		.unwrap();
	}

	let data = run(
		&gql,
		r#"{ entry(uri: "story") { ... on page_Entry { story {
			html
			chunks {
				__typename
				... on Html { html }
				... on Buttons { buttons { url label target } }
				... on text_Block { content }
			}
		} } } }"#,
		true,
	)
	.await;
	let story = &data["entry"]["story"];
	assert_eq!(
		story["chunks"],
		json!([
			{ "__typename": "Html", "html":
				"<h2>Hi</h2><p><a href=\"https://x.ch/about\">about</a></p>" },
			{ "__typename": "Buttons", "buttons": [
				{ "url": "https://x.ch/about", "label": "More", "target": null },
				{ "url": "https://y.ch", "label": null, "target": "_blank" },
			] },
			{ "__typename": "text_Block", "content": "boxed" },
		])
	);
	assert!(
		story["html"]
			.as_str()
			.unwrap()
			.contains("<p class=\"buttons\">")
	);
}

#[tokio::test]
async fn translations_for_crelte() {
	let db = Db::memory().unwrap();
	let s =
		Arc::new(site().translations("common", [("menu.open", "Menü öffnen")]));
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	// the single of the texts, as the first migration makes it
	{
		let conn = db.conn();
		crate::apply::create_singles(&conn, &s).unwrap();
		let id: i64 = conn
			.query_row(
				"SELECT id FROM entries WHERE section = 'translations'",
				[],
				|r| r.get(0),
			)
			.unwrap();
		let mut en = input(
			"translations",
			"translations",
			"T",
			json!({
				"common": { "menu.open": "Open menu", "gone": "x" }
			}),
		);
		en.site = Some("en".into());
		save(&conn, &s, Some(id), en).unwrap();
	}

	let common = |site: u8| {
		let q = format!("{{ translations(siteId: {site}) {{ common }} }}");
		let gql = gql.clone();
		async move {
			let data = run(&gql, &q, true).await;
			let common = data["translations"]["common"].as_str().unwrap();
			serde_json::from_str::<Value>(common).unwrap()
		}
	};
	// a key the code no longer has is dropped, a missing text is the code's
	assert_eq!(common(2).await, json!({ "menu.open": "Open menu" }));
	assert_eq!(common(1).await, json!({ "menu.open": "Menü öffnen" }));
}

#[tokio::test]
async fn groups_and_markdown() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	{
		let conn = db.conn();
		let fields = json!({ "seo": { "title": "Hi", "description": "D" } });
		save(&conn, &s, None, input("pages", "page", "P", fields)).unwrap();
		let fields = json!({ "notes": "**bold** <script>x</script>" });
		save(&conn, &s, None, input("events", "event", "E", fields)).unwrap();

		// errors inside a group have its path
		let bad = json!({ "seo": { "title": "x".repeat(61) } });
		let Err(crate::error::Error::Invalid(errs)) =
			save(&conn, &s, None, input("pages", "page", "Q", bad))
		else {
			panic!("should fail");
		};
		assert_eq!(errs[0].path, "seo.title");
	}

	let data = run(
		&gql,
		r#"{
			p: entry(uri: "p") { ... on page_Entry { seo { title description } } }
			e: entry(uri: "events/e") { ... on event_Entry { notes { markdown html } } }
		}"#,
		true,
	)
	.await;
	assert_eq!(
		data["p"]["seo"],
		json!({ "title": "Hi", "description": "D" })
	);
	assert_eq!(
		data["e"]["notes"]["html"],
		"<p><strong>bold</strong> x</p>\n"
	);
}

#[tokio::test]
async fn hides_drafts_without_a_session() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let mut draft = input("pages", "page", "Draft", json!({}));
	draft.published = false;
	save(&db.conn(), &s, None, draft).unwrap();

	let q = r#"{ entry(uri: "draft") { title } }"#;
	assert_eq!(run(&gql, q, true).await["entry"], Value::Null);
	assert_eq!(run(&gql, q, false).await["entry"]["title"], "Draft");
}

#[tokio::test]
async fn checks_queries_of_frontends() {
	let s = Arc::new(site());
	let gql = build(s, Db::memory().unwrap(), String::new(), SiteUrls::new())
		.unwrap();
	let ok = "query ($uri: String!) { entry(uri: $uri) { title } }";
	assert!(check_query(&gql, ok).await.is_empty());
	let bad = "{ entries { ... on event_Entry { town } } }";
	let errors = check_query(&gql, bad).await;
	assert!(errors[0].contains("Unknown field \"town\""), "{errors:?}");
}
