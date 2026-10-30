//! The cms of the example site: its content model and nothing else.

use zipp::axum::Router;
use zipp::axum::extract::State;
use zipp::axum::http::header;
use zipp::axum::response::IntoResponse;
use zipp::axum::routing::get;
use zipp::error::Result;
use zipp::query::Query;
use zipp::{
	Access, AppState, Cms, Column, Field, LinkKind, Role, Schema, Section,
	Transform,
};
use zipp_button_group::{ButtonGroup, button_group};
use zipp_seo::{Seo, SeoDefaults, seo_field};

fn schema() -> Schema {
	// like the crelte tutorial: a page starts with its intro, the content
	// is blocks
	let intro = || {
		vec![
			Field::rich_text("intro").links().translatable(),
			Field::images("hero").max(1),
		]
	};
	let content = || {
		Field::blocks("content", ["text", "images", "button", "events"])
			.translatable()
	};
	// for search engines the intro is the description, the hero the image
	let from_intro = || {
		SeoDefaults::new()
			.description_from("intro")
			.image_from("hero")
	};
	// the tabs of a page and an article, like Craft's field layout
	let tabs = |intro: Vec<Field>, content: Vec<Field>| {
		let mut tabs = vec![("Intro", intro)];
		if !content.is_empty() {
			tabs.push(("Content", content));
		}
		tabs.push(("SEO", vec![seo_field("seo")]));
		tabs
	};

	Schema::new()
		// a field type of another crate, see crates/zipp-button-group
		.field_type(ButtonGroup)
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		// the pages like the crelte tutorial: the home page is the page of
		// type home, the news page lists the articles
		.section(
			Section::structure("pages", ["home", "page", "news"])
				.uri("{parent.uri}/{slug}")
				.uri_of("home", ""),
		)
		.section(Section::channel("news", ["article"]).uri("news/{slug}"))
		// no pages: the news page filters by them
		.section(Section::structure("categories", ["category"]))
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		// globals: singles without a url
		.section(Section::single("header", "header"))
		.section(Section::single("footer", "footer"))
		// the readme of the demo, a page of the studio from this repo
		.doc("readme", "Readme", include_str!("../../readme.md"))
		.nav(
			"Content",
			["readme", "pages", "news", "categories", "events"],
		)
		.nav(
			"Settings",
			["header", "footer", "translations", "seoSettings"],
		)
		// icons of the studio, Lucide's by name or an svg of its own
		.nav_icon("Content", "folder-open")
		.nav_icon("Settings", "sliders-horizontal")
		// the SEO plugin, like the Craft plugin meierlabs-seo: the global
		// settings are the single seoSettings, defaults per section and
		// entry type here
		.plugin(
			Seo::new()
				.section(
					"events",
					SeoDefaults::new()
						.title_pattern("{title} – Veranstaltungen | {siteName}")
						.description_from("description"),
				)
				.entry_type("home", from_intro())
				.entry_type("page", from_intro())
				.entry_type("news", from_intro())
				.entry_type("article", from_intro()),
		)
		// roles of studio users, a user without one is an admin
		.role(
			Role::new("editor", "Editor")
				.sections(
					["pages", "header", "footer", "translations"],
					Access::Publish,
				)
				.sections(["news", "categories"], Access::Delete)
				.section("events", Access::Delete)
				.assets(Access::Delete)
				.ai(),
		)
		.role(
			Role::new("author", "Author")
				.section("events", Access::Edit)
				.section("news", Access::Edit)
				.section("categories", Access::View)
				.section("pages", Access::View)
				.assets(Access::Edit),
		)
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
				("news.all", "Alle"),
				("news.none", "Noch keine Neuigkeiten"),
			],
		)
		.transform(Transform::new("hero").width(1600).height(700).crop())
		.transform(Transform::new("small").width(600))
		.transform(Transform::new("medium").width(1200))
		.transform(Transform::new("thumb").width(160).height(120).crop())
		.entry_tabs("home", "Home", tabs(intro(), vec![content()]))
		.entry_tabs("page", "Page", tabs(intro(), vec![content()]))
		// the page of the news: its intro, then the articles
		.entry_tabs("news", "News", tabs(intro(), vec![]))
		.entry_tabs(
			"article",
			"Article",
			tabs(
				[intro(), vec![Field::relation("categories", ["category"])]]
					.concat(),
				vec![content()],
			),
		)
		.entry("category", "Category", [])
		.entry(
			"event",
			"Event",
			[
				// a day, sorted and filtered as 2026-11-14
				Field::date("date").required().indexed(),
				// a day and time, 2026-11-14T19:30
				Field::date("doors").time().label("Doors open"),
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
				// some of the options
				Field::checkboxes(
					"audience",
					[
						("kids", "Kinder"),
						("adults", "Erwachsene"),
						("families", "Familien"),
					],
				)
				.label("Für wen"),
				Field::dropdown(
					"spokenIn",
					[
						("de", "Deutsch"),
						("ch", "Mundart"),
						("fr", "Français"),
						("it", "Italiano"),
						("en", "English"),
					],
				)
				.indexed()
				.label("Sprache"),
				Field::number("price").range(0.0, 1000.0).indexed(),
				Field::boolean("soldOut").indexed(),
				// new images of events go into their folder
				Field::images("image").max(1).folder("Events"),
				Field::assets("program")
					.extensions(["pdf"])
					.max(1)
					.folder("Events/Programs")
					.label("Programm (PDF)"),
				Field::markdown("description").translatable(),
				Field::relation("page", ["page"]).max(1).label("More info"),
			],
		)
		.entry(
			"header",
			"Header",
			// a menu with a second level, to pages or anywhere
			[
				Field::navigation(
					"navigation",
					[LinkKind::Entry, LinkKind::Url],
				)
				.translatable()
				.label("Main navigation"),
			],
		)
		.entry(
			"footer",
			"Footer",
			[
				Field::text("text").multiline().translatable(),
				Field::table(
					"links",
					[Column::text("label"), Column::url("url")],
				)
				.translatable(),
			],
		)
		.block(
			"text",
			"Text",
			[Field::rich_text("text")
				.required()
				.headings([2, 3])
				.lists()
				.links()
				.buttons()
				.embed(["images"])],
		)
		.block(
			"images",
			"Images",
			[
				Field::images("images").required(),
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
		// in the lists, the add menus and the blocks' heads
		.icon("home", "house")
		.icon("page", "file-text")
		.icon("news", "newspaper")
		.icon("article", "notebook-pen")
		.icon("category", "tag")
		.icon("event", "calendar")
		.icon("text", "text")
		.icon("images", "images")
		.icon("button", "mouse-pointer-click")
		.icon("events", "calendar-days")
}

/// A route of the site next to zipp's: the upcoming events as JSON, for a
/// widget on another site.
async fn events(State(state): State<AppState>) -> Result<impl IntoResponse> {
	let conn = state.db.conn();
	let today = zipp::db::iso(zipp::db::now())[..10].to_string();
	let params = [("site", "de"), ("type", "event"), ("order", "date")]
		.map(|(k, v)| (k.to_string(), v.to_string()));
	let mut q = Query::parse(&state.schema, &params).unwrap();
	q.published_only = true;
	let (entries, _) = zipp::entries::list(&conn, &q)?;
	let events: Vec<_> = entries
		.into_iter()
		.filter(|e| {
			e.fields["date"]
				.as_str()
				.is_some_and(|d| d >= today.as_str())
		})
		.map(|e| {
			let url = e
				.uri
				.as_deref()
				.and_then(|u| zipp::page_url(&state.site_urls, &e.site, u));
			serde_json::json!({
				"title": e.title,
				"date": e.fields["date"],
				"city": e.fields["city"],
				"url": url,
			})
		})
		.collect();
	Ok((
		[(header::CONTENT_TYPE, "application/json")],
		serde_json::to_string(&events).unwrap(),
	))
}

#[cfg(test)]
mod tests;

fn main() {
	Cms::new(schema(), zipp::migrations!())
		.routes(Router::new().route("/events.json", get(events)))
		.on_change(|c| println!("{:?} {} in {}", c.kind, c.entry_id, c.section))
		.run();
}
