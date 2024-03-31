#![allow(dead_code)]

use std::collections::HashMap;

use crate::{types::component::Component, Error};

#[derive(Debug)]
pub struct ComponentRepository {
	components: HashMap<String, Component>,
}

impl ComponentRepository {
	pub fn new() -> Self {
		Self {
			components: HashMap::new(),
		}
	}

	pub fn set_component(&mut self, component: Component) -> Result<(), Error> {
		self.components.insert(component.name.clone(), component);

		Ok(())
	}

	pub fn get_component(
		&self,
		name: &str,
	) -> Result<Option<Component>, Error> {
		let component = self.components.get(name).cloned();

		Ok(component)
	}

	pub fn delete_component(&mut self, name: &str) -> Result<(), Error> {
		self.components.remove(name);

		Ok(())
	}
}
