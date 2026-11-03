use serde_json::json;

use super::*;
use crate::db::Db;
use crate::schema::{Column, LinkKind, Section};

fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.section(
			Section::structure("pages", ["page"]).uri("{parent.uri}/{slug}"),
		)
		.section(Section::channel("tags", ["tag"]))
		.section(Section::single("footer", "footer"))
		.entry(
			"page",
			"Page",
			[
				Field::rich_text("text").lists().links().buttons(),
				Field::relation("tags", ["tag"]),
				Field::blocks("sections", ["section"]),
				Field::group("teaser", "teaser"),
				Field::number("price"),
			],
		)
		.entry("tag", "Tag", [])
		.entry(
			"footer",
			"Footer",
			[Field::navigation(
				"navigation",
				[LinkKind::Entry, LinkKind::Url],
			)],
		)
		.block(
			"section",
			"Section",
			[Field::text("title").required(), Field::text("text")],
		)
		.block(
			"teaser",
			"Teaser",
			[Field::link("link", [LinkKind::Entry, LinkKind::Url])],
		)
}

fn link(ty: &str, id: Option<i64>, value: Option<&str>) -> Value {
	json!({
		"type": ty, "id": id, "value": value, "label": null,
		"target": null, "download": false,
	})
}

fn export() -> Export {
	let about = json!({
		"id": 10, "type": "page", "title": "Über uns", "slug": "ueber-uns",
		"enabled": true, "parentId": null,
		"postDate": "2020-05-01T10:00:00+02:00",
		"fields": {
			"text": {
				"html": "<p>Hi <a href=\"{entry:11@1:url||https://old/team}\">team</a></p>\
					<craft-entry data-entry-id=\"90\"></craft-entry>\
					<craft-entry data-entry-id=\"91\"></craft-entry>",
				"entries": [
					{ "id": 90, "type": "btn", "enabled": true,
						"fields": { "btnLink": link("entry", Some(11), None) } },
					{ "id": 91, "type": "btn", "enabled": true,
						"fields": { "btnLink": link("url", None, Some("https://x.ch")) } },
				],
			},
			"tags": [20, 99],
			"sections": [
				{ "id": 30, "type": "section", "title": "One", "enabled": true,
					"fields": { "text": "first" } },
				{ "id": 31, "type": "section", "title": "Off", "enabled": false,
					"fields": {} },
				{ "id": 32, "type": "unknown", "title": "", "enabled": true,
					"fields": {} },
			],
			"teaser": { "fields": { "link": link("entry", Some(11), None) } },
			"price": "12",
			"craftOnly": "dropped",
		},
	});
	let team = json!({
		"id": 11, "type": "page", "title": "Team", "slug": "team",
		"enabled": false, "parentId": 10, "fields": {},
	});
	serde_json::from_value(json!({
		"sites": [{ "handle": "default" }],
		"sections": {
			"pages": { "type": "structure", "sites": { "default": [about, team] } },
			"news": { "type": "channel", "sites": { "default": [] } },
		},
		"categories": {
			"tags": { "default": [{ "id": 20, "title": "Filz", "slug": "filz",
				"enabled": true, "fields": [] }] },
		},
		"globals": {
			"footer": { "default": { "fields": {
				"navigation": [
					{ "id": 40, "type": "navItem", "enabled": true, "fields": {
						"navLink": link("entry", Some(10), None),
						"subNavigation": [
							{ "id": 41, "type": "sub", "enabled": true, "fields": {
								"navLink": json!({ "type": "url", "id": null,
									"value": "https://insta.com", "label": "Insta",
									"target": "_blank", "download": false }),
							} },
						],
					} },
				],
			} } },
		},
		"assets": [{ "id": 50, "volume": "default", "path": "a.jpg",
			"title": "A", "alt": null, "focalPoint": null }],
	}))
	.unwrap()
}

