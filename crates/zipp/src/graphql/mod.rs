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
//! send a token, its [`Scope`] says which sections and sites they read.

use std::sync::Arc;

use async_graphql::dynamic::{
	Field as GqlField, FieldFuture, FieldValue, InputObject, InputValue,
	Interface, InterfaceField, Object, ResolverContext, Scalar,
	Schema as GqlSchema, TypeRef, Union,
};
use async_graphql::{Error as GqlError, Value as GqlValue};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Router;
use axum::extract::{Query as QueryParams, State};
use axum::http::HeaderMap;
use axum::response::Response;
use axum::routing::get;
use serde::Deserialize;
use serde_json::Value;

use crate::AppState;
use crate::assets::{Asset, get_many};
use crate::auth::{role_of, session};
use crate::content::richtext::{self, Chunk};
use crate::db::Db;
use crate::drafts::{Preview, docs_of, preview};
use crate::entries::query::Query;
use crate::entries::{Entry, count, select};
use crate::error::{Error, Result};
use crate::schema::field_type::{GraphqlType, Scalar as FieldScalar};
use crate::schema::{ASSET, ColumnKind, FieldKind, Format, Schema, Transform};

pub mod api;
#[cfg(test)]
mod api_tests;
mod asset;
mod entry;
pub mod ext;
mod link;
mod loaders;
mod query;
mod rich;
mod routes;
#[cfg(test)]
pub(crate) mod tests;
mod texts;
pub mod tokens;

pub(crate) use crate::content::markdown_html;
use asset::*;
use entry::*;
use link::*;
use loaders::*;
use query::*;
use rich::*;
use texts::*;
pub use tokens::Scope;

/// Fields every entry has, field handles can't use them.
pub const ENTRY_FIELDS: &[&str] = &[
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

/// Fields every block has.
pub const BLOCK_FIELDS: &[&str] = &["id", "typeHandle"];

/// Fields every asset has, asset fields can't use them.
pub const ASSET_FIELDS: &[&str] = &[
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
	"url",
	"width",
	"height",
	"srcset",
];

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

/// A part of an object value, for objects built as values.
pub fn value_field<'a>(
	ctx: ResolverContext<'a>,
	name: &'static str,
) -> FieldFuture<'a> {
	FieldFuture::new(async move {
		let GqlValue::Object(o) = ctx.parent_value.try_to_value()? else {
			return Ok(None);
		};
		Ok(match o.get(name) {
			Some(GqlValue::List(l)) => {
				Some(FieldValue::list(l.iter().cloned().map(FieldValue::value)))
			}
			Some(v) => Some(FieldValue::value(v.clone())),
			None => None,
		})
	})
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

pub fn build(
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
	for (name, ty) in entry_common() {
		entry_iface = entry_iface.field(InterfaceField::new(name, ty));
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
		.register(site_object())
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
		.finish()
		.map_err(|e| e.to_string())
}

fn scalar_name(s: FieldScalar) -> &'static str {
	match s {
		FieldScalar::String => TypeRef::STRING,
		FieldScalar::Int => TypeRef::INT,
		FieldScalar::Float => TypeRef::FLOAT,
		FieldScalar::Boolean => TypeRef::BOOLEAN,
	}
}

#[derive(Debug, Deserialize)]
struct TokenParam {
	/// a preview token, crelte passes the one of the page on
	token: Option<String>,
}

async fn graphql(
	State(state): State<AppState>,
	headers: HeaderMap,
	QueryParams(p): QueryParams<TokenParam>,
	req: GraphQLRequest,
) -> Result<GraphQLResponse> {
	let (published_only, preview, scope) = {
		let conn = state.db.conn();
		// only admins see every draft and unpublished entry, a role only
		// some sections: it reads GraphQL like a frontend
		let editor = match session(&conn, &headers)? {
			Some(user) => role_of(&conn, user)?.is_none(),
			None => false,
		};
		// private like Craft's: frontends send a token, the studio its
		// session; development without the token of serve is open
		let open = state.dev && state.graphql_token.is_none();
		let scope = match (editor, bearer(&headers)) {
			(true, _) => Scope::ALL,
			// a wrong token fails in development too, like on the server
			(false, Some(t)) => token_scope(&state, &conn, t)?
				.ok_or(Error::MissingEndpointToken)?,
			(false, None) if open => Scope::ALL,
			(false, None) => return Err(Error::MissingEndpointToken),
		};
		let preview = match &p.token {
			Some(t) if scope.previews => preview(&conn, &state.schema, t)?,
			_ => None,
		};
		(!editor, preview, scope)
	};
	let token = preview.as_ref().and(p.token);
	let req = req.into_inner().data(Viewer {
		published_only,
		preview,
		token,
		indexing: state.indexing,
		scope,
		loaded: Default::default(),
	});
	Ok(state.gql.execute(req).await.into())
}

/// The mistakes of a query of a frontend against the schema: unknown
/// fields and types, wrong arguments. It runs on an empty database, so
/// only the shape is checked; variables it needs are no mistake.
pub async fn check_query(gql: &GqlSchema, query: &str) -> Vec<String> {
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

/// The token of a request, `Authorization: Bearer <token>`.
fn bearer(headers: &HeaderMap) -> Option<&str> {
	headers
		.get(axum::http::header::AUTHORIZATION)
		.and_then(|v| v.to_str().ok())
		.and_then(|v| v.strip_prefix("Bearer "))
}

/// What a token may read: the one of `serve` everything, the ones of the
/// studio their scope; `None` for an unknown or expired one.
fn token_scope(
	state: &AppState,
	conn: &rusqlite::Connection,
	sent: &str,
) -> Result<Option<Scope>> {
	if let Some(token) = &state.graphql_token {
		// as long whatever matches
		let same = sent.len() == token.len()
			&& sent
				.bytes()
				.zip(token.bytes())
				.fold(0, |acc, (a, b)| acc | (a ^ b))
				== 0;
		if same {
			return Ok(Some(Scope::ALL));
		}
	}
	tokens::scope_of(conn, sent)
}

/// The playground, for admins and in development: a page of the studio's
/// build, see [`crate::studio`]. Its queries send the session of the
/// studio, so an admin sees drafts like in the studio; without one it goes
/// to the login, except in development where GraphQL is open.
async fn graphiql(State(state): State<AppState>) -> Response {
	crate::studio::graphiql(&state)
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route(
			"/graphql",
			get(graphiql)
				.post(graphql)
				// async-graphql reads the body itself, past axum's limit
				.layer(tower_http::limit::RequestBodyLimitLayer::new(
					256 * 1024,
				)),
		)
		.merge(routes::routes())
}
