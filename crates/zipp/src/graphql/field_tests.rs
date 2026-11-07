//! The values of the field kinds: links, rich text, markdown, groups,
//! texts, checkboxes, tables and navigations, blocks.

use serde_json::json;

use super::tests::run;
use super::*;
use crate::entries::save;
use crate::fixtures::{png, site};
use crate::testing::input;

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
		let about = save(
			&conn,
			&s,
			None,
			input("pages", "page", "About", json!({})),
			None,
		)
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
			(
				"e",
				json!({ "type": "entry", "value": about.id, "anchor": "team" }),
			),
		] {
			let fields = json!({ "ticket": link });
			save(
				&conn,
				&s,
				None,
				input("events", "event", title, fields),
				None,
			)
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
			// a section of the page
			json!({ "type": "entry", "url": "https://x.ch/about#team", "label": null,
				"defaultLabel": "About", "target": null }),
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
		let about = save(
			&conn,
			&s,
			None,
			input("pages", "page", "About", json!({})),
			None,
		)
		.unwrap();
		let story = json!({ "type": "doc", "content": [
			{ "type": "heading", "attrs": { "level": 2 }, "content": [
				{ "type": "text", "text": "Hi" }] },
			{ "type": "paragraph", "content": [{ "type": "text", "text": "about",
				"marks": [{ "type": "link", "attrs": { "link":
					{ "type": "entry", "value": about.id } } }] }] },
			{ "type": "button", "attrs": { "link":
				{ "type": "entry", "value": about.id, "label": "More" } } },
			{ "type": "button", "attrs": { "link":
				{ "type": "url", "value": "https://y.ch" } } },
			{ "type": "embed", "attrs": { "type": "text",
				"fields": { "content": "boxed" } } },
			{ "type": "paragraph" },
		] });
		save(
			&conn,
			&s,
			None,
			input("pages", "page", "Story", json!({ "story": story })),
			None,
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
		crate::migrate::apply::create_singles(&conn, &s).unwrap();
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
		save(&conn, &s, Some(id), en, None).unwrap();
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
		save(&conn, &s, None, input("pages", "page", "P", fields), None)
			.unwrap();
		save(
			&conn,
			&s,
			None,
			input("pages", "page", "N", json!({})),
			None,
		)
		.unwrap();
		let fields = json!({ "notes": "**bold** <script>x</script>" });
		save(&conn, &s, None, input("events", "event", "E", fields), None)
			.unwrap();

		// errors inside a group have its path
		let bad = json!({ "seo": { "title": "x".repeat(61) } });
		let Err(crate::error::Error::Invalid(errs)) =
			save(&conn, &s, None, input("pages", "page", "Q", bad), None)
		else {
			panic!("should fail");
		};
		assert_eq!(errs[0].path, "seo.title");
	}

	let data = run(
		&gql,
		r#"{
			p: entry(uri: "p") { ... on page_Entry { seo { title description } } }
			n: entry(uri: "n") { ... on page_Entry { seo { title description } } }
			e: entry(uri: "events/e") { ... on event_Entry { notes { markdown html } } }
		}"#,
		true,
	)
	.await;
	assert_eq!(
		data["p"]["seo"],
		json!({ "title": "Hi", "description": "D" })
	);
	// a group without a value is there like Craft's content block
	assert_eq!(
		data["n"]["seo"],
		json!({ "title": null, "description": null })
	);
	assert_eq!(
		data["e"]["notes"]["html"],
		"<p><strong>bold</strong> x</p>\n"
	);
}

/// A field type of another crate with an object type in GraphQL: a
/// number of stars out of five.
struct Rating;

impl FieldType for Rating {
	fn name(&self) -> &'static str {
		"rating"
	}

	fn validate(
		&self,
		_options: &Value,
		value: &Value,
	) -> std::result::Result<Value, String> {
		match value.as_u64() {
			Some(1..=5) => Ok(value.clone()),
			_ => Err("1 to 5 stars".into()),
		}
	}

	fn graphql(&self) -> GraphqlType {
		GraphqlType::Object {
			name: "Rating",
			fields: vec![("stars", FieldScalar::Int), ("of", FieldScalar::Int)],
		}
	}

	fn output(&self, _options: &Value, value: &Value) -> Value {
		json!({ "stars": value, "of": 5 })
	}

	fn studio(&self) -> crate::StudioModule {
		crate::StudioModule::new("")
	}
}

