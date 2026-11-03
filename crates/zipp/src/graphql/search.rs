use super::*;
use crate::search::{CLOSE, Hit, OPEN, hits};

/// The arguments of `search` and `searchCount`: the words, where and
/// which page of the hits.
fn search_args(field: GqlField) -> GqlField {
	let l = || TypeRef::named_list(TypeRef::STRING);
	let i = || TypeRef::named(TypeRef::INT);
	field
		.argument(InputValue::new("query", TypeRef::named_nn(TypeRef::STRING)))
		.argument(InputValue::new("site", l()))
		.argument(InputValue::new(
			"siteId",
			TypeRef::named_list("QueryArgument"),
		))
		.argument(InputValue::new("section", l()))
		.argument(InputValue::new("type", l()))
		.argument(InputValue::new("limit", i()))
		.argument(InputValue::new("offset", i()))
}

/// The query of a search: the pages of a site the viewer may see with
/// the words, like `entries(search:)` but only entries with a url, the
/// results link to them.
fn search_query(ctx: &ResolverContext) -> async_graphql::Result<Query> {
	let mut params = params_from_args(ctx)?;
	for p in &mut params {
		if p.0 == "query" {
			p.0 = "text".into();
		}
	}
	let mut q = query(ctx, &params)?;
	q.with_uri = true;
	Ok(q)
}

/// The website's search, over the live texts of the entries (see
/// `search/`): `search(query: "konzert", siteId: $siteId) { excerpt
/// entry { title url } }`, the best match first, and `searchCount` with
/// the same arguments for the pages.
pub(super) fn search_fields() -> [GqlField; 2] {
	let search = GqlField::new(
		"search",
		TypeRef::named_nn_list_nn("SearchHit"),
		|ctx| {
			FieldFuture::new(async move {
				let q = search_query(&ctx)?;
				let db = ctx.data::<Db>()?;
				spend(&ctx, 1)?;
				let conn = db.conn();
				let found = hits(&conn, &q)?;
				spend(&ctx, found.len())?;

				// a preview shows the drafts of the hits
				let (entries, rest): (Vec<_>, Vec<_>) = found
					.into_iter()
					.map(|h| (h.entry, (h.excerpt, h.score)))
					.unzip();
				let entries =
					with_drafts(&conn, ctx.data::<Viewer>()?, entries)?;
				let found = entries.into_iter().zip(rest).map(
					|(entry, (excerpt, score))| {
						FieldValue::owned_any(Hit {
							entry,
							excerpt,
							score,
						})
					},
				);
				Ok(Some(FieldValue::list(found)))
			})
		},
	);
	let count =
		GqlField::new("searchCount", TypeRef::named_nn(TypeRef::INT), |ctx| {
			FieldFuture::new(async move {
				let q = search_query(&ctx)?;
				let db = ctx.data::<Db>()?;
				spend(&ctx, 1)?;
				let total = count(&db.conn(), &q)?;
				Ok(Some(GqlValue::from(total)))
			})
		});
	[search_args(search), search_args(count)]
}

/// A hit of the search: the entry, the words around the match and how
/// well it matches.
pub(super) fn search_hit_object() -> Object {
	let hit = |ctx: &ResolverContext| {
		ctx.parent_value.try_downcast_ref::<Hit>().cloned()
	};
	Object::new("SearchHit")
		.field(GqlField::new(
			"entry",
			TypeRef::named_nn("Entry"),
			move |ctx| {
				FieldFuture::new(async move {
					Ok(Some(entry_value(hit(&ctx)?.entry)))
				})
			},
		))
		// plain text
		.field(GqlField::new(
			"excerpt",
			TypeRef::named(TypeRef::STRING),
			move |ctx| {
				FieldFuture::new(async move {
					let excerpt = hit(&ctx)?
						.excerpt
						.map(|e| e.replace([OPEN, CLOSE], ""));
					Ok(excerpt.map(GqlValue::from))
				})
			},
		))
		// the matched words in `<mark>`
		.field(GqlField::new(
			"excerptHtml",
			TypeRef::named(TypeRef::STRING),
			move |ctx| {
				FieldFuture::new(async move {
					let excerpt = hit(&ctx)?.excerpt.map(|e| {
						richtext::escape(&e)
							.replace(OPEN, "<mark>")
							.replace(CLOSE, "</mark>")
					});
					Ok(excerpt.map(GqlValue::from))
				})
			},
		))
		.field(GqlField::new(
			"score",
			TypeRef::named_nn(TypeRef::FLOAT),
			move |ctx| {
				FieldFuture::new(async move {
					Ok(Some(GqlValue::from(hit(&ctx)?.score)))
				})
			},
		))
}
