use std::collections::HashMap;

use serde_json::Map;

#[derive(Debug, Clone)]
pub struct Schema {
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
	Boolean,
	Int,
	Float,
	Text,
	Json,
	DateTime,
}

pub type Value = serde_json::Value;
pub type Data = Map<String, Value>;
