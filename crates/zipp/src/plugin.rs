//! Plugins: crates that add a whole feature to zipp, like SEO. A plugin
//! adds to the content model (sections, blocks, field types, transforms),
//! extends GraphQL, brings routes and a module for the studio. The site
//! turns it on with one line:
//!
//! ```ignore
//! Schema::new()
//!     // ...
//!     .plugin(zipp_seo::Seo::new())
//! ```
//!
//! What a plugin adds to the content model is content like the site's
//! own: it is in migrations and in Setup.

use std::any::Any;
use std::collections::BTreeSet;
use std::fmt;
use std::sync::Arc;

use async_graphql::dynamic::{Field as GqlField, Type, TypeRef};
use axum::Router;
use serde::Serialize;
use serde_json::Value;

use crate::AppState;
use crate::schema::Schema;

/// A crate that adds a whole feature, like `zipp-seo`: to the content
/// model, GraphQL, the routes and the studio. The site adds it with
/// [`Schema::plugin`]; every method has a default, a plugin implements
/// what it needs.
pub trait Plugin: Any + Send + Sync + 'static {
	/// A handle, also the name of its studio module, like `seo`.
	fn name(&self) -> &'static str;

	/// Adds to the content model when the site adds the plugin.
	fn schema(&self, schema: Schema) -> Schema {
		schema
	}

	/// Once the schema is complete, for what depends on all of it (like
	/// a choice of the sections). [`Cms`](crate::Cms) calls it.
	fn finish(&self, schema: Schema) -> Schema {
		schema
	}

	/// Mistakes in what the site set up for the plugin.
	fn check(&self, _schema: &Schema) -> Result<(), String> {
		Ok(())
	}

	/// What the plugin adds to GraphQL.
	fn graphql(&self, _schema: &Schema) -> GraphqlExt {
		GraphqlExt::default()
	}

	/// Routes under `/api`, for the studio; `Authed` checks the session.
	fn api_routes(&self, _schema: &Schema) -> Router<AppState> {
		Router::new()
	}

	/// Routes of the website, like `/sitemap.xml`, the frontend proxies
	/// them.
	fn routes(&self, _schema: &Schema) -> Router<AppState> {
		Router::new()
	}

	/// The module of the studio, see [`PluginStudio`].
	fn studio(&self) -> Option<PluginStudio> {
		None
	}
}

/// GraphQL of a plugin: its types, fields of every entry and of the root.
/// Made with [`GraphqlExt::new`] and its methods, so it can grow.
#[derive(Default)]
#[non_exhaustive]
pub struct GraphqlExt {
	pub(crate) types: Vec<Type>,
	/// on every entry type and the `Entry` interface; an entry type with
	/// a field of the same handle keeps its field
	pub(crate) entry_fields: Vec<EntryField>,
	pub(crate) root_fields: Vec<GqlField>,
}

impl GraphqlExt {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn types(mut self, types: impl IntoIterator<Item = Type>) -> Self {
		self.types.extend(types);
		self
	}

	pub fn entry_fields(
		mut self,
		fields: impl IntoIterator<Item = EntryField>,
	) -> Self {
		self.entry_fields.extend(fields);
		self
	}

	pub fn root_fields(
		mut self,
		fields: impl IntoIterator<Item = GqlField>,
	) -> Self {
		self.root_fields.extend(fields);
		self
	}
}

/// A field of every entry, made once per entry type.
#[non_exhaustive]
pub struct EntryField {
	pub(crate) name: String,
	pub(crate) ty: TypeRef,
	pub(crate) field: Arc<dyn Fn() -> GqlField + Send + Sync>,
}

impl EntryField {
	/// `field` makes the field for each entry type.
	pub fn new(
		name: &str,
		ty: TypeRef,
		field: impl Fn() -> GqlField + Send + Sync + 'static,
	) -> Self {
		Self {
			name: name.into(),
			ty,
			field: Arc::new(field),
		}
	}
}

/// The module of a plugin in the studio, an ES module served as
/// `/_plugins/<name>.js` that brings its own framework like a field
/// type's. It may export
/// `sectionSetup(target, { schema, section, data, host })`, a card on
/// the setup page of a section; `data` is the plugin's JSON for it. A
/// plugin without a module (`default`) can still give the studio data.
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PluginStudio {
	pub(crate) js: Option<&'static str>,
	pub(crate) data: Value,
}