#[tokio::test]
async fn button_groups_and_field_types() {
	use crate::schema::{ButtonOption, Field, Section};
	let show = |options: &[ButtonOption]| {
		site()
			.field_type(Rating)
			.section(Section::channel("shows", ["show"]).uri("{slug}"))
			.entry(
				"show",
				"Show",
				[
					Field::button_group("genre", options.iter().cloned()),
					Field::custom("rating", "rating", Value::Null),
				],
			)
	};
	let concert = ButtonOption::from(("concert", "Konzert", "#2b6cb0"));
	let reading = ButtonOption::new("reading", "Lesung").icon("book");
	let s = Arc::new(show(&[
		concert.clone(),
		ButtonOption::from(("talk", "Talk", "#2f855a")),
		reading.clone(),
	]));
	s.check().unwrap();
	let db = Db::memory().unwrap();
	{
		let conn = db.conn();
		for (title, genre) in
			[("a", "concert"), ("b", "talk"), ("c", "reading")]
		{
			let fields = json!({ "genre": genre, "rating": 4 });
			save(&conn, &s, None, input("shows", "show", title, fields), None)
				.unwrap();
		}
		let bad = json!({ "rating": 6 });
		let Err(crate::error::Error::Invalid(errs)) =
			save(&conn, &s, None, input("shows", "show", "d", bad), None)
		else {
			panic!("should fail");
		};
		assert_eq!(errs[0].message, "1 to 5 stars");
	}

	// talk was removed since, it is none
	let s = Arc::new(show(&[concert, reading]));
	let gql = build(s, db, String::new(), SiteUrls::new()).unwrap();
	let data = run(
		&gql,
		r#"{ entries(section: "shows", order: "title") { ... on show_Entry {
			genre { value label color icon } rating { stars of }
		} } }"#,
		true,
	)
	.await;
	assert_eq!(
		data["entries"],
		json!([
			{ "genre": { "value": "concert", "label": "Konzert",
				"color": "#2b6cb0", "icon": null },
				"rating": { "stars": 4, "of": 5 } },
			{ "genre": null, "rating": { "stars": 4, "of": 5 } },
			// no color, the icon's Lucide name
			{ "genre": { "value": "reading", "label": "Lesung",
				"color": null, "icon": "book" },
				"rating": { "stars": 4, "of": 5 } },
		])
	);
}

#[tokio::test]
async fn checkboxes_tables_and_navigations() {
	use crate::schema::{Column, Field, LinkKind, Section};
	let db = Db::memory().unwrap();
	let s =
		Arc::new(site().section(Section::channel("menus", ["menu"])).entry(
			"menu",
			"Menu",
			[
				Field::navigation("main", [LinkKind::Entry, LinkKind::Url]),
				Field::table(
					"links",
					[Column::text("label"), Column::url("url")],
				),
				Field::checkboxes("cycles", [("1", "One"), ("2", "Two")]),
			],
		));
	s.check().unwrap();
	let urls = SiteUrls::from([("de".to_string(), "https://x.ch".to_string())]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	{
		let conn = db.conn();
		let team = save(
			&conn,
			&s,
			None,
			input("pages", "page", "Team", json!({})),
			None,
		)
		.unwrap();
		let fields = json!({
			"main": [
				{ "type": "url", "value": "https://a.ch", "label": "A",
					"children": [{ "type": "entry", "value": team.id }] },
			],
			"links": [{ "label": "Docs", "url": "https://d.ch" }, { "label": " " }],
			"cycles": ["2", "1", "2"],
		});
		save(&conn, &s, None, input("menus", "menu", "M", fields), None)
			.unwrap();

		// mistakes: too deep, a url that isn't one, an unknown option
		let bad = json!({
			"main": [{ "type": "url", "value": "https://a.ch", "children": [
				{ "type": "url", "value": "https://b.ch", "children": [
					{ "type": "url", "value": "https://c.ch" }] }] }],
			"links": [{ "url": "javascript:x" }],
			"cycles": ["3"],
		});
		let Err(crate::error::Error::Invalid(errs)) =
			save(&conn, &s, None, input("menus", "menu", "B", bad), None)
		else {
			panic!("should fail");
		};
		let paths: Vec<&str> = errs.iter().map(|e| e.path.as_str()).collect();
		assert_eq!(paths, ["main.0.children.0", "links.0.url", "cycles"]);
	}

	let data = run(
		&gql,
		r#"{ entry(section: "menus") { ... on menu_Entry {
			main { url label target children { url defaultLabel children { url } } }
			links { label url }
			cycles
		} } }"#,
		true,
	)
	.await;
	assert_eq!(
		data["entry"],
		json!({
			"main": [{ "url": "https://a.ch", "label": "A", "target": "_blank",
				"children": [{ "url": "https://x.ch/team", "defaultLabel": "Team",
					"children": [] }] }],
			"links": [{ "label": "Docs", "url": "https://d.ch" }],
			"cycles": ["2", "1"],
		})
	);
}

