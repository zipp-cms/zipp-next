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
	Schema as GqlSchema, TypeRef,
};
use async_graphql::http::GraphiQLSource;
use async_graphql::{Error as GqlError, Value as GqlValue};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Router;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::Html;
use axum::routing::get;
use serde_json::Value;

use crate::AppState;
use crate::assets::{Asset, get_many};
use crate::auth::session;
use crate::db::Db;
use crate::entries::{Entry, list};
use crate::error::Result;
use crate::query::Query;
use crate::schema::{FieldKind, Schema};

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

/// The url of each site, by handle, like Craft's base urls.
pub type SiteUrls = std::collections::HashMap<String, String>;

/// Who asks, set per request.
struct Viewer {
	published_only: bool,
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
		.register(
			Scalar::new("QueryArgument")
				.description("An id or a handle, like Craft's QueryArgument"),
		);

	for comp in &schema.components {
		let mut obj = if comp.entry {
			let mut obj = Object::new(format!("{}_Entry", comp.handle))
				.implement("Entry");
			for (name, _) in entry_common() {
				obj = obj.field(entry_common_field(name));
			}
			obj
		} else {
			Object::new(format!("{}_Block", comp.handle))
				.implement("Block")
				.field(GqlField::new(
					"id",
					TypeRef::named_nn(TypeRef::STRING),
					|ctx| block_value(ctx, |b| GqlValue::from(b.id.clone())),
				))
				.field(GqlField::new(
					"typeHandle",
					TypeRef::named_nn(TypeRef::STRING),
					|ctx| block_value(ctx, |b| GqlValue::from(b.ty.clone())),
				))
		};
		for field in &comp.fields {
			obj = obj.field(content_field(field));
		}
		builder = builder.register(obj);
	}

	let query = Object::new("Query")
		.field(entries_args(GqlField::new(
			"entry",
			TypeRef::named("Entry"),
			|ctx| {
				FieldFuture::new(async move {
					let mut params = params_from_args(&ctx)?;
					params.push(("limit".into(), "1".into()));
					let e = load(&ctx, &params)?.into_iter().next();
					Ok(e.map(entry_value))
				})
			},
		)))
		.field(entries_args(GqlField::new(
			"entries",
			TypeRef::named_nn_list_nn("Entry"),
			|ctx| {
				FieldFuture::new(async move {
					let params = params_from_args(&ctx)?;
					let entries = load(&ctx, &params)?;
					Ok(Some(FieldValue::list(
						entries.into_iter().map(entry_value),
					)))
				})
			},
		)))
		.field(entries_args(GqlField::new(
			"entryCount",
			TypeRef::named_nn(TypeRef::INT),
			|ctx| {
				FieldFuture::new(async move {
					let params = params_from_args(&ctx)?;
					let q = query(&ctx, &params)?;
					let db = ctx.data::<Db>()?;
					let (_, total) = list(&db.conn(), &q)?;
					Ok(Some(GqlValue::from(total)))
				})
			},
		)))
		.field(sites_field("sites"))
		// what crelte asks for on start
		.field(sites_field("crelteSites"));

	builder
		.register(query)
		.data(schema)
		.data(db)
		.data(PublicUrl(public_url))
		.data(site_urls)
		.finish()
		.map_err(|e| e.to_string())
}

fn entry_common() -> Vec<(&'static str, TypeRef)> {
	let s = || TypeRef::named_nn(TypeRef::STRING);
	vec![
		("id", TypeRef::named_nn(TypeRef::INT)),
		("sectionHandle", s()),
		("typeHandle", s()),
		("site", s()),
		// the position of the site, crelte knows sites by number
		("siteId", TypeRef::named_nn(TypeRef::INT)),
		("url", TypeRef::named(TypeRef::STRING)),
		("title", s()),
		("slug", s()),
		("uri", TypeRef::named(TypeRef::STRING)),
		("published", TypeRef::named_nn(TypeRef::BOOLEAN)),
		("parent", TypeRef::named("Entry")),
		("children", TypeRef::named_nn_list_nn("Entry")),
		// milliseconds since 1970, too big for Int
		("createdAt", TypeRef::named_nn(TypeRef::FLOAT)),
		("updatedAt", TypeRef::named_nn(TypeRef::FLOAT)),
	]
}

