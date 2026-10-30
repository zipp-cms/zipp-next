use serde_json::json;

use super::*;
use crate::db::Db;
use crate::schema::{LinkKind, Section};

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
	crate::apply::create_singles(&conn, &schema).unwrap();
	let opts = Options {
		sites: vec![],
		volumes: HashMap::new(),
	};
	let files = std::env::temp_dir();

	let report = import(&conn, &schema, &files, &export(), &opts).unwrap();
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
		&crate::query::Query::parse(
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
	assert_eq!(f["sections"].as_array().unwrap().len(), 1);
	assert_eq!(f["sections"][0]["fields"]["title"], "One");
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
	assert_eq!(
		footer["navigation"],
		json!([{
			"type": "entry", "value": about.id,
			"children": [{ "type": "url", "value": "https://insta.com",
				"label": "Insta" }],
		}])
	);

	// only into an empty database
	assert!(import(&conn, &schema, &files, &export(), &opts).is_err());
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
