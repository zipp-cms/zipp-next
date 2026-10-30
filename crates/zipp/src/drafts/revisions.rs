use super::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Revision {
	pub id: i64,
	pub user: Option<String>,
	pub created_at: i64,
	/// what changed since the revision before, empty for the first
	pub changes: Vec<FieldChange>,
}

/// A field (or title, slug, published) that changed, and in which sites.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldChange {
	pub field: String,
	pub sites: Vec<String>,
}

/// What differs between two versions of an entry, in the order of the
/// fields of the newer one.
pub fn diff_docs(old: &Doc, new: &Doc) -> Vec<FieldChange> {
	let mut changes: Vec<FieldChange> = vec![];
	let mut add = |field: &str, site: &str| match changes
		.iter_mut()
		.find(|c| c.field == field)
	{
		Some(c) => c.sites.push(site.into()),
		None => changes.push(FieldChange {
			field: field.into(),
			sites: vec![site.into()],
		}),
	};

	for (site, n) in new {
		let Some(o) = old.get(site) else { continue };
		if o.title != n.title {
			add("title", site);
		}
		if o.slug != n.slug {
			add("slug", site);
		}
		if o.published != n.published {
			add("published", site);
		}
		let empty = serde_json::Map::new();
		let of = o.fields.as_object().unwrap_or(&empty);
		let nf = n.fields.as_object().unwrap_or(&empty);
		let keys = nf.keys().chain(of.keys().filter(|k| !nf.contains_key(*k)));
		for key in keys {
			if of.get(key) != nf.get(key) {
				add(key, site);
			}
		}
	}
	changes
}

/// Keeps the live state of an entry as a revision, called after every
/// save of the live entry.
pub fn record_revision(
	conn: &Connection,
	id: i64,
	user_id: Option<i64>,
) -> Result<()> {
	let doc = live_doc(conn, id)?;
	conn.execute(
		"INSERT INTO revisions (entry_id, doc, user_id, created_at)
		VALUES (?1, ?2, ?3, ?4)",
		params![id, serde_json::to_string(&doc).unwrap(), user_id, now()],
	)?;
	conn.execute(
		"DELETE FROM revisions WHERE entry_id = ?1 AND id NOT IN (
			SELECT id FROM revisions WHERE entry_id = ?1
			ORDER BY id DESC LIMIT ?2
		)",
		params![id, MAX_REVISIONS],
	)?;
	Ok(())
}

/// The revisions of an entry, newest first, each with what it changed.
pub fn revisions(conn: &Connection, id: i64) -> Result<Vec<Revision>> {
	let mut stmt = conn.prepare(
		"SELECT r.id, u.email, r.created_at, r.doc FROM revisions r
		LEFT JOIN users u ON u.id = r.user_id
		WHERE r.entry_id = ?1 ORDER BY r.id",
	)?;
	let rows = stmt
		.query_map([id], |r| {
			Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get::<_, String>(3)?))
		})?
		.collect::<rusqlite::Result<Vec<(i64, Option<String>, i64, String)>>>(
		)?;

	let mut revs = vec![];
	let mut prev: Option<Doc> = None;
	for (id, user, created_at, doc) in rows {
		let doc: Doc = serde_json::from_str(&doc).unwrap_or_default();
		let changes = match &prev {
			Some(p) => diff_docs(p, &doc),
			None => vec![],
		};
		revs.push(Revision {
			id,
			user,
			created_at,
			changes,
		});
		prev = Some(doc);
	}
	revs.reverse();
	Ok(revs)
}

pub fn get_revision(conn: &Connection, id: i64, revision: i64) -> Result<Doc> {
	let doc: String = conn.query_row(
		"SELECT doc FROM revisions WHERE id = ?1 AND entry_id = ?2",
		params![revision, id],
		|r| r.get(0),
	)?;
	Ok(serde_json::from_str(&doc).unwrap_or_default())
}

/// Puts an old revision into the draft. Sites that the revision doesn't
/// know yet keep their live content.
pub fn restore(
	conn: &Connection,
	id: i64,
	revision: i64,
	user_id: Option<i64>,
) -> Result<Draft> {
	let doc: String = conn.query_row(
		"SELECT doc FROM revisions WHERE id = ?1 AND entry_id = ?2",
		params![revision, id],
		|r| r.get(0),
	)?;
	let old: Doc = serde_json::from_str(&doc).unwrap_or_default();
	let mut doc = live_doc(conn, id)?;
	for (site, sd) in old {
		if doc.contains_key(&site) {
			doc.insert(site, sd);
		}
	}

	store_draft(conn, id, &doc, user_id)?;
	Ok(get_draft(conn, id)?.unwrap())
}
