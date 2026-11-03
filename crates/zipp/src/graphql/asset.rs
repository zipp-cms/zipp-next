use super::*;

/// An asset as the parent of its fields, with the site of the content
/// it is in for the relations of its asset fields.
pub(super) struct SiteAsset {
	pub asset: Asset,
	pub site: String,
}

pub(super) fn asset_value(asset: Asset, site: &str) -> FieldValue<'static> {
	FieldValue::owned_any(SiteAsset {
		asset,
		site: site.into(),
	})
}

fn asset_of<'a>(ctx: &ResolverContext<'a>) -> async_graphql::Result<&'a Asset> {
	Ok(&ctx.parent_value.try_downcast_ref::<SiteAsset>()?.asset)
}

/// An asset, like Craft's: `url(transform: "medium")` and the size after
/// the transform. Transforms are the named ones of the schema, the asset
/// fields of the schema are fields of it.
pub(super) fn asset_object(schema: &Schema) -> Object {
	let plain = |name: &'static str, ty: TypeRef| {
		GqlField::new(name, ty, move |ctx| {
			FieldFuture::new(async move {
				let parent =
					ctx.parent_value.try_downcast_ref::<SiteAsset>()?;
				let a = &parent.asset;
				let v = match name {
					"id" => GqlValue::from(a.id.to_string()),
					"filename" => GqlValue::from(a.filename.clone()),
					"kind" => GqlValue::from(a.kind.clone()),
					"mimeType" => GqlValue::from(a.mime.clone()),
					"size" => GqlValue::from(a.size),
					// the site's, else the primary site's
					"alt" => {
						let schema = ctx.data::<Arc<Schema>>()?;
						let primary = &schema.primary_site().handle;
						GqlValue::from(a.alt_in(&parent.site, primary))
					}
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
	let format_arg =
		|| InputValue::new("format", TypeRef::named(TypeRef::STRING));
	let s = || TypeRef::named_nn(TypeRef::STRING);
	let f = || TypeRef::named_nn(TypeRef::FLOAT);

	let mut obj = Object::new("Asset");
	if let Some(comp) = schema.get(ASSET) {
		for field in &comp.fields {
			if let Some(f) = content_field(schema, ASSET, field) {
				obj = obj.field(f);
			}
		}
	}
	obj.field(plain("id", TypeRef::named_nn(TypeRef::ID)))
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
		.field(GqlField::new(
			"folderId",
			TypeRef::named(TypeRef::INT),
			|ctx| {
				FieldFuture::new(async move {
					let id = asset_of(&ctx)?.folder_id;
					Ok(id.map(FieldValue::value))
				})
			},
		))
		// the path of its folder, `Clubs/Bern`
		.field(GqlField::new(
			"folder",
			TypeRef::named(TypeRef::STRING),
			|ctx| {
				FieldFuture::new(async move {
					let folder = folder_of(&ctx, asset_of(&ctx)?)?;
					Ok(folder.map(FieldValue::value))
				})
			},
		))
		.field(GqlField::new("path", s(), |ctx| {
			FieldFuture::new(async move {
				let path = path_of(&ctx, asset_of(&ctx)?)?;
				Ok(Some(FieldValue::value(path)))
			})
		}))
		.field(
			GqlField::new("url", s(), |ctx| {
				FieldFuture::new(async move {
					let a = asset_of(&ctx)?;
					let t = transform_of(&ctx, a)?;
					let format = format_of(&ctx)?;
					if format.is_some() && t.is_none() && a.kind == "image" {
						return Err(GqlError::new(
							"format needs a transform or a width",
						));
					}
					let url = asset_url(&ctx, a, t.as_ref(), format)?;
					Ok(Some(FieldValue::value(url)))
				})
			})
			.argument(transform_arg())
			.argument(width_arg())
			.argument(format_arg()),
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
					let a = asset_of(&ctx)?;
					let schema = ctx.data::<Arc<Schema>>()?;
					let format = format_of(&ctx)?;
					let mut parts = vec![];
					for name in ctx.args.try_get("transforms")?.list()?.iter() {
						let name = name.string()?;
						// named or `w800`, like the transform of url
						let t =
							schema.transform_named(name).ok_or_else(|| {
								GqlError::new(format!(
									"unknown transform {name}"
								))
							})?;
						let Some((w, _)) = a.size_after(&t) else {
							return Ok(None);
						};
						parts.push(format!(
							"{} {w}w",
							asset_url(&ctx, a, Some(&t), format)?
						));
					}
					Ok(Some(FieldValue::value(parts.join(", "))))
				})
			})
			.argument(InputValue::new(
				"transforms",
				TypeRef::named_nn_list_nn(TypeRef::STRING),
			))
			.argument(format_arg()),
		)
}

/// The transform of the `transform` argument, `None` for the original.
pub(super) fn transform_of(
	ctx: &ResolverContext,
	a: &Asset,
) -> async_graphql::Result<Option<Transform>> {
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

/// The `format` argument: `webp`, `jpg` or `png`, `None` for the
/// transform's.
fn format_of(ctx: &ResolverContext) -> async_graphql::Result<Option<Format>> {
	let Some(f) = ctx.args.get("format").filter(|v| !v.is_null()) else {
		return Ok(None);
	};
	let f = f.string()?;
	Format::parse(f)
		.map(Some)
		.ok_or_else(|| GqlError::new(format!("format {f}: webp, jpg or png")))
}

/// The url of the image or of a transform, in `format` or the
/// transform's.
pub(super) fn asset_url(
	ctx: &ResolverContext,
	a: &Asset,
	transform: Option<&Transform>,
	format: Option<Format>,
) -> async_graphql::Result<String> {
	let base = &ctx.data::<PublicUrl>()?.0;
	let path = match transform {
		Some(t) => a.format_url(t, format),
		None => a.url(None),
	};
	Ok(format!("{}{path}", base.trim_end_matches('/')))
}

pub(super) fn asset_size(
	ctx: ResolverContext,
	pick: fn((u32, u32)) -> u32,
) -> FieldFuture {
	FieldFuture::new(async move {
		let a = asset_of(&ctx)?;
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