fn entry_common_field(name: &'static str) -> GqlField {
	let ty = entry_common()
		.into_iter()
		.find(|(n, _)| *n == name)
		.unwrap()
		.1;
	GqlField::new(name, ty, move |ctx| {
		FieldFuture::new(async move {
			let e = ctx.parent_value.try_downcast_ref::<Entry>()?;
			let v = match name {
				"id" => GqlValue::from(e.id),
				"sectionHandle" => GqlValue::from(e.section.clone()),
				"typeHandle" => GqlValue::from(e.ty.clone()),
				"site" => GqlValue::from(e.site.clone()),
				"siteId" => GqlValue::from(site_id(&ctx, &e.site)?),
				"url" => match &e.uri {
					Some(uri) => GqlValue::from(entry_url(&ctx, &e.site, uri)?),
					None => GqlValue::Null,
				},
				"title" => GqlValue::from(e.title.clone()),
				"slug" => GqlValue::from(e.slug.clone()),
				"uri" => e.uri.clone().map(GqlValue::from).unwrap_or_default(),
				"published" => GqlValue::from(e.published),
				"createdAt" => GqlValue::from(e.created_at as f64),
				"updatedAt" => GqlValue::from(e.updated_at as f64),
				"parent" => {
					let Some(p) = e.parent_id else {
						return Ok(None);
					};
					let params = site_params(&e.site, [("ids", p.to_string())]);
					let parent = load(&ctx, &params)?.into_iter().next();
					return Ok(parent.map(entry_value));
				}
				"children" => {
					let params = site_params(
						&e.site,
						[
							("parent", e.id.to_string()),
							("order", "structure".into()),
						],
					);
					let children = load(&ctx, &params)?;
					return Ok(Some(FieldValue::list(
						children.into_iter().map(entry_value),
					)));
				}
				_ => unreachable!(),
			};
			Ok(Some(FieldValue::value(v)))
		})
	})
}

/// A field of the content model, read from the JSON of its entry or block.
fn content_field(field: &crate::schema::Field) -> GqlField {
	let ty = match &field.kind {
		FieldKind::Text { .. } => TypeRef::named(TypeRef::STRING),
		FieldKind::Number { .. } => TypeRef::named(TypeRef::FLOAT),
		FieldKind::Boolean => TypeRef::named(TypeRef::BOOLEAN),
		FieldKind::Relation { .. } => TypeRef::named_nn_list_nn("Entry"),
		FieldKind::Assets { .. } => TypeRef::named_nn_list_nn("Asset"),
		FieldKind::Blocks { .. } => TypeRef::named_nn_list_nn("Block"),
	};
	let handle = field.handle.clone();
	let kind = field.kind.clone();

	GqlField::new(field.handle.clone(), ty, move |ctx| {
		let handle = handle.clone();
		let kind = kind.clone();
		FieldFuture::new(async move {
			let (fields, site) = match ctx.parent_value.downcast_ref::<Entry>()
			{
				Some(e) => (&e.fields, e.site.clone()),
				None => {
					let b = ctx.parent_value.try_downcast_ref::<Block>()?;
					(&b.fields, b.site.clone())
				}
			};
			let value = fields.get(&handle).cloned().unwrap_or(Value::Null);

			match kind {
				FieldKind::Relation { .. } => {
					let ids: Vec<i64> = value
						.as_array()
						.map(|a| a.iter().filter_map(Value::as_i64).collect())
						.unwrap_or_default();
					if ids.is_empty() {
						return Ok(Some(FieldValue::list(
							Vec::<FieldValue>::new(),
						)));
					}
					let list: Vec<String> =
						ids.iter().map(i64::to_string).collect();
					let params = site_params(&site, [("ids", list.join(","))]);
					let mut entries = load(&ctx, &params)?;
					// in the order of the field
					entries
						.sort_by_key(|e| ids.iter().position(|i| *i == e.id));
					Ok(Some(FieldValue::list(
						entries.into_iter().map(entry_value),
					)))
				}
				FieldKind::Assets { .. } => {
					let ids: Vec<i64> = value
						.as_array()
						.map(|a| a.iter().filter_map(Value::as_i64).collect())
						.unwrap_or_default();
					let db = ctx.data::<Db>()?;
					let assets = get_many(&db.conn(), &ids)?;
					Ok(Some(FieldValue::list(
						assets.into_iter().map(FieldValue::owned_any),
					)))
				}
				FieldKind::Blocks { .. } => {
					let blocks = value.as_array().cloned().unwrap_or_default();
					let blocks = blocks.into_iter().map(|b| {
						let ty =
							b["type"].as_str().unwrap_or_default().to_string();
						FieldValue::owned_any(Block {
							id: b["id"].as_str().unwrap_or_default().into(),
							ty: ty.clone(),
							site: site.clone(),
							fields: b["fields"].clone(),
						})
						.with_type(format!("{ty}_Block"))
					});
					Ok(Some(FieldValue::list(blocks)))
				}
				_ if value.is_null() => Ok(None),
				_ => Ok(Some(FieldValue::value(GqlValue::from_json(value)?))),
			}
		})
	})
}

