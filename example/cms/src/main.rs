//! The cms of the example site: its content model and nothing else.

use zipp::{Field, Schema};

fn schema() -> Schema {
	Schema::new()
		.entry(
			"page",
			"Page",
			[
				Field::text("intro").multiline().max(300),
				Field::blocks("body", ["heading", "text", "button", "events"]),
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
				Field::text("description").multiline(),
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
				Field::relation("link", ["page", "event"]).max(1),
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
