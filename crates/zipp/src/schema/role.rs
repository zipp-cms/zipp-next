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
#[non_exhaustive]
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
#[non_exhaustive]
pub struct Role {
	pub handle: String,
	pub name: String,
	/// section handle to access, sections without one are hidden
	pub sections: BTreeMap<String, Access>,
	/// section handle to access to the entries the user made (their
	/// author), on top of `sections`; the entries of others show only
	/// as far as `sections` says
	#[serde(default)]
	pub own: BTreeMap<String, Access>,
	/// the images and files, `None` hides them
	pub assets: Option<Access>,
	/// the AI chat; it can do what the role can
	pub ai: bool,
	/// the content model read only under Setup, where the role also asks
	/// admins for changes
	#[serde(default = "yes")]
	pub setup: bool,
	/// whether its users may log into the studio; without, they only log
	/// in on the website (members), see `auth::SiteUser`
	#[serde(default = "yes")]
	pub studio: bool,
}

fn yes() -> bool {
	true
}

impl Role {
	pub fn new(handle: &str, name: &str) -> Self {
		Self {
			handle: handle.into(),
			name: name.into(),
			sections: BTreeMap::new(),
			own: BTreeMap::new(),
			assets: None,
			ai: false,
			setup: true,
			studio: true,
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

	/// Access to the entries of a section the user made, like Craft's
	/// permissions without "other authors' entries": a member edits their
	/// own recipes with `.own("recipes", Access::Delete)` and doesn't see
	/// the others in the studio; `.section("recipes", Access::View)`
	/// shows those too. New entries are theirs, so `Edit` may create.
	pub fn own(mut self, handle: &str, access: Access) -> Self {
		self.own.insert(handle.into(), access);
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

	/// Hides Setup, like Craft's settings for an editor who only writes.
	pub fn without_setup(mut self) -> Self {
		self.setup = false;
		self
	}

	/// Users of the role can't log into the studio, only on the website,
	/// like Craft's "Access the control panel" unchecked: members who
	/// edit their own content in the frontend. Admins still manage them
	/// on the Users page.
	pub fn no_studio(mut self) -> Self {
		self.studio = false;
		self.setup = false;
		self
	}
}