/// An asset, like Craft's: `url(transform: "medium")` and the size after
/// the transform. Transforms are the named ones of the schema.
fn asset_object() -> Object {
	let plain = |name: &'static str, ty: TypeRef| {
		GqlField::new(name, ty, move |ctx| {
			FieldFuture::new(async move {
				let a = ctx.parent_value.try_downcast_ref::<Asset>()?;
				let v = match name {
					"id" => GqlValue::from(a.id),
					"filename" => GqlValue::from(a.filename.clone()),
					"kind" => GqlValue::from(a.kind.clone()),
					"mimeType" => GqlValue::from(a.mime.clone()),
					"size" => GqlValue::from(a.size),
					"alt" => GqlValue::from(a.alt.clone()),
					"focalX" => GqlValue::from(a.focal_x),
					"focalY" => GqlValue::from(a.focal_y),
					_ => unreachable!(),
				};
				Ok(Some(FieldValue::value(v)))
			})
		})
	};
	let transform_arg =
		|| InputValue::new("transform", TypeRef::named(TypeRef::STRING));
	let s = || TypeRef::named_nn(TypeRef::STRING);
	let f = || TypeRef::named_nn(TypeRef::FLOAT);

	Object::new("Asset")
		.field(plain("id", TypeRef::named_nn(TypeRef::INT)))
		.field(plain("filename", s()))
		.field(plain("kind", s()))
		.field(plain("mimeType", s()))
		.field(plain("size", TypeRef::named_nn(TypeRef::INT)))
		.field(plain("alt", s()))
		.field(plain("focalX", f()))
		.field(plain("focalY", f()))
		.field(
			GqlField::new("url", s(), |ctx| {
				FieldFuture::new(async move {
					let a = ctx.parent_value.try_downcast_ref::<Asset>()?;
					let t = transform_of(&ctx, a)?;
					let url = asset_url(
						&ctx,
						a,
						t.as_ref().map(|t| t.handle.as_str()),
					)?;
					Ok(Some(FieldValue::value(url)))
				})
			})
			.argument(transform_arg()),
		)
		.field(
			GqlField::new("width", TypeRef::named(TypeRef::INT), |ctx| {
				asset_size(ctx, |(w, _)| w)
			})
			.argument(transform_arg()),
		)
		.field(
			GqlField::new("height", TypeRef::named(TypeRef::INT), |ctx| {
				asset_size(ctx, |(_, h)| h)
			})
			.argument(transform_arg()),
		)
		.field(
			GqlField::new("srcset", TypeRef::named(TypeRef::STRING), |ctx| {
				FieldFuture::new(async move {
					let a = ctx.parent_value.try_downcast_ref::<Asset>()?;
					let schema = ctx.data::<Arc<Schema>>()?;
					let mut parts = vec![];
					for name in ctx.args.try_get("transforms")?.list()?.iter() {
						let name = name.string()?;
						let t =
							schema.get_transform(name).ok_or_else(|| {
								GqlError::new(format!(
									"unknown transform {name}"
								))
							})?;
						let Some((w, _)) = a.size_after(t) else {
							return Ok(None);
						};
						parts.push(format!(
							"{} {w}w",
							asset_url(&ctx, a, Some(name))?
						));
					}
					Ok(Some(FieldValue::value(parts.join(", "))))
				})
			})
			.argument(InputValue::new(
				"transforms",
				TypeRef::named_nn_list_nn(TypeRef::STRING),
			)),
		)
}

