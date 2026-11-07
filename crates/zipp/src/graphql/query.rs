use super::*;
use crate::entries::query::relation_field;

/// The arguments of `entry`, `entries` and `entryCount`, the same as the
/// rest api (see query.rs).
pub(super) fn entries_args(field: GqlField, schema: &Schema) -> GqlField {
	query_args(schema)
		.into_iter()
		.fold(field, |f, (name, ty)| f.argument(InputValue::new(name, ty)))
}

/// The arguments of `children`: the ones about the children themselves.
pub(super) fn children_args(schema: &Schema) -> Vec<(String, TypeRef)> {
	let not = [
		"site", "siteId", "section", "uri", "parent", "level", "search",
	];
	let args = query_args(schema).into_iter();
	args.filter(|(n, _)| !not.contains(&n.as_str())).collect()
}

/// The criteria of `relatedToEntries`, like Craft's: the arguments of
/// `entries` without site, order and page, in the site of the query.
pub(super) fn criteria_input(schema: &Schema) -> InputObject {
	let not = ["site", "siteId", "order", "orderBy", "limit", "offset"];
	let args = query_args(schema).into_iter();
	args.filter(|(n, _)| !not.contains(&n.as_str()))
		.fold(InputObject::new("EntryCriteria"), |obj, (name, ty)| {
			obj.field(InputValue::new(name, ty))
		})
}

fn query_args(schema: &Schema) -> Vec<(String, TypeRef)> {
	let mut args: Vec<(String, TypeRef)> = TAKEN
		.iter()
		.map(|&n| (n.to_string(), arg_type(n)))
		.collect();
	// the relation fields like Craft's: ids, `:notempty:` or `:empty:`
	for handle in relation_args(schema) {
		args.push((handle, TypeRef::named_list("QueryArgument")));
	}
	args
}

/// The type of an argument of [`TAKEN`].
fn arg_type(name: &str) -> TypeRef {
	let s = || TypeRef::named(TypeRef::STRING);
	// lists like in Craft, so crelte's queries fit
	let l = || TypeRef::named_list(TypeRef::STRING);
	let i = || TypeRef::named(TypeRef::INT);
	let q = || TypeRef::named_list("QueryArgument");
	match name {
		// section, type: one, several or `["not", "news"]`
		"site" | "section" | "type" | "uri" | "slug" => l(),
		"siteId" => q(),
		// Craft's: an id, a list of them, or `["not", 1, 2]`
		"id" => q(),
		"ids" => TypeRef::named_nn_list(TypeRef::INT),
		// Craft's: one or several users
		"authorId" => q(),
		"parent" | "level" | "related" => i(),
		// Craft's: `[1, 2]` either, `["and", 1, 2]` both
		"relatedTo" => q(),
		// each of the list, like Craft
		"relatedToEntries" => TypeRef::named_nn_list("EntryCriteria"),
		// orderBy: Craft's `postDate DESC`
		"search" | "order" | "orderBy" => s(),
		"limit" | "offset" => i(),
		"where" => TypeRef::named_nn_list("Where"),
		_ => unreachable!("{name} is no argument of the entry queries"),
	}
}

/// The arguments of the entry queries, a relation field of the same name
/// isn't one.
pub(super) const TAKEN: &[&str] = &[
	"site",
	"siteId",
	"section",
	"type",
	"uri",
	"slug",
	"id",
	"ids",
	"authorId",
	"parent",
	"level",
	"related",
	"relatedTo",
	"relatedToEntries",
	"search",
	"order",
	"orderBy",
	"limit",
	"offset",
	"where",
];

/// The relation fields of entry types, unless an argument has the name.
fn relation_args(schema: &Schema) -> std::collections::BTreeSet<String> {
	schema
		.components
		.iter()
		.filter(|c| c.entry)
		.flat_map(|c| &c.fields)
		.filter(|f| matches!(f.kind, FieldKind::Relation { .. }))
		.map(|f| f.handle.clone())
		.filter(|h| !TAKEN.contains(&h.as_str()))
		.collect()
}

/// A filter on an indexed field: `{ field: "city", value: "Bern" }`, op is
/// one of eq (default), ne, lt, lte, gt, gte.
pub(super) fn where_input() -> InputObject {
	InputObject::new("Where")
		.field(InputValue::new("field", TypeRef::named_nn(TypeRef::STRING)))
		.field(InputValue::new("op", TypeRef::named(TypeRef::STRING)))
		.field(InputValue::new("value", TypeRef::named_nn(TypeRef::STRING)))
}

