use serde_json::Value;
use zipp::assets::get_asset;
use zipp::async_graphql::dynamic::{
	Field as GqlField, FieldFuture, FieldValue, InputValue, Object,
	ResolverContext, TypeRef,
};
use zipp::async_graphql::{Error as GqlError, Value as GqlValue};
use zipp::entries::Entry;
use zipp::graphql::ext;

use crate::SEO_SETTINGS;
use crate::resolve::{self as seo, EntryPage, Page};

/// The scalar of `seoJson`, named like the Craft plugin's.
pub(crate) const SEO_JSON: &str = "MeierLabsSeoJson";

/// The SEO of an entry, or of the site without one, as `seoJson`. Loads
/// what it needs with the viewer's rights, so a preview shows drafts.
pub(crate) fn resolve_seo(
	ctx: &ResolverContext,
	site: &str,
	entry: Option<&Entry>,
) -> zipp::async_graphql::Result<Value> {
	let schema = ext::schema(ctx)?;
	let urls = ext::site_urls(ctx)?;
	let db = ext::db(ctx)?;
	let primary = schema.primary_site().handle.clone();
	let site_def = schema
		.get_site(site)
		.ok_or_else(|| GqlError::new(format!("unknown site {site}")))?;

	let one = |s: &str, params: [(&str, String); 1]| {
		ext::entries(ctx, s, params).map(|l| l.into_iter().next())
	};
	let settings = |s: &str| -> zipp::async_graphql::Result<Value> {
		let e = one(s, [("section", SEO_SETTINGS.into())])?;
		Ok(e.map(|e| e.fields).unwrap_or_default())
	};
	let globals =
		seo::globals(site_def, &settings(site)?, &settings(&primary)?);
	let url_of = |s: &str, e: &Entry| {
		e.uri.as_deref().and_then(|u| zipp::page_url(urls, s, u))
	};

	let entry_page = match entry {
		Some(e) => {
			// the entry in every site, worth it with another language
			let mut alternates = vec![];
			for s in &schema.sites {
				let Some(other) = one(&s.handle, [("ids", e.id.to_string())])?
				else {
					continue;
				};
				if let Some(u) = url_of(&s.handle, &other) {
					alternates.push((s.language.clone(), u.clone()));
					if s.handle == primary {
						alternates.push(("x-default".into(), u));
					}
				}
			}
			if alternates.len() <= 2 {
				alternates.clear();
			}

			// the home page, the parents in a structure, the entry
			let mut breadcrumbs = vec![];
			if e.uri.as_deref().is_some_and(|u| !u.is_empty()) {
				if let Some(home) = one(site, [("uri", String::new())])?
					&& let Some(u) = url_of(site, &home)
				{
					breadcrumbs.push((home.title.clone(), u));
				}
				let mut parents = vec![];
				let mut parent = e.parent_id;
				// a parent the viewer doesn't see is left out, not its parents
				while let Some(id) = parent {
					let raw = zipp::entries::get_entry(&db.conn(), id, site);
					parent = raw.ok().and_then(|p| p.parent_id);
					parents.extend(one(site, [("ids", id.to_string())])?);
				}
				for p in parents.iter().rev() {
					if let Some(u) =
						url_of(site, p).filter(|_| p.uri.as_deref() != Some(""))
					{
						breadcrumbs.push((p.title.clone(), u));
					}
				}
				if let Some(u) = url_of(site, e) {
					breadcrumbs.push((e.title.clone(), u));
				}
			}

			Some(EntryPage {
				entry: e,
				url: url_of(site, e),
				alternates,
				breadcrumbs,
			})
		}
		None => None,
	};

	let asset = |id: i64| get_asset(&db.conn(), id).ok();
	let page = Page {
		schema,
		site: site_def,
		site_url: urls.get(site).cloned().unwrap_or_default(),
		public_url: ext::public_url(ctx)?,
		globals,
		entry: entry_page,
		asset: &asset,
		indexing: ext::indexing(ctx)?,
	};
	Ok(seo::resolve(&page))
}

