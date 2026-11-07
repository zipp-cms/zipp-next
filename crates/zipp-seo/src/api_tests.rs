//! The plugin in a site, through zipp's public api like a site uses it.

use crate::api::DefaultsReq;
use crate::{Seo, SeoDefaults, seo_field};
use serde_json::{Value, json};
use zipp::SiteUrls;
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::http::{Request, header};
use zipp::drafts::{SiteDoc, put_draft};
use zipp::entries::save;
use zipp::error::Error;
use zipp::testing::{Client, TestServer, input};
use zipp::{Field, GraphqlSchema, Schema, Section, UserPages};

/// The site as its crate builds it, before [`Cms`](zipp::Cms) finishes it.
fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		.section(
			Section::structure("pages", ["page", "landing"])
				.uri("{parent.uri}/{slug}"),
		)
		.entry(
			"event",
			"Event",
			[Field::markdown("notes"), Field::images("image").max(1)],
		)
		.entry("page", "Page", [Field::text("intro"), seo_field("seo")])
		.entry("landing", "Landing page", [Field::text("headline")])
		.plugin(
			Seo::new()
				.section(
					"events",
					SeoDefaults::new()
						.title_pattern("{title} – {sectionName} | {siteName}")
						.description_from("notes"),
				)
				.entry_type("page", SeoDefaults::new().title_from("intro"))
				.entry_type(
					"landing",
					SeoDefaults::new().title_from("headline"),
				),
		)
}

fn site() -> Schema {
	schema().finish()
}

