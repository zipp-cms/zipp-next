use super::*;
use crate::fixtures::site;

fn allowed<'a>(blocks: &'a [String]) -> Allowed<'a> {
	Allowed {
		headings: &[2, 3],
		lists: true,
		links: true,
		buttons: true,
		blocks,
		styles: STYLES.get_or_init(|| {
			vec![
				Style::block("lead", "Lead"),
				Style::inline("highlight", "Highlight"),
			]
		}),
	}
}

static STYLES: std::sync::OnceLock<Vec<Style>> = std::sync::OnceLock::new();

fn p(text: &str) -> Value {
	json!({ "type": "paragraph", "content": [{ "type": "text", "text": text }] })
}

fn resolver(link: &Value) -> Option<(String, Option<String>)> {
	Some((link["value"].as_str()?.to_string(), None))
}

#[test]
fn cleans_empty_space() {
	let s = site();
	let doc = json!({ "type": "doc", "content": [
		{ "type": "paragraph" },
		p("\u{a0}"),
		{ "type": "paragraph", "content": [
			{ "type": "text", "text": "Hi" },
			{ "type": "hardBreak" },
		] },
		p(" "),
	] });
	let mut errors = vec![];
	let out = validate(&s, &allowed(&[]), &doc, "text", false, &mut errors);
	assert!(errors.is_empty(), "{errors:?}");
	assert_eq!(out, json!({ "type": "doc", "content": [p("Hi")] }));
}

#[test]
fn keeps_no_break_spaces_between_words() {
	let s = site();
	let doc = json!({ "type": "doc", "content": [
		p("in der\u{a0}Katarakt"),
		p("two \u{a0}spaces\u{a0}"),
	] });
	let mut errors = vec![];
	let out = validate(&s, &allowed(&[]), &doc, "text", false, &mut errors);
	let doc = json!({ "type": "doc", "content": [
		p("in der\u{a0}Katarakt"),
		p("two  spaces "),
	] });
	assert_eq!(out, doc);
}

#[test]
fn allows_only_what_the_field_says() {
	let s = site();
	let doc = json!({ "type": "doc", "content": [
		{ "type": "heading", "attrs": { "level": 1 }, "content": [{ "type": "text", "text": "H" }] },
		// 258 is 2 as a byte
		{ "type": "heading", "attrs": { "level": 258 }, "content": [{ "type": "text", "text": "H" }] },
		{ "type": "embed", "attrs": { "type": "button", "fields": {} } },
		{ "type": "image" },
	] });
	let mut errors = vec![];
	validate(
		&s,
		&allowed(&["text".into()]),
		&doc,
		"text",
		false,
		&mut errors,
	);
	let messages: Vec<_> = errors.iter().map(|e| e.message.as_str()).collect();
	assert_eq!(
		messages,
		[
			"headings of level 1 aren't allowed here",
			"headings of level 258 aren't allowed here",
			"block \"button\" isn't allowed here",
			"image isn't allowed here",
		]
	);
}

