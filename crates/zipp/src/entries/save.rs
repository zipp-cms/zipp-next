use super::*;

/// Creates an entry in all sites, or updates it in one site when `id` is
/// given. Shared fields are copied to the other sites. Section and type
/// of an entry can't change.
pub fn save(
	conn: &Connection,
	schema: &Schema,
	id: Option<i64>,
	input: EntryInput,
) -> Result<Entry> {
	let c = check(conn, schema, id, input)?;
	let tx = conn.unchecked_transaction()?;
	let id = write(&tx, schema, &c)?;
	tx.commit()?;
	get_entry(conn, id, &c.site)
}

/// Validates a save without writing anything.
pub fn check(
	conn: &Connection,
	schema: &Schema,
	id: Option<i64>,
	input: EntryInput,
) -> Result<Checked> {
	let site = input
		.site
		.clone()
		.unwrap_or_else(|| schema.primary_site().handle.clone());
	if schema.get_site(&site).is_none() {
		return Err(Error::Request(format!("unknown site {site}")));
	}
	let section = schema
		.get_section(&input.section)
		.filter(|s| s.types.contains(&input.ty))
		.ok_or_else(|| {
			Error::Request(format!(
				"no type {} in section {}",
				input.ty, input.section
			))
		})?;
	let comp = schema.entry_type(&input.ty).unwrap();

	let parent_id = match id {
		Some(id) => {
			let e = get_entry(conn, id, &site)?;
			if e.section != input.section || e.ty != input.ty {
				return Err(Error::Request(
					"section and type of an entry can't change".into(),
				));
			}
			e.parent_id
		}
		None if section.kind == SectionKind::Single => {
			return Err(Error::Request(
				"a single has its entry already".into(),
			));
		}
		None => {
			if let Some(p) = input.parent_id {
				check_parent(conn, section, p)?;
			}
			input.parent_id
		}
	};

	let mut errors = vec![];
	let title = input.title.trim().to_string();
	if title.is_empty() {
		errors.push(field_error("title", "is required"));
	}
	let slug = match input.slug.trim() {
		"" => slugify(&title),
		s => slugify(s),
	};

	// the uri in every site, a new entry starts with this slug everywhere
	let mut uris = HashMap::new();
	for s in &schema.sites {
		if id.is_some() && s.handle != site {
			continue;
		}
		let uri = uri_for(
			conn,
			section,
			&input.ty,
			&slug,
			parent_id,
			&s.handle,
			&input.fields,
		)?;
		if let Some(uri) = &uri {
			let taken: Option<i64> = conn
				.query_row(
					"SELECT entry_id FROM entry_sites WHERE site = ?1 AND uri = ?2",
					params![s.handle, uri],
					|r| r.get(0),
				)
				.optional()?;
			if taken.is_some() && taken != id {
				let msg = format!("the uri /{uri} is already used");
				errors.push(field_error("slug", &msg));
				break;
			}
		}
		uris.insert(s.handle.clone(), uri);
	}

	let fields = match validate(schema, comp, &input.fields) {
		Ok(f) => f,
		Err(e) => {
			errors.extend(e);
			Value::Null
		}
	};
	if errors.is_empty() {
		errors.extend(check_relations(conn, schema, &input.ty, &fields)?);
	}
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}

	Ok(Checked {
		id,
		site,
		section: section.clone(),
		ty: input.ty,
		published: input.published,
		parent_id,
		title: title.to_string(),
		slug,
		uris,
		fields,
	})
}

