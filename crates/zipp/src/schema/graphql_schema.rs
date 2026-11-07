//! What the token of a frontend may read in GraphQL, like Craft's GraphQL
//! schemas. They are code like roles; admins make tokens in the studio
//! and give each one a schema.

use serde::{Deserialize, Serialize};

/// A GraphQL schema: the sections and sites a token reads, and whether
/// it lists assets, reads the public profiles of users and shows drafts
/// to preview tokens.
///
/// ```
/// use zipp::GraphqlSchema;
///
/// let public = GraphqlSchema::new("public", "Website")
///     .sections(["pages", "news"])
///     .assets();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GraphqlSchema {
	pub handle: String,
	pub name: String,
	/// handles of the sections it reads, none without one
	pub sections: Vec<String>,
	/// handles of the sites it reads, `None` all
	pub sites: Option<Vec<String>>,
	/// whether it lists the assets with `assets`; the assets in the
	/// fields of the entries it reads come anyway
	pub assets: bool,
	/// whether it reads the public profiles of users
	pub users: bool,
	/// whether preview tokens show drafts
	pub previews: bool,
}

impl GraphqlSchema {
	pub fn new(handle: &str, name: &str) -> Self {
		Self {
			handle: handle.into(),
			name: name.into(),
			sections: vec![],
			sites: None,
			assets: false,
			users: false,
			previews: false,
		}
	}

	/// The sections it reads, globals are sections too.
	pub fn sections<'a>(
		mut self,
		handles: impl IntoIterator<Item = &'a str>,
	) -> Self {
		self.sections.extend(handles.into_iter().map(Into::into));
		self
	}

	/// Only these sites, without it all.
	pub fn sites<'a>(
		mut self,
		handles: impl IntoIterator<Item = &'a str>,
	) -> Self {
		let sites = self.sites.get_or_insert_default();
		sites.extend(handles.into_iter().map(Into::into));
		self
	}

	/// Lists the assets with `assets` and `assetCount`.
	pub fn assets(mut self) -> Self {
		self.assets = true;
		self
	}

	/// Reads the public profiles of users: `users`, `user` and the
	/// `author` of entries.
	pub fn users(mut self) -> Self {
		self.users = true;
		self
	}

	/// Preview tokens show drafts, for the live preview of the studio.
	pub fn previews(mut self) -> Self {
		self.previews = true;
		self
	}
}
