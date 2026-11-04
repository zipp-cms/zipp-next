use super::*;
use crate::fixtures::{site, site_with_roles};

#[test]
fn site_is_valid() {
	site().check().unwrap();
	site_with_roles().check().unwrap();
}

#[test]
fn humanizes_handles() {
	assert_eq!(humanize("eventDate"), "Event date");
	assert_eq!(humanize("first_name"), "First name");
	assert_eq!(Field::text("intro").label, "Intro");
}

#[test]
fn finds_mistakes() {
	let s = site().block("text", "Again", []);
	assert!(s.check().unwrap_err().contains("twice"));

	let s = site().block("x", "X", [Field::blocks("b", ["page"])]);
	assert_eq!(s.check().unwrap_err(), "x.b: page is not a block");

	// groups in a circle
	let s = site().block("ga", "A", [Field::group("b", "gb")]).block(
		"gb",
		"B",
		[Field::group("a", "ga")],
	);
	assert_eq!(
		s.check().unwrap_err(),
		"ga.b: a group can't hold its own component"
	);

	let s = site().block("y", "Y", [Field::text("t").indexed()]);
	assert!(s.check().unwrap_err().contains("only fields of entries"));

	let s = site().entry("Bad", "Bad", []);
	assert!(s.check().unwrap_err().contains("camelCase"));

	let s = site().image_sizes([(800, 0)]);
	assert_eq!(s.check().unwrap_err(), "image sizes can't be 0");

	let s = site().timezone("Europe/Nowhere");
	assert_eq!(
		s.check().unwrap_err(),
		"timezone \"Europe/Nowhere\" is unknown"
	);
	site().timezone("Europe/Zurich").check().unwrap();

	let s = site().entry("loose", "Loose", []);
	assert_eq!(s.check().unwrap_err(), "entry type loose is in no section");

	let s = site().section(Section::channel("news", ["page"]).uri("news"));
	assert_eq!(s.check().unwrap_err(), "section news: the uri needs {slug}");

	let s = site().section(Section::structure("x", ["page"]).max_level(0));
	assert_eq!(
		s.check().unwrap_err(),
		"section x: the max level is at least 1"
	);
	let s = site().section(Section::channel("x", ["page"]).max_level(2));
	assert_eq!(
		s.check().unwrap_err(),
		"section x: only structures have a max level"
	);

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
fn asset_fields() {
	let s = site().asset_fields([Field::text("credit"), Field::videos("clip")]);
	s.check().unwrap();
	let FieldKind::Assets { extensions, .. } =
		&s.get(ASSET).unwrap().fields[1].kind
	else {
		panic!()
	};
	assert_eq!(extensions, &["mp4", "webm", "mov"]);

	let with = |f: Field| site().asset_fields([f]).check().unwrap_err();
	assert_eq!(with(Field::text("url")), "field asset.url: url is built in");
	assert!(with(Field::text("t").translatable()).contains("translatable"));
	assert!(with(Field::text("t").default("x")).contains("no defaults"));

	// they live on assets only, not in content
	let s = site().asset_fields([Field::text("credit")]).block(
		"x",
		"X",
		[Field::blocks("b", ["asset"])],
	);
	assert!(s.check().unwrap_err().contains("holds the asset fields"));
	let s = site()
		.block("x", "X", [Field::group("g", "asset")])
		.asset_fields([]);
	assert!(s.check().unwrap_err().contains("holds the asset fields"));
}

#[test]
fn widths_and_notes() {
	let with = |f: Field| site().block("x", "X", [f]).check();
	with(Field::text("t").width(25)).unwrap();
	assert_eq!(
		with(Field::text("t").width(30)).unwrap_err(),
		"x.t: the width is 25, 50, 75 or 100"
	);
	// 100 is the default and stays out of the JSON
	let json = serde_json::to_value(Field::text("t")).unwrap();
	assert!(json.get("width").is_none());
	let json = serde_json::to_value(Field::text("t").width(50)).unwrap();
	assert_eq!(json["width"], 50);

	with(Field::note("n", "**Bilder** im Querformat")).unwrap();
	assert_eq!(
		with(Field::note("n", " ")).unwrap_err(),
		"x.n: a note needs a text"
	);
	assert!(
		with(Field::note("n", "Hi").required())
			.unwrap_err()
			.contains("a note holds no value")
	);
	assert!(
		with(Field::note("n", "Hi").default("x"))
			.unwrap_err()
			.contains("only fields of plain values")
	);
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
	// only one type has pages, like Craft's `{% if type == 'blog' %}`
	let blogs = Section::channel("blogs", ["blog", "video"])
		.uri_of("blog", "blog/{slug}");
	assert_eq!(uri(&blogs, "blog", "a", None).unwrap(), "blog/a");
	assert_eq!(uri(&blogs, "video", "a", None), None);

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
fn blocks_have_tabs_too() {
	let s = site()
		.block_tabs(
			"teaser",
			"Teaser",
			[
				(
					"Content",
					vec![
						Field::text("heading"),
						Field::group_of("link", [Field::text("url")]),
					],
				),
				("Layout", vec![Field::boolean("wide")]),
			],
		)
		.tab_when("teaser", "Layout", Condition::filled("heading"));
	s.check().unwrap();

	let teaser = s.get("teaser").unwrap();
	assert!(!teaser.entry);
	let handles: Vec<_> =
		teaser.fields.iter().map(|f| f.handle.as_str()).collect();
	assert_eq!(handles, ["heading", "link", "wide"]);
	assert_eq!(teaser.tabs[0].fields, ["heading", "link"]);
	assert_eq!(teaser.tabs[1].fields, ["wide"]);
	assert!(teaser.tabs[1].when.is_some());
	// the inline group is hoisted after the block, the tabs stay on it
	let link = s.get("teaserLink").unwrap();
	assert_eq!(link.part_of.as_deref(), Some("teaser"));
	assert!(link.tabs.is_empty());

	// a hidden tab hides its fields like in an entry
	let values = serde_json::json!({ "wide": true });
	assert!(!teaser.is_shown("wide", values.as_object().unwrap()));
}

#[test]
#[should_panic(expected = "no entry type teaser")]
fn blocks_have_no_tab_cards() {
	let _ = site()
		.block_tabs("teaser", "Teaser", [("Content", vec![Field::text("a")])])
		.tab_card("teaser", "Content");
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
fn a_field_names_a_block() {
	let s = site().block_title("button", "label");
	s.check().unwrap();
	assert_eq!(
		s.get("button").unwrap().title_field.as_deref(),
		Some("label")
	);
	let json = serde_json::to_value(s.get("button").unwrap()).unwrap();
	assert_eq!(json["title_field"], "label");
	// a relation gives its entry's title
	site().block_title("button", "link").check().unwrap();

	let err = site().block_title("button", "nope").check().unwrap_err();
	assert_eq!(err, "button: title nope: the block has no such field");
	let s = site().block("gallery", "Gallery", [Field::images("images")]);
	let err = s.block_title("gallery", "images").check().unwrap_err();
	assert!(err.starts_with("gallery: title images: only text"), "{err}");
}

#[test]
#[should_panic(expected = "block_title: there is no block event")]
fn only_blocks_get_a_title_field() {
	let _ = site().block_title("event", "city");
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

#[test]
fn a_plugin_says_what_it_added() {
	struct Forms;
	impl crate::Plugin for Forms {
		fn name(&self) -> &'static str {
			"forms"
		}

		fn schema(&self, schema: Schema) -> Schema {
			schema.section(Section::single("forms", "forms")).entry(
				"forms",
				"Forms",
				[Field::email("to")],
			)
		}

		fn finish(&self, mut schema: Schema) -> Schema {
			let page =
				schema.components.iter_mut().find(|c| c.handle == "page");
			page.unwrap().fields.push(Field::text("formTitle"));
			schema
		}
	}
	let s = site().plugin(Forms).finish();
	let added = &s.plugins.added()[0];
	assert_eq!(added.plugin, "forms");
	assert_eq!(added.sections, ["forms"]);
	assert_eq!(added.components, ["forms"]);
	// the new type's own fields aren't listed, the one in page is
	assert_eq!(added.fields, ["page.formTitle"]);
	assert!(added.transforms.is_empty());
}

/// A block with a switch, a dropdown and fields that hang on them.
fn with_conditions(fields: Vec<Field>) -> Result<(), String> {
	let mut all = vec![
		Field::boolean("hasLink"),
		Field::dropdown("kind", [("talk", "Talk"), ("concert", "Concert")]),
		Field::text("note"),
	];
	all.extend(fields);
	site().block("x", "X", all).check()
}

#[test]
fn checks_conditions() {
	with_conditions(vec![
		Field::text("url").when(Condition::on("hasLink")),
		Field::text("doors").when(Condition::one_of("kind", ["concert"])),
		Field::text("more").when(Condition::filled("note")),
	])
	.unwrap();

	let err = |f: Field| with_conditions(vec![f]).unwrap_err();
	assert_eq!(
		err(Field::text("u").when(Condition::on("nope"))),
		"x.u: condition: unknown field nope"
	);
	assert_eq!(
		err(Field::text("u").when(Condition::filled("u"))),
		"x.u: condition: it can't look at u itself"
	);
	assert_eq!(
		err(Field::text("u").when(Condition::is("kind", "film"))),
		"x.u: condition: kind can't be \"film\""
	);
	assert_eq!(
		err(Field::text("u").when(Condition::is("hasLink", "yes"))),
		"x.u: condition: hasLink can't be \"yes\""
	);
	assert!(
		err(Field::text("u").when(Condition::is("note", "x"))).contains(
			"note is no boolean, dropdown, button group or field type"
		)
	);

	// a and b hide each other
	let s = site().block(
		"x",
		"X",
		[
			Field::text("a").when(Condition::filled("b")),
			Field::text("b").when(Condition::empty("a")),
		],
	);
	assert_eq!(
		s.check().unwrap_err(),
		"x: conditions go in a circle: a → b → a"
	);

	// the JSON of the studio and the migrations
	let json =
		serde_json::to_value(Field::text("url").when(Condition::on("hasLink")))
			.unwrap();
	assert_eq!(
		json["when"],
		serde_json::json!({ "test": "oneOf", "field": "hasLink", "values": [true] })
	);
}

#[test]
fn tabs_have_conditions() {
	let tabs = |when: Condition| {
		site()
			.section(Section::channel("news", ["news"]))
			.entry_tabs(
				"news",
				"News",
				[
					("Content", vec![Field::boolean("hasPage")]),
					("Page", vec![Field::text("intro")]),
				],
			)
			.tab_when("news", "Page", when)
	};
	let s = tabs(Condition::on("hasPage"));
	s.check().unwrap();
	let news = s.get("news").unwrap();
	let on = serde_json::json!({ "hasPage": true });
	assert!(news.is_shown("intro", on.as_object().unwrap()));
	assert!(!news.is_shown("intro", &Default::default()));

	assert_eq!(
		tabs(Condition::filled("intro")).check().unwrap_err(),
		"news: tab Page: condition: it can't look at intro itself"
	);
}

#[test]
fn shows_fields_by_their_conditions() {
	let s = site().block(
		"x",
		"X",
		[
			Field::boolean("hasLink"),
			Field::dropdown("kind", [("a", "A"), ("b", "B")])
				.when(Condition::on("hasLink")),
			Field::text("b").when(Condition::is("kind", "b")),
			Field::text("off").when(Condition::off("hasLink")),
		],
	);
	let x = s.get("x").unwrap();
	let shown =
		|v: serde_json::Value, h: &str| x.is_shown(h, v.as_object().unwrap());
	// an unset boolean is off
	assert!(shown(serde_json::json!({}), "off"));
	assert!(!shown(serde_json::json!({}), "kind"));
	assert!(shown(
		serde_json::json!({ "hasLink": true, "kind": "b" }),
		"b"
	));
	// b's dropdown is hidden, so is b, whatever it holds
	assert!(!shown(
		serde_json::json!({ "hasLink": false, "kind": "b" }),
		"b"
	));
	assert!(shown(serde_json::json!({ "hasLink": false }), "hasLink"));
}

#[test]
fn inline_groups_become_blocks_of_their_own() {
	let s = site().entry_tabs(
		"concert",
		"Concert",
		[(
			"Content",
			vec![
				Field::group_of(
					"tickets",
					[
						Field::number("price").width(50),
						Field::group_of("sale", [Field::date("start")]),
					],
				)
				.label("Tickets and sale"),
			],
		)],
	);
	let s = s.section(Section::channel("concerts", ["concert"]));
	s.check().unwrap();

	let concert = s.get("concert").unwrap();
	assert_eq!(
		concert.fields[0].kind,
		FieldKind::Group {
			component: "concertTickets".into()
		}
	);
	assert_eq!(concert.tabs[0].fields, ["tickets"]);
	let tickets = s.get("concertTickets").unwrap();
	assert!(!tickets.entry);
	assert_eq!(tickets.name, "Tickets and sale");
	assert_eq!(tickets.part_of.as_deref(), Some("concert"));
	assert_eq!(tickets.fields[0].width, 50);
	let sale = s.get("concertTicketsSale").unwrap();
	assert_eq!(sale.part_of.as_deref(), Some("concertTickets"));
	assert_eq!(sale.fields[0].handle, "start");

	// the schema of a migration keeps where it belongs
	let json = serde_json::to_value(tickets).unwrap();
	assert_eq!(json["part_of"], "concert");
	assert!(serde_json::to_value(concert).unwrap()["part_of"].is_null());
}

#[test]
fn an_inline_group_named_like_a_block() {
	let s = site()
		.section(Section::channel("news", ["news"]))
		.block("newsTickets", "Tickets", [Field::text("info")])
		.entry("news", "News", [Field::group_of("tickets", [])]);
	assert_eq!(
		s.check().unwrap_err(),
		"component newsTickets exists twice, an inline group of news makes it"
	);
}

#[test]
fn columns_and_styles() {
	use crate::schema::{Column, Style};
	let with = |f: Field| site().block("x", "X", [f]).check();
	let table = |c: Column| with(Field::table("t", [c]));
	table(Column::number("n").decimals(2).unit("CHF")).unwrap();
	assert_eq!(
		table(Column::number("n").decimals(11)).unwrap_err(),
		"x.t: column n: decimals go from 0 to 10"
	);
	assert_eq!(
		table(Column::dropdown("d", [])).unwrap_err(),
		"x.t: column d: it needs options"
	);
	assert_eq!(
		table(Column::link("l", [])).unwrap_err(),
		"x.t: column l: a link needs something to point to"
	);

	let rich = |styles: Vec<Style>| with(Field::rich_text("r").styles(styles));
	rich(vec![
		Style::block("lead", "Lead"),
		Style::inline("text-big", "Big"),
	])
	.unwrap();
	assert_eq!(
		rich(vec![Style::block("1x", "X")]).unwrap_err(),
		"x.r: style \"1x\" must be a class like lead or text-large"
	);
	assert_eq!(
		rich(vec![Style::block("a", "A"), Style::inline("a", "B")])
			.unwrap_err(),
		"x.r: style a exists twice"
	);
}

#[test]
fn entries_are_saved_as_a_type_of_their_section() {
	let s = site();
	let (section, comp) = s.section_type("events", "event").unwrap();
	assert_eq!((&*section.handle, &*comp.handle), ("events", "event"));
	let err = s.section_type("events", "page").unwrap_err();
	assert!(err.to_string().ends_with("no type page in section events"));
	assert!(s.section_type("nothing", "event").is_err());
}

#[test]
fn sections_of_relations_and_links() {
	let with = |f: Field| {
		site()
			.section(Section::channel("news", ["page"]))
			.block("x", "X", [f])
			.check()
	};
	let rel = || Field::relation("r", ["page"]);
	with(rel().sections(["pages", "home"])).unwrap();
	assert_eq!(
		with(rel().sections(["nope"])).unwrap_err(),
		"x.r: unknown section nope"
	);
	assert_eq!(
		with(rel().sections(["pages", "pages"])).unwrap_err(),
		"x.r: section pages is there twice"
	);
	assert_eq!(
		with(rel().sections(["events"])).unwrap_err(),
		"x.r: page is in none of the sections"
	);
	assert_eq!(
		with(Field::relation("r", ["page", "event"]).sections(["pages"]))
			.unwrap_err(),
		"x.r: event is in none of the sections"
	);
	assert_eq!(
		with(Field::relation("r", ["event"]).sections(["events", "pages"]))
			.unwrap_err(),
		"x.r: section pages has none of the types"
	);
	// a relation may take entries without a page
	with(rel().sections(["news"])).unwrap();

	let link = |to: &[LinkKind]| Field::link("l", to.iter().copied());
	with(link(&[LinkKind::Entry, LinkKind::Url]).sections(["pages"])).unwrap();
	assert_eq!(
		with(link(&[LinkKind::Url]).sections(["pages"])).unwrap_err(),
		"x.l: sections are for links to entries"
	);
	assert_eq!(
		with(link(&[LinkKind::Entry]).sections(["news"])).unwrap_err(),
		"x.l: section news has no pages"
	);
	let nav = Field::navigation("n", [LinkKind::Entry]).sections(["events"]);
	with(nav).unwrap();
	let nav = Field::navigation("n", [LinkKind::Url]).sections(["events"]);
	assert_eq!(
		with(nav).unwrap_err(),
		"x.n: sections are for links to entries"
	);

	with(Field::rich_text("t").buttons().sections(["pages"])).unwrap();
	assert_eq!(
		with(Field::rich_text("t").sections(["pages"])).unwrap_err(),
		"x.t: sections are for links and buttons"
	);

	// none is the default and stays out of the JSON
	let json = serde_json::to_value(rel()).unwrap();
	assert!(json.get("sections").is_none());
	let json = serde_json::to_value(rel().sections(["pages"])).unwrap();
	assert_eq!(json["sections"], serde_json::json!(["pages"]));
}
