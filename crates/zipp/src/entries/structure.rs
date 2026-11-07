use super::*;

/// Refuses a parent outside the section or one the user doesn't see
/// (`None`: the system), the same way: an entry under a hidden page
/// would show its uri.
pub(super) fn check_parent(
	conn: &Connection,
	schema: &Schema,
	section: &Section,
	parent: i64,
	user: Option<&Authed>,
) -> Result<()> {
	if section.kind != SectionKind::Structure {
		return Err(Error::Request(
			"only entries of a structure have parents".into(),
		));
	}
	let found = find_owner(conn, parent)?;
	let fits = found.is_some_and(|(s, author)| {
		s == section.handle
			&& user
				.is_none_or(|u| u.can_entry(schema, &s, author, Access::View))
	});
	if !fits {
		return Err(Error::Request(format!(
			"the parent {parent} is not in {}",
			section.handle
		)));
	}
	Ok(())
}

/// Refuses to put `levels` levels (an entry and its children) below
/// `parent` when they go deeper than the section's max level.
pub(super) fn check_level(
	conn: &Connection,
	section: &Section,
	parent: i64,
	levels: u32,
) -> Result<()> {
	if fits_below(conn, section, parent, levels)? {
		return Ok(());
	}
	let max = section.max_level.unwrap_or_default();
	let what = match levels {
		1 => "the entry",
		_ => "the entry with its children",
	};
	Err(Error::Request(format!(
		"{what} goes too deep, {} has entries down to level {max}",
		section.handle
	)))
}

/// Whether `levels` levels fit below `parent` in the section's max level.
pub(crate) fn fits_below(
	conn: &Connection,
	section: &Section,
	parent: i64,
	levels: u32,
) -> Result<bool> {
	let Some(max) = section.max_level else {
		return Ok(true);
	};
	// the level of the parent, 1 on the top
	let level: u32 = conn.query_row(
		"WITH RECURSIVE up(id) AS (
			SELECT ?1
			UNION ALL
			SELECT e.parent_id FROM entries e JOIN up ON e.id = up.id
			WHERE e.parent_id IS NOT NULL
		)
		SELECT COUNT(*) FROM up",
		[parent],
		|r| r.get(0),
	)?;
	Ok(level + levels <= max)
}

/// How many levels an entry and its children take, 1 without children.
/// The children that are in the same state: live ones (`None`) or the
/// ones trashed with it.
pub(crate) fn levels_of(
	conn: &Connection,
	id: i64,
	deleted_at: Option<i64>,
) -> Result<u32> {
	Ok(conn.query_row(
		"WITH RECURSIVE tree(id, depth) AS (
			SELECT ?1, 1
			UNION ALL
			SELECT e.id, t.depth + 1 FROM entries e JOIN tree t
			ON e.parent_id = t.id WHERE e.deleted_at IS ?2
		)
		SELECT MAX(depth) FROM tree",
		params![id, deleted_at],
		|r| r.get(0),
	)?)
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
	let parent_uri = match parent_id {
		Some(p) => stored_uri(conn, p, site)?,
		None => None,
	};
	Ok(section.render_uri(ty, slug, parent_uri.as_deref(), site, fields))
}

/// The slug and uri of a restored entry in a site: its own, or with
/// `-2`, `-3`, … when another entry has that uri now.
pub(crate) fn free_uri(
	conn: &Connection,
	section: &Section,
	ty: &str,
	slug: &str,
	parent_id: Option<i64>,
	site: &str,
	fields: &Value,
) -> Result<(String, Option<String>)> {
	let parent_uri = match parent_id {
		Some(p) => stored_uri(conn, p, site)?,
		None => None,
	};
	for n in 1.. {
		let candidate = match n {
			1 => slug.to_string(),
			n => format!("{slug}-{n}"),
		};
		let uri = section.render_uri(
			ty,
			&candidate,
			parent_uri.as_deref(),
			site,
			fields,
		);
		match &uri {
			Some(u) if uri_taken(conn, site, u, None)? => {}
			_ => return Ok((candidate, uri)),
		}
	}
	unreachable!()
}

