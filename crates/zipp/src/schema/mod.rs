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

use crate::schema::field_type::FieldTypes;
pub use field_type::{
	FieldRelation, FieldType, GraphqlType, Scalar, StudioModule, Target,
};
pub use role::{Access, Role};

mod builders;
mod check;
mod column;
mod condition;
mod field;
mod field_type;
mod role;
#[cfg(test)]
mod tests;

pub use column::{Column, ColumnKind, Style};
pub use condition::Condition;
pub use field::Field;
pub use field::FieldKind;
pub(crate) use field::Holds;
pub use field::LinkKind;
pub use field::humanize;
pub use field::{ButtonOption, Choice};

/// The single of the texts of the frontend, see [`Schema::translations`].
pub(crate) const TRANSLATIONS: &str = "translations";

/// The component of the fields of assets, see [`Schema::asset_fields`].
pub(crate) const ASSET: &str = "asset";

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
	pub(crate) docs: Vec<DocPage>,
	/// Field types of other crates, see [`FieldType`].
	#[serde(skip)]
	pub(crate) field_types: FieldTypes,
	/// Roles of studio users, not part of the content model.
	#[serde(skip)]
	pub(crate) roles: Vec<Role>,
	/// Plugins, see [`Schema::plugin`].
	#[serde(skip)]
	pub(crate) plugins: crate::plugin::Plugins,
	/// Sites not on the website yet, see [`Schema::offline`].
	#[serde(skip)]
	pub(crate) offline: Vec<String>,
	/// Sizes images can have without a named transform, width and
	/// height, see [`Schema::image_widths`] and [`Schema::image_sizes`].
	#[serde(skip)]
	pub(crate) image_sizes: Vec<(Option<u32>, Option<u32>)>,
	/// The time zone of formatted dates, see [`Schema::timezone`].
	#[serde(skip)]
	pub(crate) timezone: Option<String>,
}

/// A markdown file of the project shown in the studio, like a readme.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct DocPage {
	pub handle: String,
	pub name: String,
	pub markdown: String,
}

/// A group of sections in the studio, like Content or Settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct NavGroup {
	pub name: String,
	pub sections: Vec<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub icon: Option<Icon>,
}

/// An icon in the studio: a Lucide icon by its name (`calendar`, see
/// lucide.dev/icons), or an svg of its own, drawn with `currentColor` to
/// take the color of the text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Icon {
	Lucide(String),
	Svg(String),
}

impl Icon {
	/// An svg of its own, e.g. `Icon::svg(include_str!("logo.svg"))`.
	pub fn svg(svg: &str) -> Self {
		Self::Svg(svg.into())
	}
}

/// A Lucide icon by its name.
impl From<&str> for Icon {
	fn from(name: &str) -> Self {
		Self::Lucide(name.into())
	}
}

/// An image size, generated on its first request and cached. Without a
/// height the image keeps its ratio, `crop` fills both dimensions around
/// the focal point. Images are never made bigger.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Transform {
	pub handle: String,
	pub width: Option<u32>,
	pub height: Option<u32>,
	pub crop: bool,
	/// the type of its images, `None` keeps the one of the image
	#[serde(default)]
	pub format: Option<Format>,
}

/// A type a transform can make: its url ends in [`Format::extension`],
/// like Craft's transform format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum Format {
	Jpeg,
	Png,
	/// lossy, smaller than a jpeg of the same quality
	Webp,
}

impl Format {
	pub const ALL: [Format; 3] = [Format::Jpeg, Format::Png, Format::Webp];

	pub fn extension(self) -> &'static str {
		match self {
			Format::Jpeg => "jpg",
			Format::Png => "png",
			Format::Webp => "webp",
		}
	}

	pub fn mime(self) -> &'static str {
		match self {
			Format::Jpeg => "image/jpeg",
			Format::Png => "image/png",
			Format::Webp => "image/webp",
		}
	}

	/// By an extension or a name: `jpg`, `jpeg`, `png`, `webp`.
	pub fn parse(s: &str) -> Option<Format> {
		match s.to_ascii_lowercase().as_str() {
			"jpg" | "jpeg" => Some(Format::Jpeg),
			"png" => Some(Format::Png),
			"webp" => Some(Format::Webp),
			_ => None,
		}
	}
}

