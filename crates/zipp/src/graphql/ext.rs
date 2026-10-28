//! What the GraphQL resolvers of plugins use: the data of the request and
//! entries loaded with the rights of who asks, so a preview shows drafts.

use super::*;

pub use super::value_field;

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

/// Entries of a site like the query arguments of `entries`
/// (`[("section", "events".into())]`), as the one who asks sees them.
pub fn entries<const N: usize>(
	ctx: &ResolverContext,
	site: &str,
	params: [(&str, String); N],
) -> async_graphql::Result<Vec<Entry>> {
	load(ctx, &site_params(site, params))
}

/// An entry as a GraphQL value of its type.
pub fn entry(e: Entry) -> FieldValue<'static> {
	entry_value(e)
}

/// A request as a frontend sends it (published entries only) or an
/// admin, for tests of plugins.
pub fn request(
	query: &str,
	published_only: bool,
	indexing: bool,
) -> async_graphql::Request {
	async_graphql::Request::new(query).data(Viewer {
		published_only,
		preview: None,
		token: None,
		indexing,
	})
}