#[tokio::test]
async fn typed_columns_and_styles() {
	use crate::schema::{Column, Field, LinkKind, Section, Style};
	let db = Db::memory().unwrap();
	let s =
		Arc::new(site().section(Section::channel("menus", ["menu"])).entry(
			"menu",
			"Menu",
			[
				Field::table(
					"dishes",
					[
						Column::text("name"),
						Column::number("price").decimals(2).unit("CHF"),
						Column::boolean("vegan"),
						Column::date("from"),
						Column::dropdown(
							"size",
							[("s", "Small"), ("l", "Large")],
						),
						Column::link("more", [LinkKind::Entry, LinkKind::Url]),
					],
				),
				Field::rich_text("text").styles([
					Style::block("lead", "Lead"),
					Style::inline("highlight", "Highlight"),
				]),
			],
		));
	s.check().unwrap();
	let urls = SiteUrls::from([("de".to_string(), "https://x.ch".to_string())]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	{
		let conn = db.conn();
		let team = save(
			&conn,
			&s,
			None,
			input("pages", "page", "Team", json!({})),
			None,
		)
		.unwrap();
		let fields = json!({
			"dishes": [
				{ "name": " Rösti ", "price": 18.5, "vegan": true, "from": "2026-11-14",
					"size": "l", "more": { "type": "entry", "value": team.id } },
				// no is nothing, a row of only noes is no row
				{ "name": "Wurst", "vegan": false, "more": { "type": "url", "value": null } },
				{ "vegan": false },
			],
			"text": { "type": "doc", "content": [
				{ "type": "paragraph", "attrs": { "class": "lead" }, "content": [
					{ "type": "text", "text": "Hot", "marks": [
						{ "type": "style", "attrs": { "class": "highlight" } }] },
				] },
			] },
		});
		let menu =
			save(&conn, &s, None, input("menus", "menu", "M", fields), None)
				.unwrap();
		assert_eq!(
			menu.fields["dishes"],
			json!([
				{ "name": "Rösti", "price": 18.5, "vegan": true, "from": "2026-11-14",
					"size": "l", "more": { "type": "entry", "value": team.id } },
				{ "name": "Wurst" },
			])
		);
		// the link in the table is a relation, the studio shows its title
		let refs =
			crate::entries::refs(&conn, &[menu.id], "de", false).unwrap();
		assert!(refs.contains_key(&team.id));

		let bad = json!({
			"dishes": [{ "price": 1.234, "vegan": "yes", "from": "14.11.2026",
				"size": "m", "more": { "type": "email", "value": "a@b.ch" } }],
			"text": { "type": "doc", "content": [
				{ "type": "paragraph", "attrs": { "class": "huge" },
					"content": [{ "type": "text", "text": "x" }] },
			] },
		});
		let Err(crate::error::Error::Invalid(errs)) =
			save(&conn, &s, None, input("menus", "menu", "B", bad), None)
		else {
			panic!("should fail");
		};
		let errs: Vec<_> = errs
			.iter()
			.map(|e| format!("{}: {}", e.path, e.message))
			.collect();
		assert_eq!(
			errs,
			[
				"dishes.0.price: at most 2 decimal places",
				"dishes.0.vegan: wrong type",
				"dishes.0.from: a date like 2026-11-14",
				"dishes.0.size: not one of the options",
				"dishes.0.more: can't link to email",
				"text: style \"huge\" isn't allowed here",
			]
		);
	}

	let data = run(
		&gql,
		r#"{ entry(section: "menus") { ... on menu_Entry {
			dishes { name price vegan from size more { url defaultLabel } }
			text { html chunks { ... on Html { html } } }
		} } }"#,
		true,
	)
	.await;
	let html = r#"<p class="lead"><span class="highlight">Hot</span></p>"#;
	assert_eq!(
		data["entry"],
		json!({
			"dishes": [
				{ "name": "Rösti", "price": 18.5, "vegan": true, "from": "2026-11-14",
					"size": "l", "more": { "url": "https://x.ch/team", "defaultLabel": "Team" } },
				{ "name": "Wurst", "price": null, "vegan": false, "from": null,
					"size": null, "more": null },
			],
			"text": { "html": html, "chunks": [{ "html": html }] },
		})
	);
}

