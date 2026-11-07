use serde_json::json;

use super::*;
use crate::entries::save;
use crate::fixtures::{png, site};
use crate::schema::{Field, Format, Transform};
use crate::testing::input;

/// What a frontend sees, or with drafts like the studio.
pub(super) fn viewer(published_only: bool) -> Viewer {
	Viewer {
		published_only,
		preview: None,
		token: None,
		indexing: true,
		scope: Scope::ALL,
		loaded: Default::default(),
	}
}

pub(super) async fn run(
	gql: &GqlSchema,
	query: &str,
	published_only: bool,
) -> Value {
	let req = async_graphql::Request::new(query).data(viewer(published_only));
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
		let about = save(
			&conn,
			&s,
			None,
			input("pages", "page", "About", json!({})),
			None,
		)
		.unwrap();
		let fields = json!({ "city": "Bern", "price": 12, "page": [about.id] });
		let event = save(
			&conn,
			&s,
			None,
			input("events", "event", "Jazz", fields),
			None,
		)
		.unwrap();
		let body = json!({ "intro": "Hi", "body": [
			{ "type": "text", "fields": { "content": "Hello" } },
			{ "type": "button", "fields": { "label": "Go", "link": [about.id] } },
		]});
		let mut team = input("pages", "page", "Team", body);
		team.parent_id = Some(about.id);
		save(&conn, &s, None, team, None).unwrap();
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
		json!([{ "title": "Jazz", "city": "Bern", "page": [{ "id": about.id.to_string() }] }])
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
	let webp = Transform::new("webp").width(100).format(Format::Webp);
	let s = Arc::new(site().transform(webp).image_widths([200]));
	let gql = build(
		s.clone(),
		db.clone(),
		"https://cms.test".into(),
		SiteUrls::new(),
	)
	.unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();

	let png = png(400, 200);
	{
		let conn = db.conn();
		let a =
			crate::assets::store(&conn, dir, "hero.png", &png, None).unwrap();
		let fields = json!({ "city": "Bern", "image": [a.id] });
		save(
			&conn,
			&s,
			None,
			input("events", "event", "Jazz", fields),
			None,
		)
		.unwrap();
		// German only: English falls back to it
		let alt = crate::assets::AssetChange {
			alt: [("de".into(), "Ein Held".into())].into(),
			focal_x: 0.5,
			focal_y: 0.5,
			..Default::default()
		};
		crate::assets::update(&conn, dir, &s, a.id, alt, None).unwrap();
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
			webp: url(transform: "webp")
			smallWebp: url(transform: "small", format: "webp")
			w100: url(transform: "webp", format: "jpg")
			webpSet: srcset(transforms: ["small"], format: "webp")
			widthSet: srcset(transforms: ["small", "w200"])
			alt
		} } } }"#,
		true,
	)
	.await;
	let img = &data["entry"]["image"][0];
	assert_eq!(img["url"], "https://cms.test/assets/1/hero.png");
	assert_eq!(img["small"], "https://cms.test/assets/1/small/hero.png");
	// the transform's format, or the one asked for
	assert_eq!(img["webp"], "https://cms.test/assets/1/webp/hero.webp");
	assert_eq!(
		img["smallWebp"],
		"https://cms.test/assets/1/small/hero.webp"
	);
	assert_eq!(img["w100"], "https://cms.test/assets/1/webp/hero.jpg");
	assert_eq!(
		img["webpSet"],
		"https://cms.test/assets/1/small/hero.webp 100w"
	);
	assert_eq!(
		img["widthSet"],
		"https://cms.test/assets/1/small/hero.png 100w, \
		https://cms.test/assets/1/w200/hero.png 200w"
	);
	assert_eq!(img["alt"], "Ein Held");
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

	// the alt text of the site, else the primary's
	let alt = |site: &'static str| {
		let gql = &gql;
		async move {
			let q = format!(
				r#"{{ entry(uri: "events/jazz", site: "{site}") {{
					... on event_Entry {{ image {{ alt }} }} }} }}"#
			);
			run(gql, &q, true).await["entry"]["image"][0]["alt"].clone()
		}
	};
	assert_eq!(alt("en").await, "Ein Held");
	{
		let conn = db.conn();
		let both = crate::assets::AssetChange {
			alt: [
				("de".into(), "Ein Held".into()),
				("en".into(), " A hero ".into()),
				("fr".into(), "Un héros".into()),
			]
			.into(),
			focal_x: 0.5,
			focal_y: 0.5,
			..Default::default()
		};
		let a = crate::assets::update(&conn, dir, &s, 1, both, None).unwrap();
		// trimmed, of the sites there are
		assert_eq!(
			a.alt,
			[
				("de".to_string(), "Ein Held".to_string()),
				("en".to_string(), "A hero".to_string())
			]
			.into()
		);
	}
	assert_eq!(alt("en").await, "A hero");
	assert_eq!(alt("de").await, "Ein Held");

	// a format needs a transform, and one there is
	for q in [
		r#"{ entry(uri: "events/jazz") { ... on event_Entry {
			image { url(format: "webp") } } } }"#,
		r#"{ entry(uri: "events/jazz") { ... on event_Entry {
			image { url(transform: "small", format: "gif") } } } }"#,
	] {
		let req = async_graphql::Request::new(q).data(viewer(true));
		assert!(!gql.execute(req).await.errors.is_empty(), "{q}");
	}
}