#[test]
fn imports_a_site() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let schema = schema();
	crate::migrate::apply::create_singles(&conn, &schema).unwrap();
	let opts = Options {
		sites: vec![],
		volumes: HashMap::new(),
	};
	let files = tempfile::tempdir().unwrap();

	let report =
		import(&conn, &schema, files.path(), &export(), &opts).unwrap();
	assert_eq!(report.entries, 3);
	assert!(report.skipped.iter().any(|s| s.contains("section news")));
	assert!(report.skipped.iter().any(|s| s.contains("volume default")));
	assert!(
		report
			.skipped
			.iter()
			.any(|s| s.contains("block type unknown"))
	);

	let (pages, _) = entries::list(
		&conn,
		&crate::entries::query::Query::parse(
			&schema,
			&[("section".into(), "pages".into())],
		)
		.unwrap(),
	)
	.unwrap();
	let about = pages.iter().find(|e| e.title == "Über uns").unwrap();
	let team = pages.iter().find(|e| e.title == "Team").unwrap();
	assert_eq!(team.parent_id, Some(about.id));
	assert_eq!(team.uri.as_deref(), Some("ueber-uns/team"));
	assert!(about.published && !team.published);

	let f = &about.fields;
	let text = &f["text"]["content"];
	assert_eq!(
		text[0]["content"][1]["marks"][0]["attrs"]["link"],
		json!({ "type": "entry", "value": team.id })
	);
	// two nested link entries are two buttons, one row on the website
	assert_eq!(text[1]["type"], "button");
	assert_eq!(text[2]["type"], "button");
	assert_eq!(text[2]["attrs"]["link"]["value"], "https://x.ch");
	assert_eq!(f["tags"].as_array().unwrap().len(), 1);
	// a disabled entry of a matrix is a switched off block
	let sections = f["sections"].as_array().unwrap();
	assert_eq!(sections.len(), 2);
	assert_eq!(sections[0]["fields"]["title"], "One");
	assert_eq!(sections[0].get("enabled"), None);
	assert_eq!(sections[1]["enabled"], false);
	assert_eq!(f["teaser"]["link"]["value"], team.id);
	assert_eq!(f["price"], 12.0);
	assert!(f.get("craftOnly").is_none());

	let created: i64 = conn
		.query_row(
			"SELECT created_at FROM entries WHERE id = ?1",
			[about.id],
			|r| r.get(0),
		)
		.unwrap();
	assert_eq!(created, parse_iso("2020-05-01T08:00:00Z").unwrap());

	let footer: String = conn
		.query_row(
			"SELECT s.fields FROM entry_sites s JOIN entries e
			ON e.id = s.entry_id WHERE e.section = 'footer'",
			[],
			|r| r.get(0),
		)
		.unwrap();
	let footer: Value = serde_json::from_str(&footer).unwrap();
	// the url could be one of the sites' own, which GraphQL doesn't open
	// in a new tab: the link says it
	assert_eq!(
		footer["navigation"],
		json!([{
			"type": "entry", "value": about.id,
			"children": [{ "type": "url", "value": "https://insta.com",
				"label": "Insta", "newTab": true }],
		}])
	);

	// only into an empty database
	assert!(import(&conn, &schema, files.path(), &export(), &opts).is_err());
}

#[test]
fn fills_groups_from_flat_fields() {
	let schema = Schema::new()
		.site("de", "Deutsch", "de-CH")
		.section(Section::channel("mappen", ["mappe"]))
		.entry(
			"mappe",
			"Mappe",
			[
				Field::number("year"),
				Field::group("sale", "sale"),
				Field::group_of(
					"notes",
					[
						Field::text("year"),
						Field::text("note"),
						Field::text("author"),
					],
				),
				Field::group_of("extra", [Field::text("note")]),
				Field::blocks("sections", ["section"]),
			],
		)
		.block(
			"sale",
			"Sale",
			[
				Field::number("price"),
				Field::number("priceForeign"),
				Field::text("format"),
			],
		)
		.block(
			"section",
			"Section",
			[Field::group_of("contact", [Field::text("heading")])],
		);
	schema.check().unwrap();
	let export: Export = serde_json::from_value(json!({
		"sites": [{ "handle": "default" }],
		"sections": { "mappen": { "sites": { "default": [{
			"id": 1, "type": "mappe", "title": "A", "slug": "a",
			"enabled": true, "parentId": null,
			"fields": {
				"year": 2020, "price": "12", "priceForeign": 15,
				"format": "A4", "saleFormat": "A3", "note": "x",
				"author": "Sue",
				"sections": [{ "id": 2, "type": "section", "enabled": true,
					"fields": { "contactHeading": "Hi" } }],
			},
		}] } } },
	}))
	.unwrap();
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let opts = Options {
		sites: vec![],
		volumes: HashMap::new(),
	};

	let files = tempfile::tempdir().unwrap();
	let report = import(&conn, &schema, files.path(), &export, &opts).unwrap();
	assert_eq!(report.entries, 1);
	// a Craft field for two places or two for one fill nothing
	assert_eq!(
		report.skipped,
		[
			"mappe: the Craft fields format and saleFormat fit sale.format",
			"mappe: the Craft field note fits notes.note and extra.note",
		]
	);

	let fields: String = conn
		.query_row("SELECT fields FROM entry_sites", [], |r| r.get(0))
		.unwrap();
	let fields: Value = serde_json::from_str(&fields).unwrap();
	assert_eq!(fields["year"], 2020);
	assert_eq!(fields["sale"], json!({ "price": 12.0, "priceForeign": 15 }));
	// year is a field of the entry type, not of the group
	assert_eq!(fields["notes"], json!({ "author": "Sue" }));
	assert!(fields.get("extra").is_none());
	// in a block too, and with the group's handle before the field's
	assert_eq!(
		fields["sections"][0]["fields"]["contact"],
		json!({ "heading": "Hi" })
	);
}