/// Whether an entry other than `except` has the uri in a site.
pub(super) fn uri_taken(
	conn: &Connection,
	site: &str,
	uri: &str,
	except: Option<i64>,
) -> Result<bool> {
	Ok(conn.query_row(
		"SELECT EXISTS (SELECT 1 FROM entry_sites
		WHERE site = ?1 AND uri = ?2 AND entry_id IS NOT ?3)",
		params![site, uri, except],
		|r| r.get(0),
	)?)
}

/// The position at the end of the entries under a parent (`None`: the
/// top) of a section; the trash doesn't count, it gets a new position
/// when it is restored.
pub(crate) fn next_position(
	conn: &Connection,
	section: &str,
	parent_id: Option<i64>,
) -> Result<i64> {
	Ok(conn.query_row(
		"SELECT IFNULL(MAX(position) + 1, 0) FROM entries
		WHERE section = ?1 AND parent_id IS ?2 AND deleted_at IS NULL",
		params![section, parent_id],
		|r| r.get(0),
	)?)
}

/// The uri an entry has in a site.
fn stored_uri(
	conn: &Connection,
	id: i64,
	site: &str,
) -> Result<Option<String>> {
	Ok(conn
		.query_row(
			"SELECT uri FROM entry_sites WHERE entry_id = ?1 AND site = ?2",
			params![id, site],
			|r| r.get(0),
		)
		.optional()?
		.flatten())
}

/// Recomputes the uris of the entries of a section in all sites, parents
/// before their children: of all of them, or of an entry and the ones
/// below it, the others keep theirs.
pub(crate) fn refresh_uris(
	conn: &Connection,
	schema: &Schema,
	section: &Section,
	from: Option<i64>,
) -> Result<()> {
	let mut stmt = conn.prepare(
		"WITH RECURSIVE tree(id, depth) AS (
			SELECT id, 0 FROM entries
			WHERE section = ?1 AND deleted_at IS NULL
			AND (id = ?3 OR (?3 IS NULL AND parent_id IS NULL))
			UNION ALL
			SELECT e.id, t.depth + 1 FROM entries e JOIN tree t
			ON e.parent_id = t.id WHERE e.deleted_at IS NULL
		)
		SELECT t.id, e.parent_id, s.slug, s.uri, e.type, s.fields FROM tree t
		JOIN entries e ON e.id = t.id
		JOIN entry_sites s ON s.entry_id = t.id AND s.site = ?2
		ORDER BY t.depth",
	)?;

	for site in &schema.sites {
		let rows = stmt
			.query_map(params![section.handle, site.handle, from], |r| {
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
			let parent_uri = match parent {
				Some(p) if !uris.contains_key(&p) => {
					stored_uri(conn, p, &site.handle)?
				}
				Some(p) => uris[&p].clone(),
				None => None,
			};
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
				.map_err(|e| match e.sqlite_error_code() {
					// the unique index of uris
					Some(rusqlite::ErrorCode::ConstraintViolation) => {
						Error::Request(format!(
							"the uri /{} is already used",
							uri.clone().unwrap_or_default()
						))
					}
					_ => e.into(),
				})?;
			}
			uris.insert(id, uri);
		}
	}

	Ok(())
}

/// Moves an entry of a structure under `parent` (`None` is the top) at
/// `index` among its new siblings.
pub(crate) fn move_entry(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	parent: Option<i64>,
	index: usize,
	user: Option<&Authed>,
) -> Result<()> {
	let section: String = conn.query_row(
		"SELECT section FROM entries WHERE id = ?1",
		[id],
		|r| r.get(0),
	)?;
	let section = schema.get_section(&section).unwrap();
	if let Some(p) = parent {
		check_parent(conn, schema, section, p, user)?;

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
		check_level(conn, section, p, levels_of(conn, id, None)?)?;
	} else if section.kind != SectionKind::Structure {
		return Err(Error::Request("only entries of a structure move".into()));
	}

	let tx = conn.unchecked_transaction()?;
	let mut stmt = tx.prepare(
		"SELECT id FROM entries WHERE section = ?1 AND parent_id IS ?2
		AND id != ?3 AND deleted_at IS NULL ORDER BY position",
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
	refresh_uris(&tx, schema, section, Some(id))?;
	tx.commit()?;

	Ok(())
}
