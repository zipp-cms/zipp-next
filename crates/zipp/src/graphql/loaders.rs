//! Entries, children and assets by id, batched over a request: the
//! relations, links, parents and children of a list of entries load in a
//! query per site, not one per entry. A request has its own loaders, made
//! on first use; they keep what they loaded, so an entry that many link
//! to loads once. Everything loads with what the viewer may see and
//! counts against [`MAX_LOADED`].

use std::collections::{BTreeMap, HashMap};

use async_graphql::dataloader::{DataLoader, HashMapCache, Loader};

use super::*;
use crate::content::link_kind;
use crate::entries::Author;
use crate::entries::query::{MAX_IDS, Related};
use crate::schema::LinkKind;
use crate::users::profile::Profile;

/// The loaders of a request, see [`Loaded`].
pub(super) struct Loaders {
	entries: DataLoader<Entries, HashMapCache>,
	children: DataLoader<Children, HashMapCache>,
	assets: DataLoader<Assets, HashMapCache>,
	authors: DataLoader<Authors, HashMapCache>,
	public_users: DataLoader<PublicUsers, HashMapCache>,
}

/// What the loaders read with: the database, the schema and who asks.
struct Source {
	db: Db,
	schema: Arc<Schema>,
	viewer: Viewer,
}

/// Entries by site and id.
struct Entries(Arc<Source>);

/// The children of entries by site, the arguments of `children` and
/// parent, in their order.
struct Children(Arc<Source>);

/// The site, the parameters of the arguments and the parent.
type ChildrenKey = (String, Vec<(String, String)>, i64);

struct Assets(Db);

/// The authors of entries by user id.
struct Authors(Db);

/// The public profiles of users by site and id, see [`public_query`].
struct PublicUsers(Arc<Source>);

impl Loaders {
	fn of<'a>(ctx: &ResolverContext<'a>) -> async_graphql::Result<&'a Self> {
		let viewer = ctx.data::<Viewer>()?;
		let db = ctx.data::<Db>()?;
		let schema = ctx.data::<Arc<Schema>>()?;
		Ok(viewer.loaded.loaders.get_or_init(|| {
			let source = Arc::new(Source {
				db: db.clone(),
				schema: schema.clone(),
				viewer: viewer.copy(),
			});
			Loaders {
				entries: batched(Entries(source.clone())),
				children: batched(Children(source.clone())),
				public_users: batched(PublicUsers(source)),
				assets: batched(Assets(db.clone())),
				authors: batched(Authors(db.clone())),
			}
		}))
	}
}

fn batched<T>(loader: T) -> DataLoader<T, HashMapCache> {
	DataLoader::with_cache(loader, tokio::spawn, HashMapCache::default())
		.max_batch_size(MAX_IDS)
}

/// The ids by site.
fn by_site(keys: &[(String, i64)]) -> BTreeMap<&str, Vec<i64>> {
	let mut out: BTreeMap<&str, Vec<i64>> = BTreeMap::new();
	for (site, id) in keys {
		out.entry(site).or_default().push(*id);
	}
	out
}

impl Loader<(String, i64)> for Entries {
	type Value = Entry;
	type Error = GqlError;

	async fn load(
		&self,
		keys: &[(String, i64)],
	) -> async_graphql::Result<HashMap<(String, i64), Entry>> {
		let Source { db, schema, viewer } = &*self.0;
		let mut out = HashMap::new();
		for (site, ids) in by_site(keys) {
			let list: Vec<String> = ids.iter().map(i64::to_string).collect();
			let params = site_params(
				site,
				[("ids", list.join(",")), ("limit", ids.len().to_string())],
			);
			let q = viewer_query(schema, viewer, &params)?;
			let conn = db.conn();
			for e in with_drafts(&conn, viewer, select(&conn, &q)?)? {
				out.insert((e.site.clone(), e.id), e);
			}
		}
		Ok(out)
	}
}

impl Loader<ChildrenKey> for Children {
	type Value = Vec<Entry>;
	type Error = GqlError;

	async fn load(
		&self,
		keys: &[ChildrenKey],
	) -> async_graphql::Result<HashMap<ChildrenKey, Vec<Entry>>> {
		let Source { db, schema, viewer } = &*self.0;
		let mut by_args: BTreeMap<_, Vec<i64>> = BTreeMap::new();
		for (site, args, parent) in keys {
			by_args.entry((site, args)).or_default().push(*parent);
		}

		let mut out: HashMap<_, Vec<Entry>> = HashMap::new();
		for ((site, args), parents) in by_args {
			// in their order and all of them, unless the arguments say
			let mut params = site_params(
				site,
				[
					("order", "structure".into()),
					("limit", MAX_CHILDREN.to_string()),
				],
			);
			params.extend(args.iter().cloned());
			let mut q = viewer_query(schema, viewer, &params)?;
			q.parents = parents;
			q.limit = q.limit.min(MAX_CHILDREN);
			let conn = db.conn();
			let children = crate::entries::children(&conn, &q)?;
			for e in with_drafts(&conn, viewer, children)? {
				let key = (site.clone(), args.clone(), e.parent_id.unwrap());
				out.entry(key).or_default().push(e);
			}
		}
		Ok(out)
	}
}

