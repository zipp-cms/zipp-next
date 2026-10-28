//! The cms of the example site: its content model and nothing else.

use zipp::axum::Router;
use zipp::axum::extract::State;
use zipp::axum::http::header;
use zipp::axum::response::IntoResponse;
use zipp::axum::routing::get;
use zipp::error::Result;
use zipp::query::Query;
use zipp::{AppState, Cms, Field, LinkKind, Schema, Section, Transform};
use zipp_button_group::{ButtonGroup, button_group};

fn schema() -> Schema {
	let body = || {
		Field::blocks("body", ["heading", "text", "image", "button", "events"])
			.translatable()
	};

	Schema::new()
		// a field type of another crate, see example/button-group
		.field_type(ButtonGroup)
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		// the home page is the page of type home in the structure
		.section(
			Section::structure("pages", ["home", "page"])
				.uri("{parent.uri}/{slug}")
				.uri_of("home", ""),
		)
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		// globals: singles without a url
		.section(Section::single("header", "header"))
		.section(Section::single("footer", "footer"))
		// the readme of the demo, a page of the studio from this repo
		.doc("readme", "Readme", include_str!("../../readme.md"))
		.nav("Content", ["readme", "pages", "events"])
		.nav("Settings", ["header", "footer", "translations"])
		// texts of the frontend, editors translate them
		.translations(
			"common",
			[
				("nav.home", "Startseite"),
				(
					"footer.made",
					"Ein crelte-Frontend, die Inhalte kommen von zipp.",
				),
				("events.none", "Keine Veranstaltungen"),
				("events.soldOut", "Ausverkauft"),
			],
		)
		.transform(Transform::new("hero").width(1600).height(700).crop())
		.transform(Transform::new("small").width(600))
		.transform(Transform::new("medium").width(1200))
		.transform(Transform::new("thumb").width(160).height(120).crop())
		.entry(
			"home",
			"Home",
			[
				Field::images("hero").max(1),
				Field::text("intro").multiline().max(300).translatable(),
				body(),
				Field::group("seo", "seo").label("SEO").translatable(),
			],
		)
		.entry(
			"page",
			"Page",
			[
				Field::images("hero").max(1),
				Field::text("intro").multiline().max(300).translatable(),
				body(),
				Field::group("seo", "seo").label("SEO").translatable(),
			],
		)
		.entry(
			"event",
			"Event",
			[
				Field::text("date")
					.required()
					.indexed()
					.label("Date (YYYY-MM-DD)"),
				Field::text("city").required().indexed(),
				button_group(
					"genre",
					[
						("concert", "Konzert", "#2b6cb0"),
						("reading", "Lesung", "#2f855a"),
						("theatre", "Theater", "#c05621"),
					],
				)
				.required(),
				// more options: a select in the studio
				Field::dropdown(
					"language",
					[
						("de", "Deutsch"),
						("ch", "Mundart"),
						("fr", "Français"),
						("it", "Italiano"),
						("en", "English"),
					],
				)
				.indexed(),
				Field::number("price").range(0.0, 1000.0).indexed(),
				Field::boolean("soldOut").indexed(),
				Field::images("image").max(1),
				Field::markdown("description").translatable(),
				Field::relation("page", ["page"]).max(1).label("More info"),
			],
		)
		.entry(
			"header",
			"Header",
			[Field::relation("navigation", ["page", "event"])
				.label("Main navigation")],
		)
		.entry(
			"footer",
			"Footer",
			[Field::text("text").multiline().translatable()],
		)
		// the fields of the seo group of pages
		.block(
			"seo",
			"SEO",
			[
				Field::text("title")
					.max(60)
					.label("Title for search engines"),
				Field::text("description").multiline().max(160),
			],
		)
		.block(
			"heading",
			"Heading",
			[Field::text("text").required().max(80)],
		)
		.block(
			"text",
			"Text",
			[Field::rich_text("content")
				.required()
				.headings([3])
				.lists()
				.links()
				.buttons()
				.embed(["image"])],
		)
		.block(
			"image",
			"Image",
			[
				Field::images("image").required().max(1),
				Field::text("caption").max(200),
				// three options: buttons in the studio
				Field::dropdown(
					"width",
					[
						("small", "Small"),
						("wide", "Wide"),
						("full", "Full width"),
					],
				),
			],
		)
		.block(
			"button",
			"Button",
			[Field::link(
				"link",
				[
					LinkKind::Entry,
					LinkKind::Url,
					LinkKind::Email,
					LinkKind::Phone,
					LinkKind::Asset,
				],
			)
			.required()],
		)
		.block(
			"events",
			"Upcoming events",
			[Field::text("city").label("Only in city")],
		)
}

/// A route of the site next to zipp's: every published entry with a url,
/// in every site that has a url.
async fn sitemap(State(state): State<AppState>) -> Result<impl IntoResponse> {
	let conn = state.db.conn();
	let mut xml = String::from(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
	);
	for site in &state.schema.sites {
		let params = [("site", site.handle.as_str()), ("limit", "1000")]
			.map(|(k, v)| (k.to_string(), v.to_string()));
		let mut q = Query::parse(&state.schema, &params).unwrap();
		q.published_only = true;
		let (entries, _) = zipp::entries::list(&conn, &q)?;
		for uri in entries.iter().filter_map(|e| e.uri.as_ref()) {
			if let Some(url) =
				zipp::page_url(&state.site_urls, &site.handle, uri)
			{
				xml += &format!("<url><loc>{url}</loc></url>\n");
			}
		}
	}
	xml += "</urlset>\n";
	Ok(([(header::CONTENT_TYPE, "application/xml")], xml))
}

fn main() {
	Cms::new(schema(), zipp::migrations!())
		.routes(Router::new().route("/sitemap.xml", get(sitemap)))
		.on_change(|c| println!("{:?} {} in {}", c.kind, c.entry_id, c.section))
		.run();
}
