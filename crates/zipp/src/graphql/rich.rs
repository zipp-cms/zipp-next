use super::*;

/// Markdown to html, raw html in it is dropped (editors write text) and
/// links and images only point to the web or the site.
pub fn markdown_html(md: &str) -> String {
	use pulldown_cmark::{Event, Options, Parser, Tag, html};
	let safe =
		|url: &str| crate::content::safe_url(url) || url.starts_with("mailto:");
	let parser = Parser::new_ext(
		md,
		Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
	)
	.filter(|e| !matches!(e, Event::Html(_) | Event::InlineHtml(_)))
	.map(|e| match e {
		Event::Start(Tag::Link {
			link_type,
			dest_url,
			title,
			id,
		}) if !safe(&dest_url) => Event::Start(Tag::Link {
			link_type,
			dest_url: "#".into(),
			title,
			id,
		}),
		Event::Start(Tag::Image {
			link_type,
			dest_url,
			title,
			id,
		}) if !safe(&dest_url) => Event::Start(Tag::Image {
			link_type,
			dest_url: "".into(),
			title,
			id,
		}),
		e => e,
	});
	let mut out = String::new();
	html::push_html(&mut out, parser);
	out
}

/// What a chunk of a rich text is: html, a row of buttons or a block.
pub(super) fn chunk_union(schema: &Schema) -> Union {
	let mut u = Union::new("Chunk")
		.possible_type("Html")
		.possible_type("Buttons");
	for c in schema.components.iter().filter(|c| !c.entry) {
		u = u.possible_type(format!("{}_Block", c.handle));
	}
	u
}

pub(super) fn rich_text_object() -> Object {
	Object::new("RichText")
		.description(
			"Rich text: all as html, or in chunks with buttons and blocks",
		)
		.field(GqlField::new(
			"html",
			TypeRef::named_nn(TypeRef::STRING),
			|ctx| {
				FieldFuture::new(async move {
					let rt = ctx.parent_value.try_downcast_ref::<RichText>()?;
					let mut resolve = resolver(&ctx, &rt.site);
					let html = richtext::html(&rt.doc, &mut resolve);
					Ok(Some(FieldValue::value(html)))
				})
			},
		))
		.field(GqlField::new(
			"chunks",
			TypeRef::named_nn_list_nn("Chunk"),
			|ctx| {
				FieldFuture::new(async move {
					let rt = ctx.parent_value.try_downcast_ref::<RichText>()?;
					let mut resolve = resolver(&ctx, &rt.site);
					let mut out = vec![];
					for chunk in richtext::chunks(&rt.doc, &mut resolve) {
						out.push(match chunk {
							Chunk::Html(html) => {
								FieldValue::value(GqlValue::from_json(
									serde_json::json!({ "html": html }),
								)?)
								.with_type("Html")
							}
							Chunk::Buttons(links) => {
								let mut buttons = vec![];
								for l in &links {
									buttons.extend(resolve_link(
										&ctx, l, &rt.site, true,
									)?);
								}
								let mut o =
									async_graphql::indexmap::IndexMap::new();
								o.insert(
									async_graphql::Name::new("buttons"),
									GqlValue::List(buttons),
								);
								FieldValue::value(GqlValue::Object(o))
									.with_type("Buttons")
							}
							Chunk::Block(attrs) => {
								let ty = attrs["type"]
									.as_str()
									.unwrap_or_default()
									.to_string();
								FieldValue::owned_any(Block {
									id: attrs["id"]
										.as_str()
										.unwrap_or_default()
										.into(),
									ty: ty.clone(),
									site: rt.site.clone(),
									fields: attrs["fields"].clone(),
								})
								.with_type(format!("{ty}_Block"))
							}
						});
					}
					Ok(Some(FieldValue::list(out)))
				})
			},
		))
}
