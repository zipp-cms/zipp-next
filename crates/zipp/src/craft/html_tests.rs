use super::*;

#[test]
fn paragraphs_marks_and_breaks() {
	let doc = to_doc(
		"<p>Hello <strong>bold</strong> &amp; <em>it</em> <br />next</p>\n<p>&nbsp;</p>",
	);
	assert_eq!(
		doc["content"],
		json!([
			{ "type": "paragraph", "content": [
				{ "type": "text", "text": "Hello " },
				{ "type": "text", "text": "bold", "marks": [{ "type": "bold" }] },
				{ "type": "text", "text": " & " },
				{ "type": "text", "text": "it", "marks": [{ "type": "italic" }] },
				{ "type": "hardBreak" },
				{ "type": "text", "text": "next" },
			] },
			{ "type": "paragraph", "content": [
				{ "type": "text", "text": "\u{a0}" },
			] },
		])
	);
}

#[test]
fn text_outside_paragraphs() {
	// what the old sites have: inline content next to paragraphs
	let doc = to_doc("<b>Adresse</b><br />textrem!<p>CH – 3110</p>");
	let content = doc["content"].as_array().unwrap();
	assert_eq!(content.len(), 2);
	assert_eq!(content[0]["content"][0]["text"], "Adresse");
	assert_eq!(content[0]["content"][1]["type"], "hardBreak");
	assert_eq!(content[1]["content"][0]["text"], "CH – 3110");
}

#[test]
fn headings_lists_links_and_entries() {
	let doc = to_doc(
		"<h2>Title</h2><ul><li>one</li><li><p>two</p></li></ul>\
		<p><a href=\"{entry:12@1:url||https://x.ch/a}\" target=\"_blank\">a</a></p>\
		<craft-entry data-entry-id=\"7\"></craft-entry>",
	);
	let c = &doc["content"];
	assert_eq!(c[0]["attrs"]["level"], 2);
	assert_eq!(c[1]["type"], "bulletList");
	assert_eq!(
		c[1]["content"][1]["content"][0]["content"][0]["text"],
		"two"
	);
	assert_eq!(
		c[2]["content"][0]["marks"][0]["attrs"],
		json!({ "href": "{entry:12@1:url||https://x.ch/a}", "target": "_blank" })
	);
	assert_eq!(c[3], json!({ "type": "craftEntry", "attrs": { "id": 7 } }));
}

#[test]
fn attributes_with_entities() {
	let doc = to_doc("<p><a HREF='/x?a=1&amp;b=2' target=_blank>a</a></p>");
	assert_eq!(
		doc["content"][0]["content"][0]["marks"][0]["attrs"],
		json!({ "href": "/x?a=1&b=2", "target": "_blank" })
	);
}