/// The SEO of the entry that is the parent.
pub(crate) fn entry_seo(
	ctx: &ResolverContext,
) -> zipp::async_graphql::Result<Option<FieldValue<'static>>> {
	let e = ctx.parent_value.try_downcast_ref::<Entry>()?;
	let v = resolve_seo(ctx, &e.site, Some(e))?;
	Ok(Some(FieldValue::value(GqlValue::from_json(v)?)))
}

/// `seoJson` of an entry, everything as one JSON object so the npm
/// package gets new values without a new query, or `seo { ... }`, the same
/// as a type.
pub(crate) fn entry_seo_field(name: &str, ty: &str) -> GqlField {
	GqlField::new(name, TypeRef::named(ty), |ctx| {
		FieldFuture::new(async move { entry_seo(&ctx) })
	})
}

/// The root `seo(siteId: Int, site: String)` and `seoJson(...)` for pages
/// without an entry.
pub(crate) fn root_fields() -> [GqlField; 2] {
	let field = |name: &'static str, ty: TypeRef| {
		GqlField::new(name, ty, |ctx| {
			FieldFuture::new(async move {
				let schema = ext::schema(&ctx)?;
				let by_id = ctx
					.args
					.get("siteId")
					.and_then(|v| v.i64().ok())
					.and_then(|i| schema.sites.get((i - 1).max(0) as usize))
					.map(|s| s.handle.clone());
				let by_handle = ctx
					.args
					.get("site")
					.and_then(|v| v.string().ok().map(String::from));
				let site = by_handle
					.or(by_id)
					.unwrap_or_else(|| schema.primary_site().handle.clone());
				let v = resolve_seo(&ctx, &site, None)?;
				Ok(Some(FieldValue::value(GqlValue::from_json(v)?)))
			})
		})
		.argument(InputValue::new("siteId", TypeRef::named(TypeRef::INT)))
		.argument(InputValue::new("site", TypeRef::named(TypeRef::STRING)))
	};
	[
		field("seo", TypeRef::named("Seo")),
		field("seoJson", TypeRef::named(SEO_JSON)),
	]
}

/// The types: `Seo` and its parts, like the plugin's.
pub(crate) fn seo_types() -> Vec<Object> {
	let obj = |name: &str, fields: &[(&'static str, TypeRef)]| {
		let mut o = Object::new(name);
		for (f, ty) in fields {
			let f: &'static str = f;
			o = o.field(GqlField::new(f, ty.clone(), move |ctx| {
				ext::value_field(ctx, f)
			}));
		}
		o
	};
	let s = || TypeRef::named(TypeRef::STRING);
	let i = || TypeRef::named(TypeRef::INT);
	vec![
		obj(
			"Seo",
			&[
				("title", s()),
				("description", s()),
				("canonical", s()),
				("robots", s()),
				("siteName", s()),
				("locale", s()),
				("image", TypeRef::named("SeoImage")),
				("socialTitle", s()),
				("socialDescription", s()),
				("socialImage", TypeRef::named("SeoImage")),
				("alternates", TypeRef::named_nn_list_nn("SeoAlternate")),
				("meta", TypeRef::named_nn_list_nn("SeoMeta")),
				("links", TypeRef::named_nn_list_nn("SeoLink")),
				("jsonLd", s()),
			],
		),
		obj(
			"SeoImage",
			&[
				("url", s()),
				("width", i()),
				("height", i()),
				("alt", s()),
				("mimeType", s()),
			],
		),
		obj("SeoAlternate", &[("hreflang", s()), ("url", s())]),
		obj(
			"SeoMeta",
			&[("name", s()), ("property", s()), ("content", s())],
		),
		obj("SeoLink", &[("rel", s()), ("href", s()), ("hreflang", s())]),
	]
}
