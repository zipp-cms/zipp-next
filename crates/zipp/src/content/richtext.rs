//! Rich text: what TipTap edits, stored as its JSON (a ProseMirror
//! document). The schema says per field what it may contain: headings,
//! lists, links, rows of buttons and blocks of given types.
//!
//! On save the document is checked and cleaned: empty paragraphs (also
//! ones with only `&nbsp;`) and line breaks at the end of a paragraph go
//! away, so frontends need no margin hacks for empty space. For the
//! frontend it is split into chunks: html, a row of buttons, a block.

use serde_json::{Map, Value, json};

use crate::content::{FieldError, check_link, new_block_id, validate_fields};
use crate::schema::{FieldKind, LinkKind, Schema};

/// The links a rich text knows, everything but assets.
const LINKS: [LinkKind; 4] = [
	LinkKind::Entry,
	LinkKind::Url,
	LinkKind::Email,
	LinkKind::Phone,
];

/// What a rich text field allows, from its kind.
pub struct Allowed<'a> {
	pub headings: &'a [u8],
	pub lists: bool,
	pub links: bool,
	pub buttons: bool,
	pub blocks: &'a [String],
}

impl<'a> Allowed<'a> {
	pub fn of(kind: &'a FieldKind) -> Option<Self> {
		match kind {
			FieldKind::RichText {
				headings,
				lists,
				links,
				buttons,
				blocks,
			} => Some(Self {
				headings,
				lists: *lists,
				links: *links,
				buttons: *buttons,
				blocks,
			}),
			_ => None,
		}
	}
}

/// Whether a document has nothing in it.
pub fn is_empty(doc: &Value) -> bool {
	doc["content"].as_array().is_none_or(|c| c.is_empty())
}

/// Checks and cleans a document, errors go to `errors` with `path`.
pub fn validate(
	schema: &Schema,
	allowed: &Allowed,
	doc: &Value,
	path: &str,
	errors: &mut Vec<FieldError>,
) -> Value {
	let mut err = |message: String| {
		errors.push(FieldError {
			path: path.into(),
			message,
		})
	};
	if doc["type"] != "doc" {
		err("must be a rich text document".into());
		return Value::Null;
	}

	let mut out = vec![];
	for (i, node) in doc["content"].as_array().into_iter().flatten().enumerate()
	{
		let ty = node["type"].as_str().unwrap_or_default();
		let node = match ty {
			"paragraph" => clean_inline(node, allowed, &mut err),
			"heading" => {
				let level = node["attrs"]["level"].as_u64().unwrap_or(0);
				if !allowed.headings.contains(&(level as u8)) {
					err(format!(
						"headings of level {level} aren't allowed here"
					));
					continue;
				}
				clean_inline(node, allowed, &mut err)
			}
			"bulletList" | "orderedList" if allowed.lists => {
				clean_list(node, allowed, &mut err)
			}
			// a button, its link with a label; buttons next to each other
			// are one row on the website, see `chunks`. One without a target
			// yet is left out.
			"button" if allowed.buttons => {
				let link = &node["attrs"]["link"];
				if link["value"] == Value::Null || link["value"] == "" {
					continue;
				}
				let ok = link
					.as_object()
					.ok_or("not a link".to_string())
					.and_then(|o| check_link(&LINKS, true, o));
				if let Err(e) = ok {
					err(format!("button: {e}"));
				}
				Some(json!({ "type": "button", "attrs": { "link": link } }))
			}
			"embed" => {
				let attrs = &node["attrs"];
				let bt = attrs["type"].as_str().unwrap_or_default();
				let comp = match schema.get(bt) {
					Some(c) if allowed.blocks.iter().any(|b| b == bt) => c,
					_ => {
						err(format!("block {bt:?} isn't allowed here"));
						continue;
					}
				};
				let mut block_errors = vec![];
				let prefix = format!("{path}.{i}.");
				let fields = validate_fields(
					schema,
					comp,
					&attrs["fields"],
					&prefix,
					&mut block_errors,
				);
				for e in block_errors {
					err(format!("{}: {} {}", comp.name, e.path, e.message));
				}
				let id = match attrs["id"].as_str() {
					Some(id) if !id.is_empty() => id.to_string(),
					_ => new_block_id(),
				};
				Some(json!({
					"type": "embed",
					"attrs": { "id": id, "type": bt, "fields": fields },
				}))
			}
			_ => {
				err(format!("{ty} isn't allowed here"));
				continue;
			}
		};
		out.extend(node);
	}

	json!({ "type": "doc", "content": out })
}

/// A paragraph or heading with only allowed inline content, `None` if it
/// is empty.
/// A no-break space between two words stays, it keeps them on one line
/// (`der\u{a0}Kataraktchirurgie`); next to a space or at an end the editor
/// only made space with it, that becomes a space.
fn loose_nbsp(text: &str) -> String {
	let chars: Vec<char> = text.chars().collect();
	let word = |c: Option<&char>| c.is_some_and(|c| !c.is_whitespace());
	chars
		.iter()
		.enumerate()
		.map(|(i, c)| match c {
			'\u{a0}'
				if word(chars.get(i.wrapping_sub(1)))
					&& word(chars.get(i + 1)) =>
			{
				'\u{a0}'
			}
			'\u{a0}' => ' ',
			c => *c,
		})
		.collect()
}

