//! Schema migrations.
//!
//! `makemigration` compares the schema in code with the one of the last
//! migration file and writes the difference as a list of ops into a new
//! file, next to the resulting schema. On start the server applies new
//! migration files to all content.
//!
//! Labels, names, `required` and the field order are free to change, they
//! don't touch content. Everything else needs a migration. A rename can't
//! be told apart from a remove and an add, it is passed as a hint.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::content::walk_mut;
use crate::schema::{Component, Field, Schema, Section, Site};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum Op {
	AddComponent {
		component: Component,
	},
	RemoveComponent {
		component: String,
	},
	RenameComponent {
		from: String,
		to: String,
	},
	AddField {
		component: String,
		field: Field,
	},
	RemoveField {
		component: String,
		field: String,
	},
	RenameField {
		component: String,
		from: String,
		to: String,
	},
	/// A new kind or new settings. Values are dropped if the kind changed,
	/// a field that is no longer translatable gets the value of the
	/// primary site everywhere.
	ChangeField {
		component: String,
		field: Field,
	},
	/// The new site starts with a copy of the primary site.
	AddSite {
		site: Site,
	},
	RemoveSite {
		site: String,
	},
	AddSection {
		section: Section,
	},
	/// Deletes the entries of the section.
	RemoveSection {
		section: String,
	},
	/// Deletes the entries whose type the section no longer has.
	ChangeSection {
		section: Section,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum Rename {
	Component {
		from: String,
		to: String,
	},
	Field {
		component: String,
		from: String,
		to: String,
	},
}

impl Rename {
	/// `old=new` for a component, `component.old=new` for a field.
	pub fn parse(s: &str) -> Result<Self, String> {
		let (from, to) = s
			.split_once('=')
			.ok_or_else(|| format!("rename {s:?} needs old=new"))?;
		Ok(match from.split_once('.') {
			Some((component, from)) => Self::Field {
				component: component.into(),
				from: from.into(),
				to: to.into(),
			},
			None => Self::Component {
				from: from.into(),
				to: to.into(),
			},
		})
	}
}

/// What has to happen to go from `old` to `new`.
pub fn diff(
	old: &Schema,
	new: &Schema,
	renames: &[Rename],
) -> Result<Vec<Op>, String> {
	let mut ops = vec![];
	let mut cur = old.clone();
	let mut push = |op: Op, cur: &mut Schema| {
		op.apply_schema(cur);
		ops.push(op);
	};

	for rename in renames {
		let op = match rename {
			Rename::Component { from, to } => {
				if cur.get(from).is_none() || new.get(to).is_none() {
					return Err(format!("can't rename {from} to {to}"));
				}
				Op::RenameComponent {
					from: from.clone(),
					to: to.clone(),
				}
			}
			Rename::Field {
				component,
				from,
				to,
			} => {
				let has = |s: &Schema, f: &str| {
					s.get(component)
						.is_some_and(|c| c.fields.iter().any(|x| x.handle == f))
				};
				if !has(&cur, from) || !has(new, to) {
					return Err(format!(
						"can't rename {component}.{from} to {to}"
					));
				}
				Op::RenameField {
					component: component.clone(),
					from: from.clone(),
					to: to.clone(),
				}
			}
		};
		push(op, &mut cur);
	}

	// a component that switches between entry and block starts over
	let gone: Vec<String> = cur
		.components
		.iter()
		.filter(|c| new.get(&c.handle).is_none_or(|n| n.entry != c.entry))
		.map(|c| c.handle.clone())
		.collect();
	for component in gone {
		push(Op::RemoveComponent { component }, &mut cur);
	}

	for comp in &new.components {
		let Some(old_comp) = cur.get(&comp.handle).cloned() else {
			let op = Op::AddComponent {
				component: comp.clone(),
			};
			push(op, &mut cur);
			continue;
		};
		let component = comp.handle.clone();

		for f in &old_comp.fields {
			if !comp.fields.iter().any(|n| n.handle == f.handle) {
				let op = Op::RemoveField {
					component: component.clone(),
					field: f.handle.clone(),
				};
				push(op, &mut cur);
			}
		}

		for f in &comp.fields {
			let op = match old_comp.fields.iter().find(|o| o.handle == f.handle)
			{
				None => Op::AddField {
					component: component.clone(),
					field: f.clone(),
				},
				Some(o)
					if o.kind != f.kind
						|| o.indexed != f.indexed
						|| o.translatable != f.translatable =>
				{
					Op::ChangeField {
						component: component.clone(),
						field: f.clone(),
					}
				}
				Some(_) => continue,
			};
			push(op, &mut cur);
		}
	}

	// add sites first, they copy the primary site of before
	for site in &new.sites {
		if cur.get_site(&site.handle).is_none() {
			push(Op::AddSite { site: site.clone() }, &mut cur);
		}
	}
	let gone: Vec<String> = cur
		.sites
		.iter()
		.filter(|s| new.get_site(&s.handle).is_none())
		.map(|s| s.handle.clone())
		.collect();
	for site in gone {
		push(Op::RemoveSite { site }, &mut cur);
	}

	let gone: Vec<String> = cur
		.sections
		.iter()
		.filter(|s| new.get_section(&s.handle).is_none())
		.map(|s| s.handle.clone())
		.collect();
	for section in gone {
		push(Op::RemoveSection { section }, &mut cur);
	}
	for section in &new.sections {
		let op = match cur.get_section(&section.handle) {
			None => Op::AddSection {
				section: section.clone(),
			},
			Some(o)
				if o.kind != section.kind
					|| o.types != section.types
					|| o.uri != section.uri
					|| o.type_uris != section.type_uris =>
			{
				Op::ChangeSection {
					section: section.clone(),
				}
			}
			Some(_) => continue,
		};
		push(op, &mut cur);
	}

	Ok(ops)
}

impl Op {
	pub fn apply_schema(&self, schema: &mut Schema) {
		match self {
			Self::AddComponent { component } => {
				schema.components.push(component.clone())
			}
			Self::RemoveComponent { component } => {
				schema.components.retain(|c| &c.handle != component)
			}
			Self::RenameComponent { from, to } => {
				if let Some(c) = schema.get_mut(from) {
					c.handle = to.clone();
				}
			}
			Self::AddField { component, field } => {
				if let Some(c) = schema.get_mut(component) {
					c.fields.push(field.clone());
				}
			}
			Self::RemoveField { component, field } => {
				if let Some(c) = schema.get_mut(component) {
					c.fields.retain(|f| &f.handle != field);
				}
			}
			Self::RenameField {
				component,
				from,
				to,
			} => {
				let c = schema.get_mut(component);
				let f = c.and_then(|c| {
					c.fields.iter_mut().find(|f| &f.handle == from)
				});
				if let Some(f) = f {
					f.handle = to.clone();
				}
			}
			Self::ChangeField { component, field } => {
				let c = schema.get_mut(component);
				let f = c.and_then(|c| {
					c.fields.iter_mut().find(|f| f.handle == field.handle)
				});
				if let Some(f) = f {
					*f = field.clone();
				}
			}
			Self::AddSite { site } => schema.sites.push(site.clone()),
			Self::RemoveSite { site } => {
				schema.sites.retain(|s| &s.handle != site)
			}
			Self::AddSection { section } => {
				schema.sections.push(section.clone())
			}
			Self::RemoveSection { section } => {
				schema.sections.retain(|s| &s.handle != section)
			}
			Self::ChangeSection { section } => {
				let s = schema
					.sections
					.iter_mut()
					.find(|s| s.handle == section.handle);
				if let Some(s) = s {
					*s = section.clone();
				}
			}
		}
	}

	/// Changes one entry. `schema` is the schema before this op. Returns
	/// false when the entry has to be deleted.
	pub fn apply_entry(
		&self,
		schema: &Schema,
		ty: &mut String,
		fields: &mut Value,
	) -> bool {
		let Some(comp) = schema.get(ty) else {
			return true;
		};

		match self {
			// sites and sections change rows, see entries::apply_migrations
			Self::AddComponent { .. }
			| Self::AddField { .. }
			| Self::AddSite { .. }
			| Self::RemoveSite { .. }
			| Self::AddSection { .. }
			| Self::RemoveSection { .. }
			| Self::ChangeSection { .. } => {}
			Self::RemoveComponent { component } => {
				if ty == component {
					return false;
				}
				walk_mut(schema, comp, fields, &mut |_, obj| {
					for v in obj.values_mut() {
						if let Value::Array(items) = v {
							items.retain(|b| b["type"] != component.as_str());
						}
					}
				});
			}
			Self::RenameComponent { from, to } => {
				// walk with the new names, a block is renamed before the
				// walk goes into it
				let mut after = schema.clone();
				self.apply_schema(&mut after);
				if ty == from {
					*ty = to.clone();
				}
				let comp = after.get(ty).unwrap();
				walk_mut(&after, comp, fields, &mut |_, obj| {
					for v in obj.values_mut() {
						let Value::Array(items) = v else { continue };
						for b in items {
							if b["type"] == from.as_str() {
								b["type"] = to.clone().into();
							}
						}
					}
				});
			}
			Self::RemoveField { component, field } => {
				walk_mut(schema, comp, fields, &mut |c, obj| {
					if &c.handle == component {
						obj.remove(field);
					}
				});
			}
			Self::RenameField {
				component,
				from,
				to,
			} => {
				walk_mut(schema, comp, fields, &mut |c, obj| {
					if &c.handle == component
						&& let Some(v) = obj.remove(from)
					{
						obj.insert(to.clone(), v);
					}
				});
			}
			Self::ChangeField { component, field } => {
				let old = schema.get(component).and_then(|c| {
					c.fields.iter().find(|f| f.handle == field.handle)
				});
				let same_kind = old.is_some_and(|o| {
					std::mem::discriminant(&o.kind)
						== std::mem::discriminant(&field.kind)
				});
				if !same_kind {
					walk_mut(schema, comp, fields, &mut |c, obj| {
						if &c.handle == component {
							obj.remove(&field.handle);
						}
					});
				}
			}
		}

		true
	}

	/// One line for the review of a new migration, `!` marks lost content.
	pub fn describe(&self) -> String {
		match self {
			Self::AddComponent { component } => {
				format!("  add component {}", component.handle)
			}
			Self::RemoveComponent { component } => format!(
				"! remove component {component}, its entries and blocks are deleted"
			),
			Self::RenameComponent { from, to } => {
				format!("  rename component {from} to {to}")
			}
			Self::AddField { component, field } => {
				format!("  add field {component}.{}", field.handle)
			}
			Self::RemoveField { component, field } => format!(
				"! remove field {component}.{field}, its content is deleted"
			),
			Self::RenameField {
				component,
				from,
				to,
			} => format!("  rename field {component}.{from} to {to}"),
			Self::ChangeField { component, field } => format!(
				"  change field {component}.{} (content is dropped if the kind changed)",
				field.handle
			),
			Self::AddSite { site } => format!(
				"  add site {}, it starts as a copy of the primary site",
				site.handle
			),
			Self::RemoveSite { site } => {
				format!("! remove site {site}, its content is deleted")
			}
			Self::AddSection { section } => {
				format!("  add section {}", section.handle)
			}
			Self::RemoveSection { section } => {
				format!("! remove section {section}, its entries are deleted")
			}
			Self::ChangeSection { section } => format!(
				"  change section {} (entries of types it no longer has are deleted)",
				section.handle
			),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Migration {
	#[serde(skip)]
	pub name: String,
	pub ops: Vec<Op>,
	/// The schema after this migration.
	pub schema: Schema,
}

/// The migration files of a site, see [`migrations!`](crate::migrations).
pub struct Migrations {
	/// `migrations/` of the site's crate, where new ones are written
	pub dir: PathBuf,
	files: Vec<(String, Vec<u8>)>,
}

impl Migrations {
	pub fn new<E: rust_embed::Embed>(dir: &str) -> Self {
		let files = E::iter()
			.filter_map(|name| {
				Some((name.to_string(), E::get(&name)?.data.into()))
			})
			.collect();
		Self {
			dir: dir.into(),
			files,
		}
	}

	/// The migrations sorted by name.
	pub fn load(&self) -> Result<Vec<Migration>, String> {
		let mut migrations = vec![];
		for (file, data) in &self.files {
			let Some(name) = file.strip_suffix(".json") else {
				continue;
			};
			let mut m: Migration = serde_json::from_slice(data)
				.map_err(|e| format!("migration {file}: {e}"))?;
			m.name = name.into();
			migrations.push(m);
		}

		migrations.sort_by(|a, b| a.name.cmp(&b.name));
		Ok(migrations)
	}
}

/// `0003` or `0003_name` after two existing migrations.
pub fn next_name(existing: &[Migration], name: Option<&str>) -> String {
	let n = format!("{:04}", existing.len() + 1);
	match name {
		Some(name) => format!("{n}_{name}"),
		None => n,
	}
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;
	use crate::schema::tests::site;

	fn apply_all(old: &Schema, ops: &[Op]) -> Schema {
		let mut s = old.clone();
		for op in ops {
			op.apply_schema(&mut s);
		}
		s
	}

	#[test]
	fn no_changes() {
		let s = site();
		assert_eq!(diff(&s, &s, &[]).unwrap(), []);

		// labels and order are free
		let mut s2 = s.clone();
		s2.components[0].fields.reverse();
		s2.components[0].fields[0].label = "Content".into();
		assert_eq!(diff(&s, &s2, &[]).unwrap(), []);
	}

	#[test]
	fn initial_migration_adds_everything() {
		let ops = diff(&Schema::new(), &site(), &[]).unwrap();
		assert_eq!(ops.len(), 9);
		assert!(
			diff(&apply_all(&Schema::new(), &ops), &site(), &[])
				.unwrap()
				.is_empty()
		);
	}

	fn changed_site() -> Schema {
		let mut s = site();
		// page.intro renamed to lead, button.label removed, event.city
		// changed to a number, text renamed to paragraph
		let page = s.get_mut("page").unwrap();
		page.fields[0].handle = "lead".into();
		let button = s.get_mut("button").unwrap();
		button.fields.remove(0);
		let event = s.get_mut("event").unwrap();
		event.fields[0] = Field::number("city").indexed();
		let text = s.get_mut("text").unwrap();
		text.handle = "paragraph".into();
		let page = s.get_mut("page").unwrap();
		page.fields[1] = Field::blocks("body", ["paragraph", "button"]);
		s
	}

	#[test]
	fn diff_with_renames() {
		let renames = [
			Rename::parse("text=paragraph").unwrap(),
			Rename::parse("page.intro=lead").unwrap(),
		];
		let ops = diff(&site(), &changed_site(), &renames).unwrap();
		let kinds: Vec<_> = ops.iter().map(|o| o.describe()).collect();
		assert_eq!(
			kinds,
			[
				"  rename component text to paragraph",
				"  rename field page.intro to lead",
				"  change field page.body (content is dropped if the kind changed)",
				"  change field event.city (content is dropped if the kind changed)",
				"! remove field button.label, its content is deleted",
			]
		);
		let after = apply_all(&site(), &ops);
		assert!(diff(&after, &changed_site(), &[]).unwrap().is_empty());
	}

	#[test]
	fn without_hint_rename_is_remove_and_add() {
		let ops = diff(&site(), &changed_site(), &[]).unwrap();
		assert!(ops.contains(&Op::RemoveComponent {
			component: "text".into()
		}));
		assert!(ops.contains(&Op::RemoveField {
			component: "page".into(),
			field: "intro".into()
		}));
	}

	#[test]
	fn applies_to_content() {
		let renames = [
			Rename::parse("text=paragraph").unwrap(),
			Rename::parse("page.intro=lead").unwrap(),
		];
		let ops = diff(&site(), &changed_site(), &renames).unwrap();

		let mut ty = "page".to_string();
		let mut fields = json!({
			"intro": "Hi",
			"body": [
				{ "id": "a", "type": "text", "fields": { "content": "x" } },
				{ "id": "b", "type": "button",
					"fields": { "label": "Go", "link": [1] } },
			]
		});
		let mut schema = site();
		for op in &ops {
			assert!(op.apply_entry(&schema, &mut ty, &mut fields));
			op.apply_schema(&mut schema);
		}

		assert_eq!(
			fields,
			json!({
				"body": [
					{ "id": "a", "type": "paragraph",
						"fields": { "content": "x" } },
					{ "id": "b", "type": "button", "fields": { "link": [1] } },
				],
				"lead": "Hi",
			})
		);

		let mut ty = "event".to_string();
		let mut fields = json!({ "city": "Bern", "price": 3 });
		let mut schema = site();
		for op in &ops {
			op.apply_entry(&schema, &mut ty, &mut fields);
			op.apply_schema(&mut schema);
		}
		assert_eq!(fields, json!({ "price": 3 }));
	}

	#[test]
	fn removing_a_component_deletes_entries_and_blocks() {
		let op = Op::RemoveComponent {
			component: "button".into(),
		};
		let mut ty = "page".to_string();
		let mut fields = json!({ "body": [
			{ "id": "a", "type": "button", "fields": {} },
			{ "id": "b", "type": "text", "fields": { "content": "x" } },
		]});
		assert!(op.apply_entry(&site(), &mut ty, &mut fields));
		assert_eq!(fields["body"].as_array().unwrap().len(), 1);

		let op = Op::RemoveComponent {
			component: "event".into(),
		};
		let mut ty = "event".to_string();
		assert!(!op.apply_entry(&site(), &mut ty, &mut json!({})));
	}

	#[test]
	fn diff_sites_and_sections() {
		let mut new = site().site("fr", "Français", "fr");
		new.sites.retain(|s| s.handle != "en");
		new.sections[2].uri = Some("agenda/{slug}".into());
		new.sections.remove(1);

		let ops = diff(&site(), &new, &[]).unwrap();
		let lines: Vec<_> = ops.iter().map(|o| o.describe()).collect();
		assert_eq!(
			lines,
			[
				"  add site fr, it starts as a copy of the primary site",
				"! remove site en, its content is deleted",
				"! remove section pages, its entries are deleted",
				"  change section events (entries of types it no longer has \
				 are deleted)",
			]
		);
		assert!(
			diff(&apply_all(&site(), &ops), &new, &[])
				.unwrap()
				.is_empty()
		);
	}

	#[test]
	fn migration_file_roundtrip() {
		let m = Migration {
			name: String::new(),
			ops: diff(&Schema::new(), &site(), &[]).unwrap(),
			schema: site(),
		};
		let s = serde_json::to_string_pretty(&m).unwrap();
		let back: Migration = serde_json::from_str(&s).unwrap();
		assert_eq!(back.ops, m.ops);
		assert_eq!(next_name(&[back], Some("init")), "0002_init");
	}
}