/// A site has its own version of every entry: title, slug, uri, published
/// and the translatable fields. Usually one per language.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Site {
	pub handle: String,
	pub name: String,
	pub language: String,
}

impl Site {
	pub fn new(handle: &str, name: &str, language: &str) -> Self {
		Self {
			handle: handle.into(),
			name: name.into(),
			language: language.into(),
		}
	}
}

/// Where entries live, like in Craft: a single has exactly one entry, a
/// channel a list and a structure a tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
	/// How deep a structure goes, 1 is only entries on the top, like
	/// Craft's max levels. `None` has no limit.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub max_level: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum SectionKind {
	Single,
	Channel,
	Structure,
}

/// A set of fields. Entry components are the types of entries, the others
/// are blocks that live inside a blocks field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
	/// all fields of the editor in one card, see [`Schema::entry_card`]
	#[serde(default, skip_serializing_if = "std::ops::Not::not")]
	pub card: bool,
	/// in the studio's lists and menus, only looks like the tabs
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub icon: Option<Icon>,
	/// The field of a block that names it in the studio, see
	/// [`Schema::block_title`]. Only looks.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub title_field: Option<String>,
	/// The component of an inline group ([`Field::group_of`]): the
	/// component it was made for. Setup lists it there, not as a block.
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub part_of: Option<String>,
}

/// How an indexed field is compared and sorted, see
/// [`Schema::index_kind`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum IndexKind {
	Text,
	Number,
}

/// A tab of an entry type in the editor, see [`Schema::entry_tabs`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Tab {
	pub name: String,
	pub fields: Vec<String>,
	/// shown only when it holds, see [`Schema::tab_when`]
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub when: Option<Condition>,
	/// its fields in one card, see [`Schema::tab_card`]
	#[serde(default, skip_serializing_if = "std::ops::Not::not")]
	pub card: bool,
}

impl Schema {
	pub fn is_offline(&self, site: &str) -> bool {
		self.offline.iter().any(|s| s == site)
	}

	pub(crate) fn get_role(&self, handle: &str) -> Option<&Role> {
		self.roles.iter().find(|r| r.handle == handle)
	}

	pub fn get_transform(&self, handle: &str) -> Option<&Transform> {
		self.transforms.iter().find(|t| t.handle == handle)
	}

	/// The transform of a name: a named one or one of the allowed sizes,
	/// `w800`, `h800` or `w800h450`.
	pub(crate) fn transform_named(&self, name: &str) -> Option<Transform> {
		if let Some(t) = self.get_transform(name) {
			return Some(t.clone());
		}
		let num = |s: &str| s.parse::<u32>().ok();
		let (w, h) = match name.strip_prefix('w') {
			Some(rest) => match rest.split_once('h') {
				Some((w, h)) => (Some(num(w)?), Some(num(h)?)),
				None => (Some(num(rest)?), None),
			},
			None => (None, Some(num(name.strip_prefix('h')?)?)),
		};
		self.size_transform(w, h)
	}

	/// The transform of an allowed size, like Craft's `url(width: 800,
	/// height: 450)`: both sizes crop around the focal point.
	pub(crate) fn size_transform(
		&self,
		width: Option<u32>,
		height: Option<u32>,
	) -> Option<Transform> {
		if !self.image_sizes.contains(&(width, height)) {
			return None;
		}
		let (handle, mut t) = match (width, height) {
			(Some(w), Some(h)) => {
				let t = Transform::new("").width(w).height(h).crop();
				(format!("w{w}h{h}"), t)
			}
			(Some(w), None) => (format!("w{w}"), Transform::new("").width(w)),
			(None, Some(h)) => (format!("h{h}"), Transform::new("").height(h)),
			(None, None) => return None,
		};
		t.handle = handle;
		Some(t)
	}