#[tokio::test]
async fn seo_like_the_craft_plugin() {
	let urls = SiteUrls::from([
		("de".to_string(), "https://x.ch".to_string()),
		("en".to_string(), "https://x.ch/en".to_string()),
	]);
	let ts = TestServer::new(schema()).site_urls(urls);
	let s = ts.state.schema.clone();
	{
		let conn = ts.state.db.conn();
		let id: i64 = conn
			.query_row(
				"SELECT id FROM entries WHERE section = 'seoSettings'",
				[],
				|r| r.get(0),
			)
			.unwrap();
		let mut settings = input(
			"seoSettings",
			"seoSettings",
			"SEO",
			json!({
				"siteName": "Zandi",
				"description": "Augenärzte in Thun",
				"organization": {
					"type": "MedicalClinic", "name": "Zandi AG", "locality": "Thun",
					"openingHours": [{ "type": "seoOpeningHours", "fields": {
						"day": "Monday", "opens": "08:00", "closes": "17:00" } }],
				},
			}),
		);
		settings.site = Some("de".into());
		save(&conn, &s, Some(id), settings, None).unwrap();
		let fields = json!({ "notes": "Ein **schöner** Abend mit Musik." });
		let e = save(
			&conn,
			&s,
			None,
			input("events", "event", "Konzert", fields.clone()),
			None,
		)
		.unwrap();
		let mut en = input("events", "event", "Concert", fields);
		en.site = Some("en".into());
		save(&conn, &s, Some(e.id), en, None).unwrap();
		// published later than it was written: 2026-11-01T08:00:00
		conn.execute(
			"UPDATE entries SET post_date = 1793520000000 WHERE id = ?1",
			[e.id],
		)
		.unwrap();
		// a page with its own SEO in the field
		let seo = json!({ "seo": { "title": "Über uns", "robots": "noindex, follow" } });
		save(&conn, &s, None, input("pages", "page", "About", seo), None)
			.unwrap();
		// the title from a field, like SEOmatic's
		let intro = json!({ "intro": "Unser Team" });
		save(&conn, &s, None, input("pages", "page", "Team", intro), None)
			.unwrap();
		// a value the field doesn't know is refused
		let bad = json!({ "seo": { "robots": "maybe" } });
		assert!(
			save(&conn, &s, None, input("pages", "page", "Bad", bad), None)
				.is_err()
		);
	}

	let seo = async |ts: &TestServer, q: &str| {
		let resp = ts.graphql(q, json!({})).await;
		assert_eq!(resp["errors"], Value::Null, "{resp}");
		resp["data"].clone()
	};

	let data = seo(&ts, r#"{ entry(section: "events") { seoJson } }"#).await;
	let j = &data["entry"]["seoJson"];
	assert_eq!(j["title"], "Konzert – Events | Zandi");
	assert_eq!(j["description"], "Ein schöner Abend mit Musik.");
	assert_eq!(j["canonical"], "https://x.ch/events/konzert");
	assert_eq!(j["robots"], "index, follow");
	assert_eq!(j["locale"], "de_CH");
	let langs: Vec<&str> = j["alternates"]
		.as_array()
		.unwrap()
		.iter()
		.map(|a| a["hreflang"].as_str().unwrap())
		.collect();
	assert_eq!(langs, ["de-CH", "x-default", "en"]);
	let meta = j["meta"].to_string();
	assert!(meta.contains(r#""property":"og:title""#), "{meta}");
	assert!(meta.contains(r#""og:locale:alternate""#), "{meta}");
	let ld: Value =
		serde_json::from_str(j["jsonLd"].as_str().unwrap()).unwrap();
	let graph = ld["@graph"].as_array().unwrap();
	assert_eq!(graph[0]["@type"], "MedicalClinic");
	assert_eq!(graph[0]["address"]["@type"], "PostalAddress");
	assert_eq!(graph[0]["address"]["addressLocality"], "Thun");
	assert_eq!(graph[0]["openingHoursSpecification"][0]["opens"], "08:00");
	let page = graph.iter().find(|g| g["@type"] == "WebPage").unwrap();
	assert!(
		page["datePublished"]
			.as_str()
			.unwrap()
			.starts_with("2026-11-01T08:00:00"),
		"{page}"
	);

	// the field of the page: its type's `seo` resolves the whole SEO
	let data = seo(
		&ts,
		r#"{ entry(section: "pages") { ... on page_Entry { seo { title robots } } } }"#,
	)
	.await;
	assert_eq!(data["entry"]["seo"]["title"], "Über uns | Zandi");
	assert_eq!(data["entry"]["seo"]["robots"], "noindex, follow");
	let data = seo(
		&ts,
		r#"{ entry(section: "pages", slug: "team") { seoJson } }"#,
	)
	.await;
	assert_eq!(data["entry"]["seoJson"]["title"], "Unser Team | Zandi");

	// a page without an entry: the site, with the primary site's name
	let data = seo(&ts, r#"{ seoJson(siteId: 2) }"#).await;
	assert_eq!(data["seoJson"]["title"], "Zandi");
	assert_eq!(data["seoJson"]["description"], Value::Null);

	// outside production nothing is indexed
	let ts = ts.indexing(false);
	let data = seo(
		&ts,
		r#"{ entry(section: "events") { ... on event_Entry { seo { robots } } } }"#,
	)
	.await;
	assert_eq!(data["entry"]["seo"]["robots"], "noindex, nofollow");
}

/// A request of GraphQL's api as JSON, the answer as JSON.
async fn post(client: &Client, path: &str, body: Value, bearer: &str) -> Value {
	let mut req = Request::post(path)
		.header(header::CONTENT_TYPE, "application/json")
		.body(Body::from(body.to_string()))
		.unwrap();
	if !bearer.is_empty() {
		let auth = format!("Bearer {bearer}").parse().unwrap();
		req.headers_mut().insert(header::AUTHORIZATION, auth);
	}
	let resp = client.send(req).await;
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn a_token_gets_the_seo_of_its_sites() {
	let german = GraphqlSchema::new("german", "German")
		.sections(["events", "pages"])
		.sites(["de"]);
	let ts = TestServer::new(schema().graphql_schema(german));
	let token = json!({ "name": "Website", "schema": "german" });
	let created = post(&ts.admin(), "/api/graphql/tokens", token, "").await;
	let token = created["token"].as_str().unwrap();

	let anon = ts.anonymous();
	let query = |site: &str| json!({ "query": format!(r#"{{ seoJson(site: "{site}") }}"#) });
	let resp = post(&anon, "/api/graphql", query("de"), token).await;
	assert!(resp["data"]["seoJson"].is_object(), "{resp}");
	let resp = post(&anon, "/api/graphql", query("en"), token).await;
	assert_eq!(resp["data"]["seoJson"], Value::Null, "{resp}");

	// the token of serve reads every site
	let resp = ts.graphql(r#"{ seoJson(site: "en") }"#, json!({})).await;
	assert!(resp["data"]["seoJson"].is_object(), "{resp}");
}

#[test]
fn checks_what_the_site_sets_up() {
	let wrong = Schema::new()
		.site("de", "Deutsch", "de-CH")
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		.entry("event", "Event", [])
		.plugin(Seo::new().section("nope", SeoDefaults::new()))
		.finish();
	let e = wrong.check().unwrap_err();
	assert!(e.contains("seo: unknown section nope"), "{e}");
	assert!(site().plugin_of::<Seo>().is_some());
}

#[tokio::test]
async fn defaults_for_the_studio() {
	let ts = TestServer::new(schema());
	let id = {
		let conn = ts.state.db.conn();
		let intro = json!({ "intro": "Unser Team" });
		let page = input("pages", "page", "Team", intro);
		save(&conn, &ts.state.schema, None, page, None).unwrap().id
	};
	let defaults = ts
		.admin()
		.call(&DefaultsReq { id, site: None })
		.await
		.unwrap();
	assert_eq!(defaults["title"], "Unser Team");

	// a draft shows with its own type, which takes the title elsewhere
	{
		let conn = ts.state.db.conn();
		let mut draft = SiteDoc::new(
			"landing",
			"Team",
			"team",
			json!({ "headline": "Willkommen" }),
		);
		draft.published = true;
		put_draft(&conn, &ts.state.schema, id, "de", draft, None, None)
			.unwrap();
	}
	let defaults = ts
		.admin()
		.call(&DefaultsReq { id, site: None })
		.await
		.unwrap();
	assert_eq!(defaults["title"], "Willkommen");

	let wrong = DefaultsReq {
		id,
		site: Some("fr".into()),
	};
	let err = ts.admin().call(&wrong).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
	let err = ts.anonymous().call(&wrong).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn the_sitemap_lists_the_pages_of_users() {
	let pages = UserPages::new("cooks/{slug}").with_entries_in("events");
	let urls = SiteUrls::from([("de".to_string(), "https://x.ch".to_string())]);
	let ts = TestServer::new(schema().user_pages(pages)).site_urls(urls);
	let s = ts.state.schema.clone();
	{
		// Mia has a live event, the other user nothing
		let conn = ts.state.db.conn();
		let user = "INSERT INTO users (email, password_hash, created_at, slug)
			VALUES (?1, '', 0, ?2)";
		conn.execute(user, ["mia@x.ch", "mia"]).unwrap();
		let mia = conn.last_insert_rowid();
		conn.execute(user, ["sam@x.ch", "sam"]).unwrap();
		let event = input("events", "event", "Konzert", json!({}));
		save(&conn, &s, None, event, Some(mia)).unwrap();
	}
	let text = async |path: &str| {
		let resp = ts.anonymous().get(path).await;
		String::from_utf8(Client::bytes(resp).await).unwrap()
	};

	let index = text("/actions/meierlabs-seo/sitemap/index").await;
	assert!(
		index.contains("https://x.ch/sitemap-de-_users-1.xml"),
		"{index}"
	);
	let page = text(
		"/actions/meierlabs-seo/sitemap/page?site=de&section=_users&page=1",
	)
	.await;
	assert!(page.contains("<loc>https://x.ch/cooks/mia</loc>"), "{page}");
	assert!(!page.contains("sam"), "{page}");
}
