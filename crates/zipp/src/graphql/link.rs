use super::*;

/// A link like Craft's LinkData: `url label defaultLabel target`. `None`
/// if it points to an entry that isn't there (or not published).
pub(super) fn resolve_link(
	ctx: &ResolverContext,
	link: &Value,
	site: &str,
	new_tab: bool,
) -> async_graphql::Result<Option<GqlValue>> {
	let value = &link["value"];
	let text = value.as_str().unwrap_or_default().to_string();
	let id = value.as_i64().unwrap_or_default();
	let ty = link["type"].as_str().unwrap_or_default();

	let (url, default_label, external) = match ty {
		"entry" => {
			let params = site_params(site, [("ids", id.to_string())]);
			let Some(e) = load(ctx, &params)?.into_iter().next() else {
				return Ok(None);
			};
			let url = match &e.uri {
				Some(uri) => entry_url(ctx, site, uri)?,
				None => return Ok(None),
			};
			(url, e.title, false)
		}
		"asset" => {
			let db = ctx.data::<Db>()?;
			let Some(a) = get_many(&db.conn(), &[id])?.pop() else {
				return Ok(None);
			};
			(asset_url(ctx, &a, None)?, a.filename, true)
		}
		"email" => (format!("mailto:{text}"), text, false),
		"phone" => {
			let digits: String = text
				.chars()
				.filter(|c| c.is_ascii_digit() || *c == '+')
				.collect();
			(format!("tel:{digits}"), text, false)
		}
		// drafts aren't checked yet, what reaches a preview is
		_ if !crate::content::safe_url(&text) => return Ok(None),
		_ => {
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
	Ok(Some(GqlValue::from_json(serde_json::json!({
		"type": ty,
		"url": url,
		"label": link["label"].as_str().filter(|l| !l.is_empty()),
		"defaultLabel": default_label,
		"target": if new_tab { Some("_blank") } else { None },
		"download": link["download"].as_bool().unwrap_or(false),
	}))?))
}

/// The items of a navigation with the ones under them; items that link
/// nowhere (a deleted entry) are left out with theirs.
pub(super) fn nav_items(
	ctx: &ResolverContext,
	items: &Value,
	site: &str,
) -> async_graphql::Result<Vec<GqlValue>> {
	let mut out = vec![];
	for item in items.as_array().into_iter().flatten() {
		let Some(GqlValue::Object(mut o)) =
			resolve_link(ctx, item, site, true)?
		else {
			continue;
		};
		let children = nav_items(ctx, &item["children"], site)?;
		o.insert(
			async_graphql::Name::new("children"),
			GqlValue::List(children),
		);
		out.push(GqlValue::Object(o));
	}
	Ok(out)
}

/// An item of a navigation: a link and the items under it.
pub(super) fn nav_item_object() -> Object {
	let field = |name: &'static str, ty: TypeRef| {
		GqlField::new(name, ty, move |ctx| value_field(ctx, name))
	};
	let s = || TypeRef::named(TypeRef::STRING);
	Object::new("NavItem")
		.field(field("type", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("url", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("label", s()))
		.field(field("defaultLabel", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("target", s()))
		.field(field("download", TypeRef::named_nn(TypeRef::BOOLEAN)))
		.field(field("children", TypeRef::named_nn_list_nn("NavItem")))
}

pub(super) fn link_object() -> Object {
	let field = |name: &'static str, ty: TypeRef| {
		GqlField::new(name, ty, move |ctx| {
			FieldFuture::new(async move {
				let GqlValue::Object(o) = ctx.parent_value.try_to_value()?
				else {
					return Ok(None);
				};
				Ok(o.get(name).cloned().map(FieldValue::value))
			})
		})
	};
	let s = || TypeRef::named(TypeRef::STRING);
	Object::new("Link")
		.description("A link like Craft's LinkData, use label ?? defaultLabel")
		.field(field("type", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("url", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("label", s()))
		.field(field("defaultLabel", TypeRef::named_nn(TypeRef::STRING)))
		.field(field("target", s()))
		.field(field("download", TypeRef::named_nn(TypeRef::BOOLEAN)))
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
	if matches!(viewer.preview, Some(Preview::All)) {
		url += "&x-craft-live-preview=1";
	}
	Ok(url)
}