#[test]
fn parses_iso_dates() {
	assert_eq!(parse_iso("1970-01-01T00:00:00Z"), Some(0));
	assert_eq!(parse_iso("1970-01-01T01:00:00+01:00"), Some(0));
	assert_eq!(
		parse_iso("2026-10-27T17:04:08-07:00"),
		parse_iso("2026-10-28T00:04:08Z")
	);
	assert_eq!(
		crate::db::iso(parse_iso("2024-02-29T12:30:00").unwrap()),
		"2024-02-29T12:30:00+00:00"
	);
	assert_eq!(parse_iso("yesterday"), None);
}

#[test]
fn imports_the_alt_text_per_site() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let schema = Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en");
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let volume = dir.join("volume");
	fs::create_dir_all(&volume).unwrap();
	fs::write(volume.join("a.txt"), "a").unwrap();
	let export: Export = serde_json::from_value(json!({
		"sites": [{ "handle": "deCH" }, { "handle": "en" }],
		"assets": [
			{ "id": 1, "volume": "default", "path": "a.txt", "title": null,
				"alt": "Ein A", "alts": { "deCH": "Ein A", "en": "An A" },
				"focalPoint": null },
			// an export from before the alt text per site
			{ "id": 2, "volume": "default", "path": "a.txt", "title": null,
				"alt": "Noch ein A", "focalPoint": null },
			// nothing out of the volume
			{ "id": 3, "volume": "default", "path": "../secret.txt",
				"title": null, "alt": null, "focalPoint": null },
			{ "id": 4, "volume": "default", "path": "/etc/hostname",
				"title": null, "alt": null, "focalPoint": null },
		],
	}))
	.unwrap();
	let opts = Options {
		sites: vec![("deCH".into(), "de".into()), ("en".into(), "en".into())],
		volumes: HashMap::from([(
			"default".into(),
			Volume {
				dir: volume,
				folder: None,
			},
		)]),
	};

	let files = dir.join("files");
	fs::write(dir.join("secret.txt"), "s").unwrap();
	let report = import(&conn, &schema, &files, &export, &opts).unwrap();
	assert_eq!(report.assets, 2);
	let out = report.skipped.iter().filter(|s| s.contains("out of the"));
	assert_eq!(out.count(), 2, "{:?}", report.skipped);
	let alt = |id| assets::get_asset(&conn, id).unwrap().alt;
	assert_eq!(
		alt(1),
		[("de".into(), "Ein A".into()), ("en".into(), "An A".into())].into()
	);
	assert_eq!(alt(2), [("de".into(), "Noch ein A".into())].into());
}

/// zipp-seo's field type, which depends on zipp: its values as they are.
struct Seo;

impl crate::FieldType for Seo {
	fn name(&self) -> &'static str {
		"seo"
	}

	fn validate(
		&self,
		_options: &Value,
		value: &Value,
	) -> std::result::Result<Value, String> {
		Ok(value.clone())
	}

	fn graphql(&self) -> crate::GraphqlType {
		crate::GraphqlType::Named("Seo")
	}

	fn studio(&self) -> crate::StudioModule {
		crate::StudioModule::new("")
	}
}

