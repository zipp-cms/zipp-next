//! Rich text: what TipTap edits, stored as its JSON (a ProseMirror
//! document). The schema says per field what it may contain: headings,
//! lists, links, rows of buttons, blocks of given types and styles: a
//! class on a paragraph or heading (`attrs.class`) or a mark `style`
//! with a class on a part of the text.
//!
//! On save the document is checked and cleaned: empty paragraphs (also
//! ones with only `&nbsp;`) and line breaks at the end of a paragraph go
//! away, so frontends need no margin hacks for empty space. For the
//! frontend it is split into chunks: html, a row of buttons, a block.

use serde_json::{Map, Value, json};

use crate::content::validate::validate_block;
use crate::content::{FieldError, check_link, link_is_empty};
use crate::schema::{FieldKind, LinkKind, Schema, Style};

/// The links a rich text knows, everything but assets.
const LINKS: [LinkKind; 4] = [
	LinkKind::Entry,
	LinkKind::Url,
	LinkKind::Email,
	LinkKind::Phone,
];

/// What a rich text field allows, from its kind.
pub(crate) struct Allowed<'a> {
	pub headings: &'a [u8],
	pub lists: bool,
	pub links: bool,
	pub buttons: bool,
	pub blocks: &'a [String],
	pub styles: &'a [Style],
}

impl<'a> Allowed<'a> {
	/// Whether the field has this style, inline or of a block.
	fn has_style(&self, class: &str, inline: bool) -> bool {
		self.styles
			.iter()
			.any(|s| s.class == class && s.inline == inline)
	}

	pub(crate) fn of(kind: &'a FieldKind) -> Option<Self> {
		match kind {
			FieldKind::RichText {
				headings,
				lists,
				links,
				buttons,
				blocks,
				styles,
				..
			} => Some(Self {
				headings,
				lists: *lists,
				links: *links,
				buttons: *buttons,
				blocks,
				styles,
			}),
			_ => None,
		}
	}
}

/// Whether a document has nothing in it.
pub(crate) fn is_empty(doc: &Value) -> bool {
	doc["content"].as_array().is_none_or(|c| c.is_empty())
}