/// Writes a checked save, returns the id of the entry.
pub fn write(conn: &Connection, schema: &Schema, c: &Checked) -> Result<i64> {
	let comp = schema.entry_type(&c.ty).unwrap();
	let section = &c.section;
	let (site, title, slug, uris, fields) =
		(&c.site, &c.title, &c.slug, &c.uris, &c.fields);

	let now = now();
	let id = match c.id {
		Some(id) => {
			conn.execute(
				"UPDATE entry_sites SET title = ?1, slug = ?2, uri = ?3,
				published = ?4, fields = ?5, updated_at = ?6
				WHERE entry_id = ?7 AND site = ?8",
				params![
					title,
					slug,
					uris[site],
					c.published,
					fields.to_string(),
					now,
					id,
					site
				],
			)?;
			reindex(conn, schema, id, site, &c.ty, fields)?;

			for other in schema.sites.iter().filter(|s| &s.handle != site) {
				let theirs: String = conn.query_row(
					"SELECT fields FROM entry_sites
					WHERE entry_id = ?1 AND site = ?2",
					params![id, other.handle],
					|r| r.get(0),
				)?;
				let theirs = serde_json::from_str(&theirs).unwrap_or_default();
				let theirs = share(comp, fields, &theirs);
				conn.execute(
					"UPDATE entry_sites SET fields = ?1
					WHERE entry_id = ?2 AND site = ?3",
					params![theirs.to_string(), id, other.handle],
				)?;
				reindex(conn, schema, id, &other.handle, &c.ty, &theirs)?;
			}

			conn.execute(
				"UPDATE entries SET updated_at = ?1 WHERE id = ?2",
				params![now, id],
			)?;
			// children follow the uri of their parent, the uri of other
			// sites a field that is the same in all of them
			if section.kind == SectionKind::Structure
				|| section.uri_if.is_some()
			{
				refresh_uris(conn, schema, section)?;
			}
			id
		}
		None => {
			let position: i64 = conn.query_row(
				"SELECT IFNULL(MAX(position) + 1, 0) FROM entries
				WHERE section = ?1 AND parent_id IS ?2",
				params![section.handle, c.parent_id],
				|r| r.get(0),
			)?;
			conn.execute(
				"INSERT INTO entries
				(section, type, parent_id, position, created_at, updated_at)
				VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
				params![section.handle, c.ty, c.parent_id, position, now],
			)?;
			let id = conn.last_insert_rowid();

			for s in &schema.sites {
				conn.execute(
					"INSERT INTO entry_sites (entry_id, site, title, slug, uri,
					published, fields, updated_at)
					VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
					params![
						id,
						s.handle,
						title,
						slug,
						uris[&s.handle],
						c.published,
						fields.to_string(),
						now
					],
				)?;
				reindex(conn, schema, id, &s.handle, &c.ty, fields)?;
			}
			id
		}
	};
	Ok(id)
}

pub(super) fn field_error(path: &str, message: &str) -> FieldError {
	FieldError {
		path: path.into(),
		message: message.into(),
	}
}

/// Relations must point to existing entries of the allowed types, asset
/// fields to existing assets (images if they only take images).
pub(super) fn check_relations(
	conn: &Connection,
	schema: &Schema,
	ty: &str,
	fields: &Value,
) -> Result<Vec<FieldError>> {
	let comp = schema.get(ty).unwrap();
	let mut errors = vec![];

	for rel in relations(schema, comp, fields) {
		let (sql, what) = match rel.target {
			Target::Entries(_) => {
				("SELECT type FROM entries WHERE id = ?1", "entry")
			}
			Target::Assets(_) => {
				("SELECT kind FROM assets WHERE id = ?1", "asset")
			}
		};
		for id in rel.ids {
			let found: Option<String> =
				conn.query_row(sql, [id], |r| r.get(0)).optional()?;
			let error = match (&rel.target, found) {
				(_, None) => format!("{what} {id} doesn't exist"),
				(Target::Entries(types), Some(t)) if !types.contains(&t) => {
					format!("entry {id} is a {t}, not allowed here")
				}
				(Target::Assets(true), Some(k)) if k != "image" => {
					format!("asset {id} is not an image")
				}
				_ => continue,
			};
			errors.push(field_error(&rel.path, &error));
		}
	}

	Ok(errors)
}

/// Rewrites the index rows of an entry in a site from its fields.
pub fn reindex(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	site: &str,
	ty: &str,
	fields: &Value,
) -> Result<()> {
	let Some(comp) = schema.get(ty) else {
		return Ok(());
	};
	conn.execute(
		"DELETE FROM entry_values WHERE entry_id = ?1 AND site = ?2",
		params![id, site],
	)?;
	conn.execute(
		"DELETE FROM entry_relations WHERE entry_id = ?1 AND site = ?2",
		params![id, site],
	)?;
	conn.execute(
		"DELETE FROM asset_relations WHERE entry_id = ?1 AND site = ?2",
		params![id, site],
	)?;

	for v in index_values(comp, fields) {
		conn.execute(
			"INSERT INTO entry_values (entry_id, site, field, text, num)
			VALUES (?1, ?2, ?3, ?4, ?5)",
			params![id, site, v.field, v.text, v.num],
		)?;
	}
	let rels = relations(schema, comp, fields);
	// where an asset is used, for the studio
	for rel in rels
		.iter()
		.filter(|r| matches!(r.target, Target::Assets(_)))
	{
		for asset in &rel.ids {
			conn.execute(
				"INSERT INTO asset_relations (entry_id, site, path, asset_id)
				VALUES (?1, ?2, ?3, ?4)",
				params![id, site, rel.path, asset],
			)?;
		}
	}
	for rel in rels
		.iter()
		.filter(|r| matches!(r.target, Target::Entries(_)))
	{
		for (pos, target) in rel.ids.iter().enumerate() {
			conn.execute(
				"INSERT INTO entry_relations
				(entry_id, site, path, target_id, position)
				VALUES (?1, ?2, ?3, ?4, ?5)",
				params![id, site, rel.path, target, pos],
			)?;
		}
	}

	Ok(())
}