#[test]
fn imports_seo_tables_and_asset_folders() {
	let schema = Schema::new()
		.site("de", "Deutsch", "de-CH")
		.field_type(Seo)
		// only blogs have a page, like Craft's `{% if type == 'blog' %}`
		.section(
			Section::channel("blogs", ["blog", "video"])
				.uri_of("blog", "blog/{slug}"),
		)
		.entry(
			"blog",
			"Blog",
			[
				Field::custom("seo", "seo", Value::Null),
				Field::table(
					"table",
					[Column::text("day"), Column::text("time")],
				),
			],
		)
		.entry("video", "Video", [])
		.asset_fields([
			Field::text("label"),
			Field::relation("blog", ["blog"]).max(1),
		]);
	schema.check().unwrap();

	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	for v in ["timetables", "photos", "docs"] {
		fs::create_dir_all(dir.join(v).join("2026")).unwrap();
	}
	fs::write(dir.join("timetables/2026/plan.txt"), "p").unwrap();
	fs::write(dir.join("photos/a.txt"), "a").unwrap();
	fs::write(dir.join("docs/2026/b.txt"), "b").unwrap();
	let asset = |id, volume: &str, name: &str, path: &str, fields| {
		json!({ "id": id, "volume": volume, "volumeName": name,
			"path": path, "title": null, "alt": null, "focalPoint": null,
			"fields": fields })
	};
	let export: Export = serde_json::from_value(json!({
		"sites": [{ "handle": "default" }],
		"sections": { "blogs": { "sites": { "default": [
			{ "id": 1, "type": "blog", "title": "Herbst", "slug": "herbst",
				"enabled": true, "fields": {
				"seo": { "title": "Im Herbst", "description": null,
					"image": [11], "socialImage": [], "robots": "noindex, follow",
					"canonicalUrl": null },
				"table": {
					"columns": [
						{ "heading": "Tag", "width": "", "align": "",
							"type": "singleline" },
						{ "heading": "Zeit", "width": "", "align": "",
							"type": "singleline" },
						{ "heading": "Ort", "width": "", "align": "",
							"type": "singleline" },
					],
					"rows": [["Mo", "9:00", "Halle"], ["Di", null, null]],
				},
			} },
			{ "id": 2, "type": "video", "title": "Clip", "slug": "clip",
				"enabled": true, "fields": {} },
		] } } },
		"assets": [
			asset(10, "timetables", "Timetables", "2026/plan.txt",
				json!({ "label": "Herbst", "blog": [1] })),
			asset(11, "photos", "Fotos", "a.txt", json!({})),
			asset(12, "docs", "Dokumente", "2026/b.txt", json!([])),
		],
	}))
	.unwrap();
	let volume = |name: &str, folder: Option<&str>| {
		let v = Volume {
			dir: dir.join(name),
			folder: folder.map(Into::into),
		};
		(name.to_string(), v)
	};
	let opts = Options {
		sites: vec![],
		volumes: HashMap::from([
			volume("timetables", Some("Stundenpläne")),
			volume("photos", None),
			// no folder of its own, its folders on the top
			volume("docs", Some("")),
		]),
	};

	let db = Db::memory().unwrap();
	let conn = db.conn();
	let report =
		import(&conn, &schema, &dir.join("files"), &export, &opts).unwrap();
	assert_eq!((report.entries, report.assets), (2, 3));
	assert_eq!(
		report.skipped,
		["table: a table has 3 columns, only the first 2 are imported"]
	);

	// a volume is a folder, Craft's folders are in it
	let folders = assets::folders(&conn).unwrap();
	let folder = |name: &str| folders.iter().find(|f| f.name == name);
	let plans = folder("Stundenpläne").unwrap();
	assert_eq!(plans.parent_id, None);
	let in_plans = folders
		.iter()
		.find(|f| f.name == "2026" && f.parent_id == Some(plans.id))
		.unwrap();
	let photos = folder("Fotos").unwrap();
	let on_top = folders
		.iter()
		.find(|f| f.name == "2026" && f.parent_id.is_none())
		.unwrap();
	assert_eq!(folders.len(), 4);
	let asset = |id| assets::get_asset(&conn, id).unwrap();
	assert_eq!(asset(1).folder_id, Some(in_plans.id));
	assert_eq!(asset(2).folder_id, Some(photos.id));
	assert_eq!(asset(3).folder_id, Some(on_top.id));

	let (blogs, _) = entries::list(
		&conn,
		&crate::entries::query::Query::parse(
			&schema,
			&[("section".into(), "blogs".into())],
		)
		.unwrap(),
	)
	.unwrap();
	let blog = blogs.iter().find(|e| e.ty == "blog").unwrap();
	let video = blogs.iter().find(|e| e.ty == "video").unwrap();
	assert_eq!(blog.uri.as_deref(), Some("blog/herbst"));
	assert_eq!(video.uri, None);

	// the asset fields link the entry, the SEO image is the asset in zipp
	assert_eq!(
		asset(1).fields,
		json!({ "label": "Herbst", "blog": [blog.id] })
	);
	assert_eq!(asset(2).fields, json!({}));
	let seo = &blog.fields["seo"];
	assert_eq!(seo["title"], "Im Herbst");
	assert_eq!(seo["image"], json!([2]));
	assert_eq!(seo["robots"], "noindex, follow");
	assert_eq!(
		blog.fields["table"],
		json!([{ "day": "Mo", "time": "9:00" }, { "day": "Di" }])
	);
}
