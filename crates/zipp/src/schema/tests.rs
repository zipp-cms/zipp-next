use super::*;

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
				Field::number("price").indexed(),
				Field::relation("page", ["page"]).max(1),
				Field::images("image").max(1),
				Field::markdown("notes"),
				Field::link(
					"ticket",
					[LinkKind::Url, LinkKind::Entry, LinkKind::Email],
				),
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

#[test]
fn site_is_valid() {
	site().check().unwrap();
}

#[test]
fn humanizes_handles() {
	assert_eq!(humanize("eventDate"), "Event date");
	assert_eq!(Field::text("intro").label, "Intro");
}

#[test]
fn finds_mistakes() {
	let s = site().block("text", "Again", []);
	assert!(s.check().unwrap_err().contains("twice"));

	let s = site().block("x", "X", [Field::blocks("b", ["page"])]);
	assert_eq!(s.check().unwrap_err(), "x.b: page is not a block");

	let s = site().block("y", "Y", [Field::text("t").indexed()]);
	assert!(s.check().unwrap_err().contains("only fields of entries"));

	let s = site().entry("Bad", "Bad", []);
	assert!(s.check().unwrap_err().contains("camelCase"));

	let s = site().entry("loose", "Loose", []);
	assert_eq!(s.check().unwrap_err(), "entry type loose is in no section");

	let s = site().section(Section::channel("news", ["page"]).uri("news"));
	assert_eq!(s.check().unwrap_err(), "section news: the uri needs {slug}");

	let s = site().nav("Content", ["home", "pages"]);
	assert_eq!(
		s.check().unwrap_err(),
		"section events is in 0 nav groups, it needs one"
	);

	let s = site().section(Section::channel("x", ["page"]).uri("{id}/{slug}"));
	assert!(s.check().unwrap_err().contains("knows only"));

	// a doc goes into the nav like a section, but not with its name
	let s = site()
		.doc("readme", "Readme", "# Hi")
		.nav("Content", ["readme", "home", "pages", "events"]);
	s.check().unwrap();
	let s = site().doc("pages", "Pages", "");
	assert_eq!(s.check().unwrap_err(), "doc pages exists twice");
}

#[test]
fn renders_uris() {
	let s = site();
	let pages = s.get_section("pages").unwrap();
	assert_eq!(pages.render_uri("page", "team", None).unwrap(), "team");
	assert_eq!(
		pages.render_uri("page", "team", Some("about")).unwrap(),
		"about/team"
	);
	let home = s.get_section("home").unwrap();
	assert_eq!(home.render_uri("page", "home", None).unwrap(), "");
	let pages = Section::structure("pages", ["home", "page"])
		.uri("{parent.uri}/{slug}")
		.uri_of("home", "");
	assert_eq!(pages.render_uri("home", "start", None).unwrap(), "");
	assert_eq!(pages.render_uri("page", "start", None).unwrap(), "start");
}

#[test]
fn serializes_flat() {
	let f = Field::relation("link", ["page"]).max(1);
	let v = serde_json::to_value(&f).unwrap();
	assert_eq!(v["kind"], "relation");
	assert_eq!(v["types"][0], "page");
	let back: Field = serde_json::from_value(v).unwrap();
	assert_eq!(back, f);
}