/// Checks and cleans a document, errors go to `errors` with `path`. In a
/// field that is `hidden` by its condition its embedded blocks needn't be
/// filled.
pub(crate) fn validate(
	schema: &Schema,
	allowed: &Allowed,
	doc: &Value,
	path: &str,
	hidden: bool,
	errors: &mut Vec<FieldError>,
) -> Value {
	let mut err = |message: String| errors.push(FieldError::new(path, message));
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
				let ok = u8::try_from(level)
					.is_ok_and(|l| allowed.headings.contains(&l));
				if !ok {
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
				if link_is_empty(link) {
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
			// a block like in a blocks field; its errors show on the rich
			// text, the studio has no place for them in the block
			"embed" => {
				let bt = node["attrs"]["type"].as_str().unwrap_or_default();
				if !allowed.blocks.iter().any(|b| b == bt) {
					err(format!("block {bt:?} isn't allowed here"));
					continue;
				}
				let mut nested = vec![];
				let block = validate_block(
					schema,
					allowed.blocks,
					&node["attrs"],
					&format!("{path}.{i}"),
					hidden,
					&mut nested,
				);
				let name = schema.get(bt).map_or(bt, |c| c.name.as_str());
				for e in nested {
					err(format!("{name}: {} {}", e.path, e.message));
				}
				Some(json!({ "type": "embed", "attrs": block }))
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

/// A paragraph or heading with only allowed inline content, `None` if it
/// is empty.
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
	let mut attrs = Map::new();
	if node["type"] == "heading" {
		attrs.insert("level".into(), node["attrs"]["level"].clone());
	}
	// a block style; TipTap writes null for none
	match node["attrs"]["class"].as_str() {
		Some(class) if allowed.has_style(class, false) => {
			attrs.insert("class".into(), class.into());
		}
		Some(class) => err(format!("style {class:?} isn't allowed here")),
		None => {}
	}
	if !attrs.is_empty() {
		out.insert("attrs".into(), attrs.into());
	}
	out.insert("content".into(), content.into());
	Some(Value::Object(out))
}

fn check_mark(mark: &Value, allowed: &Allowed, err: &mut impl FnMut(String)) {
	match mark["type"].as_str() {
		Some("bold") | Some("italic") => {}
		Some("style") => {
			let class = mark["attrs"]["class"].as_str().unwrap_or_default();
			if !allowed.has_style(class, true) {
				err(format!("style {class:?} isn't allowed here"));
			}
		}
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
pub(crate) fn links(doc: &Value) -> Vec<&Value> {
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
pub(crate) enum Chunk {
	Html(String),
	/// one row of buttons, the links
	Buttons(Vec<Value>),
	/// the block node, `attrs` has id, type and fields
	Block(Value),
}

/// The url and target of a link, `None` for a link to nowhere (an entry
/// that is gone or not published).
pub(crate) type Resolve<'a> =
	dyn FnMut(&Value) -> Option<(String, Option<String>)> + 'a;

/// The chunks of a document: html between rows of buttons (the buttons
/// next to each other) and blocks, without the ones switched off.
pub(crate) fn chunks(doc: &Value, resolve: &mut Resolve) -> Vec<Chunk> {
	let mut out = vec![];
	let mut html = String::new();
	for node in doc["content"].as_array().into_iter().flatten() {
		match node["type"].as_str() {
			Some("embed") if node["attrs"]["enabled"] == false => {}
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
pub(crate) fn html(doc: &Value, resolve: &mut Resolve) -> String {
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
			*out += &format!("<p{}>", class(node));
			children(resolve, out);
			*out += "</p>";
		}
		Some("heading") => {
			let level =
				node["attrs"]["level"].as_u64().unwrap_or(2).clamp(1, 6);
			*out += &format!("<h{level}{}>", class(node));
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
					Some("style") => {
						format!("<span{}>{text}</span>", class(m))
					}
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

/// The class attribute of a style, ` class="lead"`, or nothing.
fn class(node: &Value) -> String {
	match node["attrs"]["class"].as_str() {
		Some(c) if !c.is_empty() => format!(" class=\"{}\"", escape(c)),
		_ => String::new(),
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

pub(crate) fn escape(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
}

/// The text of a document without formatting, a line per paragraph and
/// the labels of buttons if `buttons` (search finds them, a description
/// doesn't say them); blocks are left out (they are components).
pub fn plain_text(doc: &Value, buttons: bool) -> String {
	let mut out = String::new();
	collect_text(doc, buttons, &mut out);
	out.trim().to_string()
}

fn collect_text(node: &Value, buttons: bool, out: &mut String) {
	match node["type"].as_str() {
		Some("embed") => return,
		Some("button") if !buttons => return,
		Some("text") => out.push_str(node["text"].as_str().unwrap_or("")),
		Some("hardBreak") => out.push(' '),
		Some("button") => {
			let label = node["attrs"]["link"]["label"].as_str();
			out.push_str(label.unwrap_or(""));
			out.push('\n');
		}
		_ => {}
	}
	let Some(children) = node["content"].as_array() else {
		return;
	};
	for child in children {
		collect_text(child, buttons, out);
	}
	// words of two paragraphs stay apart
	if !out.ends_with('\n') {
		out.push('\n');
	}
}

/// The blocks of a document, to walk into them.
pub(crate) fn blocks_mut(doc: &mut Value) -> Vec<&mut Value> {
	doc["content"]
		.as_array_mut()
		.into_iter()
		.flatten()
		.filter(|n| n["type"] == "embed")
		.map(|n| &mut n["attrs"])
		.collect()
}

#[cfg(test)]
#[path = "richtext_tests.rs"]
mod tests;