	/// The sizes of [`Schema::size_transform`] for an error, `width 800,
	/// height 600, 800x450`.
	pub(crate) fn allowed_sizes(&self) -> String {
		let sizes: Vec<String> = self
			.image_sizes
			.iter()
			.map(|s| match s {
				(Some(w), Some(h)) => format!("{w}x{h}"),
				(Some(w), None) => format!("width {w}"),
				(None, Some(h)) => format!("height {h}"),
				(None, None) => unreachable!(),
			})
			.collect();
		match sizes.is_empty() {
			true => "none".into(),
			false => sizes.join(", "),
		}
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

	/// A section and one of its entry types, an entry is saved as.
	pub(crate) fn section_type(
		&self,
		section: &str,
		ty: &str,
	) -> crate::error::Result<(&Section, &Component)> {
		self.get_section(section)
			.filter(|s| s.types.iter().any(|t| t == ty))
			.zip(self.entry_type(ty))
			.ok_or_else(|| {
				crate::error::Error::Request(format!(
					"no type {ty} in section {section}"
				))
			})
	}

	pub fn get_site(&self, handle: &str) -> Option<&Site> {
		self.sites.iter().find(|s| s.handle == handle)
	}

	pub fn primary_site(&self) -> &Site {
		&self.sites[0]
	}

	/// The handle of the site a request asks for, the primary site's if
	/// it asks for none.
	pub fn site_or_primary(&self, site: Option<String>) -> String {
		site.unwrap_or_else(|| self.primary_site().handle.clone())
	}

	/// The number of a site, its position from 1 like Craft's site ids;
	/// GraphQL and crelte know sites by it.
	pub fn site_id(&self, handle: &str) -> Option<i64> {
		let i = self.sites.iter().position(|s| s.handle == handle)?;
		Some(i as i64 + 1)
	}

	/// The site of a number of [`Schema::site_id`].
	pub fn site_by_id(&self, id: i64) -> Option<&Site> {
		self.sites.get(usize::try_from(id).ok()?.checked_sub(1)?)
	}

	/// The field at a path like `pageIntro.text`, through group fields.
	pub fn field_at<'a>(
		&'a self,
		comp: &'a Component,
		path: &str,
	) -> Option<&'a Field> {
		let (first, rest) = match path.split_once('.') {
			Some((f, r)) => (f, Some(r)),
			None => (path, None),
		};
		let field = comp.fields.iter().find(|f| f.handle == first)?;
		match (rest, &field.kind) {
			(None, _) => Some(field),
			(Some(rest), FieldKind::Group { component }) => {
				self.field_at(self.get(component)?, rest)
			}
			_ => None,
		}
	}

	/// How a field of this kind is indexed, `None`: it can't be. A
	/// custom type is like the scalar GraphQL shows. Every kind is named,
	/// a new one has to say.
	pub(crate) fn index_kind(&self, kind: &FieldKind) -> Option<IndexKind> {
		match kind {
			// a date sorts as text, `2026-11-14T20:00`
			FieldKind::Text { .. }
			| FieldKind::Email
			| FieldKind::Dropdown { .. }
			| FieldKind::ButtonGroup { .. }
			| FieldKind::Date { .. } => Some(IndexKind::Text),
			FieldKind::Number { .. } | FieldKind::Boolean => {
				Some(IndexKind::Number)
			}
			FieldKind::Custom { ty, .. } => {
				match self.field_types.get(ty)?.graphql() {
					GraphqlType::Scalar(Scalar::String) => {
						Some(IndexKind::Text)
					}
					GraphqlType::Scalar(_) => Some(IndexKind::Number),
					_ => None,
				}
			}
			FieldKind::Relation { .. }
			| FieldKind::Assets { .. }
			| FieldKind::Link { .. }
			| FieldKind::RichText { .. }
			| FieldKind::Group { .. }
			| FieldKind::Markdown
			| FieldKind::Checkboxes { .. }
			| FieldKind::Table { .. }
			| FieldKind::Navigation { .. }
			| FieldKind::Texts { .. }
			| FieldKind::Note { .. }
			| FieldKind::Blocks { .. } => None,
		}
	}
}

impl Section {
	/// The uri of an entry in a site, `None` if it has no page. `fields`
	/// are its values in the site, for [`Section::uri_if`].
	pub(crate) fn render_uri(
		&self,
		ty: &str,
		slug: &str,
		parent: Option<&str>,
		site: &str,
		fields: &serde_json::Value,
	) -> Option<String> {
		if let Some(f) = &self.uri_if
			&& fields[f] != true
		{
			return None;
		}
		let format = self
			.type_uris
			.get(ty)
			.or(self.site_uris.get(site))
			.or(self.uri.as_ref())?;
		let uri = match parent {
			Some(p) => format.replace("{parent.uri}", p),
			None => format
				.replace("{parent.uri}/", "")
				.replace("{parent.uri}", ""),
		};
		Some(uri.replace("{slug}", slug).trim_matches('/').into())
	}
}
