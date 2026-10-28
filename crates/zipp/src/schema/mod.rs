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
pub use field::{Column, ColumnKind};

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
	/// Plugins, see [`Schema::plugin`].
	#[serde(skip)]
	pub plugins: crate::plugin::Plugins,
	/// Sites not on the website yet, see [`Schema::offline`].
	#[serde(skip)]
	pub offline: Vec<String>,
	/// Widths images can have without a named transform, see
	/// [`Schema::image_widths`].
	#[serde(skip)]
	pub image_widths: Vec<u32>,
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
	/// Another uri in one site, like `offres/{slug}` in French.
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	pub site_uris: BTreeMap<String, String>,
	/// A boolean field: only entries with it on have a page, like Craft's
	/// `{% if hasPage %}` in a uri format.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub uri_if: Option<String>,
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
	/// The tabs of the editor with the handles of their fields, like
	/// Craft's field layout; empty is one tab. Only looks, not in
	/// migrations' ops.
	#[serde(default, skip_serializing_if = "Vec::is_empty")]
	pub tabs: Vec<Tab>,
}

/// A tab of an entry type in the editor, see [`Schema::entry_tabs`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tab {
	pub name: String,
	pub fields: Vec<String>,
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

	/// A site that is not on the website, like a disabled site in Craft:
	/// frontends get none of its entries, only previews and admins do, and
	/// it is not in `crelteSites` and the sitemaps. Editors fill it in.
	pub fn offline(mut self, site: &str) -> Self {
		self.offline.push(site.into());
		self
	}

	pub fn is_offline(&self, site: &str) -> bool {
		self.offline.iter().any(|s| s == site)
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

	/// Widths images can be resized to without a named transform, like
	/// Craft's `url(width: 800)`: only these, so nobody can make the
	/// server render any size.
	pub fn image_widths(
		mut self,
		widths: impl IntoIterator<Item = u32>,
	) -> Self {
		self.image_widths.extend(widths);
		self
	}

	/// The transform of a name: a named one or `w800` of an allowed width.
	pub fn transform_named(&self, name: &str) -> Option<Transform> {
		if let Some(t) = self.get_transform(name) {
			return Some(t.clone());
		}
		let width: u32 = name.strip_prefix('w')?.parse().ok()?;
		self.width_transform(width)
	}

	/// The transform of an allowed width.
	pub fn width_transform(&self, width: u32) -> Option<Transform> {
		self.image_widths
			.contains(&width)
			.then(|| Transform::new(&format!("w{width}")).width(width))
	}

	pub fn entry(
		self,
		handle: &str,
		name: &str,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.component(handle, name, true, fields)
	}

	/// An entry type with its fields in tabs, like Craft's field layout:
	/// `[("Content", vec![...]), ("SEO", vec![...])]`.
	pub fn entry_tabs<'a>(
		mut self,
		handle: &str,
		name: &str,
		tabs: impl IntoIterator<Item = (&'a str, Vec<Field>)>,
	) -> Self {
		let mut fields = vec![];
		let mut layout = vec![];
		for (tab, tab_fields) in tabs {
			layout.push(Tab {
				name: tab.into(),
				fields: tab_fields.iter().map(|f| f.handle.clone()).collect(),
			});
			fields.extend(tab_fields);
		}
		self = self.component(handle, name, true, fields);
		self.components.last_mut().unwrap().tabs = layout;
		self
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
			tabs: vec![],
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
