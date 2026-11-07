use std::collections::HashMap;

use super::*;
use crate::assets::{self, AssetList, Kind};

/// The arguments of `assets` and `assetCount`, like Craft's: `folder` is
/// Craft's `volume`, the import makes a folder of each.
fn assets_args(field: GqlField) -> GqlField {
	let l = || TypeRef::named_list(TypeRef::STRING);
	let i = || TypeRef::named(TypeRef::INT);
	field
		.argument(InputValue::new("folder", l()))
		.argument(InputValue::new(
			"folderId",
			TypeRef::named_list("QueryArgument"),
		))
		.argument(InputValue::new("kind", l()))
		.argument(InputValue::new("search", TypeRef::named(TypeRef::STRING)))
		.argument(InputValue::new("site", l()))
		.argument(InputValue::new(
			"siteId",
			TypeRef::named_list("QueryArgument"),
		))
		.argument(InputValue::new("orderBy", TypeRef::named(TypeRef::STRING)))
		.argument(InputValue::new("limit", i()))
		.argument(InputValue::new("offset", i()))
}

/// The list of the arguments, with the site for the alt texts; `None`
/// when the viewer reads nothing of it.
fn list_of(
	ctx: &ResolverContext,
) -> async_graphql::Result<Option<(AssetList, String)>> {
	let site = site_arg(ctx)?;
	// like entries, nothing of a site the token doesn't read; the list
	// only for a schema with assets
	let scope = &ctx.data::<Viewer>()?.scope;
	if !scope.assets || !scope.site(&site) {
		return Ok(None);
	}

	let mut q = AssetList::default();
	// every folder in them too, like a volume with its folders
	let mut folders = None;
	if let Some(paths) = arg(ctx, "folder") {
		let known = folder_paths(ctx)?;
		let ids = all(&paths)?
			.into_iter()
			.flat_map(|p| assets::folders_under(known, &p));
		folders = Some(ids.collect::<Vec<_>>());
	}
	if let Some(ids) = arg(ctx, "folderId") {
		let ids = all(&ids)?.into_iter().map(|id| {
			id.parse::<i64>()
				.map_err(|_| GqlError::new(format!("folderId {id}")))
		});
		let ids = ids.collect::<async_graphql::Result<Vec<_>>>()?;
		// both: the folders of the ids in the paths
		folders = Some(match folders {
			Some(f) => f.into_iter().filter(|f| ids.contains(f)).collect(),
			None => ids,
		});
	}
	q.folders = folders;
	q.kinds = kinds_of(ctx)?;
	if let Some(s) = arg(ctx, "search") {
		q.words = s.string()?.split_whitespace().map(Into::into).collect();
	}
	if let Some(o) = arg(ctx, "orderBy") {
		q.order_by = Some(o.string()?.into());
	}
	// never more than a request may load, a big library isn't read
	// before the error
	let limit = match arg(ctx, "limit") {
		Some(l) => l.i64()?,
		None => MAX_LOADED as i64,
	};
	q.limit = Some(limit.clamp(0, MAX_LOADED as i64));
	if let Some(o) = arg(ctx, "offset") {
		q.offset = Some(o.i64()?);
	}
	Ok(Some((q, site)))
}

/// The kinds of the `kind` argument, none for all: `"image"` or
/// `["pdf", "word"]` (see [`Kind`]).
pub(super) fn kinds_of(
	ctx: &ResolverContext,
) -> async_graphql::Result<Vec<Kind>> {
	let Some(kind) = arg(ctx, "kind") else {
		return Ok(vec![]);
	};
	let kinds = all(&kind)?.into_iter().map(|k| Kind::parse(&k));
	Ok(kinds.collect::<crate::error::Result<_>>()?)
}

/// The paths of the folders, read once per request.
fn folder_paths<'a>(
	ctx: &ResolverContext<'a>,
) -> async_graphql::Result<&'a HashMap<i64, String>> {
	let loaded = &ctx.data::<Viewer>()?.loaded;
	if let Some(paths) = loaded.folders.get() {
		return Ok(paths);
	}
	let paths = assets::folder_paths(&ctx.data::<Db>()?.conn())?;
	Ok(loaded.folders.get_or_init(|| paths))
}

/// The path of the folder of an asset, `None` on the top.
pub(super) fn folder_of(
	ctx: &ResolverContext,
	a: &Asset,
) -> async_graphql::Result<Option<String>> {
	let Some(id) = a.folder_id else {
		return Ok(None);
	};
	Ok(folder_paths(ctx)?.get(&id).cloned())
}

/// Craft's path of an asset in its volume: in its top folder, which the
/// import made of the volume, `2025/HS/1a.pdf` for
/// `Timetables/2025/HS/1a.pdf`.
pub(super) fn path_of(
	ctx: &ResolverContext,
	a: &Asset,
) -> async_graphql::Result<String> {
	let folder = folder_of(ctx, a)?.unwrap_or_default();
	Ok(match folder.split_once('/') {
		Some((_, inner)) => format!("{inner}/{}", a.filename),
		None => a.filename.clone(),
	})
}

/// The assets of the library, like Craft's: `assets(folder: "Clubs",
/// kind: "image") { title url }`, the newest first; and `assetCount`
/// with the same arguments for the pages. Files are public by their url,
/// so every viewer may list them; trashed ones are gone.
pub(super) fn asset_list_fields() -> [GqlField; 2] {
	let list =
		GqlField::new("assets", TypeRef::named_nn_list_nn("Asset"), |ctx| {
			FieldFuture::new(async move {
				let Some((q, site)) = list_of(&ctx)? else {
					return Ok(Some(
						FieldValue::list(Vec::<FieldValue>::new()),
					));
				};
				spend(&ctx, 1)?;
				let found = assets::list(&ctx.data::<Db>()?.conn(), &q)?;
				spend(&ctx, found.len())?;
				Ok(Some(FieldValue::list(
					found.into_iter().map(|a| asset_value(a, &site)),
				)))
			})
		});
	let count =
		GqlField::new("assetCount", TypeRef::named_nn(TypeRef::INT), |ctx| {
			FieldFuture::new(async move {
				let Some((q, _)) = list_of(&ctx)? else {
					return Ok(Some(GqlValue::from(0)));
				};
				spend(&ctx, 1)?;
				let n = assets::count(&ctx.data::<Db>()?.conn(), &q)?;
				Ok(Some(GqlValue::from(n)))
			})
		});
	[assets_args(list), assets_args(count)]
}
