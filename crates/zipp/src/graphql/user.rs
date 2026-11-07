//! Users in GraphQL, like Craft's users query: the public profiles of
//! the users with live entries (see `users/profile.rs`), for tokens that
//! read users. An entry's author is a `User` too; without the scope or
//! when the author isn't public only with id and name.

use super::*;
use crate::entries::query::MAX_LIMIT;
use crate::users::profile::{self, Profile, PublicQuery};

/// A user as the parent of their fields, with the site they are in;
/// `public` false has only id and name.
pub(super) struct GqlUser {
	pub profile: Profile,
	pub site: String,
	pub public: bool,
}

impl GqlUser {
	/// An author who is not public (or a viewer without the scope).
	pub(super) fn named(id: i64, name: String, site: &str) -> Self {
		Self {
			profile: Profile {
				id,
				name,
				slug: String::new(),
				fields: Value::Null,
			},
			site: site.into(),
			public: false,
		}
	}
}

pub(super) fn user_value(user: GqlUser) -> FieldValue<'static> {
	FieldValue::owned_any(user)
}

/// The public users of a site the viewer may see, `None`: none at all
/// (no users scope, a site it doesn't read). Users are public by their
/// live entries in the section of the user pages, else in any section
/// the viewer reads.
pub(super) fn public_query(
	schema: &Schema,
	viewer: &Viewer,
	site: &str,
) -> Option<PublicQuery> {
	let scope = &viewer.scope;
	let offline = hides_offline(viewer) && schema.is_offline(site);
	if !scope.users || !scope.site(site) || offline {
		return None;
	}
	let pages = schema.user_pages.as_ref();
	let public = match pages.and_then(|p| p.section.clone()) {
		Some(s) => Some(vec![s]),
		None => scope.sections.clone(),
	};
	let public = public.map(|mut l| {
		l.retain(|s| scope.section(s));
		l
	});
	Some(PublicQuery {
		site: site.into(),
		public,
		..Default::default()
	})
}

/// `User`: id and name of everyone GraphQL shows, the slug, the page and
/// the user fields of public users.
pub(super) fn user_object(schema: &Schema) -> Object {
	let field = |name: &'static str, ty: TypeRef| {
		GqlField::new(name, ty, move |ctx| {
			FieldFuture::new(async move {
				let u = ctx.parent_value.try_downcast_ref::<GqlUser>()?;
				let p = &u.profile;
				let schema = ctx.data::<Arc<Schema>>()?;
				let uri = match (&schema.user_pages, u.public) {
					(Some(pages), true) => {
						Some(pages.render_uri(&u.site, &p.slug))
					}
					_ => None,
				};
				let v = match name {
					"id" => GqlValue::from(p.id.to_string()),
					"name" => GqlValue::from(p.name.clone()),
					"slug" if u.public => GqlValue::from(p.slug.clone()),
					"uri" => uri.map(GqlValue::from).unwrap_or_default(),
					"url" => match uri {
						Some(uri) => {
							GqlValue::from(entry_url(&ctx, &u.site, &uri)?)
						}
						None => GqlValue::Null,
					},
					_ => GqlValue::Null,
				};
				Ok(Some(FieldValue::value(v)))
			})
		})
	};
	let mut obj = Object::new("User")
		.description("A user: id and name, of public users their profile")
		.field(field("id", TypeRef::named_nn(TypeRef::ID)))
		.field(field("name", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("slug", TypeRef::named(TypeRef::STRING)))
		.field(field("uri", TypeRef::named(TypeRef::STRING)))
		.field(field("url", TypeRef::named(TypeRef::STRING)));
	if let Some(comp) = schema.get(USER) {
		for field in &comp.fields {
			if let Some(f) = content_field(schema, USER, field) {
				obj = obj.field(f);
			}
		}
	}
	obj
}

/// `hasEntries: { section: "recipes" }`: only the authors of live
/// entries of the section.
pub(super) fn has_entries_input() -> InputObject {
	InputObject::new("UserEntriesCriteria").field(InputValue::new(
		"section",
		TypeRef::named_nn(TypeRef::STRING),
	))
}

/// The site of the `site` or `siteId` argument, else the primary site.
fn site_arg(ctx: &ResolverContext) -> async_graphql::Result<String> {
	let schema = ctx.data::<Arc<Schema>>()?;
	let arg = |k: &str| ctx.args.get(k).filter(|v| !v.is_null());
	if let Some(v) = arg("site") {
		return Ok(v.string()?.into());
	}
	if let Some(v) = arg("siteId").map(|v| first(&v)).transpose()?.flatten() {
		let site = match v.parse::<i64>() {
			Ok(n) => schema.site_by_id(n),
			Err(_) => schema.get_site(&v),
		};
		let site =
			site.ok_or_else(|| GqlError::new(format!("unknown site {v}")))?;
		return Ok(site.handle.clone());
	}
	Ok(schema.primary_site().handle.clone())
}

