//! The cms of the example site: its content model and nothing else.

use zipp::{Field, Schema, Section};

fn schema() -> Schema {
	let body = || {
		Field::blocks("body", ["heading", "text", "button", "events"])
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
		.entry(
			"home",
			"Home",
			[
				Field::text("intro").multiline().max(300).translatable(),
				body(),
			],
		)
		.entry(
			"page",
			"Page",
			[
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
				Field::text("description").multiline().translatable(),
				Field::relation("page", ["page"]).max(1).label("More info"),
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
			[Field::text("content").required().multiline()],
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
