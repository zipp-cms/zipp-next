use super::id::Kind;

use std::collections::BTreeMap;

use serde_json::Value;

/// A database schema
///
/// The name needs to be unique across the entire database
#[derive(Debug, Clone)]
pub struct SetSchema {
	pub name: String,
	pub fields: Vec<Field>,
}

/// A database schema
///
/// The name needs to be unique across the entire database
#[derive(Debug, Clone)]
pub struct Schema {
	pub name: String,
	pub kind: Kind,
	pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
	pub name: String,
	pub kind: FieldKind,
	pub related: Option<Related>,
	pub primary: bool,
	pub index: bool,
	pub nullable: bool,
	// should a field have a default value?
}

#[derive(Debug, Clone)]
pub struct Related {
	pub schema: String,
	pub field: String,
}

#[derive(Debug, Clone)]
pub enum FieldKind {
	Id,
	ComponentId,
	Boolean,
	Int,
	Float,
	Text,
	Json,
	DateTime,
}

#[derive(Debug, Clone)]
pub struct SchemaEntries(pub Vec<SchemaEntry>);

#[derive(Debug, Clone)]
pub struct SchemaEntry(pub BTreeMap<String, SchemaFieldValue>);

#[derive(Debug, Clone)]
pub enum SchemaFieldValue {
	Value(Value),
	Entries(SchemaEntries),
}

/// builder

impl Schema {
	pub fn builder(name: impl Into<String>) -> SchemaBuilder {
		SchemaBuilder::new(name)
	}
}

impl Field {
	pub fn builder(name: impl Into<String>, kind: FieldKind) -> FieldBuilder {
		FieldBuilder::new(name, kind)
	}
}

#[derive(Debug, Clone)]
pub struct SchemaBuilder {
	inner: SetSchema,
}

impl SchemaBuilder {
	fn new(name: impl Into<String>) -> Self {
		Self {
			inner: SetSchema {
				name: name.into(),
				fields: Vec::new(),
			},
		}
	}

	pub fn field(mut self, field: FieldBuilder) -> Self {
		self.inner.fields.push(field.build());
		self
	}

	pub fn build(self) -> SetSchema {
		self.inner
	}
}

#[derive(Debug, Clone)]
pub struct FieldBuilder {
	inner: Field,
}

impl FieldBuilder {
	fn new(name: impl Into<String>, kind: FieldKind) -> Self {
		Self {
			inner: Field {
				name: name.into(),
				kind,
				related: None,
				primary: false,
				index: false,
				nullable: false,
			},
		}
	}

	pub fn related(
		mut self,
		schema: impl Into<String>,
		field: impl Into<String>,
	) -> Self {
		self.inner.related = Some(Related {
			schema: schema.into(),
			field: field.into(),
		});
		self
	}

	pub fn primary(mut self) -> Self {
		self.inner.primary = true;
		self
	}

	pub fn index(mut self) -> Self {
		self.inner.index = true;
		self
	}

	pub fn build(self) -> Field {
		self.inner
	}
}

impl SchemaEntries {
	pub fn builder() -> SchemaEntriesBuilder {
		SchemaEntriesBuilder::new()
	}
}

#[derive(Debug, Clone)]
pub struct SchemaEntriesBuilder {
	inner: SchemaEntries,
}

impl SchemaEntriesBuilder {
	fn new() -> Self {
		Self {
			inner: SchemaEntries(Vec::new()),
		}
	}

	pub fn entry(mut self, entry: SchemaEntryBuilder) -> Self {
		self.inner.0.push(entry.build());
		self
	}

	pub fn entries(
		mut self,
		entries: impl Iterator<Item = SchemaEntry>,
	) -> Self {
		self.inner.0.extend(entries);
		self
	}

	pub fn build(self) -> SchemaEntries {
		self.inner
	}
}

impl SchemaEntry {
	pub fn builder() -> SchemaEntryBuilder {
		SchemaEntryBuilder::new()
	}
}

pub struct SchemaEntryBuilder {
	inner: SchemaEntry,
}

impl SchemaEntryBuilder {
	pub fn new() -> Self {
		Self {
			inner: SchemaEntry(BTreeMap::new()),
		}
	}

	// this makes no sense
	// an entry does not only contain a single field
	pub fn field(
		mut self,
		name: impl Into<String>,
		value: impl Into<Value>,
	) -> Self {
		self.inner
			.0
			.insert(name.into(), SchemaFieldValue::Value(value.into()));

		self
	}

	pub fn nested(
		mut self,
		name: impl Into<String>,
		entries: SchemaEntriesBuilder,
	) -> Self {
		self.inner
			.0
			.insert(name.into(), SchemaFieldValue::Entries(entries.build()));

		self
	}

	pub fn build(self) -> SchemaEntry {
		self.inner
	}
}

impl From<SchemaEntries> for Value {
	fn from(entries: SchemaEntries) -> Self {
		Self::Array(entries.0.into_iter().map(Into::into).collect())
	}
}

impl From<SchemaEntry> for Value {
	fn from(entry: SchemaEntry) -> Self {
		Self::Object(entry.0.into_iter().map(|(k, v)| (k, v.into())).collect())
	}
}

impl From<SchemaFieldValue> for Value {
	fn from(value: SchemaFieldValue) -> Self {
		match value {
			SchemaFieldValue::Value(value) => value,
			SchemaFieldValue::Entries(entries) => entries.into(),
		}
	}
}
