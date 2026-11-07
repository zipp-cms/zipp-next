//! What the tests of several areas share: the test site and a PNG.

use std::io::Cursor;

use image::{ImageFormat, RgbImage};

use crate::schema::{
	Access, Field, GraphqlSchema, LinkKind, Role, Schema, Section, Transform,
};

/// A site with two languages, pages, events and blocks; the tests of
/// most areas use it.
pub fn site() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		.section(Section::single("home", "page").uri(""))
		.section(
			Section::structure("pages", ["page"]).uri("{parent.uri}/{slug}"),
		)
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		.entry(
			"page",
			"Page",
			[
				Field::text("intro").multiline().translatable(),
				Field::blocks("body", ["text", "button"]).translatable(),
				Field::group("seo", "seo"),
				Field::rich_text("story")
					.headings([2, 3])
					.links()
					.buttons()
					.embed(["text"])
					.translatable(),
			],
		)
		.entry(
			"event",
			"Event",
			[
				Field::text("city").indexed(),
				Field::number("price").decimals(2).indexed(),
				Field::relation("page", ["page"]).max(1),
				Field::images("image").max(1),
				Field::markdown("notes"),
				Field::link(
					"ticket",
					[LinkKind::Url, LinkKind::Entry, LinkKind::Email],
				),
				Field::email("contact"),
				Field::boolean("listed").default(true),
				Field::note("howto", "Images in **landscape**.").width(50),
			],
		)
		.transform(Transform::new("small").width(100))
		.transform(Transform::new("square").width(50).height(50).crop())
		.block("text", "Text", [Field::text("content").required()])
		.block(
			"seo",
			"SEO",
			[Field::text("title").max(60), Field::text("description")],
		)
		.block(
			"button",
			"Button",
			[
				Field::text("label").required().max(20),
				Field::relation("link", ["page"]).max(1),
			],
		)
}

/// [`site`] with the roles of the example: an editor publishes pages
/// and deletes events, an author only edits events.
pub fn site_with_roles() -> Schema {
	site()
		.role(
			Role::new("editor", "Editor")
				.sections(["home", "pages"], Access::Publish)
				.section("events", Access::Delete)
				.assets(Access::Delete)
				.ai(),
		)
		.role(
			Role::new("author", "Author")
				.section("events", Access::Edit)
				.section("pages", Access::View)
				.assets(Access::Edit),
		)
		.graphql_schema(
			GraphqlSchema::new("website", "Website")
				.sections(ALL)
				.assets()
				.users()
				.previews(),
		)
		.graphql_schema(
			GraphqlSchema::new("live", "Without drafts")
				.sections(ALL)
				.assets(),
		)
		.graphql_schema(
			GraphqlSchema::new("english", "English")
				.sections(ALL)
				.sites(["en"])
				.assets(),
		)
		.graphql_schema(
			GraphqlSchema::new("events", "Events").sections(["events"]),
		)
		.graphql_schema(
			GraphqlSchema::new("eventsDe", "Events in German")
				.sections(["events"])
				.sites(["de"]),
		)
		.graphql_schema(
			GraphqlSchema::new("pages", "Pages")
				.sections(["pages"])
				.users(),
		)
}

/// The sections of [`site`].
const ALL: [&str; 3] = ["home", "pages", "events"];

/// A black PNG of a size.
pub fn png(width: u32, height: u32) -> Vec<u8> {
	let mut png = vec![];
	RgbImage::new(width, height)
		.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
		.unwrap();
	png
}
