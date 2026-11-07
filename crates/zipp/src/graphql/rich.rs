use super::*;

/// What a chunk of a rich text is: html, a row of buttons or a block.
pub(super) fn chunk_union(schema: &Schema) -> Union {
	let mut u = Union::new("Chunk")
		.possible_type("Html")
		.possible_type("Buttons");
	for c in schema
		.components
		.iter()
		.filter(|c| !c.entry && c.handle != ASSET && c.handle != USER)
	{
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
					let to = targets(&ctx, &rt.site, &rt.doc).await?;
					let mut resolve = resolver(&ctx, &rt.site, &to);
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
					let to = targets(&ctx, &rt.site, &rt.doc).await?;
					let mut resolve = resolver(&ctx, &rt.site, &to);
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
										&ctx, l, &rt.site, true, &to,
									)?);
								}
								let buttons =
									serde_json::json!({ "buttons": buttons });
								link_value(&buttons)?.with_type("Buttons")
							}
							Chunk::Block(attrs) => block_of(&attrs, &rt.site),
						});
					}
					Ok(Some(FieldValue::list(out)))
				})
			},
		))
}