#[test]
fn splits_into_chunks() {
	let s = site();
	let link = json!({ "type": "url", "value": "https://x.ch", "label": "X" });
	let doc = json!({ "type": "doc", "content": [
		p("a <b>"),
		// two buttons next to each other are one row
		{ "type": "button", "attrs": { "link": link } },
		{ "type": "button", "attrs": { "link": link } },
		{ "type": "embed", "attrs": { "type": "text", "fields": { "content": "c" } } },
		{ "type": "paragraph", "content": [
			{ "type": "text", "text": "d", "marks": [{ "type": "bold" },
				{ "type": "link", "attrs": { "link": { "type": "url", "value": "/d" } } }] },
		] },
	] });
	let mut errors = vec![];
	let doc = validate(
		&s,
		&allowed(&["text".into()]),
		&doc,
		"t",
		false,
		&mut errors,
	);
	assert!(errors.is_empty(), "{errors:?}");

	let chunks = chunks(&doc, &mut resolver);
	assert_eq!(chunks[0], Chunk::Html("<p>a &lt;b&gt;</p>".into()));
	assert!(matches!(&chunks[1], Chunk::Buttons(l) if l.len() == 2));
	assert!(
		matches!(&chunks[2], Chunk::Block(b) if b["fields"]["content"] == "c")
	);
	assert_eq!(
		chunks[3],
		Chunk::Html("<p><a href=\"/d\"><strong>d</strong></a></p>".into())
	);

	// text between them: two rows
	let other = json!({ "type": "doc", "content": [
		{ "type": "button", "attrs": { "link": link } },
		p("b"),
		{ "type": "button", "attrs": { "link": link } },
	] });
	let rows = super::chunks(&other, &mut resolver);
	assert!(matches!(
		&rows[..],
		[Chunk::Buttons(_), Chunk::Html(_), Chunk::Buttons(_)]
	));
	// a button without a target yet goes
	let other = json!({ "type": "doc", "content": [
		{ "type": "button", "attrs": { "link": { "type": "url", "value": null } } },
	] });
	let other = validate(
		&s,
		&allowed(&["text".into()]),
		&other,
		"t",
		false,
		&mut errors,
	);
	assert_eq!(other["content"], json!([]), "{errors:?}");
	// the two buttons and the link in the text
	assert_eq!(links(&doc).len(), 3);
}

#[test]
fn list_items_of_one_paragraph_are_tight() {
	let item =
		|content: Value| json!({ "type": "listItem", "content": content });
	let doc = json!({ "type": "doc", "content": [{ "type": "bulletList", "content": [
		item(json!([p("one")])),
		item(json!([p("two"), p("three")])),
	] }] });
	assert_eq!(
		html(&doc, &mut resolver),
		"<ul><li>one</li><li><p>two</p><p>three</p></li></ul>"
	);
}

#[test]
fn styles_are_classes() {
	let s = site();
	let highlight =
		json!([{ "type": "style", "attrs": { "class": "highlight" } }]);
	let doc = json!({ "type": "doc", "content": [
		{ "type": "paragraph", "attrs": { "class": "lead" }, "content": [
			{ "type": "text", "text": "Big " },
			{ "type": "text", "text": "news", "marks": highlight },
		] },
		// TipTap writes null for no style
		{ "type": "heading", "attrs": { "level": 2, "class": null },
			"content": [{ "type": "text", "text": "H" }] },
	] });
	let mut errors = vec![];
	let doc = validate(&s, &allowed(&[]), &doc, "t", false, &mut errors);
	assert!(errors.is_empty(), "{errors:?}");
	assert_eq!(doc["content"][1]["attrs"], json!({ "level": 2 }));

	let out = "<p class=\"lead\">Big <span class=\"highlight\">news</span></p>\
		<h2>H</h2>";
	assert_eq!(html(&doc, &mut resolver), out);
	assert_eq!(chunks(&doc, &mut resolver), [Chunk::Html(out.into())]);
	assert_eq!(plain_text(&doc), "Big news\nH");
}

#[test]
fn refuses_unknown_styles() {
	let s = site();
	let mark =
		|class: &str| json!([{ "type": "style", "attrs": { "class": class } }]);
	let doc = json!({ "type": "doc", "content": [
		{ "type": "paragraph", "attrs": { "class": "huge" },
			"content": [{ "type": "text", "text": "a" }] },
		// a block style is no inline style
		{ "type": "paragraph", "content": [
			{ "type": "text", "text": "b", "marks": mark("lead") },
		] },
		{ "type": "heading", "attrs": { "level": 2, "class": "highlight" },
			"content": [{ "type": "text", "text": "c" }] },
	] });
	let mut errors = vec![];
	validate(&s, &allowed(&[]), &doc, "t", false, &mut errors);
	let messages: Vec<_> = errors.iter().map(|e| e.message.as_str()).collect();
	assert_eq!(
		messages,
		[
			"style \"huge\" isn't allowed here",
			"style \"lead\" isn't allowed here",
			"style \"highlight\" isn't allowed here",
		]
	);
}
