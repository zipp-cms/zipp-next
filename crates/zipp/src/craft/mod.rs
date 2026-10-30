//! An import of a Craft 5 site: `craft/export.php` writes its content as
//! JSON, `import-craft` puts it into an empty zipp database. Everything is
//! matched by handle: a section, global set or category group of Craft
//! goes to the zipp section of the same handle, an entry type or a nested
//! entry type to the entry type or block of the same handle, a field to
//! the field of the same handle. What zipp's model doesn't have is left
//! out and reported.
//!
//! Values are converted to the kind of the zipp field: matrix to blocks or
//! to a navigation (each nested entry's link field and the nested entries
//! under it), content blocks to groups, CKEditor's html to rich text
//! (nested entries become embedded blocks or, if they are only a link,
//! buttons), categories to relations. Assets are copied with their title,
//! alt text and focal point.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, params};
use serde::Deserialize;
use serde_json::{Map, Value, json};

use crate::assets;
use crate::db::{now, parse_iso};
use crate::entries::{self, EntryInput};
use crate::error::{Error, Result};
use crate::schema::{Component, Field, FieldKind, Schema, SectionKind};

mod html;

#[derive(Debug, Deserialize)]
pub struct Export {
	pub sites: Vec<CraftSite>,
	#[serde(default, deserialize_with = "map_or_list")]
	pub sections: BTreeMap<String, CraftSection>,
	/// group, site, categories
	#[serde(default, deserialize_with = "map_or_list")]
	pub categories: BTreeMap<String, BTreeMap<String, Vec<Element>>>,
	/// set, site, its fields
	#[serde(default, deserialize_with = "map_or_list")]
	pub globals: BTreeMap<String, BTreeMap<String, Global>>,
	#[serde(default)]
	pub assets: Vec<CraftAsset>,
}

#[derive(Debug, Deserialize)]
pub struct CraftSite {
	pub handle: String,
}

#[derive(Debug, Deserialize)]
pub struct CraftSection {
	/// the elements per site, a structure in its order
	#[serde(deserialize_with = "map_or_list")]
	pub sites: BTreeMap<String, Vec<Element>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
	pub id: i64,
	#[serde(rename = "type")]
	pub ty: Option<String>,
	pub title: Option<String>,
	pub slug: Option<String>,
	pub enabled: bool,
	pub parent_id: Option<i64>,
	pub post_date: Option<String>,
	pub expiry_date: Option<String>,
	pub date_created: Option<String>,
	#[serde(default)]
	pub fields: Value,
}

