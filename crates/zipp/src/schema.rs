//! The content model of a site, written as code in the site's cms crate.
//!
//! ```
//! use zipp::{Field, Schema, Section};
//!
//! let schema = Schema::new()
//!     .site("de", "Deutsch", "de-CH")
//!     .section(Section::structure("pages", ["page"]).uri("{parent.uri}/{slug}"))
//!     .entry("page", "Page", [
//!         Field::text("intro").multiline(),
//!         Field::blocks("body", ["button"]),
//!     ])
//!     .block("button", "Button", [
//!         Field::text("label").required(),
//!         Field::relation("link", ["page"]).max(1),
//!     ]);
//! assert!(schema.check().is_ok());
//! ```

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Schema {
	/// The first one is the primary site.
	#[serde(default)]
	pub sites: Vec<Site>,
	#[serde(default)]
	pub sections: Vec<Section>,
	pub components: Vec<Component>,
}

/// A site has its own version of every entry: title, slug, uri, published
/// and the translatable fields. Usually one per language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Site {
	pub handle: String,
	pub name: String,
	pub language: String,
}

/// Where entries live, like in Craft: a single has exactly one entry, a
/// channel a list and a structure a tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
	pub handle: String,
	pub name: String,
	pub kind: SectionKind,
	/// The entry types allowed in this section.
	pub types: Vec<String>,
	/// `events/{slug}`, in a structure also `{parent.uri}/{slug}`, the
	/// uri itself for a single (`""` is the home page). Without it the
	/// entries have no url.
	pub uri: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SectionKind {
	Single,
	Channel,
	Structure,
}

/// A set of fields. Entry components are the types of entries, the others
/// are blocks that live inside a blocks field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Component {
	pub handle: String,
	pub name: String,
	pub entry: bool,
	pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
	pub handle: String,
	pub label: String,
	#[serde(default)]
	pub required: bool,
	/// Indexed fields are copied to `entry_values`, so entries can be
	/// filtered and sorted by them.
	#[serde(default)]
	pub indexed: bool,
	/// Translatable fields have a value per site, the others are the same
	/// in all sites.
	#[serde(default)]
	pub translatable: bool,
	#[serde(flatten)]
	pub kind: FieldKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum FieldKind {
	Text {
		max: Option<u32>,
		multiline: bool,
	},
	Number {
		min: Option<f64>,
		max: Option<f64>,
	},
	Boolean,
	/// Ids of entries of the given types.
	Relation {
		types: Vec<String>,
		max: Option<u32>,
	},
	/// A list of blocks of the given types.
	Blocks {
		types: Vec<String>,
		min: Option<u32>,
		max: Option<u32>,
	},
}

impl Schema {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn site(mut self, handle: &str, name: &str, language: &str) -> Self {
		self.sites.push(Site {
			handle: handle.into(),
			name: name.into(),
			language: language.into(),
		});
		self
	}

	pub fn section(mut self, section: Section) -> Self {
		self.sections.push(section);
		self
	}

	pub fn entry(
		self,
		handle: &str,
		name: &str,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.component(handle, name, true, fields)
	}

	pub fn block(
		self,
		handle: &str,
		name: &str,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.component(handle, name, false, fields)
	}

	fn component(
		mut self,
		handle: &str,
		name: &str,
		entry: bool,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.components.push(Component {
			handle: handle.into(),
			name: name.into(),
			entry,
			fields: fields.into_iter().collect(),
		});
		self
	}

	pub fn get(&self, handle: &str) -> Option<&Component> {
		self.components.iter().find(|c| c.handle == handle)
	}

	pub fn get_mut(&mut self, handle: &str) -> Option<&mut Component> {
		self.components.iter_mut().find(|c| c.handle == handle)
	}

	pub fn entry_type(&self, handle: &str) -> Option<&Component> {
		self.get(handle).filter(|c| c.entry)
	}

	pub fn get_section(&self, handle: &str) -> Option<&Section> {
		self.sections.iter().find(|s| s.handle == handle)
	}

