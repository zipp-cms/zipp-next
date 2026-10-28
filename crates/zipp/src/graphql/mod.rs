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
//! Requests without a session only see published entries.

use std::sync::Arc;

use async_graphql::dynamic::{
	Field as GqlField, FieldFuture, FieldValue, InputObject, InputValue,
	Interface, InterfaceField, Object, ResolverContext, Scalar,
	Schema as GqlSchema, TypeRef, Union,
};
use async_graphql::http::GraphiQLSource;
use async_graphql::{Error as GqlError, Value as GqlValue};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Router;
use axum::extract::{Query as QueryParams, State};
use axum::http::HeaderMap;
use axum::response::Html;
use axum::routing::get;
use serde::Deserialize;
use serde_json::Value;

use crate::AppState;
use crate::assets::{Asset, get_many};
use crate::auth::session;
use crate::db::Db;
use crate::drafts::{Preview, get_draft, preview};
use crate::entries::{Entry, list};
use crate::error::{Error, Result};
use crate::field_type::{GraphqlType, Scalar as FieldScalar};
use crate::query::Query;
use crate::richtext::{self, Chunk};
use crate::schema::{FieldKind, Schema};

mod asset;
mod entry;
mod link;
mod query;
mod rich;
#[cfg(test)]
pub(crate) mod tests;
mod texts;

use asset::*;
use entry::*;
use link::*;
use query::*;
pub(crate) use rich::markdown_html;
use rich::*;
use texts::*;

/// Fields every entry has, field handles can't use them.
pub const ENTRY_FIELDS: &[&str] = &[
	"id",
	"sectionHandle",
	"typeHandle",
	"site",
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
];

/// Fields every block has.
pub const BLOCK_FIELDS: &[&str] = &["id", "typeHandle"];

/// Put before asset urls.
struct PublicUrl(String);

pub use crate::SiteUrls;

/// Who asks, set per request.
struct Viewer {
	published_only: bool,
	/// what a preview token shows
	preview: Option<Preview>,
	/// the valid preview token, urls of entries keep it like Craft's do
	token: Option<String>,
}

/// A rich text field as parent of html and chunks.
struct RichText {
	doc: Value,
	site: String,
}

/// A part of an object value, for objects built as values.
fn value_field<'a>(
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
) -> impl FnMut(&Value) -> Option<(String, Option<String>)> + 'a {
	move |link| {
		let v = resolve_link(ctx, link, site, true).ok()??;
		let GqlValue::Object(o) = v else { return None };
		let url = match o.get("url")? {
			GqlValue::String(u) => u.clone(),
			_ => return None,
		};
		let target = match o.get("target") {
			Some(GqlValue::String(t)) => Some(t.clone()),
			_ => None,
		};
		Some((url, target))
	}
}

/// A block as parent of its fields, it knows its site for relations.
struct Block {
	id: String,
	ty: String,
	site: String,
	fields: Value,
}

pub fn build(
	schema: Arc<Schema>,
	db: Db,
	public_url: String,
	site_urls: SiteUrls,
) -> std::result::Result<GqlSchema, String> {
	let mut entry_iface = Interface::new("Entry");
	for (name, ty) in entry_common() {
		entry_iface = entry_iface.field(InterfaceField::new(name, ty));
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
		.register(asset_object())
		.register(link_object())
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

	for obj in content_objects(&schema) {
		builder = builder.register(obj);
	}
	let mut query = query_object();
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
		// nested relations multiply the work, anyone may ask
		.limit_depth(12)
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
	let (published_only, preview) = {
		let conn = state.db.conn();
		// only admins see every draft and unpublished entry, a role only
		// some sections: it reads GraphQL like a frontend
		let editor = match session(&conn, &headers)? {
			Some(user) => conn.query_row(
				"SELECT role IS NULL FROM users WHERE id = ?1",
				[user],
				|r| r.get::<_, bool>(0),
			)?,
			None => false,
		};
		// private like Craft's: frontends send the token, the studio its
		// session; development without a token is open
		let open = state.dev && state.graphql_token.is_none();
		if !editor && !open && !has_token(&state, &headers) {
			return Err(Error::MissingEndpointToken);
		}
		let preview = match &p.token {
			Some(t) => preview(&conn, t)?,
			None => None,
		};
		(!editor, preview)
	};
	let token = preview.as_ref().and(p.token);
	let req = req.into_inner().data(Viewer {
		published_only,
		preview,
		token,
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
	});
	gql.execute(req)
		.await
		.errors
		.into_iter()
		.filter(|e| !e.message.contains("not provided"))
		.map(|e| match e.locations.first() {
			Some(l) => format!("{}:{}: {}", l.line, l.column, e.message),
			None => e.message,
		})
		.collect()
}

/// Whether the request has the endpoint token.
fn has_token(state: &AppState, headers: &HeaderMap) -> bool {
	let Some(token) = &state.graphql_token else {
		return false;
	};
	let sent = headers
		.get(axum::http::header::AUTHORIZATION)
		.and_then(|v| v.to_str().ok())
		.and_then(|v| v.strip_prefix("Bearer "))
		.unwrap_or_default();
	// as long whatever matches
	sent.len() == token.len()
		&& sent
			.bytes()
			.zip(token.bytes())
			.fold(0, |acc, (a, b)| acc | (a ^ b))
			== 0
}

/// The playground, only in development: it loads its scripts from a cdn.
async fn graphiql(State(state): State<AppState>) -> Result<Html<String>> {
	if !state.dev {
		return Err(Error::NotFound);
	}
	Ok(Html(
		GraphiQLSource::build().endpoint("/api/graphql").finish(),
	))
}

pub fn routes() -> Router<AppState> {
	Router::new().route(
		"/graphql",
		get(graphiql)
			.post(graphql)
			// async-graphql reads the body itself, past axum's limit
			.layer(tower_http::limit::RequestBodyLimitLayer::new(256 * 1024)),
	)
}
