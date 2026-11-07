use serde::Serialize;

use super::*;
use crate::content::link_kind;
use crate::schema::LinkKind;

/// A link like Craft's LinkData: `url label defaultLabel target`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ResolvedLink {
	#[serde(rename = "type")]
	pub kind: LinkKind,
	pub url: String,
	pub label: Option<String>,
	pub default_label: String,
	pub target: Option<&'static str>,
	pub download: bool,
}

/// An item of a navigation with the ones under it.
#[derive(Debug, Clone, Serialize)]
pub(super) struct NavItem {
	#[serde(flatten)]
	pub link: ResolvedLink,
	pub children: Vec<NavItem>,
}

/// A link as the value of a GraphQL field.
pub(super) fn link_value<T: Serialize>(
	link: &T,
) -> async_graphql::Result<FieldValue<'static>> {
	Ok(FieldValue::value(async_graphql::to_value(link)?))
}

/// A link value resolved, `None` if it points to an entry that isn't
/// there (or not published). The entries and assets are loaded before,
/// see [`targets`].
pub(super) fn resolve_link(
	ctx: &ResolverContext,
	link: &Value,
	site: &str,
	new_tab: bool,
	targets: &Targets,
) -> async_graphql::Result<Option<ResolvedLink>> {
	let value = &link["value"];
	let text = value.as_str().unwrap_or_default().to_string();
	let id = value.as_i64().unwrap_or_default();
	let Some(kind) = link_kind(link) else {
		return Ok(None);
	};

	let (url, default_label, external) = match kind {
		LinkKind::Entry => {
			let Some(e) = targets.entries.get(&id) else {
				return Ok(None);
			};
			let url = match &e.uri {
				Some(uri) => entry_url(ctx, site, uri)?,
				None => return Ok(None),
			};
			(url, e.title.clone(), false)
		}
		LinkKind::Asset => {
			let Some(a) = targets.assets.get(&id) else {
				return Ok(None);
			};
			(asset_url(ctx, a, None, None)?, a.filename.clone(), true)
		}
		LinkKind::Email => (format!("mailto:{text}"), text, false),
		LinkKind::Phone => {
			let digits: String = text
				.chars()
				.filter(|c| c.is_ascii_digit() || *c == '+')
				.collect();
			(format!("tel:{digits}"), text, false)
		}
		// drafts aren't checked yet, what reaches a preview is
		LinkKind::Url if !crate::content::safe_url(&text) => return Ok(None),
		LinkKind::Url => {
			// our own sites aren't external
			let urls = ctx.data::<SiteUrls>()?;
			let own = urls
				.values()
				.any(|b| !b.is_empty() && text.starts_with(b.as_str()));
			let external = text.starts_with("http") && !own;
			(text.clone(), text, external)
		}
	};

	// a section of the page, instead of the url's own
	let url = match link["anchor"].as_str().filter(|a| !a.is_empty()) {
		Some(a) => format!("{}#{a}", url.split('#').next().unwrap_or_default()),
		None => url,
	};
	let new_tab = link["newTab"].as_bool().unwrap_or(new_tab && external);
	Ok(Some(ResolvedLink {
		kind,
		url,
		label: link["label"]
			.as_str()
			.filter(|l| !l.is_empty())
			.map(Into::into),
		default_label,
		target: new_tab.then_some("_blank"),
		download: link["download"].as_bool().unwrap_or(false),
	}))
}

/// The items of a navigation with the ones under them; items that link
/// nowhere (a deleted entry) are left out with theirs.
pub(super) fn nav_items(
	ctx: &ResolverContext,
	items: &Value,
	site: &str,
	targets: &Targets,
) -> async_graphql::Result<Vec<NavItem>> {
	let mut out = vec![];
	for item in items.as_array().into_iter().flatten() {
		let Some(link) = resolve_link(ctx, item, site, true, targets)? else {
			continue;
		};
		let children = nav_items(ctx, &item["children"], site, targets)?;
		out.push(NavItem { link, children });
	}
	Ok(out)
}

fn value_object_field(name: &'static str, ty: TypeRef) -> GqlField {
	GqlField::new(name, ty, move |ctx| value_field(ctx, name))
}

/// The fields of a link on `obj`.
fn link_fields(obj: Object) -> Object {
	let field = value_object_field;
	let s = || TypeRef::named(TypeRef::STRING);
	obj.field(field("type", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("url", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("label", s()))
		.field(field("defaultLabel", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("target", s()))
		.field(field("download", TypeRef::named_nn(TypeRef::BOOLEAN)))
}

/// An item of a navigation: a link and the items under it.
pub(super) fn nav_item_object() -> Object {
	let children = TypeRef::named_nn_list_nn("NavItem");
	link_fields(Object::new("NavItem"))
		.field(value_object_field("children", children))
}

pub(super) fn link_object() -> Object {
	link_fields(
		Object::new("Link").description(
			"A link like Craft's LinkData, use label ?? defaultLabel",
		),
	)
}

pub(super) fn entry_url(
	ctx: &ResolverContext,
	site: &str,
	uri: &str,
) -> async_graphql::Result<String> {
	let urls = ctx.data::<SiteUrls>()?;
	let url =
		crate::page_url(urls, site, uri).unwrap_or_else(|| format!("/{uri}"));
	let viewer = ctx.data::<Viewer>()?;
	let Some(token) = &viewer.token else {
		return Ok(url);
	};
	// links of a preview stay in it; a preview of the whole site is crelte's
	// live preview, which keeps the scroll position on reloads
	let mut url = format!("{url}?token={token}");
	if matches!(viewer.preview, Some(Preview::All(_))) {
		url += "&x-craft-live-preview=1";
	}
	Ok(url)
}
