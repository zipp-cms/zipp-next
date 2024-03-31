use core::fmt;

use super::{json_storage, Component};

#[async_trait::async_trait]
pub trait Persistent<T> {
	async fn load(&self) -> Option<T>;
	async fn save(&self, contents: &T);
}

pub struct ComponentStore {
	components: Vec<Component>,
	persistent: Box<dyn Persistent<Vec<Component>>>,
}

impl ComponentStore {
	pub async fn new_json_storage(file_name: &str) -> Self {
		let persistent = Box::new(json_storage::JsonStorage::new(file_name));
		let components = persistent.load().await.unwrap_or(Vec::new());

		Self {
			components,
			persistent,
		}
	}

	pub fn add(&mut self, component: Component) {
		self.components.push(component);
		// todo: maybe save to persistent storage
	}
}

impl fmt::Debug for ComponentStore {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("ComponentStore")
			.field("components", &self.components)
			.finish()
	}
}