pub(super) fn sites_field(name: &str) -> GqlField {
	GqlField::new(name, TypeRef::named_nn_list_nn("Site"), |ctx| {
		FieldFuture::new(async move {
			let schema = ctx.data::<Arc<Schema>>()?;
			let urls = ctx.data::<SiteUrls>()?;
			// offline sites only for previews and admins, the ids stay;
			// the ones a token reads
			let viewer = ctx.data::<Viewer>()?;
			let all = !hides_offline(viewer);
			let sites = schema.sites.iter().enumerate();
			let sites = sites.filter(|(_, s)| {
				(all || !schema.is_offline(&s.handle))
					&& viewer.scope.site(&s.handle)
			});
			let sites = sites.map(|(i, s)| {
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

pub(super) fn site_object() -> Object {
	let field = |name: &'static str, ty: &'static str| {
		GqlField::new(name, TypeRef::named_nn(ty), move |ctx| {
			value_field(ctx, name)
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

pub(super) fn params_from_args(
	ctx: &ResolverContext,
) -> async_graphql::Result<Vec<(String, String)>> {
	let schema = ctx.data::<Arc<Schema>>()?;
	params_of(schema, ctx.args.iter())
}

/// The parameters of the rest api of some arguments, also of the
/// criteria of `relatedToEntries`.
fn params_of<'a>(
	schema: &Schema,
	args: impl Iterator<
		Item = (
			&'a async_graphql::Name,
			async_graphql::dynamic::ValueAccessor<'a>,
		),
	>,
) -> async_graphql::Result<Vec<(String, String)>> {
	let mut params = vec![];
	for (name, value) in args {
		let name = name.as_str();
		if value.is_null() {
			continue;
		}
		match name {
			"id" => {
				let items = all(&value)?;
				match items.split_first() {
					Some((not, rest)) if not == "not" => {
						params.push(("notIds".into(), rest.join(",")))
					}
					_ => params.push(("ids".into(), items.join(","))),
				}
			}
			// one or several, or Craft's `["not", "news"]`
			"section" | "type" => {
				let items = all(&value)?;
				let not = if name == "type" {
					"notTypes"
				} else {
					"notSections"
				};
				match items.split_first() {
					Some((n, rest)) if n == "not" => {
						params.push((not.into(), rest.join(",")))
					}
					_ => params.push((name.into(), items.join(","))),
				}
			}
			// either of the list, `["and", ..]` each of them
			"relatedTo" => {
				let items = all(&value)?;
				match items.split_first() {
					Some((and, rest)) if and == "and" => {
						for id in rest {
							params.push((name.into(), id.clone()));
						}
					}
					Some((or, rest)) if or == "or" => {
						params.push((name.into(), rest.join(",")))
					}
					_ => params.push((name.into(), items.join(","))),
				}
			}
			"authorId" => {
				params.push(("author".into(), all(&value)?.join(",")))
			}
			"relatedToEntries" => {
				for criteria in value.list()?.iter() {
					let criteria =
						params_of(schema, criteria.object()?.iter())?;
					let criteria = serde_urlencoded::to_string(&criteria)?;
					params.push((name.into(), criteria));
				}
			}
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
			// full text, like Craft's
			"search" => params.push(("text".into(), value.string()?.into())),
			"parent" | "level" | "related" | "limit" | "offset" => {
				params.push((name.into(), value.i64()?.to_string()))
			}
			"site" | "uri" | "slug" | "handle" => {
				let Some(v) = first(&value)? else { continue };
				let v = match v.as_str() {
					// Craft's name of the home page
					"__home__" if name == "uri" => String::new(),
					_ => v,
				};
				params.push((name.into(), v));
			}
			_ if relation_field(schema, name) => {
				params.push((format!("r.{name}"), all(&value)?.join(",")));
			}
			// the position of a site, or its handle
			"siteId" => {
				let Some(v) = first(&value)? else { continue };
				params.push(("site".into(), site_of(schema, &v)?));
			}
			_ => params.push((name.into(), value.string()?.into())),
		}
	}
	Ok(params)
}

/// The site of the `site` or `siteId` argument, else the primary site.
pub(super) fn site_arg(ctx: &ResolverContext) -> async_graphql::Result<String> {
	let schema = ctx.data::<Arc<Schema>>()?;
	let arg = |k: &str| ctx.args.get(k).filter(|v| !v.is_null());
	if let Some(v) = arg("site").map(|v| first(&v)).transpose()?.flatten() {
		return match schema.get_site(&v) {
			Some(_) => Ok(v),
			None => Err(GqlError::new(format!("unknown site {v}"))),
		};
	}
	if let Some(v) = arg("siteId").map(|v| first(&v)).transpose()?.flatten() {
		return site_of(schema, &v);
	}
	Ok(schema.primary_site().handle.clone())
}

/// The handle of a site of `siteId`: its position or its handle.
fn site_of(schema: &Schema, v: &str) -> async_graphql::Result<String> {
	let site = match v.parse::<i64>() {
		Ok(n) => schema.site_by_id(n),
		Err(_) => schema.get_site(v),
	};
	let site =
		site.ok_or_else(|| GqlError::new(format!("unknown site {v}")))?;
	Ok(site.handle.clone())
}

/// The first value of a list argument, as text. A single value counts
/// as a list.
pub(super) fn first(
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

/// Every value of a list argument, or the single one, as strings.
pub(super) fn all(
	value: &async_graphql::dynamic::ValueAccessor,
) -> async_graphql::Result<Vec<String>> {
	let items = match value.as_value() {
		GqlValue::List(l) => l.clone(),
		v => vec![v.clone()],
	};
	Ok(items
		.into_iter()
		.filter_map(|v| match v {
			GqlValue::String(s) => Some(s),
			GqlValue::Number(n) => Some(n.to_string()),
			_ => None,
		})
		.collect())
}

/// The position of a site, from 1, like Craft's site ids.
pub(super) fn site_id(
	ctx: &ResolverContext,
	site: &str,
) -> async_graphql::Result<i64> {
	let schema = ctx.data::<Arc<Schema>>()?;
	Ok(schema.site_id(site).unwrap_or(1))
}

pub(super) fn site_params<const N: usize>(
	site: &str,
	more: [(&str, String); N],
) -> Vec<(String, String)> {
	let mut params = vec![("site".to_string(), site.to_string())];
	params.extend(more.into_iter().map(|(k, v)| (k.to_string(), v)));
	params
}

/// The root: entries like Craft's, globals and the sites.
pub(super) fn query_object(schema: &Schema) -> Object {
	Object::new("Query")
		.field(entries_args(
			GqlField::new("entry", TypeRef::named("Entry"), |ctx| {
				FieldFuture::new(async move {
					let mut params = params_from_args(&ctx)?;
					params.push(("limit".into(), "1".into()));
					let e = load(&ctx, &params)?.into_iter().next();
					Ok(e.map(entry_value))
				})
			}),
			schema,
		))
		.field(entries_args(
			GqlField::new(
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
			),
			schema,
		))
		.field(entries_args(
			GqlField::new(
				"entryCount",
				TypeRef::named_nn(TypeRef::INT),
				|ctx| {
					FieldFuture::new(async move {
						let params = params_from_args(&ctx)?;
						let q = query(&ctx, &params)?;
						let db = ctx.data::<Db>()?;
						spend(&ctx, 1)?;
						let total = count(&db.conn(), &q)?;
						Ok(Some(GqlValue::from(total)))
					})
				},
			),
			schema,
		))
		// Craft's globals are singles here: the entry of the section
		.field(
			GqlField::new("globalSet", TypeRef::named("Entry"), |ctx| {
				FieldFuture::new(async move {
					let mut params = params_from_args(&ctx)?;
					for p in &mut params {
						if p.0 == "handle" {
							p.0 = "section".into();
						}
					}
					params.push(("limit".into(), "1".into()));
					let e = load(&ctx, &params)?.into_iter().next();
					Ok(e.map(entry_value))
				})
			})
			.argument(InputValue::new(
				"handle",
				TypeRef::named_list(TypeRef::STRING),
			))
			.argument(InputValue::new(
				"site",
				TypeRef::named_list(TypeRef::STRING),
			))
			.argument(InputValue::new(
				"siteId",
				TypeRef::named_list("QueryArgument"),
			)),
		)
		.field(sites_field("sites"))
		// what crelte asks for on start
		.field(sites_field("crelteSites"))
}

/// Whether offline sites are hidden: from frontends, not from previews
/// and admins (which entries a preview sees, see [`query`]).
pub(super) fn hides_offline(viewer: &Viewer) -> bool {
	viewer.published_only && viewer.preview.is_none()
}
