//! CKEditor's html as a TipTap document. Links keep their `href` and
//! `target` in the mark (`{ "type": "link", "attrs": { "href" } }`) and
//! nested entries become `{ "type": "craftEntry", "attrs": { "id" } }`;
//! the importer turns both into what the field allows.

use serde_json::{Value, json};

#[derive(Debug, PartialEq)]
enum Node {
	Element {
		name: String,
		attrs: Vec<(String, String)>,
		children: Vec<Node>,
	},
	Text(String),
}

/// An element still open while parsing: name, attributes, children.
type Open = (String, Vec<(String, String)>, Vec<Node>);

const VOID: [&str; 6] = ["br", "img", "hr", "input", "meta", "link"];

/// The tree of an html fragment. Unknown or unclosed tags are forgiven:
/// a closing tag closes up to its opening one, stray ones are ignored.
fn parse(html: &str) -> Vec<Node> {
	// the open elements, the first is the fragment itself
	let mut stack: Vec<Open> = vec![(String::new(), vec![], vec![])];
	let mut rest = html;

	while !rest.is_empty() {
		if let Some(after) = rest.strip_prefix("<!--") {
			rest = after.split_once("-->").map_or("", |(_, r)| r);
			continue;
		}
		let tag = rest
			.strip_prefix('<')
			.filter(|r| {
				r.starts_with(|c: char| c.is_ascii_alphabetic() || c == '/')
			})
			.and_then(|r| r.split_once('>'));
		let Some((tag, after)) = tag else {
			// a `<` that starts no tag is text
			let skip = usize::from(rest.starts_with('<'));
			let end = rest[skip..].find('<').map_or(rest.len(), |i| i + skip);
			let text = decode(&rest[..end]);
			stack.last_mut().unwrap().2.push(Node::Text(text));
			rest = &rest[end..];
			continue;
		};
		rest = after;

		if let Some(name) = tag.strip_prefix('/') {
			let name = name.trim().to_ascii_lowercase();
			let Some(i) = stack.iter().rposition(|(n, ..)| *n == name) else {
				continue;
			};
			if i == 0 {
				continue;
			}
			while stack.len() > i {
				close(&mut stack);
			}
			continue;
		}

		let tag = tag.trim_end_matches('/');
		let (name, attrs) = match tag.split_once(char::is_whitespace) {
			Some((n, a)) => (n, parse_attrs(a)),
			None => (tag, vec![]),
		};
		let name = name.to_ascii_lowercase();
		if VOID.contains(&name.as_str()) {
			stack.last_mut().unwrap().2.push(Node::Element {
				name,
				attrs,
				children: vec![],
			});
		} else {
			stack.push((name, attrs, vec![]));
		}
	}

	while stack.len() > 1 {
		close(&mut stack);
	}
	stack.pop().unwrap().2
}

fn close(stack: &mut Vec<Open>) {
	let (name, attrs, children) = stack.pop().unwrap();
	stack.last_mut().unwrap().2.push(Node::Element {
		name,
		attrs,
		children,
	});
}

fn parse_attrs(mut s: &str) -> Vec<(String, String)> {
	let mut out = vec![];
	loop {
		s = s.trim_start();
		let end = s
			.find(|c: char| c == '=' || c.is_whitespace())
			.unwrap_or(s.len());
		if end == 0 {
			break;
		}
		let name = s[..end].to_ascii_lowercase();
		s = s[end..].trim_start();

		let Some(after) = s.strip_prefix('=') else {
			out.push((name, String::new()));
			continue;
		};
		let after = after.trim_start();
		let (value, rest) = match after.chars().next() {
			Some(q @ ('"' | '\'')) => {
				let inner = &after[1..];
				inner.split_once(q).unwrap_or((inner, ""))
			}
			_ => after.split_once(char::is_whitespace).unwrap_or((after, "")),
		};
		out.push((name, decode(value)));
		s = rest;
	}
	out
}

fn decode(s: &str) -> String {
	let mut out = String::new();
	let mut rest = s;
	while let Some(i) = rest.find('&') {
		out += &rest[..i];
		rest = &rest[i..];
		let entity = rest[1..]
			.find(';')
			.filter(|&end| end <= 10)
			.map(|end| &rest[1..end + 1]);
		let c = entity.and_then(|e| match e {
			"amp" => Some('&'),
			"lt" => Some('<'),
			"gt" => Some('>'),
			"quot" => Some('"'),
			"apos" => Some('\''),
			"nbsp" => Some('\u{a0}'),
			_ => {
				let n = e.strip_prefix('#')?;
				let code = match n.strip_prefix(['x', 'X']) {
					Some(hex) => u32::from_str_radix(hex, 16).ok()?,
					None => n.parse().ok()?,
				};
				char::from_u32(code)
			}
		});
		match (c, entity) {
			(Some(c), Some(e)) => {
				out.push(c);
				rest = &rest[e.len() + 2..];
			}
			_ => {
				out.push('&');
				rest = &rest[1..];
			}
		}
	}
	out + rest
}

/// The document of an html fragment.
pub fn to_doc(html: &str) -> Value {
	json!({ "type": "doc", "content": blocks(&parse(html)) })
}

fn is_inline(node: &Node) -> bool {
	match node {
		Node::Text(_) => true,
		Node::Element { name, .. } => matches!(
			name.as_str(),
			"b" | "strong"
				| "i" | "em" | "a"
				| "br" | "span"
				| "u" | "s" | "sub"
				| "sup" | "small"
				| "code"
		),
	}
}

