#[derive(Debug, Clone)]
pub struct Component {
	pub name: String,
	pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Field {
	pub name: String,
	pub kind: FieldKind,
	pub related: Option<String>,
	pub primary: bool,
	pub index: bool,
}

#[derive(Debug, Clone)]
pub enum FieldKind {
	Id,
	ComponentId,
	Component { name: String },
	Boolean,
	Int8,
	Int16,
	Int32,
	Int64,
	Float32,
	Float64,
	Text,
	Json,
	DateTime,
}
