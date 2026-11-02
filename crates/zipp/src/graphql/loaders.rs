//! Entries, children and assets by id, batched over a request: the
//! relations, links, parents and children of a list of entries load in a
//! query per site, not one per entry. A request has its own loaders, made
//! on first use; they keep what they loaded, so an entry that many link
//! to loads once.

use std::collections::{BTreeMap, HashMap};

use async_graphql::dataloader::{DataLoader, HashMapCache, Loader};

use super::*;
use crate::query::MAX_IDS;

/// The loaders of a request, see [`Loaded`].
pub(super) struct Loaders {
	entries: DataLoader<Entries, HashMapCache>,
	children: DataLoader<Children, HashMapCache>,
	assets: DataLoader<Assets, HashMapCache>,
}

/// What the loaders read with: the database, the schema and who asks.
struct Source {
	db: Db,
	schema: Arc<Schema>,
	viewer: Viewer,
}

/// Entries by site and id.
struct Entries(Arc<Source>);

/// The children of entries by site and parent, in their order.
struct Children(Arc<Source>);

struct Assets(Db);

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
				children: batched(Children(source)),
				assets: batched(Assets(db.clone())),
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

impl Loader<(String, i64)> for Children {
	type Value = Vec<Entry>;
	type Error = GqlError;

	async fn load(
		&self,
		keys: &[(String, i64)],
	) -> async_graphql::Result<HashMap<(String, i64), Vec<Entry>>> {
		let Source { db, schema, viewer } = &*self.0;
		let mut out: HashMap<_, Vec<Entry>> = HashMap::new();
		for (site, parents) in by_site(keys) {
			let params = site_params(site, [("order", "structure".into())]);
			let mut q = viewer_query(schema, viewer, &params)?;
			q.parents = parents;
			let conn = db.conn();
			let children = crate::entries::children(&conn, &q, MAX_CHILDREN)?;
			for e in with_drafts(&conn, viewer, children)? {
				let parent = e.parent_id.unwrap_or_default();
				out.entry((e.site.clone(), parent)).or_default().push(e);
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

/// The children of an entry the viewer sees, in their order.
pub(super) async fn children_of(
	ctx: &ResolverContext<'_>,
	site: &str,
	id: i64,
) -> async_graphql::Result<Vec<Entry>> {
	spend(ctx, 1)?;
	let key = (site.to_string(), id);
	let children = Loaders::of(ctx)?.children.load_one(key).await?;
	let children = children.unwrap_or_default();
	spend(ctx, children.len())?;
	Ok(children)
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
			match (o.get("type").and_then(Value::as_str), id) {
				(Some("entry"), Some(id)) if !entries.contains(&id) => {
					entries.push(id)
				}
				(Some("asset"), Some(id)) if !assets.contains(&id) => {
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
