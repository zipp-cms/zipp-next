//! Schema migrations.
//!
//! `makemigration` compares the schema in code with the one of the last
//! migration file and writes the difference as a list of ops into a new
//! file, next to the resulting schema. On start the server applies new
//! migration files to all content.
//!
//! Labels, names, `required`, defaults, widths, notes, conditions and the
//! field order are free to change, they don't touch content. Everything
//! else needs a migration.
//! A rename can't be told apart from a remove and an add, it is passed as
//! a hint. What the ops can't express is a data migration in Rust, see
//! [`DataTx`].

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::content::{default_of, walk_mut};
use crate::schema::{
	Column, Component, Field, FieldKind, Holds, Schema, Section, Site,
};

pub(crate) mod apply;
pub(crate) mod data;
mod diff;
mod ops;
#[cfg(test)]
pub(crate) mod tests;

pub(crate) use data::DataStep;
pub use data::{Content, DataResult, DataTx};
pub(crate) use diff::diff;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub(crate) enum Op {
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
	/// A new kind or new settings. Values start over with the default if
	/// the kind changed (a group of another component too),
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
pub(crate) enum Rename {
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
	pub(crate) fn parse(s: &str) -> Result<Self, String> {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct Migration {
	#[serde(skip)]
	pub name: String,
	/// A data migration: its step runs after the ops.
	#[serde(default, skip_serializing_if = "std::ops::Not::not")]
	pub data: bool,
	pub ops: Vec<Op>,
	/// The schema after this migration.
	pub schema: Schema,
	/// The registered step of a data migration.
	#[serde(skip)]
	pub step: Option<DataStep>,
}

/// The version of the migration files' format, written first into each
/// file. Bump it when the shape of `Migration` (also of `Op` and the
/// schema it holds) changes in a way old files don't parse, and read the
/// older ones still. A file without one is format 1.
pub const FORMAT: u32 = 1;

/// The format of a file, read before the rest: a file of a newer zipp
/// gets a clear error instead of one about a field.
#[derive(Deserialize)]
struct Head {
	#[serde(default = "first_format")]
	format: u32,
}

fn first_format() -> u32 {
	1
}

#[derive(Serialize)]
struct File<'a> {
	format: u32,
	#[serde(flatten)]
	migration: &'a Migration,
}

impl Migration {
	/// The json of its file.
	pub fn to_file(&self) -> String {
		let file = File {
			format: FORMAT,
			migration: self,
		};
		serde_json::to_string_pretty(&file).unwrap() + "\n"
	}

	/// Reads a file, `name` is the file's name without `.json`.
	pub fn from_file(name: &str, data: &[u8]) -> Result<Self, String> {
		let head: Head = serde_json::from_slice(data)
			.map_err(|e| format!("migration {name}: {e}"))?;
		if head.format > FORMAT {
			return Err(format!(
				"migration {name} has format {}, this zipp reads up to \
				{FORMAT}: update zipp",
				head.format
			));
		}

		let mut m: Migration = serde_json::from_slice(data)
			.map_err(|e| format!("migration {name}: {e}"))?;
		m.name = name.into();
		Ok(m)
	}
}

/// The migration files of a site, see [`migrations!`](crate::migrations).
pub struct Migrations {
	/// `migrations/` of the site's crate, where new ones are written
	pub dir: PathBuf,
	files: Vec<(String, Vec<u8>)>,
	/// The steps of the data migrations by name.
	steps: BTreeMap<String, DataStep>,
}

impl Migrations {
	#[doc(hidden)]
	pub fn new<E: rust_embed::Embed>(dir: &str) -> Self {
		let files = E::iter()
			.filter_map(|name| {
				Some((name.to_string(), E::get(&name)?.data.into()))
			})
			.collect();
		Self {
			dir: dir.into(),
			files,
			steps: BTreeMap::new(),
		}
	}

	/// Registers the step of a data migration, see
	/// [`Cms::data_migration`](crate::Cms::data_migration).
	pub(crate) fn data(
		mut self,
		name: &str,
		step: impl Fn(&DataTx) -> DataResult + Send + Sync + 'static,
	) -> Self {
		self.steps.insert(name.into(), DataStep(Arc::new(step)));
		self
	}

	/// The migrations sorted by name, data migrations with their step.
	/// Fails when a step and the files don't match.
	pub(crate) fn load(&self) -> Result<Vec<Migration>, String> {
		let mut migrations = self.read()?;
		let wrong = self.unmatched(&migrations);
		if !wrong.is_empty() {
			return Err(wrong.join("\n"));
		}
		for m in &mut migrations {
			m.step = self.steps.get(&m.name).cloned();
		}
		Ok(migrations)
	}

	/// The files sorted by name, without the steps.
	pub(crate) fn read(&self) -> Result<Vec<Migration>, String> {
		let mut migrations = vec![];
		for (file, data) in &self.files {
			let Some(name) = file.strip_suffix(".json") else {
				continue;
			};
			migrations.push(Migration::from_file(name, data)?);
		}

		migrations.sort_by(|a, b| a.name.cmp(&b.name));
		Ok(migrations)
	}

	/// Data migrations without a step and steps without their file. A
	/// step stays registered as long as its file exists.
	pub(crate) fn unmatched(&self, migrations: &[Migration]) -> Vec<String> {
		let mut wrong = vec![];
		for m in migrations.iter().filter(|m| m.data) {
			if !self.steps.contains_key(&m.name) {
				wrong.push(format!(
					"the data migration {} has no step, register it with \
					Cms::data_migration(\"{}\", ...)",
					m.name, m.name
				));
			}
		}
		for name in self.steps.keys() {
			if !migrations.iter().any(|m| m.data && &m.name == name) {
				wrong.push(format!(
					"Cms::data_migration(\"{name}\", ...) has no data \
					migration file, see makemigration --data"
				));
			}
		}
		wrong
	}
}

/// What changed in the code since the last migration, nothing when the
/// migrations are up to date.
pub(crate) fn unmigrated(
	migrations: &[Migration],
	schema: &Schema,
) -> Result<Vec<Op>, String> {
	let last = migrations
		.last()
		.map(|m| m.schema.clone())
		.unwrap_or_default();
	diff(&last, schema, &[])
}

/// `0003` or `0003_name` after two existing migrations.
pub(crate) fn next_name(existing: &[Migration], name: Option<&str>) -> String {
	let n = format!("{:04}", existing.len() + 1);
	match name {
		Some(name) => format!("{n}_{name}"),
		None => n,
	}
}
