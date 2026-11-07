//! GraphQL generated from the schema, for frontends like crelte: one
//! query loads a page with everything it shows, relations come with their
//! fields.
//!
//! Every entry type is an object `<handle>_Entry` with the `Entry`
//! interface, every block `<handle>_Block` with the `Block` interface,
//! named like Craft's types:
//!
//! ```graphql
//! query ($uri: String, $site: String) {
//!     entry(uri: $uri, site: $site) {
//!         sectionHandle
//!         typeHandle
//!         title
//!         ... on page_Entry {
//!             body {
//!                 ... on button_Block { label link { title uri } }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! Requests without a session only see published entries. Frontends
//! send a token, its `Scope` says which sections and sites they read.

use std::sync::Arc;

use async_graphql::dynamic::{
	Field as GqlField, FieldFuture, FieldValue, InputObject, InputValue,
	Interface, InterfaceField, Object, ResolverContext, Scalar,
	Schema as GqlSchema, TypeRef, Union,
};
use async_graphql::extensions::{
	Extension, ExtensionContext, ExtensionFactory, NextExecute,
};
use async_graphql::{Error as GqlError, Value as GqlValue};
use serde_json::Value;

use crate::assets::{Asset, get_many};
use crate::content::richtext::{self, Chunk};
use crate::db::Db;
use crate::drafts::{Preview, docs_of};
use crate::entries::query::Query;
use crate::entries::{Entry, count, select};
use crate::schema::{
	ASSET, ColumnKind, Component, FieldKind, Format, Schema, Transform, USER,
};
use crate::schema::{FieldType, GraphqlType, Scalar as FieldScalar};
use ext::value_field;

pub(crate) mod api;
#[cfg(test)]
mod api_tests;
mod asset;
mod asset_list;
#[cfg(test)]
mod asset_list_tests;
mod dates;
#[cfg(test)]
mod dates_tests;
mod entry;
pub mod ext;
#[cfg(test)]
mod field_tests;
mod link;
mod loaders;
mod query;
#[cfg(test)]
mod query_tests;
mod rich;
mod routes;
mod search;
#[cfg(test)]
mod search_tests;
#[cfg(test)]
pub(crate) mod tests;
mod texts;
pub(crate) mod tokens;
mod user;
#[cfg(test)]
mod user_tests;
#[cfg(test)]
mod viewer_tests;

pub(crate) use crate::content::markdown_html;
use asset::*;
use asset_list::*;
use entry::*;
use link::*;
use loaders::*;
use query::*;
use rich::*;
pub(crate) use routes::routes;
use search::*;
use texts::*;
pub(crate) use tokens::Scope;
use user::*;

/// Fields every entry has, field handles can't use them.
pub(crate) const ENTRY_FIELDS: &[&str] = &[
	"id",
	"sectionHandle",
	"typeHandle",
	"site",
	"siteHandle",
	"siteId",
	"url",
	"title",
	"slug",
	"uri",
	"published",
	"parent",
	"children",
	"createdAt",
	"updatedAt",
	"postDate",
	"expiryDate",
	"dateCreated",
	"dateUpdated",
	"language",
	"localized",
];

/// Fields every entry has unless its type has a field of the name: they
/// came after 0.1.0, sites may have such fields.
pub(crate) const OPTIONAL_ENTRY_FIELDS: &[&str] = &["authorId", "author"];

/// Fields every block has.
pub(crate) const BLOCK_FIELDS: &[&str] = &["id", "typeHandle"];

/// Fields every asset has, asset fields can't use them.
pub(crate) const ASSET_FIELDS: &[&str] = &[
	"id",
	"filename",
	"kind",
	"mimeType",
	"size",
	"alt",
	"focalX",
	"focalY",
	"focalPoint",
	"title",
	"folderId",
	"folder",
	"path",
	"url",
	"width",
	"height",
	"srcset",
];

/// Fields every user has, user fields can't use them.
pub(crate) const USER_FIELDS: &[&str] = &["id", "name", "slug", "url", "uri"];

/// Put before asset urls.
struct PublicUrl(String);

use crate::SiteUrls;

/// Who asks, set per request.
struct Viewer {
	published_only: bool,
	/// what a preview token shows
	preview: Option<Preview>,
	/// the valid preview token, urls of entries keep it like Craft's do
	token: Option<String>,
	/// whether search engines may index, not in development
	indexing: bool,
	/// what the token of a frontend may read
	scope: Scope,
	/// what the request loaded so far
	loaded: Loaded,
}