	pub fn get_site(&self, handle: &str) -> Option<&Site> {
		self.sites.iter().find(|s| s.handle == handle)
	}

	pub fn primary_site(&self) -> &Site {
		&self.sites[0]
	}

	/// Finds mistakes a compiler can't: unknown or duplicate handles and
	/// settings that don't fit the field.
	pub fn check(&self) -> Result<(), String> {
		if self.sites.is_empty() {
			return Err("a schema needs at least one site".into());
		}
		for (i, site) in self.sites.iter().enumerate() {
			check_handle(&site.handle)?;
			if self.sites[..i].iter().any(|s| s.handle == site.handle) {
				return Err(format!("site {} exists twice", site.handle));
			}
		}

		for (i, section) in self.sections.iter().enumerate() {
			check_handle(&section.handle)?;
			if self.sections[..i]
				.iter()
				.any(|s| s.handle == section.handle)
			{
				return Err(format!("section {} exists twice", section.handle));
			}
			self.check_section(section)
				.map_err(|e| format!("section {}: {e}", section.handle))?;
		}

		for (i, comp) in self.components.iter().enumerate() {
			check_handle(&comp.handle)?;
			if self.components[..i].iter().any(|c| c.handle == comp.handle) {
				return Err(format!("component {} exists twice", comp.handle));
			}

			for (j, field) in comp.fields.iter().enumerate() {
				let at = format!("{}.{}", comp.handle, field.handle);
				check_handle(&field.handle)
					.map_err(|e| format!("{at}: {e}"))?;
				if comp.fields[..j].iter().any(|f| f.handle == field.handle) {
					return Err(format!("field {at} exists twice"));
				}
				self.check_field(comp, field)
					.map_err(|e| format!("{at}: {e}"))?;
			}
		}

		for comp in self.components.iter().filter(|c| c.entry) {
			if !self.sections.iter().any(|s| s.types.contains(&comp.handle)) {
				return Err(format!(
					"entry type {} is in no section",
					comp.handle
				));
			}
		}

		Ok(())
	}

	fn check_section(&self, section: &Section) -> Result<(), String> {
		if section.types.is_empty() {
			return Err("needs at least one entry type".into());
		}
		for t in &section.types {
			if self.entry_type(t).is_none() {
				return Err(format!("{t} is not an entry type"));
			}
		}

		let uri = section.uri.as_deref().unwrap_or_default();
		let parts = uri.replace("{slug}", "").replace("{parent.uri}", "");
		match section.kind {
			SectionKind::Single if section.types.len() > 1 => {
				Err("a single has one entry type".into())
			}
			SectionKind::Single if uri.contains('{') => {
				Err("the uri of a single can't have placeholders".into())
			}
			SectionKind::Channel if uri.contains("{parent.uri}") => {
				Err("only structures have {parent.uri}".into())
			}
			SectionKind::Channel | SectionKind::Structure
				if section.uri.is_some() && !uri.contains("{slug}") =>
			{
				Err("the uri needs {slug}".into())
			}
			_ if parts.contains(['{', '}']) => {
				Err("the uri knows only {slug} and {parent.uri}".into())
			}
			_ => Ok(()),
		}
	}

	fn check_field(
		&self,
		comp: &Component,
		field: &Field,
	) -> Result<(), String> {
		let (types, entry) = match &field.kind {
			FieldKind::Relation { types, .. } => (types, true),
			FieldKind::Blocks { types, .. } => (types, false),
			_ => (&vec![], false),
		};
		if matches!(
			field.kind,
			FieldKind::Relation { .. } | FieldKind::Blocks { .. }
		) && types.is_empty()
		{
			return Err("needs at least one type".into());
		}
		for t in types {
			match self.get(t) {
				Some(c) if c.entry == entry => {}
				Some(_) if entry => return Err(format!("{t} is not an entry")),
				Some(_) => return Err(format!("{t} is not a block")),
				None => return Err(format!("unknown component {t}")),
			}
		}

		if field.translatable && !comp.entry {
			return Err("only fields of entries can be translatable, blocks \
				follow their field"
				.into());
		}

		if field.indexed {
			if !comp.entry {
				return Err("only fields of entries can be indexed".into());
			}
			if !matches!(
				field.kind,
				FieldKind::Text { .. }
					| FieldKind::Number { .. }
					| FieldKind::Boolean
			) {
				return Err(
					"only text, number and boolean can be indexed".into()
				);
			}
		}

		Ok(())
	}
}