#[tokio::test]
async fn switched_off_blocks_and_dates() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let expiry = crate::db::now() + crate::db::DAY_MS;
	{
		let conn = db.conn();
		let body = json!({ "body": [
			{ "type": "text", "fields": { "content": "On" } },
			{ "type": "text", "enabled": false, "fields": { "content": "Off" } },
		]});
		let mut page = input("pages", "page", "Blocks", body);
		page.dates.expiry_date = Some(expiry);
		save(&conn, &s, None, page, None).unwrap();
	}
	let data = run(
		&gql,
		r#"{ entry(uri: "blocks") { postDate expiryDate
			... on page_Entry { body { ... on text_Block { content } } } } }"#,
		true,
	)
	.await;
	let e = &data["entry"];
	assert_eq!(e["body"], json!([{ "content": "On" }]));
	assert_eq!(e["expiryDate"], crate::db::iso(expiry));
	assert!(e["postDate"].as_str().is_some_and(|d| d.starts_with("20")));
}

/// A field type with blocks in its items: each block has a width in
/// columns, GraphQL gives `Place { span block }`.
struct Places;

/// An item of [`Places`] for GraphQL, with the site for its block.
struct Place {
	span: i64,
	block: Value,
	site: String,
}

impl FieldType for Places {
	fn name(&self) -> &'static str {
		"places"
	}

	fn validate(
		&self,
		_options: &Value,
		value: &Value,
	) -> std::result::Result<Value, String> {
		// the blocks come checked
		let items = value.as_array().unwrap();
		match items
			.iter()
			.all(|i| matches!(i["span"].as_u64(), Some(1..=12)))
		{
			true => Ok(value.clone()),
			false => Err("a span of 1 to 12".into()),
		}
	}

	fn graphql(&self) -> GraphqlType {
		GraphqlType::List("Place")
	}

	fn resolve<'a>(
		&self,
		ctx: &ResolverContext<'a>,
		_options: &Value,
		value: &Value,
	) -> async_graphql::Result<Option<FieldValue<'a>>> {
		let site = ext::site(ctx)?;
		// a switched off block isn't on the website, nor its item
		let items = value.as_array().into_iter().flatten();
		let items = items.filter(|i| i["block"]["enabled"] != false).map(|i| {
			FieldValue::owned_any(Place {
				span: i["span"].as_i64().unwrap_or(12),
				block: i["block"].clone(),
				site: site.clone(),
			})
		});
		Ok(Some(FieldValue::list(items)))
	}

	fn studio(&self) -> crate::StudioModule {
		crate::StudioModule::new("")
	}
}

/// Registers the GraphQL type of [`Places`].
struct PlacesPlugin;