impl Viewer {
	/// The same viewer, for the loaders of its request.
	fn copy(&self) -> Viewer {
		Viewer {
			published_only: self.published_only,
			preview: self.preview.clone(),
			token: self.token.clone(),
			indexing: self.indexing,
			scope: self.scope.clone(),
			loaded: Default::default(),
		}
	}
}

/// What a request loaded so far.
#[derive(Default)]
struct Loaded {
	/// the number of entries and assets, see [`MAX_LOADED`]
	count: std::sync::atomic::AtomicUsize,
	/// the loaders, made on first use, see [`Loaders`]
	loaders: std::sync::OnceLock<Loaders>,
	/// the paths of the asset folders by id, read on first use
	folders: std::sync::OnceLock<std::collections::HashMap<i64, String>>,
}

/// The most entries and assets a request may load, each query counts as
/// one more: depth and complexity limit the shape of a query, not how
/// many entries nested lists of a thousand load. Pages of real sites
/// load a few hundred.
const MAX_LOADED: usize = 10_000;

/// Most children an entry has in GraphQL, they are loaded per entry.
const MAX_CHILDREN: i64 = 500;

/// A rich text field as parent of html and chunks.
struct RichText {
	doc: Value,
	site: String,
}

/// A row of a table field, it knows its site for links.
struct TableRow {
	cells: Value,
	site: String,
}

/// Url and target of the links of a rich text, external ones in a new tab.
fn resolver<'a>(
	ctx: &'a ResolverContext,
	site: &'a str,
	targets: &'a Targets,
) -> impl FnMut(&Value) -> Option<(String, Option<String>)> + 'a {
	move |link| {
		let l = resolve_link(ctx, link, site, true, targets).ok()??;
		Some((l.url, l.target.map(Into::into)))
	}
}

/// A block as parent of its fields, it knows its site for relations.
struct Block {
	id: String,
	ty: String,
	site: String,
	fields: Value,
}

/// A block of a blocks field or a rich text, `{ "id", "type", "fields" }`,
/// as its `<type>_Block`.
fn block_of(b: &Value, site: &str) -> FieldValue<'static> {
	let ty = b["type"].as_str().unwrap_or_default().to_string();
	FieldValue::owned_any(Block {
		id: b["id"].as_str().unwrap_or_default().into(),
		ty: ty.clone(),
		site: site.into(),
		fields: b["fields"].clone(),
	})
	.with_type(format!("{ty}_Block"))
}