#[derive(Debug, Deserialize)]
pub struct Global {
	#[serde(default)]
	pub fields: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CraftAsset {
	pub id: i64,
	pub volume: String,
	pub path: String,
	pub title: Option<String>,
	pub alt: Option<String>,
	pub focal_point: Option<(f64, f64)>,
}

/// PHP writes an empty map as `[]`.
fn map_or_list<'de, D, T>(
	d: D,
) -> std::result::Result<BTreeMap<String, T>, D::Error>
where
	D: serde::Deserializer<'de>,
	T: Deserialize<'de>,
{
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum Either<T> {
		Map(BTreeMap<String, T>),
		List(#[allow(dead_code)] Vec<Value>),
	}
	Ok(match Either::deserialize(d)? {
		Either::Map(m) => m,
		Either::List(_) => BTreeMap::new(),
	})
}

pub struct Options {
	/// Craft site handle to zipp site handle; without it sites match by
	/// handle, or the only site of each
	pub sites: Vec<(String, String)>,
	/// Craft volume handle to the folder of its files
	pub volumes: HashMap<String, PathBuf>,
}

/// What an import did and what it left out.
#[derive(Debug, Default)]
pub struct Report {
	pub entries: usize,
	pub assets: usize,
	pub skipped: Vec<String>,
}

/// Imports an export into an empty database (its singles may exist).
pub fn import(
	conn: &Connection,
	schema: &Schema,
	files: &Path,
	export: &Export,
	opts: &Options,
) -> Result<Report> {
	let content: i64 = conn.query_row(
		"SELECT (SELECT COUNT(*) FROM assets) + (SELECT COUNT(*) FROM entries
		WHERE section NOT IN (SELECT value FROM json_each(?1)))",
		[singles(schema).to_string()],
		|r| r.get(0),
	)?;
	if content > 0 {
		return Err(Error::Request(
			"import-craft needs a database without content".into(),
		));
	}

	let sites = site_map(schema, export, opts)?;
	let mut im = Importer {
		schema,
		entries: HashMap::new(),
		assets: HashMap::new(),
		report: Report::default(),
	};

	let tx = conn.unchecked_transaction()?;
	im.assets(&tx, files, export, opts)?;

	// every element of a section with its entry type, in Craft's order
	// section, entry type, the element in each zipp site
	type Planned = (String, String, Vec<(String, Element)>);
	let mut planned: Vec<Planned> = vec![];
	let groups = export.sections.iter().map(|(h, s)| (h, &s.sites, false));
	let cats = export.categories.iter().map(|(h, s)| (h, s, true));
	for (handle, per_site, category) in groups.chain(cats) {
		let Some(section) = schema.get_section(handle) else {
			im.skip(format!("section {handle}: not in the schema"));
			continue;
		};
		if section.kind == SectionKind::Single {
			im.skip(format!("section {handle}: a single in zipp"));
			continue;
		}

		let mut order: Vec<i64> = vec![];
		let mut by_id: HashMap<i64, Vec<(String, Element)>> = HashMap::new();
		for (craft_site, elements) in per_site {
			let Some(site) = sites.get(craft_site) else {
				continue;
			};
			for e in elements {
				if !by_id.contains_key(&e.id) {
					order.push(e.id);
				}
				by_id
					.entry(e.id)
					.or_default()
					.push((site.clone(), e.clone()));
			}
		}

		for id in order {
			let versions = by_id.remove(&id).unwrap();
			let craft_ty = versions[0].1.ty.clone();
			let ty = match craft_ty {
				Some(t) if section.types.contains(&t) => t,
				_ if category || section.types.len() == 1 => {
					section.types[0].clone()
				}
				t => {
					let title = versions[0].1.title.clone().unwrap_or_default();
					im.skip(format!(
						"{handle}: {title}: no entry type {}",
						t.unwrap_or_default()
					));
					continue;
				}
			};
			planned.push((handle.clone(), ty, versions));
		}
	}

	// first every entry without fields, so relations find each other
	for (section, ty, versions) in &planned {
		let e = &versions[0].1;
		let parent = e.parent_id.and_then(|p| im.entries.get(&p).copied());
		let id = create_empty(&tx, schema, section, ty, &title_of(e), parent)?;
		im.entries.insert(e.id, id);
	}

	for (section, ty, versions) in &planned {
		let id = im.entries[&versions[0].1.id];
		let comp = schema.entry_type(ty).unwrap();
		let mut saved = false;
		for (site, e) in versions {
			let fields = im.fields(comp, &e.fields);
			let input = EntryInput {
				section: section.clone(),
				ty: ty.clone(),
				site: Some(site.clone()),
				title: title_of(e),
				slug: e.slug.clone().unwrap_or_default(),
				published: e.enabled,
				dates: entries::Dates {
					post_date: e.post_date.as_deref().and_then(parse_iso),
					expiry_date: e.expiry_date.as_deref().and_then(parse_iso),
				},
				parent_id: None,
				fields,
			};
			match save(&tx, schema, id, input) {
				Ok(()) => saved = true,
				Err(err) => im.skip(format!(
					"{section}: {}: {}",
					title_of(e),
					describe(err)
				)),
			}
		}
		// not even an empty entry of what fits in no site
		if !saved {
			entries::delete(&tx, schema, id)?;
			continue;
		}
		let e = &versions[0].1;
		let created = e.post_date.as_deref().or(e.date_created.as_deref());
		if let Some(ms) = created.and_then(parse_iso) {
			tx.execute(
				"UPDATE entries SET created_at = ?1 WHERE id = ?2",
				params![ms, id],
			)?;
		}
		im.report.entries += 1;
	}

	// global sets fill the singles of the same handle
	for (handle, per_site) in &export.globals {
		let single = schema
			.get_section(handle)
			.filter(|s| s.kind == SectionKind::Single);
		let Some(section) = single else {
			im.skip(format!("global set {handle}: no single {handle}"));
			continue;
		};
		let id: i64 = tx.query_row(
			"SELECT id FROM entries WHERE section = ?1",
			[handle],
			|r| r.get(0),
		)?;
		let comp = schema.entry_type(&section.types[0]).unwrap();
		for (craft_site, global) in per_site {
			let Some(site) = sites.get(craft_site) else {
				continue;
			};
			let current = entries::get_entry(&tx, id, site)?;
			let input = EntryInput {
				section: handle.clone(),
				ty: section.types[0].clone(),
				site: Some(site.clone()),
				title: current.title,
				slug: current.slug,
				published: true,
				dates: Default::default(),
				parent_id: None,
				fields: im.fields(comp, &global.fields),
			};
			if let Err(err) = save(&tx, schema, id, input) {
				im.skip(format!("global set {handle}: {}", describe(err)));
			}
		}
	}

	tx.commit()?;
	Ok(im.report)
}

