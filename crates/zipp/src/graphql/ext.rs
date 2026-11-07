//! What the GraphQL resolvers of plugins use: the data of the request and
//! entries loaded with the rights of who asks, so a preview shows drafts.

use super::*;

pub fn schema<'a>(
	ctx: &ResolverContext<'a>,
) -> async_graphql::Result<&'a Arc<Schema>> {
	ctx.data::<Arc<Schema>>()
}

pub fn db<'a>(ctx: &ResolverContext<'a>) -> async_graphql::Result<&'a Db> {
	ctx.data::<Db>()
}

pub fn site_urls<'a>(
	ctx: &ResolverContext<'a>,
) -> async_graphql::Result<&'a SiteUrls> {
	ctx.data::<SiteUrls>()
}

/// Put before asset urls.
pub fn public_url<'a>(
	ctx: &ResolverContext<'a>,
) -> async_graphql::Result<&'a str> {
	Ok(&ctx.data::<PublicUrl>()?.0)
}

/// Whether search engines may index, not in development.
pub fn indexing(ctx: &ResolverContext) -> async_graphql::Result<bool> {
	Ok(ctx.data::<Viewer>()?.indexing)
}

/// The site of what the field is on (an entry, asset, user or block), in
/// [`FieldType::resolve`](crate::FieldType::resolve).
pub fn site(ctx: &ResolverContext) -> async_graphql::Result<String> {
	Ok(entry::parent_fields(ctx)?.1)
}

/// A block `{ "id", "type", "fields" }` as its `<type>_Block` of the
/// `Block` interface, like in a blocks field: the items of a field with
/// [`Field::item_blocks`](crate::Field::item_blocks) give theirs so, with
/// the [`site`] of the field. A switched off block (`"enabled": false`)
/// isn't on the website, leave it out.
pub fn block(block: &Value, site: &str) -> FieldValue<'static> {
	block_of(block, site)
}

/// Entries of a site like the query arguments of `entries`
/// (`[("section", "events".into())]`), as the one who asks sees them.
pub fn entries<const N: usize>(
	ctx: &ResolverContext,
	site: &str,
	params: [(&str, String); N],
) -> async_graphql::Result<Vec<Entry>> {
	load(ctx, &site_params(site, params))
}

/// A part of an object value, for objects built as values.
pub fn value_field<'a>(
	ctx: ResolverContext<'a>,
	name: &'static str,
) -> FieldFuture<'a> {
	FieldFuture::new(async move {
		let GqlValue::Object(o) = ctx.parent_value.try_to_value()? else {
			return Ok(None);
		};
		Ok(match o.get(name) {
			Some(GqlValue::List(l)) => {
				Some(FieldValue::list(l.iter().cloned().map(FieldValue::value)))
			}
			Some(v) => Some(FieldValue::value(v.clone())),
			None => None,
		})
	})
}
