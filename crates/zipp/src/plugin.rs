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
use std::fmt;
use std::sync::Arc;

use async_graphql::dynamic::{Field as GqlField, Type, TypeRef};
use axum::Router;
use serde_json::Value;

use crate::AppState;
use crate::schema::Schema;

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
	fn api_routes(&self) -> Router<AppState> {
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
#[derive(Default)]
pub struct GraphqlExt {
	pub types: Vec<Type>,
	/// on every entry type and the `Entry` interface; an entry type with
	/// a field of the same handle keeps its field
	pub entry_fields: Vec<EntryField>,
	pub root_fields: Vec<GqlField>,
}

/// A field of every entry, made once per entry type.
pub struct EntryField {
	pub name: String,
	pub ty: TypeRef,
	pub field: Arc<dyn Fn() -> GqlField + Send + Sync>,
}

/// The module of a plugin in the studio, an ES module served as
/// `/_plugins/<name>.js` that brings its own framework like a field
/// type's. It may export
/// `sectionSetup(target, { schema, section, data, host })`, a card on
/// the setup page of a section; `data` is the plugin's JSON for it.
#[derive(Debug, Clone)]
pub struct PluginStudio {
	pub js: &'static str,
	pub data: Value,
}

/// The plugins of a schema, code like field types.
#[derive(Clone, Default)]
pub struct Plugins(pub Vec<Arc<dyn Plugin>>);

impl Plugins {
	pub fn get(&self, name: &str) -> Option<&Arc<dyn Plugin>> {
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
		let mut schema = plugin.schema(self);
		schema.plugins.0.push(plugin);
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
		for p in self.plugins.0.clone() {
			self = p.finish(self);
		}
		self
	}
}