/// The transform of the `transform` argument, `None` for the original.
fn transform_of(
	ctx: &ResolverContext,
	a: &Asset,
) -> async_graphql::Result<Option<crate::schema::Transform>> {
	let Some(name) = ctx.args.get("transform").filter(|v| !v.is_null()) else {
		return Ok(None);
	};
	let name = name.string()?;
	let schema = ctx.data::<Arc<Schema>>()?;
	let t = schema
		.get_transform(name)
		.ok_or_else(|| GqlError::new(format!("unknown transform {name}")))?;
	// only images have transforms, a file stays itself
	Ok((a.kind == "image").then(|| t.clone()))
}

fn asset_url(
	ctx: &ResolverContext,
	a: &Asset,
	transform: Option<&str>,
) -> async_graphql::Result<String> {
	let base = &ctx.data::<PublicUrl>()?.0;
	Ok(format!(
		"{}{}",
		base.trim_end_matches('/'),
		a.url(transform)
	))
}

fn asset_size(
	ctx: ResolverContext,
	pick: fn((u32, u32)) -> u32,
) -> FieldFuture {
	FieldFuture::new(async move {
		let a = ctx.parent_value.try_downcast_ref::<Asset>()?;
		let size = match transform_of(&ctx, a)? {
			Some(t) => a.size_after(&t),
			None => a.width.zip(a.height),
		};
		Ok(size.map(|s| FieldValue::value(pick(s))))
	})
}

fn block_value(
	ctx: ResolverContext,
	f: impl Fn(&Block) -> GqlValue + Send + Sync + 'static,
) -> FieldFuture {
	FieldFuture::new(async move {
		let b = ctx.parent_value.try_downcast_ref::<Block>()?;
		Ok(Some(FieldValue::value(f(b))))
	})
}

fn entry_value(e: Entry) -> FieldValue<'static> {
	let ty = format!("{}_Entry", e.ty);
	FieldValue::owned_any(e).with_type(ty)
}

/// The arguments of `entry`, `entries` and `entryCount`, the same as the
/// rest api (see query.rs).
fn entries_args(field: GqlField) -> GqlField {
	let s = || TypeRef::named(TypeRef::STRING);
	// lists like in Craft, so crelte's queries fit; one value is used
	let l = || TypeRef::named_list(TypeRef::STRING);
	let i = || TypeRef::named(TypeRef::INT);
	field
		.argument(InputValue::new("site", l()))
		.argument(InputValue::new(
			"siteId",
			TypeRef::named_list("QueryArgument"),
		))
		.argument(InputValue::new("section", l()))
		.argument(InputValue::new("type", l()))
		.argument(InputValue::new("uri", l()))
		.argument(InputValue::new("slug", l()))
		.argument(InputValue::new("id", i()))
		.argument(InputValue::new("ids", TypeRef::named_nn_list(TypeRef::INT)))
		.argument(InputValue::new("parent", i()))
		.argument(InputValue::new("related", i()))
		.argument(InputValue::new("search", s()))
		.argument(InputValue::new("order", s()))
		.argument(InputValue::new("limit", i()))
		.argument(InputValue::new("offset", i()))
		.argument(InputValue::new("where", TypeRef::named_nn_list("Where")))
}

/// A filter on an indexed field: `{ field: "city", value: "Bern" }`, op is
/// one of eq (default), ne, lt, lte, gt, gte.
fn where_input() -> InputObject {
	InputObject::new("Where")
		.field(InputValue::new("field", TypeRef::named_nn(TypeRef::STRING)))
		.field(InputValue::new("op", TypeRef::named(TypeRef::STRING)))
		.field(InputValue::new("value", TypeRef::named_nn(TypeRef::STRING)))
}

fn sites_field(name: &str) -> GqlField {
	GqlField::new(name, TypeRef::named_nn_list_nn("Site"), |ctx| {
		FieldFuture::new(async move {
			let schema = ctx.data::<Arc<Schema>>()?;
			let urls = ctx.data::<SiteUrls>()?;
			let sites = schema.sites.iter().enumerate().map(|(i, s)| {
				let site = serde_json::json!({
					"id": i + 1,
					"handle": s.handle,
					"name": s.name,
					"language": s.language,
					"primary": i == 0,
					"baseUrl": urls.get(&s.handle).cloned().unwrap_or_default(),
				});
				FieldValue::value(GqlValue::from_json(site).unwrap())
			});
			Ok(Some(FieldValue::list(sites)))
		})
	})
}