#[tokio::test]
async fn asset_fields_and_videos() {
	let db = Db::memory().unwrap();
	let mut s = site().asset_fields([
		Field::text("credit"),
		Field::relation("event", ["event"]).max(1),
	]);
	let event = s.get_mut("event").unwrap();
	event.fields.push(Field::videos("clip").max(1));
	let s = Arc::new(s);
	let gql = build(
		s.clone(),
		db.clone(),
		"https://cms.test".into(),
		SiteUrls::new(),
	)
	.unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	{
		let conn = db.conn();
		let a =
			crate::assets::store(&conn, dir, "clip.mp4", b"v", None).unwrap();
		let jazz = input("events", "event", "Jazz", json!({}));
		let jazz = save(&conn, &s, None, jazz, None).unwrap();
		let change = crate::assets::AssetChange {
			fields: Some(json!({ "credit": "Anna", "event": [jazz.id] })),
			..Default::default()
		};
		crate::assets::update(&conn, dir, &s, a.id, change, None).unwrap();
		let fields = json!({ "clip": [a.id] });
		let e = input("events", "event", "Film", fields);
		save(&conn, &s, None, e, None).unwrap();
	}

	// a frontend knows a video by its kind, the event in the site asked
	let data = run(
		&gql,
		r#"{ entry(uri: "events/film", site: "en") {
			... on event_Entry { clip {
				kind mimeType size credit event { title site }
			} } } }"#,
		true,
	)
	.await;
	assert_eq!(
		data["entry"]["clip"][0],
		json!({
			"kind": "video",
			"mimeType": "video/mp4",
			"size": 1,
			"credit": "Anna",
			"event": [{ "title": "Jazz", "site": "en" }],
		})
	);
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
		..Default::default()
	};
	crate::migrate::apply::apply_migrations(&db.conn(), &[first], &s).unwrap();

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
	.data(viewer(true));
	let resp = gql.execute(req).await;
	assert!(resp.errors.is_empty(), "{:?}", resp.errors);
	let data = resp.data.into_json().unwrap();
	assert_eq!(
		data["entry"],
		json!({ "id": "1", "siteId": 2, "url": "https://x.ch/en/",
			"sectionHandle": "home", "typeHandle": "page", "title": "Home" })
	);
}

#[tokio::test]
async fn checks_queries_of_frontends() {
	let s = Arc::new(site());
	let gql = build(s, Db::memory().unwrap(), String::new(), SiteUrls::new())
		.unwrap();
	let ok = "query ($uri: String!) { entry(uri: $uri) { title } }";
	let errors = check_query(&gql, ok).await;
	assert!(errors.is_empty(), "{errors:?}");
	let bad = "{ entries { ... on event_Entry { town } } }";
	let errors = check_query(&gql, bad).await;
	assert!(errors[0].contains("Unknown field \"town\""), "{errors:?}");
	// a note has no value to ask for
	let note = "{ entries { ... on event_Entry { howto } } }";
	let errors = check_query(&gql, note).await;
	assert!(errors[0].contains("Unknown field \"howto\""), "{errors:?}");
}

