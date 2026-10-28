use serde_json::json;

use super::*;

#[test]
fn texts_for_descriptions() {
	assert_eq!(
		plain_text(&json!("<p>Hi</p><p>there <b>you</b></p>")),
		"Hi there you"
	);
	let doc = json!({ "type": "doc", "content": [
		{ "type": "paragraph", "content": [{ "type": "text", "text": "One" }] },
		{ "type": "buttons", "content": [{ "type": "text", "text": "no" }] },
		{ "type": "paragraph", "content": [{ "type": "text", "text": "two" }] },
	]});
	assert_eq!(plain_text(&doc), "One two");
	// entities decoded, no space after inline tags
	assert_eq!(
		plain_text(&json!("<p><em>Konzert</em>, Müller &amp; Söhne</p>")),
		"Konzert, Müller & Söhne"
	);

	assert_eq!(truncate("short", 160), "short");
	let long = "word ".repeat(50);
	let t = truncate(&long, 20);
	assert!(t.ends_with('…') && t.chars().count() <= 20, "{t}");
}

#[test]
fn titles_and_dates() {
	assert_eq!(
		render_title(
			"{title} – {sectionName} | {siteName}",
			"Konzert",
			"Zandi",
			"Events"
		),
		"Konzert – Events | Zandi"
	);
	assert_eq!(iso(1_793_145_600_000), "2026-10-28T00:00:00+00:00");
}

#[test]
fn globals_inherit_but_not_the_description() {
	let site = Site {
		handle: "en".into(),
		name: "English".into(),
		language: "en".into(),
	};
	let primary = json!({ "siteName": "Zandi", "description": "Augenärzte", "robotsTxt": "x" });
	let g = globals(&site, &json!({}), &primary);
	assert_eq!(g.site_name, "Zandi");
	assert_eq!(g.title_pattern, DEFAULT_TITLE_PATTERN);
	assert_eq!(g.description, None);
	assert_eq!(g.robots_txt, None);
	assert_eq!(g.robots, "index, follow");
}

#[test]
fn settings_override_the_code_per_section_and_type() {
	use crate::schema::{Section, Seo};
	let seo = Seo::new()
		.section(
			"events",
			SeoDefaults::new().title_pattern("code").no_sitemap(),
		)
		.entry_type("event", SeoDefaults::new().description("code type"));
	// the SEO before the sections: `finish` fills the choice afterwards
	let early = Schema::new().site("de", "Deutsch", "de-CH").seo(seo);
	let schema = early
		.section(Section::channel("events", ["event"]).uri("events/{slug}"))
		.entry("event", "Event", []);
	assert!(schema.check().is_err());
	let schema = schema.finish();
	schema.check().unwrap();

	let primary = json!({ "overrides": [
		{ "type": "seoOverride", "fields": {
			"appliesTo": "section:events", "titlePattern": "settings",
			"description": "only de", "sitemap": "yes" } },
	]});
	let de = schema.sites[0].clone();
	let g = globals(&de, &primary, &primary);
	let l = levels(&schema, &g, Some("event"), "events");
	// the code's entry type, the settings' section, the code's section
	assert_eq!(l.len(), 3);
	assert_eq!(first(&l, |d| d.title_pattern.as_ref()).unwrap(), "settings");
	assert_eq!(first(&l, |d| d.description.as_ref()).unwrap(), "code type");
	// put back into the sitemap by the settings
	assert!(section_in_sitemap(&schema, &g, "events"));

	// another site inherits the pattern, not the description
	let en = Site {
		handle: "en".into(),
		name: "English".into(),
		language: "en".into(),
	};
	let g = globals(&en, &json!({}), &primary);
	let set = &g.overrides["section:events"];
	assert_eq!(set.title_pattern.as_deref(), Some("settings"));
	assert_eq!(set.description, None);

	// without settings the code leaves the section out
	let g = globals(&de, &json!({}), &json!({}));
	assert!(!section_in_sitemap(&schema, &g, "events"));
}