fn site_object() -> Object {
	let field = |name: &'static str, ty: &'static str| {
		GqlField::new(name, TypeRef::named_nn(ty), move |ctx| {
			FieldFuture::new(async move {
				let v = ctx.parent_value.try_to_value()?;
				let GqlValue::Object(o) = v else {
					return Ok(None);
				};
				Ok(o.get(name).cloned().map(FieldValue::value))
			})
		})
	};
	Object::new("Site")
		.field(field("id", TypeRef::INT))
		.field(field("baseUrl", TypeRef::STRING))
		.field(field("handle", TypeRef::STRING))
		.field(field("name", TypeRef::STRING))
		.field(field("language", TypeRef::STRING))
		.field(field("primary", TypeRef::BOOLEAN))
}

fn params_from_args(
	ctx: &ResolverContext,
) -> async_graphql::Result<Vec<(String, String)>> {
	let mut params = vec![];
	for (name, value) in ctx.args.iter() {
		let name = name.as_str();
		if value.is_null() {
			continue;
		}
		match name {
			"id" => params.push(("ids".into(), value.i64()?.to_string())),
			"ids" => {
				let ids: Vec<String> = value
					.list()?
					.iter()
					.map(|v| v.i64().map(|i| i.to_string()))
					.collect::<async_graphql::Result<_>>()?;
				params.push(("ids".into(), ids.join(",")));
			}
			"where" => {
				for w in value.list()?.iter() {
					let w = w.object()?;
					let field = w.try_get("field")?.string()?;
					let op = match w.get("op") {
						Some(op) if !op.is_null() => op.string()?,
						_ => "eq",
					};
					let v = w.try_get("value")?.string()?;
					params.push((format!("f.{field}.{op}"), v.into()));
				}
			}
			"parent" | "related" | "limit" | "offset" => {
				params.push((name.into(), value.i64()?.to_string()))
			}
			"site" | "section" | "type" | "uri" | "slug" => {
				let Some(v) = first(&value)? else { continue };
				let v = match v.as_str() {
					// Craft's name of the home page
					"__home__" if name == "uri" => String::new(),
					_ => v,
				};
				params.push((name.into(), v));
			}
			// the position of a site, or its handle
			"siteId" => {
				let Some(v) = first(&value)? else { continue };
				let schema = ctx.data::<Arc<Schema>>()?;
				let site = match v.parse::<usize>() {
					Ok(n) => schema.sites.get(n.wrapping_sub(1)),
					Err(_) => schema.get_site(&v),
				};
				let site = site.ok_or_else(|| {
					GqlError::new(format!("unknown site {v}"))
				})?;
				params.push(("site".into(), site.handle.clone()));
			}
			_ => params.push((name.into(), value.string()?.into())),
		}
	}
	Ok(params)
}

/// The first value of a list argument, as text. A single value counts
/// as a list.
fn first(
	value: &async_graphql::dynamic::ValueAccessor,
) -> async_graphql::Result<Option<String>> {
	let v = match value.as_value() {
		GqlValue::List(l) => l.first().cloned(),
		v => Some(v.clone()),
	};
	Ok(match v {
		Some(GqlValue::String(s)) => Some(s),
		Some(GqlValue::Number(n)) => Some(n.to_string()),
		_ => None,
	})
}

/// The position of a site, from 1, like Craft's site ids.
fn site_id(ctx: &ResolverContext, site: &str) -> async_graphql::Result<i64> {
	let schema = ctx.data::<Arc<Schema>>()?;
	let i = schema
		.sites
		.iter()
		.position(|s| s.handle == site)
		.unwrap_or(0);
	Ok(i as i64 + 1)
}

fn entry_url(
	ctx: &ResolverContext,
	site: &str,
	uri: &str,
) -> async_graphql::Result<String> {
	let base = ctx
		.data::<SiteUrls>()?
		.get(site)
		.cloned()
		.unwrap_or_default();
	Ok(format!("{}/{uri}", base.trim_end_matches('/')))
}

fn site_params<const N: usize>(
	site: &str,
	more: [(&str, String); N],
) -> Vec<(String, String)> {
	let mut params = vec![("site".to_string(), site.to_string())];
	params.extend(more.into_iter().map(|(k, v)| (k.to_string(), v)));
	params
}