fn singles(schema: &Schema) -> Value {
	let handles = schema
		.sections
		.iter()
		.filter(|s| s.kind == SectionKind::Single)
		.map(|s| s.handle.clone());
	Value::from(handles.collect::<Vec<_>>())
}

fn site_map(
	schema: &Schema,
	export: &Export,
	opts: &Options,
) -> Result<HashMap<String, String>> {
	let mut map = HashMap::new();
	if !opts.sites.is_empty() {
		for (craft, zipp) in &opts.sites {
			if schema.get_site(zipp).is_none() {
				return Err(Error::Request(format!("unknown site {zipp}")));
			}
			map.insert(craft.clone(), zipp.clone());
		}
	} else if export.sites.len() == 1 && schema.sites.len() == 1 {
		map.insert(
			export.sites[0].handle.clone(),
			schema.sites[0].handle.clone(),
		);
	} else {
		for s in &export.sites {
			if schema.get_site(&s.handle).is_some() {
				map.insert(s.handle.clone(), s.handle.clone());
			}
		}
	}
	if map.is_empty() {
		return Err(Error::Request(
			"no site of Craft matches one of zipp, use --site craft=zipp"
				.into(),
		));
	}
	Ok(map)
}

fn title_of(e: &Element) -> String {
	match e.title.as_deref().map(str::trim) {
		Some(t) if !t.is_empty() => t.to_string(),
		_ => e.slug.clone().unwrap_or_else(|| format!("Entry {}", e.id)),
	}
}

fn describe(err: Error) -> String {
	match err {
		Error::Invalid(errors) => errors
			.iter()
			.map(|e| format!("{} {}", e.path, e.message))
			.collect::<Vec<_>>()
			.join(", "),
		e => e.to_string(),
	}
}

/// `entries::save` inside the import's transaction.
fn save(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	input: EntryInput,
) -> Result<()> {
	let checked = entries::check(conn, schema, Some(id), input)?;
	entries::write(conn, schema, &checked)?;
	Ok(())
}

/// An entry without fields, unpublished in every site, for the save that
/// follows once all entries exist.
fn create_empty(
	conn: &Connection,
	schema: &Schema,
	section: &str,
	ty: &str,
	title: &str,
	parent: Option<i64>,
) -> Result<i64> {
	let now = now();
	conn.execute(
		"INSERT INTO entries (section, type, parent_id, position, created_at,
		updated_at) VALUES (?1, ?2, ?3, (SELECT IFNULL(MAX(position) + 1, 0)
		FROM entries WHERE section = ?1 AND parent_id IS ?3), ?4, ?4)",
		params![section, ty, parent, now],
	)?;
	let id = conn.last_insert_rowid();
	for site in &schema.sites {
		conn.execute(
			"INSERT INTO entry_sites (entry_id, site, title, slug, uri,
			published, fields, updated_at)
			VALUES (?1, ?2, ?3, ?4, NULL, 0, '{}', ?5)",
			params![id, site.handle, title, format!("import-{id}"), now],
		)?;
	}
	Ok(id)
}

