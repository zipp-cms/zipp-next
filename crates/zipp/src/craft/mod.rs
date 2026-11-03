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
//! under it), content blocks to groups (and flat fields to a group with
//! their handles), CKEditor's html to rich text (nested entries become
//! embedded blocks or, if they are only a link, buttons), categories to
//! relations, meierlabs-seo's field to zipp-seo's, TableMaker to a table
//! by the order of its columns. Assets are copied with their title, alt
//! text, focal point and fields, each volume a folder with Craft's folders
//! in it.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Component as PathPart, Path, PathBuf};

use rusqlite::{Connection, params};
use serde_json::{Map, Value, json};

use crate::assets;
use crate::db::{now, parse_iso};
use crate::entries::{self, EntryInput, next_position};
use crate::error::{Error, Result};
use crate::schema::{ASSET, Component, Field, FieldKind, Schema, SectionKind};

mod export;
mod html;
mod rich;

use export::Element;
pub use export::Export;

pub struct Options {
	/// Craft site handle to zipp site handle; without it sites match by
	/// handle, or the only site of each
	pub sites: Vec<(String, String)>,
	/// Craft volume handle to where its files are
	pub volumes: HashMap<String, Volume>,
}

pub struct Volume {
	/// the folder of its files
	pub dir: PathBuf,
	/// the asset folder it becomes, `None` the volume's name, `""` none:
	/// its files and folders go on the top
	pub folder: Option<String>,
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
		folders: HashMap::new(),
		report: Report::default(),
	};

	let tx = conn.unchecked_transaction()?;
	im.assets(&tx, files, export, opts, &sites)?;
	let planned = im.plan(export, &sites);
	im.save_planned(&tx, &planned)?;
	im.asset_fields(&tx, export)?;
	im.import_globals(&tx, export, &sites)?;

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
	let checked = entries::check(conn, schema, Some(id), input, None)?;
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
	let position = next_position(conn, section, parent)?;
	conn.execute(
		"INSERT INTO entries (section, type, parent_id, position, created_at,
		updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
		params![section, ty, parent, position, now],
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

/// An element of Craft in each zipp site, with its section and entry type.
struct Planned {
	section: String,
	ty: String,
	versions: Vec<(String, Element)>,
}

struct Importer<'a> {
	schema: &'a Schema,
	/// Craft element id to zipp entry id
	entries: HashMap<i64, i64>,
	/// Craft asset id to zipp asset id
	assets: HashMap<i64, i64>,
	/// a folder's parent and name to its id
	folders: HashMap<(Option<i64>, String), i64>,
	report: Report,
}