/// The query of the arguments, `None` when the viewer sees no user.
fn query_of(
	ctx: &ResolverContext,
) -> async_graphql::Result<Option<PublicQuery>> {
	let schema = ctx.data::<Arc<Schema>>()?;
	let viewer = ctx.data::<Viewer>()?;
	let site = site_arg(ctx)?;
	if schema.get_site(&site).is_none() {
		return Err(GqlError::new(format!("unknown site {site}")));
	}
	let Some(mut q) = public_query(schema, viewer, &site) else {
		return Ok(None);
	};
	let arg = |k: &str| ctx.args.get(k).filter(|v| !v.is_null());
	if let Some(h) = arg("hasEntries") {
		let section = h.object()?.try_get("section")?.string()?.to_string();
		if schema.get_section(&section).is_none() {
			return Err(GqlError::new(format!("unknown section {section}")));
		}
		// a section the token doesn't read has no authors for it
		if !viewer.scope.section(&section) {
			return Ok(None);
		}
		q.has_entries = Some(section);
	}
	if let Some(id) = arg("id") {
		let id = first(&id)?.and_then(|i| i.parse().ok()).unwrap_or(0);
		q.ids = Some(vec![id]);
	}
	if let Some(slug) = arg("slug") {
		q.slug = Some(slug.string()?.into());
	}
	if let Some(order) = arg("orderBy") {
		let order = order.string()?;
		let (field, dir) = order.split_once(' ').unwrap_or((order, "ASC"));
		q.by_id = match field {
			"name" => false,
			"id" => true,
			_ => {
				return Err(GqlError::new(format!(
					"orderBy {order}: name or id, ASC or DESC"
				)));
			}
		};
		q.desc = dir.trim().eq_ignore_ascii_case("desc");
	}
	if let Some(limit) = arg("limit") {
		q.limit = Some(limit.i64()?.clamp(0, MAX_LIMIT));
	}
	if let Some(offset) = arg("offset") {
		q.offset = offset.i64()?.max(0);
	}
	Ok(Some(q))
}

/// The users of a query in its site.
fn load_users(
	ctx: &ResolverContext,
	q: &PublicQuery,
) -> async_graphql::Result<Vec<FieldValue<'static>>> {
	let mut q = q.clone();
	q.limit = Some(q.limit.unwrap_or(MAX_LIMIT));
	spend(ctx, 1)?;
	let users = profile::public(&ctx.data::<Db>()?.conn(), &q)?;
	spend(ctx, users.len())?;
	Ok(users
		.into_iter()
		.map(|profile| {
			user_value(GqlUser {
				profile,
				site: q.site.clone(),
				public: true,
			})
		})
		.collect())
}

/// `users`, `userCount` and `user` of the root.
pub(super) fn user_fields() -> Vec<GqlField> {
	let site_args = |f: GqlField| {
		f.argument(InputValue::new("site", TypeRef::named(TypeRef::STRING)))
			.argument(InputValue::new(
				"siteId",
				TypeRef::named("QueryArgument"),
			))
	};
	let has_entries =
		|| InputValue::new("hasEntries", TypeRef::named("UserEntriesCriteria"));
	let users =
		GqlField::new("users", TypeRef::named_nn_list_nn("User"), |ctx| {
			FieldFuture::new(async move {
				let users = match query_of(&ctx)? {
					Some(q) => load_users(&ctx, &q)?,
					None => vec![],
				};
				Ok(Some(FieldValue::list(users)))
			})
		})
		.argument(has_entries())
		.argument(InputValue::new("orderBy", TypeRef::named(TypeRef::STRING)))
		.argument(InputValue::new("limit", TypeRef::named(TypeRef::INT)))
		.argument(InputValue::new("offset", TypeRef::named(TypeRef::INT)));

	let count =
		GqlField::new("userCount", TypeRef::named_nn(TypeRef::INT), |ctx| {
			FieldFuture::new(async move {
				let n = match query_of(&ctx)? {
					Some(q) => {
						spend(&ctx, 1)?;
						profile::public_count(&ctx.data::<Db>()?.conn(), &q)?
					}
					None => 0,
				};
				Ok(Some(GqlValue::from(n)))
			})
		})
		.argument(has_entries());

	let user = GqlField::new("user", TypeRef::named("User"), |ctx| {
		FieldFuture::new(async move {
			let Some(mut q) = query_of(&ctx)? else {
				return Ok(None);
			};
			if q.ids.is_none() && q.slug.is_none() {
				return Err(GqlError::new("user needs an id or a slug"));
			}
			q.limit = Some(1);
			Ok(load_users(&ctx, &q)?.into_iter().next())
		})
	})
	.argument(InputValue::new("id", TypeRef::named("QueryArgument")))
	.argument(InputValue::new("slug", TypeRef::named(TypeRef::STRING)))
	.argument(has_entries());

	vec![site_args(users), site_args(count), site_args(user)]
}