impl PluginStudio {
	pub fn new(js: &'static str) -> Self {
		Self {
			js: Some(js),
			data: Value::Null,
		}
	}

	/// What the module gets as `data`.
	pub fn data(mut self, data: Value) -> Self {
		self.data = data;
		self
	}
}

/// The plugins of a schema, code like field types, and what each added
/// to the content model, in the same order.
#[derive(Clone, Default)]
pub(crate) struct Plugins(
	pub(crate) Vec<Arc<dyn Plugin>>,
	pub(crate) Vec<Added>,
);

/// What a plugin added to the content model, for Setup.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Added {
	pub plugin: String,
	pub sections: Vec<String>,
	/// entry types and blocks
	pub components: Vec<String>,
	/// fields in the site's own types, `page.seo`
	pub fields: Vec<String>,
	pub field_types: Vec<String>,
	pub transforms: Vec<String>,
}

/// The handles of a schema, to see what a plugin added: sections,
/// components, `type.field`, field types, transforms.
fn handles(s: &Schema) -> [BTreeSet<String>; 5] {
	let fields = s.components.iter().flat_map(|c| {
		c.fields
			.iter()
			.map(|f| format!("{}.{}", c.handle, f.handle))
	});
	[
		s.sections.iter().map(|x| x.handle.clone()).collect(),
		// inline groups belong to their component
		s.components
			.iter()
			.filter(|c| c.part_of.is_none())
			.map(|c| c.handle.clone())
			.collect(),
		fields.collect(),
		s.field_types
			.0
			.iter()
			.map(|t| t.name().to_string())
			.collect(),
		s.transforms.iter().map(|t| t.handle.clone()).collect(),
	]
}

impl Added {
	/// Adds what is in `after` but not in `before`.
	fn add(&mut self, before: &[BTreeSet<String>; 5], after: &Schema) {
		let after = handles(after);
		let new = |i: usize| after[i].difference(&before[i]).cloned();
		self.sections.extend(new(0));
		self.components.extend(new(1));
		// the fields of new components are theirs
		let fields = new(2).filter(|f| {
			let comp = f.split('.').next().unwrap_or_default();
			!self.components.iter().any(|c| c == comp)
		});
		self.fields.extend(fields.collect::<Vec<_>>());
		self.field_types.extend(new(3));
		self.transforms.extend(new(4));
	}
}

impl Plugins {
	/// What the plugins added, see [`Added`].
	pub(crate) fn added(&self) -> &[Added] {
		&self.1
	}
}

impl Plugins {
	pub(crate) fn get(&self, name: &str) -> Option<&Arc<dyn Plugin>> {
		self.0.iter().find(|p| p.name() == name)
	}
}

impl fmt::Debug for Plugins {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_list()
			.entries(self.0.iter().map(|p| p.name()))
			.finish()
	}
}

/// Code, not content: two schemas are the same whatever plugins they have.
impl PartialEq for Plugins {
	fn eq(&self, _: &Self) -> bool {
		true
	}
}

impl Schema {
	/// Adds a plugin: what it adds to the content model now, the rest
	/// when the server starts.
	pub fn plugin(self, plugin: impl Plugin) -> Self {
		let plugin = Arc::new(plugin);
		let before = handles(&self);
		let mut schema = plugin.schema(self);
		let mut added = Added {
			plugin: plugin.name().into(),
			..Added::default()
		};
		added.add(&before, &schema);
		schema.plugins.0.push(plugin);
		schema.plugins.1.push(added);
		schema
	}

	/// The plugin of a type, for its settings.
	pub fn plugin_of<T: Plugin>(&self) -> Option<&T> {
		self.plugins
			.0
			.iter()
			.find_map(|p| (&**p as &dyn Any).downcast_ref::<T>())
	}

	/// Lets the plugins finish once the schema is complete.
	/// [`Cms`](crate::Cms) calls it.
	pub fn finish(mut self) -> Self {
		for (i, p) in self.plugins.0.clone().into_iter().enumerate() {
			let before = handles(&self);
			self = p.finish(self);
			let mut added = std::mem::take(&mut self.plugins.1[i]);
			added.add(&before, &self);
			self.plugins.1[i] = added;
		}
		self
	}
}