fn clean_inline(
	node: &Value,
	allowed: &Allowed,
	err: &mut impl FnMut(String),
) -> Option<Value> {
	let mut content = vec![];
	for inline in node["content"].as_array().into_iter().flatten() {
		match inline["type"].as_str() {
			Some("text") => {
				let Some(text) = inline["text"].as_str() else {
					continue;
				};
				let mut t = inline.clone();
				t["text"] = loose_nbsp(text).into();
				if let Some(marks) = inline["marks"].as_array() {
					for m in marks {
						check_mark(m, allowed, err);
					}
				}
				content.push(t);
			}
			Some("hardBreak") => content.push(json!({ "type": "hardBreak" })),
			other => {
				err(format!("{} isn't allowed in text", other.unwrap_or("?")))
			}
		}
	}

	// no line breaks or spaces at the end
	while let Some(last) = content.last() {
		if last["type"] == "hardBreak" {
			content.pop();
			continue;
		}
		let text = last["text"].as_str().unwrap_or_default();
		if text.trim_end().is_empty() {
			content.pop();
			continue;
		}
		break;
	}
	if content.is_empty() {
		return None;
	}

	let mut out = Map::new();
	out.insert("type".into(), node["type"].clone());
	if node["type"] == "heading" {
		out.insert("attrs".into(), json!({ "level": node["attrs"]["level"] }));
	}
	out.insert("content".into(), content.into());
	Some(Value::Object(out))
}

fn check_mark(mark: &Value, allowed: &Allowed, err: &mut impl FnMut(String)) {
	match mark["type"].as_str() {
		Some("bold") | Some("italic") => {}
		Some("link") if allowed.links => {
			let link = &mark["attrs"]["link"];
			let ok = link
				.as_object()
				.ok_or("not a link".to_string())
				.and_then(|o| check_link(&LINKS, false, o));
			if let Err(e) = ok {
				err(format!("link: {e}"));
			}
		}
		other => err(format!("{} isn't allowed here", other.unwrap_or("?"))),
	}
}

fn clean_list(
	node: &Value,
	allowed: &Allowed,
	err: &mut impl FnMut(String),
) -> Option<Value> {
	let mut items = vec![];
	for item in node["content"].as_array().into_iter().flatten() {
		let mut content = vec![];
		for child in item["content"].as_array().into_iter().flatten() {
			let c = match child["type"].as_str() {
				Some("paragraph") => clean_inline(child, allowed, err),
				Some("bulletList") | Some("orderedList") => {
					clean_list(child, allowed, err)
				}
				other => {
					err(format!(
						"{} isn't allowed in a list",
						other.unwrap_or("?")
					));
					None
				}
			};
			content.extend(c);
		}
		if !content.is_empty() {
			items.push(json!({ "type": "listItem", "content": content }));
		}
	}
	(!items.is_empty())
		.then(|| json!({ "type": node["type"], "content": items }))
}

/// The links of a document (in text and in buttons), for relations.
pub fn links(doc: &Value) -> Vec<&Value> {
	let mut out = vec![];
	collect_links(doc, &mut out);
	out
}

fn collect_links<'a>(node: &'a Value, out: &mut Vec<&'a Value>) {
	if node["type"] == "button" {
		out.push(&node["attrs"]["link"]);
	}
	for m in node["marks"].as_array().into_iter().flatten() {
		if m["type"] == "link" {
			out.push(&m["attrs"]["link"]);
		}
	}
	for child in node["content"].as_array().into_iter().flatten() {
		collect_links(child, out);
	}
}

/// A piece of a rich text for the frontend.
#[derive(Debug, Clone, PartialEq)]
pub enum Chunk {
	Html(String),
	/// one row of buttons, the links
	Buttons(Vec<Value>),
	/// the block node, `attrs` has id, type and fields
	Block(Value),
}

/// The url and target of a link, `None` for a link to nowhere (an entry
/// that is gone or not published).
pub type Resolve<'a> =
	dyn FnMut(&Value) -> Option<(String, Option<String>)> + 'a;

/// The chunks of a document: html between rows of buttons (the buttons
/// next to each other) and blocks.
pub fn chunks(doc: &Value, resolve: &mut Resolve) -> Vec<Chunk> {
	let mut out = vec![];
	let mut html = String::new();
	for node in doc["content"].as_array().into_iter().flatten() {
		match node["type"].as_str() {
			Some("button") | Some("embed") => {
				if !html.is_empty() {
					out.push(Chunk::Html(std::mem::take(&mut html)));
				}
				let link = node["attrs"]["link"].clone();
				match (node["type"].as_str(), out.last_mut()) {
					(Some("button"), Some(Chunk::Buttons(row))) => {
						row.push(link)
					}
					(Some("button"), _) => out.push(Chunk::Buttons(vec![link])),
					_ => out.push(Chunk::Block(node["attrs"].clone())),
				}
			}
			_ => render(node, resolve, &mut html),
		}
	}
	if !html.is_empty() {
		out.push(Chunk::Html(html));
	}
	out
}

