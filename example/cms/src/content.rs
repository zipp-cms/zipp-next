//! The content model of the example site: sites, sections, entry types,
//! blocks, plugins and roles.

use zipp::{
	Access, ButtonOption, Column, Condition, Field, Format, LinkKind, Role,
	Schema, Section, Style, Transform,
};
use zipp_forms::{Form, FormField, Forms};
use zipp_seo::{Seo, SeoDefaults, seo_field};

pub fn schema() -> Schema {
	// like the crelte tutorial: a page starts with its intro, the content
	// is blocks
	let intro = || {
		vec![
			Field::rich_text("intro")
				.links()
				.translatable()
				.instructions(
					"The first lines of the page, also its description for search engines.",
				),
			Field::images("hero").max(1).instructions(
				"Wide, at least **1600 px**; it is cropped around the focal point.",
			),
		]
	};
	let content = || {
		Field::blocks(
			"content",
			["text", "images", "video", "button", "events", "contact"],
		)
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
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		// formatted dates (`@formatDateTime`) are in Swiss time
		.timezone("Europe/Zurich")
		// the pages like the crelte tutorial: the home page is the page of
		// type home, the news page lists the articles
		.section(
			// a new page is a page, the home and news page are there once
			Section::structure("pages", ["page", "home", "news"])
				.uri("{parent.uri}/{slug}")
				.uri_of("home", ""),
		)
		.section(Section::channel("news", ["article"]).uri("news/{slug}"))
		// no pages: the news page filters by them
		.section(Section::structure("categories", ["category"]).max_level(2))
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
			[
				"header",
				"footer",
				"translations",
				"seoSettings",
				"formSettings",
			],
		)
		// icons of the studio, Lucide's by name or an svg of its own
		.nav_icon("Content", "folder-open")
		.nav_icon("Settings", "sliders-horizontal")
		// a contact form like valgistesshots', mailed to who Forms names
		.plugin(
			Forms::new().form(
				Form::new("contact", "Contact")
					.field(FormField::text("name").required())
					.field(FormField::email("email").required().reply_to())
					.field(FormField::text("phone"))
					.field(FormField::multiline("message").required()),
			),
		)
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
					[
						"pages",
						"header",
						"footer",
						"translations",
						"formSettings",
					],
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
				("contact.name", "Name"),
				("contact.email", "E-Mail"),
				("contact.phone", "Telefon"),
				("contact.message", "Nachricht"),
				("contact.send", "Senden"),
				(
					"contact.failed",
					"Das hat nicht geklappt, bitte später nochmals.",
				),
			],
		)
		// on every image and file beside title and alt, GraphQL has them
		// on Asset
		.asset_fields([
			Field::text("credit")
				.width(50)
				.instructions("Who made it, shown under the image."),
			Field::dropdown(
				"licence",
				[("own", "Own"), ("cc", "CC BY"), ("press", "Press image")],
			)
			.width(50),
		])
		// big photos: WebP is a third smaller than a jpeg
		.transform(
			Transform::new("hero")
				.width(1600)
				.height(700)
				.crop()
				.format(Format::Webp),
		)
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
				[
					intro(),
					// an article is in a few categories, not in all
					vec![Field::relation("categories", ["category"]).max(3)],
				]
				.concat(),
				vec![content()],
			),
		)
		.entry("category", "Category", [])
		.entry(
			"event",
			"Event",
			[
				// a day, sorted and filtered as 2026-11-14; in a row with
				// doors and city
				Field::date("date")
					.required()
					.indexed()
					.instructions(
						"Past events leave the list of upcoming ones.",
					)
					.width(25),
				// a day and time, 2026-11-14T19:30; only concerts have
				// doors, the editor shows it when the genre is concert
				Field::date("doors")
					.time()
					.label("Doors open")
					.instructions("Empty: the website shows none.")
					.width(50)
					.when(Condition::is("genre", "concert")),
				Field::text("city").required().indexed().width(25),
				// buttons in colors with icons, GraphQL gives the label,
				// color and icon
				Field::button_group(
					"genre",
					[
						ButtonOption::new("concert", "Konzert")
							.color("#2b6cb0")
							.icon("music"),
						ButtonOption::new("reading", "Lesung")
							.color("#2f855a")
							.icon("book-open"),
						ButtonOption::new("theatre", "Theater")
							.color("#c05621")
							.icon("drama"),
					],
				)
				.required(),
				// some of the options
				Field::checkboxes(
					"audience",
					[
						("kids", "Kinder"),
						("adults", "Erwachsene"),
						("families", "Familien"),
					],
				)
				.label("Für wen")
				.width(50),
				// more options: a select in the studio
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
				// most events are in German, new ones start so
				.default("de")
				.indexed()
				.label("Sprache")
				.width(50),
				// how the image should be, for the editors
				Field::note(
					"imageHint",
					"**Bild** im Querformat (3:2), mindestens 1600 px breit: \
					es steht in der Liste und oben auf der Seite.",
				),
				// new images of events go into their folder
				Field::images("image").max(1).folder("Events"),
				// fields that belong together in a box of their own, only
				// used here: an inline group, the block eventTickets
				Field::group_of(
					"tickets",
					[
						Field::number("price")
							.range(0.0, 1000.0)
							.decimals(2)
							.unit("CHF")
							.width(50),
						Field::boolean("soldOut").width(50),
						// prices by category, typed columns
						Field::table(
							"categories",
							[
								Column::text("name").label("Kategorie"),
								Column::number("price")
									.decimals(2)
									.unit("CHF")
									.label("Preis"),
								Column::boolean("soldOut").label("Ausverkauft"),
							],
						)
						.label("Kategorien")
						.instructions("Leer: es gilt der Preis oben."),
						// where to get on the waiting list, only while sold out
						Field::link(
							"waitlist",
							[LinkKind::Url, LinkKind::Email],
						)
						.label("Warteliste")
						.when(Condition::on("soldOut")),
						// each event's program in a folder of its own
						Field::assets("program")
							.extensions(["pdf"])
							.max(1)
							.folder("Events/{slug}")
							.label("Programm (PDF)"),
					],
				),
				Field::markdown("description").translatable(),
				Field::relation("page", ["page"]).max(1).label("More info"),
			],
		)
		.entry(
			"header",
			"Header",
			// a menu with a second level, to pages (not to articles or
			// events), anywhere or a file
			[Field::navigation(
				"navigation",
				[LinkKind::Entry, LinkKind::Url, LinkKind::Asset],
			)
			.sections(["pages"])
			.translatable()
			.label("Main navigation")],
		)
		.entry(
			"footer",
			"Footer",
			[
				Field::text("text").multiline().translatable(),
				Field::email("email").label("Contact email"),
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
				.embed(["images"])
				// classes for the frontend's CSS, see components/content/text.svelte
				.styles([
					Style::block("lead", "Lead"),
					Style::inline("highlight", "Highlight"),
				])
				// the bar's tools in order, | a line; images get an icon
				// of their own, the Insert menu goes away
				.toolbar([
					"format",
					"styles",
					"|",
					"bold",
					"italic",
					"|",
					"bulletList",
					"orderedList",
					"|",
					"link",
					"button",
					"block:images",
				])],
		)
		// what it shows and how wide, in tabs inside the block
		.block_tabs(
			"images",
			"Images",
			[
				(
					"Content",
					vec![
						Field::images("images").required(),
						Field::text("caption").max(200),
					],
				),
				(
					"Layout",
					// three options: buttons in the studio
					vec![
						Field::dropdown(
							"width",
							[
								("small", "Small"),
								("wide", "Wide"),
								("full", "Full width"),
							],
						)
						.default("wide"),
					],
				),
			],
		)
		.block(
			"video",
			"Video",
			[
				Field::videos("video").max(1).required(),
				Field::images("poster").max(1).instructions(
					"Shown until it plays, empty: its first frame.",
				),
				Field::boolean("loop").instructions(
					"Plays muted and over again, like a moving image.",
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
		// the contact form of zipp-forms, its texts are the frontend's
		.block(
			"contact",
			"Contact form",
			[
				Field::text("title")
					.instructions("Above the form, empty: none."),
			],
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
		.icon("video", "video")
		.icon("button", "mouse-pointer-click")
		.icon("events", "calendar-days")
		.icon("contact", "mail")
		// the field that names a block in its head and the outline
		.block_title("images", "caption")
		.block_title("button", "link")
		.block_title("events", "city")
		// short fields in one card with the labels on the left, like a group
		.tab_card("article", "Intro")
		.entry_card("event")
}