#[tokio::test]
async fn craft_arguments_and_fields() {
	let db = Db::memory().unwrap();
	let s = Arc::new(
		site()
			.image_widths([400, 800])
			.image_heights([450])
			.image_sizes([(800, 400)]),
	);
	let urls = SiteUrls::from([
		("de".to_string(), "https://x.ch".to_string()),
		("en".to_string(), "https://x.ch/en".to_string()),
	]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let (a, b, c) = {
		let conn = db.conn();
		let png = png(1600, 900);
		let img =
			crate::assets::store(&conn, dir, "piano-night.png", &png, None)
				.unwrap();
		let a = save(
			&conn,
			&s,
			None,
			input("events", "event", "A", json!({ "image": [img.id] })),
			None,
		)
		.unwrap();
		let b = save(
			&conn,
			&s,
			None,
			input("events", "event", "B", json!({})),
			None,
		)
		.unwrap();
		let c = save(
			&conn,
			&s,
			None,
			input("pages", "page", "C", json!({})),
			None,
		)
		.unwrap();
		(a.id, b.id, c.id)
	};

	// a channel is newest first, `not` leaves out, several types at once
	let q = format!(
		r#"{{
			news: entries(section: "events") {{ title }}
			titled: entries(section: "events", orderBy: "title ASC") {{ title }}
			without: entries(section: "events", id: ["not", {a}]) {{ title }}
			both: entries(id: [{a}, {c}], type: ["event", "page"], orderBy: "title") {{ title }}
			one: entry(id: [{b}]) {{ title language postDate
				localized {{ language url }} }}
			img: entry(id: [{a}]) {{ ... on event_Entry {{ image {{
				w800: url(width: 800) width(width: 800) focalPoint title
				h450: url(height: 450) h450Width: width(height: 450)
				crop: url(width: 800, height: 400, format: "webp")
				cropWidth: width(width: 800, height: 400)
				cropHeight: height(width: 800, height: 400)
				cropSet: srcset(transforms: ["w800h400", "h450"]) }} }} }}
		}}"#
	);
	let data = run(&gql, &q, true).await;
	assert_eq!(data["news"], json!([{ "title": "B" }, { "title": "A" }]));
	assert_eq!(data["titled"], json!([{ "title": "A" }, { "title": "B" }]));
	assert_eq!(data["without"], json!([{ "title": "B" }]));
	assert_eq!(data["both"], json!([{ "title": "A" }, { "title": "C" }]));
	assert_eq!(data["one"]["language"], "de-CH");
	assert!(
		data["one"]["postDate"]
			.as_str()
			.unwrap()
			.ends_with("+00:00")
	);
	assert_eq!(
		data["one"]["localized"],
		json!([{ "language": "en", "url": "https://x.ch/en/events/b" }])
	);
	let img = &data["img"]["image"][0];
	assert!(img["w800"].as_str().unwrap().contains("/w800/"), "{img}");
	assert_eq!(img["width"], 800);
	assert_eq!(img["focalPoint"], json!([0.5, 0.5]));
	assert_eq!(img["title"], "Piano night");
	// Craft's height, and width with height cropped
	assert!(img["h450"].as_str().unwrap().contains("/h450/"), "{img}");
	assert_eq!(img["h450Width"], 800);
	assert!(
		img["crop"]
			.as_str()
			.unwrap()
			.ends_with("/w800h400/piano-night.webp"),
		"{img}"
	);
	assert_eq!(
		(&img["cropWidth"], &img["cropHeight"]),
		(&json!(800), &json!(400))
	);
	assert!(
		img["cropSet"]
			.as_str()
			.unwrap()
			.contains("/w800h400/piano-night.png 800w"),
		"{img}"
	);

	// sizes the schema doesn't allow
	let errors = check_query(
		&gql,
		&format!(
			r#"{{ entry(id: [{a}]) {{ ... on event_Entry {{ image {{
				url(width: 800, height: 450) }} }} }} }}"#
		),
	)
	.await;
	assert!(
		errors[0].contains(
			"width: 800, height: 450 is not an allowed image size, the \
			schema allows: width 400, width 800, height 450, 800x400"
		),
		"{errors:?}"
	);
	let errors = check_query(
		&gql,
		&format!(
			r#"{{ entry(id: [{a}]) {{ ... on event_Entry {{ image {{
				srcset(transforms: ["w500"]) }} }} }} }}"#
		),
	)
	.await;
	assert!(errors[0].contains("unknown transform w500"), "{errors:?}");
}

