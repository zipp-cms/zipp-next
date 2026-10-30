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
}

#[test]
fn site_is_valid() {
	site().check().unwrap();
	site_with_roles().check().unwrap();
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
	let none = serde_json::Value::Null;
	let uri = |s: &Section, ty, slug, parent| {
		s.render_uri(ty, slug, parent, "de", &none)
	};
	let s = site();
	let pages = s.get_section("pages").unwrap();
	assert_eq!(uri(pages, "page", "team", None).unwrap(), "team");
	assert_eq!(
		uri(pages, "page", "team", Some("about")).unwrap(),
		"about/team"
	);
	let home = s.get_section("home").unwrap();
	assert_eq!(uri(home, "page", "home", None).unwrap(), "");
	let pages = Section::structure("pages", ["home", "page"])
		.uri("{parent.uri}/{slug}")
		.uri_of("home", "");
	assert_eq!(uri(&pages, "home", "start", None).unwrap(), "");
	assert_eq!(uri(&pages, "page", "start", None).unwrap(), "start");

	// another uri in English, a page only with hasPage on
	let offers = Section::channel("offers", ["offer"])
		.uri("angebote/{slug}")
		.uri_in("en", "offers/{slug}")
		.uri_if("hasPage");
	let on = serde_json::json!({ "hasPage": true });
	let render = |site| offers.render_uri("offer", "star", None, site, &on);
	assert_eq!(render("de").unwrap(), "angebote/star");
	assert_eq!(render("en").unwrap(), "offers/star");
	assert_eq!(offers.render_uri("offer", "star", None, "de", &none), None);
	let checked = site().section(offers.clone()).entry(
		"offer",
		"Offer",
		[Field::boolean("hasPage")],
	);
	checked.check().unwrap();
	let wrong = site().section(offers).entry("offer", "Offer", []);
	assert!(wrong.check().unwrap_err().contains("no boolean hasPage"));
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

#[test]
fn tabs_keep_the_fields_in_order() {
	let news = || {
		site().section(Section::channel("news", ["news"]).uri("news/{slug}"))
	};
	let s = news().entry_tabs(
		"news",
		"News",
		[
			(
				"Content",
				vec![Field::text("lead"), Field::markdown("text")],
			),
			("SEO", vec![Field::text("keywords")]),
		],
	);
	s.check().unwrap();
	let c = s.get("news").unwrap();
	let handles: Vec<_> = c.fields.iter().map(|f| f.handle.as_str()).collect();
	assert_eq!(handles, ["lead", "text", "keywords"]);
	assert_eq!(c.tabs[1].fields, ["keywords"]);
	let empty = news().entry_tabs("news", "News", [("Content", vec![])]);
	assert!(empty.check().unwrap_err().contains("no fields"));
}

#[test]
fn icons_of_types_and_nav_groups() {
	let s = site()
		.icon("event", "calendar")
		.icon("text", Icon::svg("<svg/>"))
		.nav("Content", ["home", "pages", "events"])
		.nav_icon("Content", "folder-open");
	s.check().unwrap();
	assert_eq!(s.get("event").unwrap().icon, Some(Icon::from("calendar")));
	let json = serde_json::to_value(&s).unwrap();
	let event = json["components"]
		.as_array()
		.unwrap()
		.iter()
		.find(|c| c["handle"] == "event")
		.unwrap();
	assert_eq!(event["icon"], serde_json::json!({ "lucide": "calendar" }));
	assert_eq!(json["nav"][0]["icon"]["lucide"], "folder-open");
	// without one the json stays as before
	let page = json["components"].as_array().unwrap();
	assert!(
		page.iter()
			.any(|c| c["handle"] == "page" && c["icon"].is_null())
	);
}

#[test]
#[should_panic(expected = "there is no entry type or block nope")]
fn icon_of_an_unknown_type() {
	let _ = site().icon("nope", "calendar");
}

#[test]
fn asset_fields_with_a_folder_and_extensions() {
	let f = Field::assets("docs")
		.folder("/Docs/Forms/")
		.extensions([".PDF"]);
	let FieldKind::Assets {
		folder, extensions, ..
	} = &f.kind
	else {
		unreachable!()
	};
	assert_eq!(folder.as_deref(), Some("Docs/Forms"));
	assert_eq!(extensions, &["pdf"]);
	site().block("d", "D", [f]).check().unwrap();

	let s = site().block("d", "D", [Field::assets("x").folder("A//B")]);
	assert!(s.check().unwrap_err().contains("a path like"));
	let s = site().block("d", "D", [Field::assets("x").extensions(["p f"])]);
	assert!(s.check().unwrap_err().contains("like pdf"));
}

#[test]
fn a_plugin_once() {
	struct Twice;
	impl crate::Plugin for Twice {
		fn name(&self) -> &'static str {
			"twice"
		}
	}
	let s = site().plugin(Twice).plugin(Twice);
	assert_eq!(s.check().unwrap_err(), "two plugins or field types twice");
}
