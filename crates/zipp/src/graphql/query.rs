use super::*;

/// The arguments of `entry`, `entries` and `entryCount`, the same as the
/// rest api (see query.rs).
pub(super) fn entries_args(field: GqlField) -> GqlField {
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
		// Craft's: an id, a list of them, or `["not", 1, 2]`
		.argument(InputValue::new("id", TypeRef::named_list("QueryArgument")))
		.argument(InputValue::new("ids", TypeRef::named_nn_list(TypeRef::INT)))
		.argument(InputValue::new("parent", i()))
		.argument(InputValue::new("related", i()))
		.argument(InputValue::new("search", s()))
		.argument(InputValue::new("order", s()))
		// Craft's `postDate DESC`
		.argument(InputValue::new("orderBy", s()))
		.argument(InputValue::new("limit", i()))
		.argument(InputValue::new("offset", i()))
		.argument(InputValue::new("where", TypeRef::named_nn_list("Where")))
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
			// offline sites only for previews and admins, the ids stay
			let all = !hides_offline(ctx.data::<Viewer>()?);
			let sites = schema.sites.iter().enumerate();
			let sites =
				sites.filter(|(_, s)| all || !schema.is_offline(&s.handle));
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

pub(super) fn params_from_args(
	ctx: &ResolverContext,
) -> async_graphql::Result<Vec<(String, String)>> {
	let mut params = vec![];
	for (name, value) in ctx.args.iter() {
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
			// several types
			"type" => params.push(("type".into(), all(&value)?.join(","))),
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
			"site" | "section" | "uri" | "slug" | "handle" => {
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
	let i = schema
		.sites
		.iter()
		.position(|s| s.handle == site)
		.unwrap_or(0);
	Ok(i as i64 + 1)
}

pub(super) fn site_params<const N: usize>(
	site: &str,
	more: [(&str, String); N],
) -> Vec<(String, String)> {
	let mut params = vec![("site".to_string(), site.to_string())];
	params.extend(more.into_iter().map(|(k, v)| (k.to_string(), v)));
	params
}

pub(super) fn query(
	ctx: &ResolverContext,
	params: &[(String, String)],
) -> async_graphql::Result<Query> {
	let schema = ctx.data::<Arc<Schema>>()?;
	let viewer = ctx.data::<Viewer>()?;
	let mut q = Query::parse(schema, params).map_err(GqlError::new)?;
	q.published_only = viewer.published_only;
	match &viewer.preview {
		Some(Preview::Entry(id, _)) => q.preview_id = Some(*id),
		Some(Preview::All) => q.preview_all = true,
		None => {}
	}
	// a frontend gets nothing of an offline site, counts neither
	q.nothing = schema.is_offline(&q.site) && hides_offline(viewer);
	Ok(q)
}

pub(super) fn load(
	ctx: &ResolverContext,
	params: &[(String, String)],
) -> async_graphql::Result<Vec<Entry>> {
	let q = query(ctx, params)?;
	let db = ctx.data::<Db>()?;
	let (mut entries, _) = list(&db.conn(), &q)?;

	// the entry of a preview shows its draft, or every entry its own
	match &ctx.data::<Viewer>()?.preview {
		Some(Preview::Entry(id, doc)) => {
			for e in entries.iter_mut().filter(|e| e.id == *id) {
				if let Some(sd) = doc.get(&e.site) {
					e.title = sd.title.clone();
					e.fields = sd.fields.clone();
				}
			}
		}
		Some(Preview::All) => {
			let conn = db.conn();
			for e in entries.iter_mut() {
				let draft = get_draft(&conn, e.id)?;
				if let Some(sd) =
					draft.as_ref().and_then(|d| d.doc.get(&e.site))
				{
					e.title = sd.title.clone();
					e.fields = sd.fields.clone();
				}
			}
		}
		None => {}
	}
	Ok(entries)
}

/// The root: entries like Craft's, globals and the sites.
pub(super) fn query_object() -> Object {
	Object::new("Query")
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
/// and admins.
pub(super) fn hides_offline(viewer: &Viewer) -> bool {
	viewer.published_only && viewer.preview.is_none()
}