pub(crate) fn build(
	schema: Arc<Schema>,
	db: Db,
	public_url: String,
	site_urls: SiteUrls,
) -> std::result::Result<GqlSchema, String> {
	// what the plugins add
	let mut plugin_types = vec![];
	let mut entry_fields = vec![];
	let mut root_fields = vec![];
	for p in &schema.plugins.0 {
		let ext = p.graphql(&schema);
		plugin_types.extend(ext.types);
		entry_fields.extend(ext.entry_fields);
		root_fields.extend(ext.root_fields);
	}

	let mut entry_iface = Interface::new("Entry");
	let own_field = |name: &str| {
		schema
			.components
			.iter()
			.any(|c| c.entry && c.fields.iter().any(|f| f.handle == name))
	};
	for (name, ty) in entry_common() {
		if OPTIONAL_ENTRY_FIELDS.contains(&name) && own_field(name) {
			continue;
		}
		let mut field = InterfaceField::new(name, ty);
		if name == "children" {
			for (arg, ty) in children_args(&schema) {
				field = field.argument(InputValue::new(arg, ty));
			}
		}
		if DATES.contains(&name) {
			for arg in dates::args() {
				field = field.argument(arg);
			}
		}
		entry_iface = entry_iface.field(field);
	}
	for f in &entry_fields {
		// unless an entry type has a field of its own named so
		let own = schema
			.components
			.iter()
			.any(|c| c.entry && c.fields.iter().any(|x| x.handle == f.name));
		if !own {
			entry_iface = entry_iface
				.field(InterfaceField::new(f.name.clone(), f.ty.clone()));
		}
	}
	let block_iface = Interface::new("Block")
		.field(InterfaceField::new(
			"id",
			TypeRef::named_nn(TypeRef::STRING),
		))
		.field(InterfaceField::new(
			"typeHandle",
			TypeRef::named_nn(TypeRef::STRING),
		));

	let mut builder = GqlSchema::build("Query", None, None)
		.register(entry_iface)
		.register(block_iface)
		.register(where_input())
		.register(criteria_input(&schema))
		.register(site_object())
		.register(user_object(&schema))
		.register(has_entries_input())
		.register(asset_object(&schema))
		.register(link_object())
		.register(nav_item_object())
		.register(rich_text_object())
		.register(Object::new("Html").field(GqlField::new(
			"html",
			TypeRef::named_nn(TypeRef::STRING),
			|ctx| value_field(ctx, "html"),
		)))
		.register(Object::new("Buttons").field(GqlField::new(
			"buttons",
			TypeRef::named_nn_list_nn("Link"),
			|ctx| value_field(ctx, "buttons"),
		)))
		.register(chunk_union(&schema))
		.register(search_hit_object())
		.register(
			Object::new("Markdown")
				.field(GqlField::new(
					"markdown",
					TypeRef::named_nn(TypeRef::STRING),
					|ctx| value_field(ctx, "markdown"),
				))
				.field(GqlField::new(
					"html",
					TypeRef::named_nn(TypeRef::STRING),
					|ctx| value_field(ctx, "html"),
				)),
		)
		.register(button_group_option())
		.register(
			Scalar::new("QueryArgument")
				.description("An id or a handle, like Craft's QueryArgument"),
		);

	for ty in plugin_types {
		builder = builder.register(ty);
	}
	for obj in content_objects(&schema, &entry_fields) {
		builder = builder.register(obj);
	}
	let mut query = query_object(&schema);
	let more = search_fields().into_iter().chain(asset_list_fields());
	for field in more.chain(user_fields()) {
		query = query.field(field);
	}
	for field in root_fields {
		query = query.field(field);
	}
	if let Some((obj, field)) = translations_field(&schema) {
		builder = builder.register(obj);
		query = query.field(field);
	}

	builder
		.register(query)
		.data(schema)
		.data(db)
		.data(PublicUrl(public_url))
		.data(site_urls)
		// nested relations multiply the work, anyone may ask; GraphiQL's
		// introspection goes 15 deep
		.limit_depth(15)
		.limit_complexity(2000)
		.extension(PublicErrors)
		.extension(dates::FormatDateTime)
		.finish()
		.map_err(|e| e.to_string())
}

/// Shows the errors of resolvers like the rest api does: what failed
/// inside (SQL, paths) only in the log. Every resolver, also of plugins,
/// can `?` a [`crate::error::Error`] or a database error.
struct PublicErrors;

impl ExtensionFactory for PublicErrors {
	fn create(&self) -> Arc<dyn Extension> {
		Arc::new(PublicErrors)
	}
}

#[async_graphql::async_trait::async_trait]
impl Extension for PublicErrors {
	async fn execute(
		&self,
		ctx: &ExtensionContext<'_>,
		operation_name: Option<&str>,
		next: NextExecute<'_>,
	) -> async_graphql::Response {
		let mut resp = next.run(ctx, operation_name).await;
		for e in &mut resp.errors {
			let Some(source) = &e.source else { continue };
			let public = if let Some(err) =
				source.downcast_ref::<crate::error::Error>()
			{
				err.clone().public()
			} else if let Some(err) = source.downcast_ref::<rusqlite::Error>() {
				crate::error::Error::Internal(err.to_string()).public()
			} else {
				continue;
			};
			e.message = public.to_string();
		}
		resp
	}
}

fn scalar_name(s: FieldScalar) -> &'static str {
	match s {
		FieldScalar::String => TypeRef::STRING,
		FieldScalar::Int => TypeRef::INT,
		FieldScalar::Float => TypeRef::FLOAT,
		FieldScalar::Boolean => TypeRef::BOOLEAN,
	}
}

/// The mistakes of a query of a frontend against the schema: unknown
/// fields and types, wrong arguments. It runs on an empty database, so
/// only the shape is checked; variables it needs are no mistake.
pub(crate) async fn check_query(gql: &GqlSchema, query: &str) -> Vec<String> {
	let req = async_graphql::Request::new(query).data(Viewer {
		published_only: true,
		preview: None,
		token: None,
		indexing: true,
		scope: Scope::ALL,
		loaded: Default::default(),
	});
	gql.execute(req)
		.await
		.errors
		.into_iter()
		// the values of the variables come from the frontend
		.filter(|e| {
			!e.message.contains("not provided")
				&& !(e.message.starts_with("Variable ")
					&& e.message.ends_with(" is not defined."))
		})
		.map(|e| match e.locations.first() {
			Some(l) => format!("{}:{}: {}", l.line, l.column, e.message),
			None => e.message,
		})
		.collect()
}
