//! The content model of a site, written as code in the site's cms crate.
//!
//! ```
//! use zipp::{Field, Schema, Section};
//!
//! let schema = Schema::new()
//!     .site("de", "Deutsch", "de-CH")
//!     .section(Section::structure("pages", ["page"]).uri("{parent.uri}/{slug}"))
//!     .entry("page", "Page", [
//!         Field::text("intro").multiline(),
//!         Field::blocks("body", ["button"]),
//!     ])
//!     .block("button", "Button", [
//!         Field::text("label").required(),
//!         Field::relation("link", ["page"]).max(1),
//!     ]);
//! assert!(schema.check().is_ok());
//! ```

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::field_type::{FieldType, FieldTypes, GraphqlType};
pub use role::{Access, Role};

mod builders;
mod check;
mod field;
pub mod field_type;
pub mod role;
#[cfg(test)]
pub(crate) mod tests;

pub use field::Choice;
pub use field::Field;
pub use field::FieldKind;
pub use field::LinkKind;
use field::*;

/// The single of the texts of the frontend, see [`Schema::translations`].
pub const TRANSLATIONS: &str = "translations";

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Schema {
	/// The first one is the primary site.
	#[serde(default)]
	pub sites: Vec<Site>,
	#[serde(default)]
	pub sections: Vec<Section>,
	pub components: Vec<Component>,
	/// Named image transforms, only these are public.
	#[serde(default)]
	pub transforms: Vec<Transform>,
	/// Groups of sections in the top bar of the studio, empty puts all
	/// sections in one group. Only looks, like labels.
	#[serde(default)]
	pub nav: Vec<NavGroup>,
	/// Pages of markdown from the code, the studio shows them read only.
	/// Not part of the content model, so not in migrations.
	#[serde(skip)]
	pub docs: Vec<DocPage>,
	/// Field types of other crates, see [`FieldType`].
	#[serde(skip)]
	pub field_types: FieldTypes,
	/// Roles of studio users, not part of the content model.
	#[serde(skip)]
	pub roles: Vec<Role>,
}

/// A markdown file of the project shown in the studio, like a readme.
#[derive(Debug, Clone, PartialEq)]
pub struct DocPage {
	pub handle: String,
	pub name: String,
	pub markdown: String,
}

/// A group of sections in the studio, like Content or Settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavGroup {
	pub name: String,
	pub sections: Vec<String>,
}

/// An image size, generated on its first request and cached. Without a
/// height the image keeps its ratio, `crop` fills both dimensions around
/// the focal point. Images are never made bigger.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
	pub handle: String,
	pub width: Option<u32>,
	pub height: Option<u32>,
	pub crop: bool,
}

/// A site has its own version of every entry: title, slug, uri, published
/// and the translatable fields. Usually one per language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Site {
	pub handle: String,
	pub name: String,
	pub language: String,
}

/// Where entries live, like in Craft: a single has exactly one entry, a
/// channel a list and a structure a tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Section {
	pub handle: String,
	pub name: String,
	pub kind: SectionKind,
	/// The entry types allowed in this section.
	pub types: Vec<String>,
	/// `events/{slug}`, in a structure also `{parent.uri}/{slug}`, the
	/// uri itself for a single (`""` is the home page). Without it the
	/// entries have no url.
	pub uri: Option<String>,
	/// Another uri for one entry type, `""` makes the entry of type `home`
	/// of a pages structure the home page.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub type_uris: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SectionKind {
	Single,
	Channel,
	Structure,
}

/// A set of fields. Entry components are the types of entries, the others
/// are blocks that live inside a blocks field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Component {
	pub handle: String,
	pub name: String,
	pub entry: bool,
	pub fields: Vec<Field>,
}

impl Schema {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn site(mut self, handle: &str, name: &str, language: &str) -> Self {
		self.sites.push(Site {
			handle: handle.into(),
			name: name.into(),
			language: language.into(),
		});
		self
	}

	pub fn section(mut self, section: Section) -> Self {
		self.sections.push(section);
		self
	}

	/// A group of sections in the top bar of the studio. Once there is one,
	/// every section has to be in exactly one group.
	pub fn nav<'a>(
		mut self,
		name: &str,
		sections: impl IntoIterator<Item = &'a str>,
	) -> Self {
		self.nav.push(NavGroup {
			name: name.into(),
			sections: sections.into_iter().map(Into::into).collect(),
		});
		self
	}

	/// Registers a field type of another crate, fields use it with
	/// [`Field::custom`].
	pub fn field_type(mut self, ty: impl FieldType) -> Self {
		self.field_types.0.push(std::sync::Arc::new(ty));
		self
	}

	/// A page of markdown in the studio, read only, e.g.
	/// `.doc("readme", "Readme", include_str!("../readme.md"))`. Put its
	/// handle into a nav group like a section.
	pub fn doc(mut self, handle: &str, name: &str, markdown: &str) -> Self {
		self.docs.push(DocPage {
			handle: handle.into(),
			name: name.into(),
			markdown: markdown.into(),
		});
		self
	}

	/// Texts of the frontend in a namespace, the key and its text in the
	/// primary site; editors translate them. They are a single
	/// `translations` with a texts field per namespace, so they have
	/// drafts like any entry; put `translations` into a nav group.
	pub fn translations<'a>(
		mut self,
		namespace: &str,
		keys: impl IntoIterator<Item = (&'a str, &'a str)>,
	) -> Self {
		if self.get(TRANSLATIONS).is_none() {
			self.sections.push(
				Section::single(TRANSLATIONS, TRANSLATIONS)
					.name("Translations"),
			);
			self = self.component(TRANSLATIONS, "Translations", true, []);
		}
		let field = Field::texts(namespace, keys);
		self.get_mut(TRANSLATIONS).unwrap().fields.push(field);
		self
	}

	/// A role of studio users, see [`Role`].
	pub fn role(mut self, role: Role) -> Self {
		self.roles.push(role);
		self
	}

	pub fn get_role(&self, handle: &str) -> Option<&Role> {
		self.roles.iter().find(|r| r.handle == handle)
	}

	pub fn transform(mut self, transform: Transform) -> Self {
		self.transforms.push(transform);
		self
	}

	pub fn get_transform(&self, handle: &str) -> Option<&Transform> {
		self.transforms.iter().find(|t| t.handle == handle)
	}

	pub fn entry(
		self,
		handle: &str,
		name: &str,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.component(handle, name, true, fields)
	}

	pub fn block(
		self,
		handle: &str,
		name: &str,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.component(handle, name, false, fields)
	}

	fn component(
		mut self,
		handle: &str,
		name: &str,
		entry: bool,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.components.push(Component {
			handle: handle.into(),
			name: name.into(),
			entry,
			fields: fields.into_iter().collect(),
		});
		self
	}

	pub fn get(&self, handle: &str) -> Option<&Component> {
		self.components.iter().find(|c| c.handle == handle)
	}

	pub fn get_mut(&mut self, handle: &str) -> Option<&mut Component> {
		self.components.iter_mut().find(|c| c.handle == handle)
	}

	pub fn entry_type(&self, handle: &str) -> Option<&Component> {
		self.get(handle).filter(|c| c.entry)
	}

	pub fn get_section(&self, handle: &str) -> Option<&Section> {
		self.sections.iter().find(|s| s.handle == handle)
	}

	pub fn get_site(&self, handle: &str) -> Option<&Site> {
		self.sites.iter().find(|s| s.handle == handle)
	}

	pub fn primary_site(&self) -> &Site {
		&self.sites[0]
	}
}