fn check_handle(handle: &str) -> Result<(), String> {
	let mut chars = handle.chars();
	let ok = chars.next().is_some_and(|c| c.is_ascii_lowercase())
		&& chars.all(|c| c.is_ascii_alphanumeric());
	if ok {
		Ok(())
	} else {
		Err(format!("handle {handle:?} must be camelCase"))
	}
}

impl Field {
	fn new(handle: &str, kind: FieldKind) -> Self {
		Self {
			handle: handle.into(),
			label: humanize(handle),
			required: false,
			indexed: false,
			translatable: false,
			kind,
		}
	}

	pub fn text(handle: &str) -> Self {
		Self::new(
			handle,
			FieldKind::Text {
				max: None,
				multiline: false,
			},
		)
	}

	pub fn number(handle: &str) -> Self {
		Self::new(
			handle,
			FieldKind::Number {
				min: None,
				max: None,
			},
		)
	}

	pub fn boolean(handle: &str) -> Self {
		Self::new(handle, FieldKind::Boolean)
	}

	pub fn relation<'a>(
		handle: &str,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self::new(
			handle,
			FieldKind::Relation {
				types: types.into_iter().map(Into::into).collect(),
				max: None,
			},
		)
	}

	pub fn blocks<'a>(
		handle: &str,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self::new(
			handle,
			FieldKind::Blocks {
				types: types.into_iter().map(Into::into).collect(),
				min: None,
				max: None,
			},
		)
	}

	pub fn label(mut self, label: &str) -> Self {
		self.label = label.into();
		self
	}

	pub fn required(mut self) -> Self {
		self.required = true;
		self
	}

	pub fn indexed(mut self) -> Self {
		self.indexed = true;
		self
	}

	pub fn translatable(mut self) -> Self {
		self.translatable = true;
		self
	}

	pub fn multiline(mut self) -> Self {
		match &mut self.kind {
			FieldKind::Text { multiline, .. } => *multiline = true,
			_ => panic!("{}: multiline is only for text", self.handle),
		}
		self
	}

	/// Number range.
	pub fn range(mut self, from: f64, to: f64) -> Self {
		match &mut self.kind {
			FieldKind::Number { min, max } => {
				*min = Some(from);
				*max = Some(to);
			}
			_ => panic!("{}: range is only for numbers", self.handle),
		}
		self
	}

	/// Least number of blocks.
	pub fn min(mut self, n: u32) -> Self {
		match &mut self.kind {
			FieldKind::Blocks { min, .. } => *min = Some(n),
			_ => panic!("{}: min is only for blocks", self.handle),
		}
		self
	}

	/// Most characters of a text, entries of a relation or blocks.
	pub fn max(mut self, n: u32) -> Self {
		match &mut self.kind {
			FieldKind::Text { max, .. }
			| FieldKind::Relation { max, .. }
			| FieldKind::Blocks { max, .. } => *max = Some(n),
			_ => panic!("{}: max is not for numbers, use range", self.handle),
		}
		self
	}
}

impl Section {
	fn new<'a>(
		handle: &str,
		kind: SectionKind,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self {
			handle: handle.into(),
			name: humanize(handle),
			kind,
			types: types.into_iter().map(Into::into).collect(),
			uri: None,
		}
	}

	/// A section with exactly one entry, it is created on start.
	pub fn single(handle: &str, ty: &str) -> Self {
		Self::new(handle, SectionKind::Single, [ty])
	}

	pub fn channel<'a>(
		handle: &str,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self::new(handle, SectionKind::Channel, types)
	}

	pub fn structure<'a>(
		handle: &str,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self::new(handle, SectionKind::Structure, types)
	}

	pub fn name(mut self, name: &str) -> Self {
		self.name = name.into();
		self
	}

	pub fn uri(mut self, uri: &str) -> Self {
		self.uri = Some(uri.into());
		self
	}

	/// The uri of an entry, `None` if the section has no urls.
	pub fn render_uri(
		&self,
		slug: &str,
		parent: Option<&str>,
	) -> Option<String> {
		let format = self.uri.as_ref()?;
		let uri = match parent {
			Some(p) => format.replace("{parent.uri}", p),
			None => format
				.replace("{parent.uri}/", "")
				.replace("{parent.uri}", ""),
		};
		Some(uri.replace("{slug}", slug).trim_matches('/').into())
	}
}

