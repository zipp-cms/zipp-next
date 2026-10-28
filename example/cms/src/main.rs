//! The cms of the example site: its content model and nothing else.

use zipp::{Field, LinkKind, Schema, Section, Transform};

fn schema() -> Schema {
	let body = || {
		Field::blocks("body", ["heading", "text", "image", "button", "events"])
			.translatable()
	};

	Schema::new()
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
		.nav("Content", ["pages", "events"])
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
			],
		)
		.entry(
			"page",
			"Page",
			[
				Field::images("hero").max(1),
				Field::text("intro").multiline().max(300).translatable(),
				body(),
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
				Field::number("price").range(0.0, 1000.0).indexed(),
				Field::images("image").max(1),
				Field::text("description").multiline().translatable(),
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

fn main() {
	zipp::run(schema(), zipp::migrations!());
}