impl Loader<i64> for Assets {
	type Value = Asset;
	type Error = GqlError;

	async fn load(
		&self,
		ids: &[i64],
	) -> async_graphql::Result<HashMap<i64, Asset>> {
		let assets = get_many(&self.0.conn(), ids)?;
		Ok(assets.into_iter().map(|a| (a.id, a)).collect())
	}
}

/// A query of entries with the arguments, what the viewer may see.
pub(super) fn query(
	ctx: &ResolverContext,
	params: &[(String, String)],
) -> async_graphql::Result<Query> {
	viewer_query(ctx.data::<Arc<Schema>>()?, ctx.data::<Viewer>()?, params)
}

/// A query of entries with what the viewer may see.
pub(super) fn viewer_query(
	schema: &Schema,
	viewer: &Viewer,
	params: &[(String, String)],
) -> async_graphql::Result<Query> {
	let mut q = Query::parse(schema, params).map_err(GqlError::new)?;
	// the same for the entries of `relatedToEntries`
	q.each_mut(&mut |q| {
		// `relatedTo` only of entries it sees too, else the relations of
		// a draft or of a section it doesn't read show
		for r in &mut q.related_to {
			if let Related::Ids(ids) = r {
				let mut of = Query::new(schema);
				of.site = q.site.clone();
				of.ids = Some(std::mem::take(ids));
				*r = Related::Entries(Box::new(of));
			}
		}
		q.published_only = viewer.published_only;
		match &viewer.preview {
			Some(Preview::Entry(id, _)) => q.preview_id = Some(*id),
			Some(Preview::All(sections)) => {
				q.preview_sections = sections.clone()
			}
			None => {}
		}
		// a frontend gets nothing of an offline site, counts neither; the
		// preview of one entry only that entry, its link opens no more
		if schema.is_offline(&q.site) && viewer.published_only {
			match &viewer.preview {
				None => q.nothing = true,
				Some(Preview::Entry(id, _)) => match &mut q.ids {
					Some(ids) => ids.retain(|i| i == id),
					None => q.ids = Some(vec![*id]),
				},
				Some(Preview::All(_)) => {}
			}
		}

		// nor of what its token doesn't read
		let scope = &viewer.scope;
		match &q.section {
			Some(s) if !scope.section(s) => q.nothing = true,
			Some(_) => {}
			None => q.sections = scope.sections.clone(),
		}
		if !scope.site(&q.site) {
			q.nothing = true;
		}
	});
	Ok(q)
}

/// Counts `n` loaded entries or assets of the request, an error past
/// [`MAX_LOADED`].
pub(super) fn spend(
	ctx: &ResolverContext,
	n: usize,
) -> async_graphql::Result<()> {
	use std::sync::atomic::Ordering;
	let viewer = ctx.data::<Viewer>()?;
	if viewer.loaded.count.fetch_add(n, Ordering::Relaxed) + n > MAX_LOADED {
		return Err(GqlError::new(format!(
			"the query loads more than {MAX_LOADED} entries and assets"
		)));
	}
	Ok(())
}

/// The entries of a query, counted against [`MAX_LOADED`].
pub(super) fn load(
	ctx: &ResolverContext,
	params: &[(String, String)],
) -> async_graphql::Result<Vec<Entry>> {
	let q = query(ctx, params)?;
	let db = ctx.data::<Db>()?;
	// the query counts too, before it runs
	spend(ctx, 1)?;
	let conn = db.conn();
	let entries =
		with_drafts(&conn, ctx.data::<Viewer>()?, select(&conn, &q)?)?;
	spend(ctx, entries.len())?;
	Ok(entries)
}

/// The entry of a preview shows its draft, or every entry its own.
pub(super) fn with_drafts(
	conn: &rusqlite::Connection,
	viewer: &Viewer,
	mut entries: Vec<Entry>,
) -> async_graphql::Result<Vec<Entry>> {
	match &viewer.preview {
		Some(Preview::Entry(id, doc)) => {
			for e in entries.iter_mut().filter(|e| e.id == *id) {
				e.show_draft(doc);
			}
		}
		Some(Preview::All(sections)) => {
			// drafts only of the sections the token's user may view
			let ids: Vec<i64> = entries
				.iter()
				.filter(|e| sections.contains(&e.section))
				.map(|e| e.id)
				.collect();
			let drafts = docs_of(conn, &ids)?;
			for e in entries.iter_mut() {
				if let Some(doc) = drafts.get(&e.id) {
					e.show_draft(doc);
				}
			}
		}
		None => {}
	}
	Ok(entries)
}