/// `eventDate` becomes `Event date`.
fn humanize(handle: &str) -> String {
	let mut s = String::new();
	for (i, c) in handle.chars().enumerate() {
		if i == 0 {
			s.push(c.to_ascii_uppercase());
		} else if c.is_ascii_uppercase() {
			s.push(' ');
			s.push(c.to_ascii_lowercase());
		} else {
			s.push(c);
		}
	}
	s
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;

	pub fn site() -> Schema {
		Schema::new()
			.site("de", "Deutsch", "de-CH")
			.site("en", "English", "en")
			.section(Section::single("home", "page").uri(""))
			.section(
				Section::structure("pages", ["page"])
					.uri("{parent.uri}/{slug}"),
			)
			.section(Section::channel("events", ["event"]).uri("events/{slug}"))
			.entry(
				"page",
				"Page",
				[
					Field::text("intro").multiline().translatable(),
					Field::blocks("body", ["text", "button"]).translatable(),
				],
			)
			.entry(
				"event",
				"Event",
				[
					Field::text("city").indexed(),
					Field::number("price").indexed(),
					Field::relation("page", ["page"]).max(1),
				],
			)
			.block("text", "Text", [Field::text("content").required()])
			.block(
				"button",
				"Button",
				[
					Field::text("label").required().max(20),
					Field::relation("link", ["page"]).max(1),
				],
			)
	}

	#[test]
	fn site_is_valid() {
		site().check().unwrap();
	}

	#[test]
	fn humanizes_handles() {
		assert_eq!(humanize("eventDate"), "Event date");
		assert_eq!(Field::text("intro").label, "Intro");
	}

	#[test]
	fn finds_mistakes() {
		let s = site().block("text", "Again", []);
		assert!(s.check().unwrap_err().contains("twice"));

		let s = site().block("x", "X", [Field::blocks("b", ["page"])]);
		assert_eq!(s.check().unwrap_err(), "x.b: page is not a block");

		let s = site().block("y", "Y", [Field::text("t").indexed()]);
		assert!(s.check().unwrap_err().contains("only fields of entries"));

		let s = site().entry("Bad", "Bad", []);
		assert!(s.check().unwrap_err().contains("camelCase"));

		let s = site().entry("loose", "Loose", []);
		assert_eq!(s.check().unwrap_err(), "entry type loose is in no section");

		let s = site().section(Section::channel("news", ["page"]).uri("news"));
		assert_eq!(
			s.check().unwrap_err(),
			"section news: the uri needs {slug}"
		);

		let s =
			site().section(Section::channel("x", ["page"]).uri("{id}/{slug}"));
		assert!(s.check().unwrap_err().contains("knows only"));
	}

	#[test]
	fn renders_uris() {
		let s = site();
		let pages = s.get_section("pages").unwrap();
		assert_eq!(pages.render_uri("team", None).unwrap(), "team");
		assert_eq!(
			pages.render_uri("team", Some("about")).unwrap(),
			"about/team"
		);
		let home = s.get_section("home").unwrap();
		assert_eq!(home.render_uri("home", None).unwrap(), "");
	}

	#[test]
	fn serializes_flat() {
		let f = Field::relation("link", ["page"]).max(1);
		let v = serde_json::to_value(&f).unwrap();
		assert_eq!(v["kind"], "relation");
		assert_eq!(v["types"][0], "page");
		let back: Field = serde_json::from_value(v).unwrap();
		assert_eq!(back, f);
	}
}