fn query(
	ctx: &ResolverContext,
	params: &[(String, String)],
) -> async_graphql::Result<Query> {
	let schema = ctx.data::<Arc<Schema>>()?;
	let viewer = ctx.data::<Viewer>()?;
	let mut q = Query::parse(schema, params).map_err(GqlError::new)?;
	q.published_only = viewer.published_only;
	Ok(q)
}

fn load(
	ctx: &ResolverContext,
	params: &[(String, String)],
) -> async_graphql::Result<Vec<Entry>> {
	let q = query(ctx, params)?;
	let db = ctx.data::<Db>()?;
	let (entries, _) = list(&db.conn(), &q)?;
	Ok(entries)
}

async fn graphql(
	State(state): State<AppState>,
	headers: HeaderMap,
	req: GraphQLRequest,
) -> Result<GraphQLResponse> {
	let published_only = session(&state.db.conn(), &headers)?.is_none();
	let req = req.into_inner().data(Viewer { published_only });
	Ok(state.gql.execute(req).await.into())
}

async fn graphiql() -> Html<String> {
	Html(GraphiQLSource::build().endpoint("/api/graphql").finish())
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/graphql", get(graphiql).post(graphql))
}

#[cfg(test)]
mod tests {
	use serde_json::json;

	use super::*;
	use crate::entries::save;
	use crate::entries::tests::input;
	use crate::schema::tests::site;

	async fn run(gql: &GqlSchema, query: &str, published_only: bool) -> Value {
		let req =
			async_graphql::Request::new(query).data(Viewer { published_only });
		let resp = gql.execute(req).await;
		assert!(resp.errors.is_empty(), "{:?}", resp.errors);
		resp.data.into_json().unwrap()
	}

	#[tokio::test]
	async fn loads_a_page_with_its_relations() {
		let db = Db::memory().unwrap();
		let s = Arc::new(site());
		let gql = build(s.clone(), db.clone(), String::new(), SiteUrls::new())
			.unwrap();

		let (about, event) = {
			let conn = db.conn();
			let about = save(
				&conn,
				&s,
				None,
				input("pages", "page", "About", json!({})),
			)
			.unwrap();
			let fields =
				json!({ "city": "Bern", "price": 12, "page": [about.id] });
			let event =
				save(&conn, &s, None, input("events", "event", "Jazz", fields))
					.unwrap();
			let body = json!({ "intro": "Hi", "body": [
				{ "type": "text", "fields": { "content": "Hello" } },
				{ "type": "button", "fields": { "label": "Go", "link": [about.id] } },
			]});
			let mut team = input("pages", "page", "Team", body);
			team.parent_id = Some(about.id);
			save(&conn, &s, None, team).unwrap();
			(about, event)
		};

		let data = run(
			&gql,
			r#"{
				entry(uri: "about/team", site: "en") {
					typeHandle
					title
					parent { title }
					... on page_Entry {
						intro
						body {
							typeHandle
							... on text_Block { content }
							... on button_Block { label link { title uri } }
						}
					}
				}
			}"#,
			true,
		)
		.await;
		assert_eq!(
			data["entry"],
			json!({
				"typeHandle": "page",
				"title": "Team",
				"parent": { "title": "About" },
				"intro": "Hi",
				"body": [
					{ "typeHandle": "text", "content": "Hello" },
					{ "typeHandle": "button", "label": "Go",
						"link": [{ "title": "About", "uri": "about" }] },
				],
			})
		);

