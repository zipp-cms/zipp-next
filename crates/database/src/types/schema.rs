use serde_json::Map;

/// A database schema
///
/// The name needs to be unique across the entire database
#[derive(Debug, Clone)]
pub struct Schema {
	pub name: String,
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

pub type Value = serde_json::Value;
pub type Data = Map<String, Value>;

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
	inner: Schema,
}

impl SchemaBuilder {
	fn new(name: impl Into<String>) -> Self {
		Self {
			inner: Schema {
				name: name.into(),
				fields: Vec::new(),
			},
		}
	}

	pub fn field(mut self, field: FieldBuilder) -> Self {
		self.inner.fields.push(field.build());
		self
	}

	pub fn build(self) -> Schema {
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
