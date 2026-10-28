//! The content model of a site, written as code in the site's cms crate.
//!
//! ```
//! use zipp::{Field, Schema};
//!
//! let schema = Schema::new()
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
	pub components: Vec<Component>,
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

	/// Finds mistakes a compiler can't: unknown or duplicate handles and
	/// settings that don't fit the field.
	pub fn check(&self) -> Result<(), String> {
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

		Ok(())
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
			.entry(
				"page",
				"Page",
				[
					Field::text("intro").multiline(),
					Field::blocks("body", ["text", "button"]),
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