		let data = run(
			&gql,
			r#"{
				entries(type: "event", where: [{ field: "price", op: "lt", value: "20" }]) {
					title
					... on event_Entry { city page { id } }
				}
				entryCount(section: "pages")
				about: entry(id: 0) { title }
				sites { handle primary }
			}"#,
			true,
		)
		.await;
		assert_eq!(
			data["entries"],
			json!([{ "title": "Jazz", "city": "Bern", "page": [{ "id": about.id }] }])
		);
		assert_eq!(data["entryCount"], 2);
		assert_eq!(data["about"], Value::Null);
		assert_eq!(
			data["sites"][1],
			json!({ "handle": "en", "primary": false })
		);
		let _ = event;
	}

	#[tokio::test]
	async fn assets_with_transforms() {
		let db = Db::memory().unwrap();
		let s = Arc::new(site());
		let gql = build(
			s.clone(),
			db.clone(),
			"https://cms.test".into(),
			SiteUrls::new(),
		)
		.unwrap();
		let dir = std::env::temp_dir()
			.join(format!("zipp-gql-{}", rand::random::<u32>()));

		let mut png = vec![];
		image::RgbImage::new(400, 200)
			.write_to(
				&mut std::io::Cursor::new(&mut png),
				image::ImageFormat::Png,
			)
			.unwrap();
		{
			let conn = db.conn();
			let a = crate::assets::store(&conn, &dir, "hero.png", &png, None)
				.unwrap();
			let fields = json!({ "city": "Bern", "image": [a.id] });
			save(&conn, &s, None, input("events", "event", "Jazz", fields))
				.unwrap();
		}

		let data = run(
			&gql,
			r#"{ entry(uri: "events/jazz") { ... on event_Entry { image {
				url
				small: url(transform: "small")
				width(transform: "small")
				height(transform: "small")
				sqHeight: height(transform: "square")
				srcset(transforms: ["small", "square"])
			} } } }"#,
			true,
		)
		.await;
		let img = &data["entry"]["image"][0];
		assert_eq!(img["url"], "https://cms.test/assets/1/hero.png");
		assert_eq!(img["small"], "https://cms.test/assets/1/small/hero.png");
		assert_eq!(
			(img["width"].clone(), img["height"].clone()),
			(json!(100), json!(50))
		);
		assert_eq!(img["sqHeight"], 50);
		assert_eq!(
			img["srcset"],
			"https://cms.test/assets/1/small/hero.png 100w, \
			https://cms.test/assets/1/square/hero.png 50w"
		);
		std::fs::remove_dir_all(dir).unwrap();
	}

	#[tokio::test]
	async fn answers_crelte_like_craft() {
		let db = Db::memory().unwrap();
		let s = Arc::new(site());
		let urls = SiteUrls::from([
			("de".to_string(), "https://x.ch".to_string()),
			("en".to_string(), "https://x.ch/en/".to_string()),
		]);
		let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
		let first = crate::migrate::Migration {
			name: "0001".into(),
			ops: crate::migrate::diff(&Schema::new(), &s, &[]).unwrap(),
			schema: (*s).clone(),
		};
		crate::apply::apply_migrations(&db.conn(), &[first]).unwrap();

		let data = run(
			&gql,
			r#"{ crelteSites { id baseUrl language name handle primary } }"#,
			true,
		)
		.await;
		assert_eq!(
			data["crelteSites"][1],
			json!({ "id": 2, "baseUrl": "https://x.ch/en/", "language": "en",
				"name": "English", "handle": "en", "primary": false })
		);

		// crelte's entry.graphql with its variables
		let req = async_graphql::Request::new(
			r#"query ($uri: [String], $siteId: [QueryArgument]) {
				entry(uri: $uri, siteId: $siteId) {
					id siteId url sectionHandle typeHandle title
				}
			}"#,
		)
		.variables(async_graphql::Variables::from_json(
			json!({ "uri": "__home__", "siteId": 2 }),
		))
		.data(Viewer {
			published_only: true,
		});
		let resp = gql.execute(req).await;
		assert!(resp.errors.is_empty(), "{:?}", resp.errors);
		let data = resp.data.into_json().unwrap();
		assert_eq!(
			data["entry"],
			json!({ "id": 1, "siteId": 2, "url": "https://x.ch/en/",
				"sectionHandle": "home", "typeHandle": "page", "title": "Home" })
		);
	}

	#[tokio::test]
	async fn hides_drafts_without_a_session() {
		let db = Db::memory().unwrap();
		let s = Arc::new(site());
		let gql = build(s.clone(), db.clone(), String::new(), SiteUrls::new())
			.unwrap();
		let mut draft = input("pages", "page", "Draft", json!({}));
		draft.published = false;
		save(&db.conn(), &s, None, draft).unwrap();

		let q = r#"{ entry(uri: "draft") { title } }"#;
		assert_eq!(run(&gql, q, true).await["entry"], Value::Null);
		assert_eq!(run(&gql, q, false).await["entry"]["title"], "Draft");
	}
}