/// The whole document as html, a row of buttons as links in a paragraph,
/// blocks left out (they are for the chunks).
pub fn html(doc: &Value, resolve: &mut Resolve) -> String {
	let mut html = String::new();
	let mut in_row = false;
	for node in doc["content"].as_array().into_iter().flatten() {
		let button = node["type"] == "button";
		if in_row && !button {
			html += "</p>";
		}
		if button {
			if !in_row {
				html += "<p class=\"buttons\">";
			}
			let link = &node["attrs"]["link"];
			if let Some((url, target)) = resolve(link) {
				let label = link["label"].as_str().unwrap_or(&url).to_string();
				html += &anchor(&url, target.as_deref(), &escape(&label));
			}
		} else if node["type"] != "embed" {
			render(node, resolve, &mut html);
		}
		in_row = button;
	}
	if in_row {
		html += "</p>";
	}
	html
}

fn render(node: &Value, resolve: &mut Resolve, out: &mut String) {
	let children = |resolve: &mut Resolve, out: &mut String| {
		for child in node["content"].as_array().into_iter().flatten() {
			render(child, resolve, out);
		}
	};
	match node["type"].as_str() {
		Some("paragraph") => {
			*out += "<p>";
			children(resolve, out);
			*out += "</p>";
		}
		Some("heading") => {
			let level =
				node["attrs"]["level"].as_u64().unwrap_or(2).clamp(1, 6);
			*out += &format!("<h{level}>");
			children(resolve, out);
			*out += &format!("</h{level}>");
		}
		Some("bulletList") => {
			*out += "<ul>";
			children(resolve, out);
			*out += "</ul>";
		}
		Some("orderedList") => {
			*out += "<ol>";
			children(resolve, out);
			*out += "</ol>";
		}
		Some("listItem") => {
			*out += "<li>";
			// an item of one paragraph is only its text, like CKEditor's
			match node["content"].as_array().map(Vec::as_slice) {
				Some([p]) if p["type"] == "paragraph" => {
					for child in p["content"].as_array().into_iter().flatten() {
						render(child, resolve, out);
					}
				}
				_ => children(resolve, out),
			}
			*out += "</li>";
		}
		Some("hardBreak") => *out += "<br>",
		Some("text") => {
			let mut text = escape(node["text"].as_str().unwrap_or_default());
			for m in node["marks"].as_array().into_iter().flatten() {
				text = match m["type"].as_str() {
					Some("bold") => format!("<strong>{text}</strong>"),
					Some("italic") => format!("<em>{text}</em>"),
					Some("link") => match resolve(&m["attrs"]["link"]) {
						Some((url, target)) => {
							anchor(&url, target.as_deref(), &text)
						}
						None => text,
					},
					_ => text,
				};
			}
			*out += &text;
		}
		_ => {}
	}
}

fn anchor(url: &str, target: Option<&str>, inner: &str) -> String {
	match target {
		Some(t) => format!(
			"<a href=\"{}\" target=\"{}\" rel=\"noopener\">{inner}</a>",
			escape(url),
			escape(t)
		),
		None => format!("<a href=\"{}\">{inner}</a>", escape(url)),
	}
}

fn escape(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
}

/// The blocks of a document, to walk into them.
pub fn blocks_mut(doc: &mut Value) -> Vec<&mut Value> {
	doc["content"]
		.as_array_mut()
		.into_iter()
		.flatten()
		.filter(|n| n["type"] == "embed")
		.map(|n| &mut n["attrs"])
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::schema::tests::site;

	fn allowed<'a>(blocks: &'a [String]) -> Allowed<'a> {
		Allowed {
			headings: &[2, 3],
			lists: true,
			links: true,
			buttons: true,
			blocks,
		}
	}

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
		let out = validate(&s, &allowed(&[]), &doc, "text", &mut errors);
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
		let out = validate(&s, &allowed(&[]), &doc, "text", &mut errors);
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
			{ "type": "embed", "attrs": { "type": "button", "fields": {} } },
			{ "type": "image" },
		] });
		let mut errors = vec![];
		validate(&s, &allowed(&["text".into()]), &doc, "text", &mut errors);
		let messages: Vec<_> =
			errors.iter().map(|e| e.message.as_str()).collect();
		assert_eq!(
			messages,
			[
				"headings of level 1 aren't allowed here",
				"block \"button\" isn't allowed here",
				"image isn't allowed here",
			]
		);
	}

	#[test]
	fn splits_into_chunks() {
		let s = site();
		let link =
			json!({ "type": "url", "value": "https://x.ch", "label": "X" });
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
		let doc =
			validate(&s, &allowed(&["text".into()]), &doc, "t", &mut errors);
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
		let other =
			validate(&s, &allowed(&["text".into()]), &other, "t", &mut errors);
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
}
