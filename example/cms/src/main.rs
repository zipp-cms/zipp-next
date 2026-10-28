//! The cms of the example site: its content model and nothing else.

use zipp::{Field, Schema, Section, Transform};

fn schema() -> Schema {
	let body = || {
		Field::blocks("body", ["heading", "text", "image", "button", "events"])
			.translatable()
	};

	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		.section(Section::single("home", "home").uri(""))
		.section(
			Section::structure("pages", ["page"]).uri("{parent.uri}/{slug}"),
		)
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		// globals: singles without a url
		.section(Section::single("header", "header"))
		.section(Section::single("footer", "footer"))
		.nav("Content", ["home", "pages", "events"])
		.nav("Settings", ["header", "footer"])
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
			[Field::text("content").required().multiline()],
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
			[
				Field::text("label").required().max(30),
				Field::relation("link", ["home", "page", "event"]).max(1),
			],
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