/// Entries of a site in the order of the ids, the ones the viewer sees.
pub(super) async fn entries_by_id(
	ctx: &ResolverContext<'_>,
	site: &str,
	ids: &[i64],
) -> async_graphql::Result<Vec<Entry>> {
	if ids.is_empty() {
		return Ok(vec![]);
	}
	// each list counts too, before it loads
	spend(ctx, 1)?;
	let keys = ids.iter().map(|id| (site.to_string(), *id));
	let found = Loaders::of(ctx)?.entries.load_many(keys).await?;
	let entries: Vec<Entry> = ids
		.iter()
		.filter_map(|id| found.get(&(site.to_string(), *id)).cloned())
		.collect();
	spend(ctx, entries.len())?;
	Ok(entries)
}

/// The children of an entry the viewer sees with the parameters of the
/// arguments of `children`, in their order.
pub(super) async fn children_of(
	ctx: &ResolverContext<'_>,
	site: &str,
	id: i64,
	args: Vec<(String, String)>,
) -> async_graphql::Result<Vec<Entry>> {
	spend(ctx, 1)?;
	let key = (site.to_string(), args, id);
	let children = Loaders::of(ctx)?.children.load_one(key).await?;
	let children = children.unwrap_or_default();
	spend(ctx, children.len())?;
	Ok(children)
}

impl Loader<i64> for Authors {
	type Value = Author;
	type Error = GqlError;

	async fn load(
		&self,
		ids: &[i64],
	) -> async_graphql::Result<HashMap<i64, Author>> {
		Ok(crate::entries::authors(&self.0.conn(), ids)?)
	}
}

impl Loader<(String, i64)> for PublicUsers {
	type Value = Profile;
	type Error = GqlError;

	async fn load(
		&self,
		keys: &[(String, i64)],
	) -> async_graphql::Result<HashMap<(String, i64), Profile>> {
		let Source { db, schema, viewer } = &*self.0;
		let mut out = HashMap::new();
		for (site, ids) in by_site(keys) {
			let Some(mut q) = public_query(schema, viewer, site) else {
				continue;
			};
			q.ids = Some(ids);
			for p in crate::users::profile::public(&db.conn(), &q)? {
				out.insert((site.to_string(), p.id), p);
			}
		}
		Ok(out)
	}
}

/// The profile of an author if they are public to the viewer.
pub(super) async fn public_author(
	ctx: &ResolverContext<'_>,
	site: &str,
	id: i64,
) -> async_graphql::Result<Option<Profile>> {
	if !ctx.data::<Viewer>()?.scope.users {
		return Ok(None);
	}
	let key = (site.to_string(), id);
	Loaders::of(ctx)?.public_users.load_one(key).await
}

/// The author of an entry, `None` if the user is gone.
pub(super) async fn author_by_id(
	ctx: &ResolverContext<'_>,
	id: i64,
) -> async_graphql::Result<Option<Author>> {
	Loaders::of(ctx)?.authors.load_one(id).await
}

/// Assets in the order of the ids, the trashed ones left out.
pub(super) async fn assets_by_id(
	ctx: &ResolverContext<'_>,
	ids: &[i64],
) -> async_graphql::Result<Vec<Asset>> {
	if ids.is_empty() {
		return Ok(vec![]);
	}
	spend(ctx, 1)?;
	let found = Loaders::of(ctx)?
		.assets
		.load_many(ids.iter().copied())
		.await?;
	let assets: Vec<Asset> =
		ids.iter().filter_map(|id| found.get(id).cloned()).collect();
	spend(ctx, assets.len())?;
	Ok(assets)
}

/// The entries and assets that the links in a value point to, by id: the
/// link of a field, the items of a navigation, the links of a rich text.
#[derive(Default)]
pub(super) struct Targets {
	pub entries: HashMap<i64, Entry>,
	pub assets: HashMap<i64, Asset>,
}

pub(super) async fn targets(
	ctx: &ResolverContext<'_>,
	site: &str,
	value: &Value,
) -> async_graphql::Result<Targets> {
	let (mut entries, mut assets) = (vec![], vec![]);
	link_ids(value, &mut entries, &mut assets);
	let entries = entries_by_id(ctx, site, &entries).await?;
	let assets = assets_by_id(ctx, &assets).await?;
	Ok(Targets {
		entries: entries.into_iter().map(|e| (e.id, e)).collect(),
		assets: assets.into_iter().map(|a| (a.id, a)).collect(),
	})
}

/// The ids of links `{ "type": "entry", "value": 5 }` anywhere in a value.
fn link_ids(value: &Value, entries: &mut Vec<i64>, assets: &mut Vec<i64>) {
	match value {
		Value::Object(o) => {
			let id = o.get("value").and_then(Value::as_i64);
			match (link_kind(value), id) {
				(Some(LinkKind::Entry), Some(id)) if !entries.contains(&id) => {
					entries.push(id)
				}
				(Some(LinkKind::Asset), Some(id)) if !assets.contains(&id) => {
					assets.push(id)
				}
				_ => {}
			}
			for v in o.values() {
				link_ids(v, entries, assets);
			}
		}
		Value::Array(a) => {
			for v in a {
				link_ids(v, entries, assets);
			}
		}
		_ => {}
	}
}
