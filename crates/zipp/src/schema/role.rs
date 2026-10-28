//! Roles of studio users, like Craft Pro's user groups. They are code like
//! the content model; which user has which role is in the database. A user
//! without a role is an admin: everything, also managing users.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// What a role may do in a section or with assets, each level includes
/// the ones before.
#[derive(
	Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum Access {
	/// see the entries
	View,
	/// change them as drafts and create new ones, unpublished
	Edit,
	/// publish drafts and move entries in a structure
	Publish,
	/// delete entries (assets: delete files)
	Delete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Role {
	pub handle: String,
	pub name: String,
	/// section handle to access, sections without one are hidden
	pub sections: BTreeMap<String, Access>,
	/// the images and files, `None` hides them
	pub assets: Option<Access>,
	/// the AI chat; it can do what the role can
	pub ai: bool,
}

impl Role {
	pub fn new(handle: &str, name: &str) -> Self {
		Self {
			handle: handle.into(),
			name: name.into(),
			sections: BTreeMap::new(),
			assets: None,
			ai: false,
		}
	}

	pub fn section(mut self, handle: &str, access: Access) -> Self {
		self.sections.insert(handle.into(), access);
		self
	}

	pub fn sections<'a>(
		mut self,
		handles: impl IntoIterator<Item = &'a str>,
		access: Access,
	) -> Self {
		for h in handles {
			self.sections.insert(h.into(), access);
		}
		self
	}

	pub fn assets(mut self, access: Access) -> Self {
		self.assets = Some(access);
		self
	}

	pub fn ai(mut self) -> Self {
		self.ai = true;
		self
	}
}
