use std::collections::BTreeMap;

pub mod component_store;
pub mod json_storage;

#[derive(Debug)]
pub struct Component {
	pub name: String,
	pub children: BTreeMap<String, Component>,
}

impl Component {
	pub fn new(name: String) -> Self {
		Self { name }
	}
}
