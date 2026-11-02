//! Links, navigations and rich text of Craft as zipp's.

use std::collections::HashMap;

use serde_json::{Value, json};

use super::{Importer, html};
use crate::schema::{Field, FieldKind, LinkKind};

impl Importer<'_> {
	/// Craft's link `{ type, id, value, label, target, download }`.
	pub(super) fn link(&self, v: &Value, new_tab: bool) -> Option<Value> {
		let ty = v["type"].as_str()?;
		let (kind, value) = match ty {
			"entry" | "category" => (
				LinkKind::Entry,
				Value::from(*self.entries.get(&v["id"].as_i64()?)?),
			),
			"asset" => (
				LinkKind::Asset,
				Value::from(*self.assets.get(&v["id"].as_i64()?)?),
			),
			"url" => (LinkKind::Url, v["value"].clone()),
			"email" => {
				let e = v["value"].as_str()?;
				(
					LinkKind::Email,
					Value::from(e.trim_start_matches("mailto:")),
				)
			}
			"tel" => {
				let p = v["value"].as_str()?;
				(LinkKind::Phone, Value::from(p.trim_start_matches("tel:")))
			}
			_ => return None,
		};
		let mut link = json!({ "type": kind, "value": value });
		if let Some(l) = v["label"].as_str().filter(|l| !l.is_empty()) {
			link["label"] = Value::from(l);
		}
		// zipp opens external links in a new tab, Craft only with a
		// target. The import doesn't know the urls of the sites, and a url
		// of their own isn't external (see graphql's links): where that
		// would matter, the link says what Craft's does.
		let http = value.as_str().is_some_and(|u| u.starts_with("http"));
		let default = match kind {
			LinkKind::Asset => Some(new_tab),
			LinkKind::Url if http && new_tab => None,
			_ => Some(false),
		};
		let blank = v["target"] == "_blank";
		if default != Some(blank) {
			link["newTab"] = Value::from(blank);
		}
		if kind == LinkKind::Asset && v["download"] == true {
			link["download"] = Value::from(true);
		}
		Some(link)
	}

	/// A matrix as a navigation: each nested entry's link and the nested
	/// entries of its own matrix under it.
	pub(super) fn nav(&self, v: &Value) -> Value {
		let mut items = vec![];
		for nested in v.as_array().into_iter().flatten() {
			if nested["enabled"] == false {
				continue;
			}
			let fields = nested["fields"].as_object();
			let values = || fields.into_iter().flat_map(|f| f.values());
			let link = values()
				.filter(|f| f.get("type").is_some() && f.get("label").is_some())
				.find_map(|f| self.link(f, true));
			let Some(mut item) = link else {
				continue;
			};
			let children = values().find(|f| {
				f.as_array().is_some_and(|a| {
					a.first().is_some_and(|n| n.get("fields").is_some())
				})
			});
			if let Some(children) = children {
				item["children"] = self.nav(children);
			}
			items.push(item);
		}
		Value::from(items)
	}

	/// CKEditor's `{ html, entries }` as a document of what the field
	/// allows.
	pub(super) fn rich_text(&mut self, field: &Field, v: &Value) -> Value {
		let FieldKind::RichText {
			headings,
			lists,
			links,
			buttons,
			blocks,
			..
		} = &field.kind
		else {
			unreachable!()
		};
		let doc = html::to_doc(v["html"].as_str().unwrap_or_default());
		let nested: HashMap<i64, &Value> = v["entries"]
			.as_array()
			.into_iter()
			.flatten()
			.filter_map(|e| Some((e["id"].as_i64()?, e)))
			.collect();

		let mut content: Vec<Value> = vec![];
		for mut node in doc["content"].as_array().cloned().unwrap_or_default() {
			match node["type"].as_str().unwrap_or_default() {
				"craftEntry" => {
					let id = node["attrs"]["id"].as_i64().unwrap_or_default();
					let Some(e) =
						nested.get(&id).filter(|e| e["enabled"] != false)
					else {
						continue;
					};
					let ty = e["type"].as_str().unwrap_or_default();
					if blocks.iter().any(|b| b == ty) {
						let comp = self.schema.get(ty).unwrap();
						let fields = self.fields(comp, &e["fields"]);
						content.push(json!({
							"type": "embed",
							"attrs": { "type": ty, "fields": fields },
						}));
						continue;
					}
					let link = e["fields"]
						.as_object()
						.into_iter()
						.flat_map(|f| f.values())
						.find_map(|f| self.link(f, true));
					match link {
						// buttons next to each other are one row on the
						// website
						Some(link) if *buttons => content.push(json!({
							"type": "button",
							"attrs": { "link": link },
						})),
						_ => self.skip(format!(
							"{}: an entry {ty} in the text",
							field.handle
						)),
					}
					continue;
				}
				"heading" => {
					let level =
						node["attrs"]["level"].as_u64().unwrap_or(0) as u8;
					let nearest =
						headings.iter().min_by_key(|l| l.abs_diff(level));
					match nearest {
						Some(l) => node["attrs"]["level"] = Value::from(*l),
						None => {
							node = json!({
								"type": "paragraph",
								"content": node["content"],
							})
						}
					}
				}
				"bulletList" | "orderedList" if !lists => {
					for item in node["content"].as_array().into_iter().flatten()
					{
						content.extend(
							item["content"]
								.as_array()
								.cloned()
								.unwrap_or_default(),
						);
					}
					continue;
				}
				_ => {}
			}
			content.push(node);
		}

		let mut doc = json!({ "type": "doc", "content": content });
		self.links(&mut doc, *links);
		doc
	}

	/// The link marks of a document as zipp's links, or plain text when
	/// the field has no links.
	fn links(&self, node: &mut Value, allowed: bool) {
		if let Some(marks) = node.get_mut("marks").and_then(Value::as_array_mut)
		{
			let mut kept = vec![];
			for m in marks.drain(..) {
				if m["type"] != "link" {
					kept.push(m);
					continue;
				}
				let link = self.href(&m["attrs"]);
				if let (true, Some(link)) = (allowed, link) {
					kept.push(
						json!({ "type": "link", "attrs": { "link": link } }),
					);
				}
			}
			*marks = kept;
		}
		for child in node
			.get_mut("content")
			.and_then(Value::as_array_mut)
			.into_iter()
			.flatten()
		{
			self.links(child, allowed);
		}
	}

	/// An `href` of CKEditor: `{entry:12@1:url||https://…}`, mailto:,
	/// tel: or a url.
	fn href(&self, attrs: &Value) -> Option<Value> {
		let href = attrs["href"].as_str()?;
		let target = attrs["target"].clone();
		let link = if let Some(r) = href.strip_prefix('{') {
			let (reference, fallback) =
				r.trim_end_matches('}').split_once("||").unwrap_or((r, ""));
			let (kind, rest) = reference.split_once(':')?;
			let id: i64 = rest.split(['@', ':']).next()?.parse().ok()?;
			match kind {
				"entry" | "category" => json!({
					"type": "entry",
					"id": id,
					"target": target,
				}),
				// rich text links don't go to files, the file's url stays
				_ => {
					json!({ "type": "url", "value": fallback, "target": target })
				}
			}
		} else if let Some(e) = href.strip_prefix("mailto:") {
			json!({ "type": "email", "value": e, "target": target })
		} else if let Some(p) = href.strip_prefix("tel:") {
			json!({ "type": "tel", "value": p, "target": target })
		} else {
			json!({ "type": "url", "value": href, "target": target })
		};
		let mut link = self.link(&link, true)?;
		if let Some(o) = link.as_object_mut() {
			o.remove("label");
		}
		Some(link)
	}
}
