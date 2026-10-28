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