impl crate::Plugin for PlacesPlugin {
	fn name(&self) -> &'static str {
		"places"
	}

	fn graphql(&self, _schema: &Schema) -> crate::GraphqlExt {
		let place = Object::new("Place")
			.field(GqlField::new(
				"span",
				TypeRef::named_nn(TypeRef::INT),
				|ctx| {
					FieldFuture::new(async move {
						let p = ctx.parent_value.try_downcast_ref::<Place>()?;
						Ok(Some(FieldValue::value(p.span)))
					})
				},
			))
			.field(GqlField::new("block", TypeRef::named_nn("Block"), |ctx| {
				FieldFuture::new(async move {
					let p = ctx.parent_value.try_downcast_ref::<Place>()?;
					Ok(ext::block(&p.block, &p.site))
				})
			}));
		crate::GraphqlExt::new().types([place.into()])
	}
}

#[tokio::test]
async fn blocks_in_the_items_of_a_field_type() {
	use crate::schema::{Field, Section};
	let s = Arc::new(
		site()
			.field_type(Places)
			.plugin(PlacesPlugin)
			.block("photo", "Photo", [Field::images("image").max(1)])
			.section(Section::channel("grids", ["grid"]).uri("{slug}"))
			.entry(
				"grid",
				"Grid",
				[Field::custom("places", "places", Value::Null)
					.item_blocks(["text", "photo"])],
			),
	);
	s.check().unwrap();
	let db = Db::memory().unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let id = {
		let conn = db.conn();
		let a = crate::assets::store(
			&conn,
			tmp.path(),
			"hero.png",
			&png(40, 20),
			None,
		)
		.unwrap();

		// zipp checks the blocks, with the path of the item's block
		let save_places = |places: Value| {
			let fields = json!({ "places": places });
			save(&conn, &s, None, input("grids", "grid", "G", fields), None)
		};
		let Err(crate::error::Error::Invalid(errs)) = save_places(json!([
			{ "span": 6, "block": { "type": "text", "fields": {} } },
			{ "span": 6, "block": { "type": "button", "fields": {} } },
		])) else {
			panic!("should fail");
		};
		let errs: Vec<_> = errs.iter().map(|e| e.path.as_str()).collect();
		assert_eq!(errs, ["places.0.block.content", "places.1.block"]);
		// and the type the rest of an item
		let Err(crate::error::Error::Invalid(errs)) = save_places(json!([
			{ "span": 13, "block": { "type": "text", "fields": { "content": "A" } } },
		])) else {
			panic!("should fail");
		};
		assert_eq!(errs[0].message, "a span of 1 to 12");

		let id = save_places(json!([
			{ "span": 4, "block": { "id": "a1", "type": "photo",
				"fields": { "image": [a.id] } } },
			{ "span": 8, "block": { "id": "a1", "type": "text",
				"fields": { "content": "A" } } },
			{ "span": 6, "block": { "type": "text", "enabled": false,
				"fields": { "content": "Off" } } },
		]))
		.unwrap()
		.id;
		// the image is in use, at the path of its block
		let path: String = conn
			.query_row(
				"SELECT path FROM asset_relations WHERE asset_id = ?1",
				[a.id],
				|r| r.get(0),
			)
			.unwrap();
		assert_eq!(path, "places.0.block.image");
		// a copied block gets its own id
		let saved = crate::entries::get_entry(&conn, id, "en").unwrap();
		let places = &saved.fields["places"];
		assert_eq!(places[0]["block"]["id"], "a1");
		assert_ne!(places[1]["block"]["id"], "a1");
		id
	};

	let gql = build(s, db, String::new(), SiteUrls::new()).unwrap();
	let data = run(
		&gql,
		&format!(
			r#"{{ entry(id: {id}) {{ ... on grid_Entry {{ places {{ span
			block {{ typeHandle ... on photo_Block {{ image {{ filename }} }}
				... on text_Block {{ content }} }} }} }} }} }}"#
		),
		true,
	)
	.await;
	assert_eq!(
		data["entry"]["places"],
		json!([
			{ "span": 4, "block": { "typeHandle": "photo",
				"image": [{ "filename": "hero.png" }] } },
			{ "span": 8, "block": { "typeHandle": "text", "content": "A" } },
		])
	);
}