fn blocks(nodes: &[Node]) -> Vec<Value> {
	let mut out = vec![];
	let mut inline: Vec<&Node> = vec![];
	let flush = |inline: &mut Vec<&Node>, out: &mut Vec<Value>| {
		let content = inline_content(inline.drain(..));
		if !content.is_empty() {
			out.push(json!({ "type": "paragraph", "content": content }));
		}
	};

	for node in nodes {
		if is_inline(node) {
			inline.push(node);
			continue;
		}
		flush(&mut inline, &mut out);
		let Node::Element {
			name,
			attrs,
			children,
		} = node
		else {
			continue;
		};
		match name.as_str() {
			"p" => {
				let content = inline_content(children.iter());
				out.push(json!({ "type": "paragraph", "content": content }));
			}
			h if h.len() == 2
				&& h.starts_with('h')
				&& h.as_bytes()[1].is_ascii_digit() =>
			{
				let level = h[1..].parse::<u8>().unwrap_or(2);
				let content = inline_content(children.iter());
				out.push(json!({
					"type": "heading",
					"attrs": { "level": level },
					"content": content,
				}));
			}
			"ul" | "ol" => {
				let items: Vec<Value> = children
					.iter()
					.filter_map(|li| match li {
						Node::Element { name, children, .. }
							if name == "li" =>
						{
							Some(json!({
								"type": "listItem",
								"content": blocks(children),
							}))
						}
						_ => None,
					})
					.collect();
				let ty = if name == "ul" {
					"bulletList"
				} else {
					"orderedList"
				};
				out.push(json!({ "type": ty, "content": items }));
			}
			"craft-entry" => {
				let id = attr(attrs, "data-entry-id")
					.and_then(|id| id.parse::<i64>().ok());
				if let Some(id) = id {
					out.push(
						json!({ "type": "craftEntry", "attrs": { "id": id } }),
					);
				}
			}
			// containers like div or blockquote: what is inside them
			_ => out.extend(blocks(children)),
		}
	}
	flush(&mut inline, &mut out);
	out
}

fn attr<'a>(attrs: &'a [(String, String)], name: &str) -> Option<&'a str> {
	attrs
		.iter()
		.find(|(n, _)| n == name)
		.map(|(_, v)| v.as_str())
}

/// Text and line breaks with their marks; white space collapses like in
/// a browser and goes away at the start, the end and around breaks.
fn inline_content<'a>(nodes: impl Iterator<Item = &'a Node>) -> Vec<Value> {
	let mut out = vec![];
	for node in nodes {
		inline(node, &[], &mut out);
	}

	let mut content: Vec<Value> = vec![];
	for mut node in out {
		let prev_text = content.last().is_some_and(|p| p["type"] == "text");
		let is_text = node["type"] == "text";
		if is_text {
			let text = node["text"].as_str().unwrap_or_default();
			let mut text = collapse(text);
			let after_space = match content.last() {
				None => true,
				Some(p) if p["type"] == "hardBreak" => true,
				Some(p) => p["text"].as_str().is_some_and(|t| t.ends_with(' ')),
			};
			if after_space {
				text = text.trim_start_matches(' ').to_string();
			}
			if text.is_empty() {
				continue;
			}
			node["text"] = Value::from(text);
		} else if prev_text {
			trim_end(content.last_mut().unwrap());
		}

		let same_marks = content.last().is_some_and(|p: &Value| {
			p["type"] == "text" && p["marks"] == node["marks"]
		});
		if is_text && same_marks {
			let last = content.last_mut().unwrap();
			let joined = format!(
				"{}{}",
				last["text"].as_str().unwrap_or_default(),
				node["text"].as_str().unwrap_or_default()
			);
			last["text"] = Value::from(joined);
		} else {
			content.push(node);
		}
	}
	if let Some(last) = content.last_mut() {
		trim_end(last);
	}
	content.retain(|n| n["type"] != "text" || n["text"] != "");
	content
}

fn trim_end(node: &mut Value) {
	if let Some(t) = node["text"].as_str() {
		node["text"] = Value::from(t.trim_end_matches(' '));
	}
}

/// Runs of white space as one space; `&nbsp;` stays.
fn collapse(text: &str) -> String {
	let mut out = String::new();
	for c in text.chars() {
		if c.is_whitespace() && c != '\u{a0}' {
			if !out.ends_with(' ') {
				out.push(' ');
			}
		} else {
			out.push(c);
		}
	}
	out
}

fn inline(node: &Node, marks: &[Value], out: &mut Vec<Value>) {
	match node {
		Node::Text(text) => {
			let mut n = json!({ "type": "text", "text": text });
			if !marks.is_empty() {
				n["marks"] = Value::from(marks.to_vec());
			}
			out.push(n);
		}
		Node::Element { name, .. } if name == "br" => {
			out.push(json!({ "type": "hardBreak" }));
		}
		Node::Element {
			name,
			attrs,
			children,
		} => {
			let mark = match name.as_str() {
				"b" | "strong" => Some(json!({ "type": "bold" })),
				"i" | "em" => Some(json!({ "type": "italic" })),
				"a" => attr(attrs, "href").map(|href| {
					let mut a = json!({ "href": href });
					if let Some(t) = attr(attrs, "target") {
						a["target"] = Value::from(t);
					}
					json!({ "type": "link", "attrs": a })
				}),
				_ => None,
			};
			let mut marks = marks.to_vec();
			if let Some(m) = mark
				&& !marks.contains(&m)
			{
				marks.push(m);
			}
			for child in children {
				// blocks inside inline content: only their text
				inline(child, &marks, out);
			}
		}
	}
}

#[cfg(test)]
mod tests {
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
}