impl Importer<'_> {
	fn skip(&mut self, what: String) {
		self.report.skipped.push(what);
	}

	/// Every element of a section with its entry type, in Craft's order.
	fn plan(
		&mut self,
		export: &Export,
		sites: &HashMap<String, String>,
	) -> Vec<Planned> {
		let mut planned: Vec<Planned> = vec![];
		let groups = export.sections.iter().map(|(h, s)| (h, &s.sites, false));
		let cats = export.categories.iter().map(|(h, s)| (h, s, true));
		for (handle, per_site, category) in groups.chain(cats) {
			let Some(section) = self.schema.get_section(handle) else {
				self.skip(format!("section {handle}: not in the schema"));
				continue;
			};
			if section.kind == SectionKind::Single {
				self.skip(format!("section {handle}: a single in zipp"));
				continue;
			}

			let mut order: Vec<i64> = vec![];
			let mut by_id: HashMap<i64, Vec<(String, Element)>> =
				HashMap::new();
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
						let title =
							versions[0].1.title.clone().unwrap_or_default();
						self.skip(format!(
							"{handle}: {title}: no entry type {}",
							t.unwrap_or_default()
						));
						continue;
					}
				};
				planned.push(Planned {
					section: handle.clone(),
					ty,
					versions,
				});
			}
		}
		planned
	}

	/// Saves the planned entries: first every entry without fields, so
	/// relations find each other, then their fields in each site.
	fn save_planned(
		&mut self,
		tx: &Connection,
		planned: &[Planned],
	) -> Result<()> {
		for p in planned {
			let e = &p.versions[0].1;
			let parent =
				e.parent_id.and_then(|p| self.entries.get(&p).copied());
			let id = create_empty(
				tx,
				self.schema,
				&p.section,
				&p.ty,
				&title_of(e),
				parent,
			)?;
			self.entries.insert(e.id, id);
		}

		for p in planned {
			let id = self.entries[&p.versions[0].1.id];
			let comp = self.schema.entry_type(&p.ty).unwrap();
			let mut saved = false;
			for (site, e) in &p.versions {
				let fields = self.fields(comp, &e.fields);
				let input = EntryInput {
					section: p.section.clone(),
					ty: p.ty.clone(),
					site: Some(site.clone()),
					title: title_of(e),
					slug: e.slug.clone().unwrap_or_default(),
					published: e.enabled,
					dates: entries::Dates {
						post_date: e.post_date.as_deref().and_then(parse_iso),
						expiry_date: e
							.expiry_date
							.as_deref()
							.and_then(parse_iso),
					},
					parent_id: None,
					fields,
				};
				match save(tx, self.schema, id, input) {
					Ok(()) => saved = true,
					Err(err) => self.skip(format!(
						"{}: {}: {}",
						p.section,
						title_of(e),
						describe(err)
					)),
				}
			}
			// not even an empty entry of what fits in no site
			if !saved {
				entries::delete(tx, self.schema, id)?;
				continue;
			}
			let e = &p.versions[0].1;
			let created = e.post_date.as_deref().or(e.date_created.as_deref());
			if let Some(ms) = created.and_then(parse_iso) {
				tx.execute(
					"UPDATE entries SET created_at = ?1 WHERE id = ?2",
					params![ms, id],
				)?;
			}
			self.report.entries += 1;
		}
		Ok(())
	}

	/// Global sets fill the singles of the same handle.
	fn import_globals(
		&mut self,
		tx: &Connection,
		export: &Export,
		sites: &HashMap<String, String>,
	) -> Result<()> {
		for (handle, per_site) in &export.globals {
			let single = self
				.schema
				.get_section(handle)
				.filter(|s| s.kind == SectionKind::Single);
			let Some(section) = single else {
				self.skip(format!("global set {handle}: no single {handle}"));
				continue;
			};
			let id: i64 = tx.query_row(
				"SELECT id FROM entries WHERE section = ?1",
				[handle],
				|r| r.get(0),
			)?;
			let comp = self.schema.entry_type(&section.types[0]).unwrap();
			for (craft_site, global) in per_site {
				let Some(site) = sites.get(craft_site) else {
					continue;
				};
				let current = entries::get_entry(tx, id, site)?;
				let input = EntryInput {
					section: handle.clone(),
					ty: section.types[0].clone(),
					site: Some(site.clone()),
					title: current.title,
					slug: current.slug,
					published: true,
					dates: Default::default(),
					parent_id: None,
					fields: self.fields(comp, &global.fields),
				};
				if let Err(err) = save(tx, self.schema, id, input) {
					self.skip(format!(
						"global set {handle}: {}",
						describe(err)
					));
				}
			}
		}
		Ok(())
	}

	fn assets(
		&mut self,
		conn: &Connection,
		files: &Path,
		export: &Export,
		opts: &Options,
		sites: &HashMap<String, String>,
	) -> Result<()> {
		for a in &export.assets {
			let Some(volume) = opts.volumes.get(&a.volume) else {
				self.skip(format!(
					"asset {}: no folder for volume {}, use --volume {}=<dir>",
					a.path, a.volume, a.volume
				));
				continue;
			};
			// the export says where in the volume, never out of it
			let inside = Path::new(&a.path)
				.components()
				.all(|c| matches!(c, PathPart::Normal(_) | PathPart::CurDir));
			if !inside {
				self.skip(format!(
					"asset {}: a path out of the volume",
					a.path
				));
				continue;
			}
			let data = match fs::read(volume.dir.join(&a.path)) {
				Ok(d) => d,
				Err(e) => {
					self.skip(format!("asset {}: {e}", a.path));
					continue;
				}
			};
			let (dirs, name) = match a.path.rsplit_once('/') {
				Some((dirs, name)) => (Some(dirs), name),
				None => (None, a.path.as_str()),
			};
			let top = match &volume.folder {
				Some(f) => f.as_str(),
				None => a.volume_name.as_deref().unwrap_or(&a.volume),
			};
			let path = [top]
				.into_iter()
				.chain(dirs.into_iter().flat_map(|d| d.split('/')));
			let folder = match self.folder(conn, path) {
				Ok(f) => f,
				Err(e) => {
					self.skip(format!("asset {}: {e}", a.path));
					continue;
				}
			};
			let stored = match assets::store(conn, files, name, &data, None) {
				Ok(s) => s,
				Err(e) => {
					self.skip(format!("asset {}: {e}", a.path));
					continue;
				}
			};
			let (fx, fy) = a.focal_point.unwrap_or((0.5, 0.5));
			// per zipp site, an older export has only the primary's
			let mut alt: BTreeMap<&str, &str> = a
				.alts
				.iter()
				.filter_map(|(craft, alt)| {
					Some((sites.get(craft)?.as_str(), alt.as_str()))
				})
				.collect();
			if let (true, Some(primary)) = (alt.is_empty(), a.alt.as_deref()) {
				alt.insert(&self.schema.primary_site().handle, primary);
			}
			alt.retain(|_, a| !a.trim().is_empty());
			conn.execute(
				"UPDATE assets SET title = ?1, alt = ?2, focal_x = ?3,
				focal_y = ?4, folder_id = ?5 WHERE id = ?6",
				params![
					a.title.as_deref().unwrap_or_default(),
					serde_json::to_string(&alt).unwrap(),
					fx,
					fy,
					folder,
					stored.id
				],
			)?;
			self.assets.insert(a.id, stored.id);
			self.report.assets += 1;
		}
		Ok(())
	}

	/// The folder of a path of names, made if it isn't there; empty names
	/// stay where they are, so a volume without a name is the top.
	fn folder<'p>(
		&mut self,
		conn: &Connection,
		path: impl Iterator<Item = &'p str>,
	) -> Result<Option<i64>> {
		let mut parent = None;
		for name in path.filter(|n| !n.is_empty()) {
			let key = (parent, name.to_string());
			let id = match self.folders.get(&key) {
				Some(id) => *id,
				None => {
					let id = assets::create_folder(conn, parent, name)?.id;
					self.folders.insert(key, id);
					id
				}
			};
			parent = Some(id);
		}
		Ok(parent)
	}

	/// The asset fields, once the entries exist that they may link.
	fn asset_fields(
		&mut self,
		conn: &Connection,
		export: &Export,
	) -> Result<()> {
		let Some(comp) = self.schema.get(ASSET) else {
			return Ok(());
		};
		for a in &export.assets {
			let Some(&id) = self.assets.get(&a.id) else {
				continue;
			};
			let fields = self.fields(comp, &a.fields);
			if fields.as_object().is_none_or(Map::is_empty) {
				continue;
			}
			let checked = assets::check_fields(
				conn,
				self.schema,
				&fields,
				&json!({}),
				None,
			);
			match checked {
				Ok(f) => {
					conn.execute(
						"UPDATE assets SET fields = ?1 WHERE id = ?2",
						params![f.to_string(), id],
					)?;
				}
				Err(err) => {
					self.skip(format!("asset {}: {}", a.path, describe(err)))
				}
			}
		}
		Ok(())
	}

	/// The fields of a component from Craft's values of the same handles.
	fn fields(&mut self, comp: &Component, craft: &Value) -> Value {
		let mut flat = self.flat_groups(comp, craft);
		let mut out = Map::new();
		for field in &comp.fields {
			let v = match craft.get(&field.handle).filter(|v| !v.is_null()) {
				Some(v) => self.value(field, v),
				None => match flat.remove(&field.handle) {
					Some(group) => {
						self.value(field, &json!({ "fields": group }))
					}
					None => continue,
				},
			};
			if let Some(v) = v {
				out.insert(field.handle.clone(), v);
			}
		}
		Value::Object(out)
	}

	/// Groups that Craft didn't have: a group without a value of its own
	/// takes the flat Craft fields of its fields' handles, or of the
	/// group's handle and theirs (`contactHeading` for `heading` of a
	/// group `contact`), so a port can group what was flat in Craft. A
	/// Craft field fills one place only: none when the component has a
	/// field of its handle, and none when it fits several places or a
	/// place fits several Craft fields (that is reported).
	fn flat_groups(
		&mut self,
		comp: &Component,
		craft: &Value,
	) -> HashMap<String, Map<String, Value>> {
		let schema = self.schema;
		let groups: Vec<(&str, &Component)> = comp
			.fields
			.iter()
			.filter(|f| craft.get(&f.handle).is_none_or(Value::is_null))
			.filter_map(|f| match &f.kind {
				FieldKind::Group { component } => {
					Some((f.handle.as_str(), schema.get(component)?))
				}
				_ => None,
			})
			.collect();
		if groups.is_empty() {
			return HashMap::new();
		}

		// each flat Craft field with the places it fits
		let mut fits = vec![];
		for (handle, v) in craft.as_object().into_iter().flatten() {
			if v.is_null() || comp.fields.iter().any(|f| &f.handle == handle) {
				continue;
			}
			let mut places = vec![];
			for (group, g) in &groups {
				for f in &g.fields {
					if &f.handle == handle
						|| handle.strip_prefix(group)
							== Some(&upper_first(&f.handle))
					{
						places.push((*group, f.handle.as_str()));
					}
				}
			}
			if !places.is_empty() {
				fits.push((handle, v, places));
			}
		}

		let mut flat: HashMap<String, Map<String, Value>> = HashMap::new();
		for (handle, v, places) in &fits {
			let place = places[0];
			let shared = fits
				.iter()
				.filter(|(h, ..)| h != handle)
				.find(|(.., p)| p.contains(&place));
			let what = match (&places[..], shared) {
				([_], None) => {
					let fields = flat.entry(place.0.to_string()).or_default();
					fields.insert(place.1.to_string(), (*v).clone());
					continue;
				}
				([_], Some((other, ..))) => {
					let mut both = [handle, other];
					both.sort();
					format!(
						"{}: the Craft fields {} and {} fit {}.{}",
						comp.handle, both[0], both[1], place.0, place.1
					)
				}
				_ => {
					let places: Vec<String> = places
						.iter()
						.map(|(g, f)| format!("{g}.{f}"))
						.collect();
					format!(
						"{}: the Craft field {handle} fits {}",
						comp.handle,
						places.join(" and ")
					)
				}
			};
			// once, not for every entry
			if !self.report.skipped.contains(&what) {
				self.skip(what);
			}
		}
		flat
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
			| FieldKind::Email
			| FieldKind::Markdown
			| FieldKind::Dropdown { .. }
			| FieldKind::ButtonGroup { .. } => match v {
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
			FieldKind::Checkboxes { .. } => Some(v.clone()),
			// each cell like the field of its column, TableMaker's by
			// their order: its columns are each entry's own
			FieldKind::Table { columns } => {
				let maker = v.get("rows").is_some();
				let craft_rows = if maker { &v["rows"] } else { v };
				let more = v["columns"].as_array().map_or(0, Vec::len);
				if more > columns.len() {
					let what = format!(
						"{}: a table has {more} columns, only the first {} \
						are imported",
						field.handle,
						columns.len()
					);
					if !self.report.skipped.contains(&what) {
						self.skip(what);
					}
				}
				let mut rows = vec![];
				for row in craft_rows.as_array().into_iter().flatten() {
					let mut cells = serde_json::Map::new();
					for (i, c) in columns.iter().enumerate() {
						let cell =
							if maker { &row[i] } else { &row[&c.handle] };
						match self.value(&c.as_field(), cell) {
							None | Some(Value::Bool(false)) => {}
							Some(v) => {
								cells.insert(c.handle.clone(), v);
							}
						}
					}
					rows.push(Value::Object(cells));
				}
				Some(Value::from(rows))
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
			// zipp-seo's field from meierlabs-seo's, its images are ours
			FieldKind::Custom { ty, .. } if ty == "seo" => {
				let mut seo = v.as_object()?.clone();
				for key in ["image", "socialImage"] {
					if let Some(ids) = seo.get(key) {
						let ids = self.ids(ids, true);
						seo.insert(key.into(), ids);
					}
				}
				Some(Value::Object(seo))
			}
			FieldKind::Custom { .. } => Some(v.clone()),
			FieldKind::Note { .. } => None,
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
}

fn upper_first(s: &str) -> String {
	let mut c = s.chars();
	c.next()
		.map(|f| f.to_uppercase().chain(c).collect())
		.unwrap_or_default()
}

#[cfg(test)]
mod tests;
