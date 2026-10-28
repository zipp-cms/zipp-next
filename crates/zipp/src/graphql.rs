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
	Interface, InterfaceField, Object, ResolverContext, Schema as GqlSchema,
	TypeRef,
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
		.register(site_object());

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
		.field(GqlField::new(
			"sites",
			TypeRef::named_nn_list_nn("Site"),
			|ctx| {
				FieldFuture::new(async move {
					let schema = ctx.data::<Arc<Schema>>()?;
					let sites =
						schema.sites.iter().enumerate().map(|(i, s)| {
							FieldValue::value(
								GqlValue::from_json(serde_json::json!({
									"handle": s.handle,
									"name": s.name,
									"language": s.language,
									"primary": i == 0,
								}))
								.unwrap(),
							)
						});
					Ok(Some(FieldValue::list(sites)))
				})
			},
		));

	builder
		.register(query)
		.data(schema)
		.data(db)
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
	let i = || TypeRef::named(TypeRef::INT);
	field
		.argument(InputValue::new("site", s()))
		.argument(InputValue::new("section", s()))
		.argument(InputValue::new("type", s()))
		.argument(InputValue::new("uri", s()))
		.argument(InputValue::new("slug", s()))
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
			_ => params.push((name.into(), value.string()?.into())),
		}
	}
	Ok(params)
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
		let gql = build(s.clone(), db.clone()).unwrap();

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
	async fn hides_drafts_without_a_session() {
		let db = Db::memory().unwrap();
		let s = Arc::new(site());
		let gql = build(s.clone(), db.clone()).unwrap();
		let mut draft = input("pages", "page", "Draft", json!({}));
		draft.published = false;
		save(&db.conn(), &s, None, draft).unwrap();

		let q = r#"{ entry(uri: "draft") { title } }"#;
		assert_eq!(run(&gql, q, true).await["entry"], Value::Null);
		assert_eq!(run(&gql, q, false).await["entry"]["title"], "Draft");
	}
}
