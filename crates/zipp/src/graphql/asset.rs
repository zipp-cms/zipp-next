use super::*;

/// An asset, like Craft's: `url(transform: "medium")` and the size after
/// the transform. Transforms are the named ones of the schema.
pub(super) fn asset_object() -> Object {
	let plain = |name: &'static str, ty: TypeRef| {
		GqlField::new(name, ty, move |ctx| {
			FieldFuture::new(async move {
				let a = ctx.parent_value.try_downcast_ref::<Asset>()?;
				let v = match name {
					"id" => GqlValue::from(a.id.to_string()),
					"filename" => GqlValue::from(a.filename.clone()),
					"kind" => GqlValue::from(a.kind.clone()),
					"mimeType" => GqlValue::from(a.mime.clone()),
					"size" => GqlValue::from(a.size),
					"alt" => GqlValue::from(a.alt.clone()),
					"focalX" => GqlValue::from(a.focal_x),
					"focalY" => GqlValue::from(a.focal_y),
					// Craft's
					"focalPoint" => {
						GqlValue::List(vec![a.focal_x.into(), a.focal_y.into()])
					}
					"title" => GqlValue::from(match a.title.as_str() {
						"" => title_of(&a.filename),
						t => t.to_string(),
					}),
					_ => unreachable!(),
				};
				Ok(Some(FieldValue::value(v)))
			})
		})
	};
	let transform_arg =
		|| InputValue::new("transform", TypeRef::named(TypeRef::STRING));
	let width_arg = || InputValue::new("width", TypeRef::named(TypeRef::INT));
	let s = || TypeRef::named_nn(TypeRef::STRING);
	let f = || TypeRef::named_nn(TypeRef::FLOAT);

	Object::new("Asset")
		.field(plain("id", TypeRef::named_nn(TypeRef::ID)))
		.field(plain("filename", s()))
		.field(plain("kind", s()))
		.field(plain("mimeType", s()))
		.field(plain("size", TypeRef::named_nn(TypeRef::INT)))
		.field(plain("alt", s()))
		.field(plain("focalX", f()))
		.field(plain("focalY", f()))
		.field(plain(
			"focalPoint",
			TypeRef::named_nn_list_nn(TypeRef::FLOAT),
		))
		.field(plain("title", s()))
		.field(
			GqlField::new("url", s(), |ctx| {
				FieldFuture::new(async move {
					let a = ctx.parent_value.try_downcast_ref::<Asset>()?;
					let t = transform_of(&ctx, a)?;
					let url = asset_url(
						&ctx,
						a,
						t.as_ref().map(|t| t.handle.as_str()),
					)?;
					Ok(Some(FieldValue::value(url)))
				})
			})
			.argument(transform_arg())
			.argument(width_arg()),
		)
		.field(
			GqlField::new("width", TypeRef::named(TypeRef::INT), |ctx| {
				asset_size(ctx, |(w, _)| w)
			})
			.argument(transform_arg())
			.argument(width_arg()),
		)
		.field(
			GqlField::new("height", TypeRef::named(TypeRef::INT), |ctx| {
				asset_size(ctx, |(_, h)| h)
			})
			.argument(transform_arg())
			.argument(width_arg()),
		)
		.field(
			GqlField::new("srcset", TypeRef::named(TypeRef::STRING), |ctx| {
				FieldFuture::new(async move {
					let a = ctx.parent_value.try_downcast_ref::<Asset>()?;
					let schema = ctx.data::<Arc<Schema>>()?;
					let mut parts = vec![];
					for name in ctx.args.try_get("transforms")?.list()?.iter() {
						let name = name.string()?;
						let t =
							schema.get_transform(name).ok_or_else(|| {
								GqlError::new(format!(
									"unknown transform {name}"
								))
							})?;
						let Some((w, _)) = a.size_after(t) else {
							return Ok(None);
						};
						parts.push(format!(
							"{} {w}w",
							asset_url(&ctx, a, Some(name))?
						));
					}
					Ok(Some(FieldValue::value(parts.join(", "))))
				})
			})
			.argument(InputValue::new(
				"transforms",
				TypeRef::named_nn_list_nn(TypeRef::STRING),
			)),
		)
}

/// The transform of the `transform` argument, `None` for the original.
pub(super) fn transform_of(
	ctx: &ResolverContext,
	a: &Asset,
) -> async_graphql::Result<Option<crate::schema::Transform>> {
	let schema = ctx.data::<Arc<Schema>>()?;
	let arg = |k: &str| ctx.args.get(k).filter(|v| !v.is_null());
	let t = match (arg("transform"), arg("width")) {
		(Some(name), _) => {
			let name = name.string()?;
			schema.transform_named(name).ok_or_else(|| {
				GqlError::new(format!("unknown transform {name}"))
			})?
		}
		// Craft's `url(width: 800)`, of the widths the schema allows
		(None, Some(w)) => {
			let w = u32::try_from(w.i64()?).unwrap_or(0);
			schema.width_transform(w).ok_or_else(|| {
				GqlError::new(format!(
					"width {w} is not allowed, the schema has {:?}",
					schema.image_widths
				))
			})?
		}
		(None, None) => return Ok(None),
	};
	// only images have transforms, a file stays itself
	Ok((a.kind == "image").then(|| t.clone()))
}

pub(super) fn asset_url(
	ctx: &ResolverContext,
	a: &Asset,
	transform: Option<&str>,
) -> async_graphql::Result<String> {
	let base = &ctx.data::<PublicUrl>()?.0;
	Ok(format!(
		"{}{}",
		base.trim_end_matches('/'),
		a.url(transform)
	))
}

pub(super) fn asset_size(
	ctx: ResolverContext,
	pick: fn((u32, u32)) -> u32,
) -> FieldFuture {
	FieldFuture::new(async move {
		let a = ctx.parent_value.try_downcast_ref::<Asset>()?;
		let size = match transform_of(&ctx, a)? {
			Some(t) => a.size_after(&t),
			None => a.width.zip(a.height),
		};
		Ok(size.map(|s| FieldValue::value(pick(s))))
	})
}

/// Like Craft's title of an asset: `piano-night.jpg` is `Piano night`.
pub(super) fn title_of(filename: &str) -> String {
	let stem = filename.rsplit_once('.').map_or(filename, |(s, _)| s);
	let words = stem.replace(['-', '_'], " ");
	let mut chars = words.trim().chars();
	match chars.next() {
		Some(c) => c.to_uppercase().chain(chars).collect(),
		None => String::new(),
	}
}