#[tokio::test]
async fn filters_by_relation_fields_like_craft() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let about = {
		let conn = db.conn();
		let about = save(
			&conn,
			&s,
			None,
			input("pages", "page", "About", json!({})),
			None,
		)
		.unwrap();
		let with = json!({ "city": "Bern", "page": [about.id] });
		save(
			&conn,
			&s,
			None,
			input("events", "event", "Jazz", with),
			None,
		)
		.unwrap();
		save(
			&conn,
			&s,
			None,
			input("events", "event", "Folk", json!({})),
			None,
		)
		.unwrap();
		about
	};
	let titles = |q: String| {
		let gql = gql.clone();
		async move {
			let data = run(&gql, &q, true).await;
			data["entries"]
				.as_array()
				.unwrap()
				.iter()
				.map(|e| e["title"].as_str().unwrap().to_string())
				.collect::<Vec<_>>()
		}
	};
	let by = |arg: &str| {
		format!(r#"{{ entries(section: "events", page: {arg}) {{ title }} }}"#)
	};
	assert_eq!(titles(by(&format!("[{}]", about.id))).await, ["Jazz"]);
	assert_eq!(titles(by(r#"":notempty:""#)).await, ["Jazz"]);
	assert_eq!(titles(by(r#"":empty:""#)).await, ["Folk"]);
	assert!(titles(by("[999]")).await.is_empty());
}

#[tokio::test]
async fn children_of_a_list_load_together_in_their_order() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	{
		let conn = db.conn();
		let page = |title: &str, parent: Option<i64>| {
			let mut p = input("pages", "page", title, json!({}));
			p.parent_id = parent;
			save(&conn, &s, None, p, None).unwrap().id
		};
		let about = page("About", None);
		let jobs = page("Jobs", None);
		let team = page("Team", Some(about));
		page("Board", Some(about));
		page("Open", Some(jobs));
		crate::entries::move_entry(&conn, &s, team, Some(about), 1, None)
			.unwrap();
	}

	let data = run(
		&gql,
		r#"{ entries(section: "pages", order: "structure") {
			title children { title parent { title } }
		} }"#,
		true,
	)
	.await;
	assert_eq!(
		data["entries"],
		json!([
			{ "title": "About", "children": [
				{ "title": "Board", "parent": { "title": "About" } },
				{ "title": "Team", "parent": { "title": "About" } },
			] },
			{ "title": "Board", "children": [] },
			{ "title": "Team", "children": [] },
			{ "title": "Jobs", "children": [
				{ "title": "Open", "parent": { "title": "Jobs" } },
			] },
			{ "title": "Open", "children": [] },
		])
	);
}

#[tokio::test]
async fn the_name_lists_match_the_types() {
	use crate::schema::{Field, LinkKind, Section};
	let db = Db::memory().unwrap();
	// with a navigation for its type
	let s = site().section(Section::single("menu", "menu")).entry(
		"menu",
		"Menu",
		[Field::navigation("main", [LinkKind::Url])],
	);
	let gql = build(Arc::new(s), db, String::new(), SiteUrls::new()).unwrap();
	let query = "{
		entry: __type(name: \"Entry\") { fields { name } }
		asset: __type(name: \"Asset\") { fields { name } }
		link: __type(name: \"Link\") { fields { name } }
		navItem: __type(name: \"NavItem\") { fields { name } }
		__schema { queryType { fields { name args { name } } } }
	}";
	let v = run(&gql, query, false).await;
	let names = |list: &Value| -> std::collections::BTreeSet<String> {
		let list = list.as_array().unwrap().iter();
		list.map(|f| f["name"].as_str().unwrap().to_string())
			.collect()
	};
	let set = |l: &[&str]| l.iter().map(|s| s.to_string()).collect();

	let all = [ENTRY_FIELDS, OPTIONAL_ENTRY_FIELDS].concat();
	assert_eq!(names(&v["entry"]["fields"]), set(&all));
	assert_eq!(names(&v["asset"]["fields"]), set(ASSET_FIELDS));

	// the arguments of entries, and the relation field of the test site
	let fields = v["__schema"]["queryType"]["fields"].as_array().unwrap();
	let entries = fields.iter().find(|f| f["name"] == "entries").unwrap();
	let mut taken = set(query::TAKEN);
	taken.insert("page".into());
	assert_eq!(names(&entries["args"]), taken);

	// what a resolved link has
	let link = ResolvedLink {
		kind: LinkKind::Url,
		url: String::new(),
		label: None,
		default_label: String::new(),
		target: None,
		download: false,
	};
	let keys = |v: Value| -> std::collections::BTreeSet<String> {
		v.as_object().unwrap().keys().cloned().collect()
	};
	let json = serde_json::to_value(&link).unwrap();
	assert_eq!(names(&v["link"]["fields"]), keys(json));
	let item = NavItem {
		link,
		children: vec![],
	};
	let json = serde_json::to_value(&item).unwrap();
	assert_eq!(names(&v["navItem"]["fields"]), keys(json));
}

#[tokio::test]
async fn errors_show_no_sql() {
	let db = Db::memory().unwrap();
	let gql =
		build(Arc::new(site()), db.clone(), String::new(), SiteUrls::new())
			.unwrap();
	db.conn().execute("DROP TABLE entry_sites", []).unwrap();

	let req =
		async_graphql::Request::new("{ entries { title } }").data(viewer(true));
	let resp = gql.execute(req).await;
	assert_eq!(resp.errors.len(), 1);
	assert_eq!(resp.errors[0].message, "Internal server error: see the log");

	// mistakes of the query still say what is wrong
	let req = async_graphql::Request::new("{ entries(site: \"xx\") { id } }")
		.data(viewer(true));
	let resp = gql.execute(req).await;
	assert!(resp.errors[0].message.contains("xx"), "{:?}", resp.errors);
}