struct Importer<'a> {
	schema: &'a Schema,
	/// Craft element id to zipp entry id
	entries: HashMap<i64, i64>,
	/// Craft asset id to zipp asset id
	assets: HashMap<i64, i64>,
	report: Report,
}

impl Importer<'_> {
	fn skip(&mut self, what: String) {
		self.report.skipped.push(what);
	}

	fn assets(
		&mut self,
		conn: &Connection,
		files: &Path,
		export: &Export,
		opts: &Options,
	) -> Result<()> {
		for a in &export.assets {
			let Some(dir) = opts.volumes.get(&a.volume) else {
				self.skip(format!(
					"asset {}: no folder for volume {}, use --volume {}=<dir>",
					a.path, a.volume, a.volume
				));
				continue;
			};
			let data = match fs::read(dir.join(&a.path)) {
				Ok(d) => d,
				Err(e) => {
					self.skip(format!("asset {}: {e}", a.path));
					continue;
				}
			};
			let name = a.path.rsplit('/').next().unwrap_or(&a.path);
			let stored = match assets::store(conn, files, name, &data, None) {
				Ok(s) => s,
				Err(e) => {
					self.skip(format!("asset {}: {e}", a.path));
					continue;
				}
			};
			let (fx, fy) = a.focal_point.unwrap_or((0.5, 0.5));
			conn.execute(
				"UPDATE assets SET title = ?1, alt = ?2, focal_x = ?3,
				focal_y = ?4 WHERE id = ?5",
				params![
					a.title.as_deref().unwrap_or_default(),
					a.alt.as_deref().unwrap_or_default(),
					fx,
					fy,
					stored.id
				],
			)?;
			self.assets.insert(a.id, stored.id);
			self.report.assets += 1;
		}
		Ok(())
	}

	/// The fields of a component from Craft's values of the same handles.
	fn fields(&mut self, comp: &Component, craft: &Value) -> Value {
		let mut out = Map::new();
		for field in &comp.fields {
			let Some(v) = craft.get(&field.handle).filter(|v| !v.is_null())
			else {
				continue;
			};
			if let Some(v) = self.value(field, v) {
				out.insert(field.handle.clone(), v);
			}
		}
		Value::Object(out)
	}

	fn value(&mut self, field: &Field, v: &Value) -> Option<Value> {
		let schema = self.schema;
		match &field.kind {
			// Craft's `2026-11-14T20:00:00+01:00`: the day and time as
			// they are there, without the zone
			FieldKind::Date { time } => {
				let s = v.as_str().or_else(|| v["date"].as_str())?;
				let s = s.get(..if *time { 16 } else { 10 })?;
				crate::content::valid_date(s, *time).then(|| Value::from(s))
			}
			FieldKind::Text { .. }
			| FieldKind::Markdown
			| FieldKind::Dropdown { .. } => match v {
				Value::String(s) if !s.is_empty() => Some(v.clone()),
				Value::Number(n) => Some(Value::from(n.to_string())),
				_ => None,
			},
			FieldKind::Number { .. } => match v {
				Value::Number(_) => Some(v.clone()),
				Value::String(s) => s.parse::<f64>().ok().map(Value::from),
				_ => None,
			},
			FieldKind::Boolean => v.as_bool().map(Value::from),
			FieldKind::Checkboxes { .. } | FieldKind::Table { .. } => {
				Some(v.clone())
			}
			FieldKind::Texts { .. } => v.is_object().then(|| v.clone()),
			FieldKind::Relation { .. } => Some(self.ids(v, false)),
			FieldKind::Assets { .. } => Some(self.ids(v, true)),
			FieldKind::Link { new_tab, .. } => self.link(v, *new_tab),
			FieldKind::Navigation { .. } => Some(self.nav(v)),
			FieldKind::Group { component } => {
				let comp = schema.get(component).unwrap();
				Some(self.fields(comp, &v["fields"]))
			}
			FieldKind::Blocks { types, .. } => {
				let mut blocks = vec![];
				for nested in v.as_array().into_iter().flatten() {
					if let Some(b) = self.block(field, types, nested) {
						blocks.push(b);
					}
				}
				Some(Value::from(blocks))
			}
			FieldKind::RichText { .. } => Some(self.rich_text(field, v)),
			FieldKind::Custom { .. } => Some(v.clone()),
		}
	}

	fn ids(&self, v: &Value, assets: bool) -> Value {
		let map = if assets { &self.assets } else { &self.entries };
		let ids = v.as_array().into_iter().flatten();
		let ids = ids.filter_map(|id| map.get(&id.as_i64()?).copied());
		Value::from(ids.collect::<Vec<_>>())
	}

	/// A nested entry as a block of the same type, its title into a field
	/// `title` if the block has one; a disabled one is a switched off block.
	fn block(
		&mut self,
		field: &Field,
		types: &[String],
		nested: &Value,
	) -> Option<Value> {
		let ty = nested["type"].as_str().unwrap_or_default();
		let Some(comp) = self
			.schema
			.get(ty)
			.filter(|_| types.iter().any(|t| t == ty))
		else {
			self.skip(format!("{}: no block type {ty}", field.handle));
			return None;
		};
		let mut fields = self.fields(comp, &nested["fields"]);
		let title = nested["title"].as_str().filter(|t| !t.is_empty());
		if let Some(t) = title
			&& comp.fields.iter().any(|f| f.handle == "title")
		{
			fields["title"] = Value::from(t);
		}
		let mut block = json!({ "type": ty, "fields": fields });
		if nested["enabled"] == false {
			block["enabled"] = false.into();
		}
		Some(block)
	}

	/// Craft's link `{ type, id, value, label, target, download }`.
	fn link(&self, v: &Value, new_tab: bool) -> Option<Value> {
		let ty = v["type"].as_str()?;
		let (kind, value) = match ty {
			"entry" | "category" => {
				("entry", Value::from(*self.entries.get(&v["id"].as_i64()?)?))
			}
			"asset" => {
				("asset", Value::from(*self.assets.get(&v["id"].as_i64()?)?))
			}
			"url" => ("url", v["value"].clone()),
			"email" => {
				let e = v["value"].as_str()?;
				("email", Value::from(e.trim_start_matches("mailto:")))
			}
			"tel" => {
				let p = v["value"].as_str()?;
				("phone", Value::from(p.trim_start_matches("tel:")))
			}
			_ => return None,
		};
		let mut link = json!({ "type": kind, "value": value });
		if let Some(l) = v["label"].as_str().filter(|l| !l.is_empty()) {
			link["label"] = Value::from(l);
		}
		// zipp opens external links in a new tab, Craft only with a target
		let external = kind == "asset"
			|| value.as_str().is_some_and(|u| u.starts_with("http"));
		let blank = v["target"] == "_blank";
		if blank != (new_tab && external) {
			link["newTab"] = Value::from(blank);
		}
		if kind == "asset" && v["download"] == true {
			link["download"] = Value::from(true);
		}
		Some(link)
	}

	/// A matrix as a navigation: each nested entry's link and the nested
	/// entries of its own matrix under it.
	fn nav(&self, v: &Value) -> Value {
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
	fn rich_text(&mut self, field: &Field, v: &Value) -> Value {
		let FieldKind::RichText {
			headings,
			lists,
			links,
			buttons,
			blocks,
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

#[cfg(test)]
mod tests;
