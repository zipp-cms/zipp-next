//! The plugin in a site, through zipp's public api like a site uses it.

use std::sync::Arc;

use serde_json::{Value, json};
use zipp::db::Db;
use zipp::entries::{EntryInput, save};
use zipp::error::Error;
use zipp::graphql::{SiteUrls, build, ext};
use zipp::testing::TestServer;
use zipp::{Field, Schema, Section};
use zipp_seo::api::DefaultsReq;
use zipp_seo::{Seo, SeoDefaults, seo_field};

fn input(section: &str, ty: &str, title: &str, fields: Value) -> EntryInput {
	EntryInput {
		section: section.into(),
		ty: ty.into(),
		site: None,
		title: title.into(),
		slug: String::new(),
		published: true,
		parent_id: None,
		fields,
	}
}

/// The site as its crate builds it, before [`Cms`](zipp::Cms) finishes it.
fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		.section(
			Section::structure("pages", ["page"]).uri("{parent.uri}/{slug}"),
		)
		.entry(
			"event",
			"Event",
			[Field::markdown("notes"), Field::images("image").max(1)],
		)
		.entry("page", "Page", [Field::text("intro"), seo_field("seo")])
		.plugin(
			Seo::new()
				.section(
					"events",
					SeoDefaults::new()
						.title_pattern("{title} – {sectionName} | {siteName}")
						.description_from("notes"),
				)
				.entry_type("page", SeoDefaults::new().title_from("intro")),
		)
}

fn site() -> Schema {
	schema().finish()
}

#[tokio::test]
async fn seo_like_the_craft_plugin() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	s.check().unwrap();
	let urls = SiteUrls::from([
		("de".to_string(), "https://x.ch".to_string()),
		("en".to_string(), "https://x.ch/en".to_string()),
	]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	{
		let conn = db.conn();
		zipp::apply::create_singles(&conn, &s).unwrap();
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

	let seo = |q: &'static str, indexing: bool| {
		let gql = gql.clone();
		async move {
			let resp = gql.execute(ext::request(q, true, indexing)).await;
			assert!(resp.errors.is_empty(), "{:?}", resp.errors);
			resp.data.into_json().unwrap()
		}
	};

	let data = seo(r#"{ entry(section: "events") { seoJson } }"#, true).await;
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
	assert_eq!(graph[0]["address"]["addressLocality"], "Thun");
	assert_eq!(graph[0]["openingHoursSpecification"][0]["opens"], "08:00");
	assert!(graph.iter().any(|g| g["@type"] == "WebPage"));

	// the field of the page: its type's `seo` resolves the whole SEO
	let data = seo(
		r#"{ entry(section: "pages") { ... on page_Entry { seo { title robots } } } }"#,
		true,
	)
	.await;
	assert_eq!(data["entry"]["seo"]["title"], "Über uns | Zandi");
	assert_eq!(data["entry"]["seo"]["robots"], "noindex, follow");
	let data = seo(
		r#"{ entry(section: "pages", slug: "team") { seoJson } }"#,
		true,
	)
	.await;
	assert_eq!(data["entry"]["seoJson"]["title"], "Unser Team | Zandi");

	// outside production nothing is indexed
	let data = seo(
		r#"{ entry(section: "events") { ... on event_Entry { seo { robots } } } }"#,
		false,
	)
	.await;
	assert_eq!(data["entry"]["seo"]["robots"], "noindex, nofollow");

	// a page without an entry: the site, with the primary site's name
	let data = seo(r#"{ seoJson(siteId: 2) }"#, true).await;
	assert_eq!(data["seoJson"]["title"], "Zandi");
	assert_eq!(data["seoJson"]["description"], Value::Null);
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

	let wrong = DefaultsReq {
		id,
		site: Some("fr".into()),
	};
	let err = ts.admin().call(&wrong).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
	let err = ts.anonymous().call(&wrong).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}
