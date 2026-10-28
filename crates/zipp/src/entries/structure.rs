use super::*;

pub(super) fn check_parent(
	conn: &Connection,
	section: &Section,
	parent: i64,
) -> Result<()> {
	if section.kind != SectionKind::Structure {
		return Err(Error::Request(
			"only entries of a structure have parents".into(),
		));
	}
	let parent_section: Option<String> = conn
		.query_row("SELECT section FROM entries WHERE id = ?1", [parent], |r| {
			r.get(0)
		})
		.optional()?;
	if parent_section.as_ref() != Some(&section.handle) {
		return Err(Error::Request(format!(
			"the parent {parent} is not in {}",
			section.handle
		)));
	}
	Ok(())
}

/// The uri of an entry in a site, after the uri of its parent.
pub(super) fn uri_for(
	conn: &Connection,
	section: &Section,
	ty: &str,
	slug: &str,
	parent_id: Option<i64>,
	site: &str,
	fields: &Value,
) -> Result<Option<String>> {
	let parent_uri: Option<String> = match parent_id {
		Some(p) => conn
			.query_row(
				"SELECT uri FROM entry_sites WHERE entry_id = ?1 AND site = ?2",
				params![p, site],
				|r| r.get(0),
			)
			.optional()?
			.flatten(),
		None => None,
	};
	Ok(section.render_uri(ty, slug, parent_uri.as_deref(), site, fields))
}

/// Recomputes the uris of all entries of a section in all sites, parents
/// before their children.
pub fn refresh_uris(
	conn: &Connection,
	schema: &Schema,
	section: &Section,
) -> Result<()> {
	let mut stmt = conn.prepare(
		"WITH RECURSIVE tree(id, depth) AS (
			SELECT id, 0 FROM entries
			WHERE section = ?1 AND parent_id IS NULL
			UNION ALL
			SELECT e.id, t.depth + 1 FROM entries e JOIN tree t
			ON e.parent_id = t.id
		)
		SELECT t.id, e.parent_id, s.slug, s.uri, e.type, s.fields FROM tree t
		JOIN entries e ON e.id = t.id
		JOIN entry_sites s ON s.entry_id = t.id AND s.site = ?2
		ORDER BY t.depth",
	)?;

	for site in &schema.sites {
		let rows = stmt
			.query_map(params![section.handle, site.handle], |r| {
				Ok((
					r.get(0)?,
					r.get(1)?,
					r.get(2)?,
					r.get(3)?,
					r.get(4)?,
					r.get::<_, String>(5)?,
				))
			})?
			.collect::<rusqlite::Result<
				Vec<(i64, Option<i64>, String, Option<String>, String, String)>,
			>>()?;

		let mut uris: HashMap<i64, Option<String>> = HashMap::new();
		for (id, parent, slug, old, ty, fields) in rows {
			let parent_uri =
				parent.and_then(|p| uris.get(&p).cloned().flatten());
			let fields = serde_json::from_str(&fields).unwrap_or_default();
			let uri = section.render_uri(
				&ty,
				&slug,
				parent_uri.as_deref(),
				&site.handle,
				&fields,
			);
			if uri != old {
				conn.execute(
					"UPDATE entry_sites SET uri = ?1
					WHERE entry_id = ?2 AND site = ?3",
					params![uri, id, site.handle],
				)
				.map_err(|_| {
					Error::Request(format!(
						"the uri /{} is already used",
						uri.clone().unwrap_or_default()
					))
				})?;
			}
			uris.insert(id, uri);
		}
	}

	Ok(())
}

/// Moves an entry of a structure under `parent` (`None` is the top) at
/// `index` among its new siblings.
pub fn move_entry(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	parent: Option<i64>,
	index: usize,
) -> Result<()> {
	let section: String = conn.query_row(
		"SELECT section FROM entries WHERE id = ?1",
		[id],
		|r| r.get(0),
	)?;
	let section = schema.get_section(&section).unwrap();
	if let Some(p) = parent {
		check_parent(conn, section, p)?;

		// the new parent can't be the entry or below it
		let mut up = Some(p);
		while let Some(cur) = up {
			if cur == id {
				return Err(Error::Request(
					"can't move an entry below itself".into(),
				));
			}
			up = conn.query_row(
				"SELECT parent_id FROM entries WHERE id = ?1",
				[cur],
				|r| r.get(0),
			)?;
		}
	} else if section.kind != SectionKind::Structure {
		return Err(Error::Request("only entries of a structure move".into()));
	}

	let tx = conn.unchecked_transaction()?;
	let mut stmt = tx.prepare(
		"SELECT id FROM entries WHERE section = ?1 AND parent_id IS ?2
		AND id != ?3 ORDER BY position",
	)?;
	let mut siblings = stmt
		.query_map(params![section.handle, parent, id], |r| r.get(0))?
		.collect::<rusqlite::Result<Vec<i64>>>()?;
	drop(stmt);
	siblings.insert(index.min(siblings.len()), id);

	for (pos, sibling) in siblings.iter().enumerate() {
		tx.execute(
			"UPDATE entries SET position = ?1 WHERE id = ?2",
			params![pos, sibling],
		)?;
	}
	tx.execute(
		"UPDATE entries SET parent_id = ?1 WHERE id = ?2",
		params![parent, id],
	)?;
	refresh_uris(&tx, schema, section)?;
	tx.commit()?;

	Ok(())
}
