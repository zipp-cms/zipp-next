//! Public profiles of users, like WordPress' author pages: the name, a
//! slug for the page and the values of the user fields
//! ([`Schema::user_fields`](crate::Schema::user_fields)). A user is public
//! while they have a live entry, see [`PublicQuery`]; their email, role
//! and logins never are.

use rusqlite::types::Value as SqlValue;
use rusqlite::{Connection, OptionalExtension, params, params_from_iter};
use serde_json::Value;

use crate::content::{FieldError, validate};
use crate::db::marks;
use crate::entries::{LIVE_DATES, check_relations, links_of, slugify};
use crate::error::{Error, Result};
use crate::schema::{Schema, USER};

/// The public part of a user.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Profile {
	pub id: i64,
	/// empty when it isn't set
	pub name: String,
	pub slug: String,
	pub fields: Value,
}

/// A slug of `name` no other user has, `user-<id>` without a name (an
/// email would be public), `-2`, `-3`, … when it is taken.
pub(crate) fn free_slug(
	conn: &Connection,
	id: i64,
	name: &str,
) -> Result<String> {
	let base = match slugify(name) {
		s if s.is_empty() => format!("user-{id}"),
		s => s,
	};
	let mut slug = base.clone();
	for n in 2.. {
		if !slug_taken(conn, id, &slug)? {
			break;
		}
		slug = format!("{base}-{n}");
	}
	Ok(slug)
}

fn slug_taken(conn: &Connection, id: i64, slug: &str) -> Result<bool> {
	Ok(conn.query_row(
		"SELECT EXISTS (SELECT 1 FROM users WHERE slug = ?1 AND id <> ?2)",
		params![slug, id],
		|r| r.get(0),
	)?)
}

pub(crate) fn set_slug(conn: &Connection, id: i64, slug: &str) -> Result<()> {
	match conn.execute(
		"UPDATE users SET slug = ?1 WHERE id = ?2",
		params![slug, id],
	)? {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

/// Gives users from before slugs (an empty one) one of their name, on
/// start.
pub(crate) fn fill_slugs(conn: &Connection) -> Result<()> {
	let mut stmt =
		conn.prepare("SELECT id, name FROM users WHERE slug = '' ORDER BY id")?;
	let users = stmt
		.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
		.collect::<rusqlite::Result<Vec<(i64, String)>>>()?;
	for (id, name) in users {
		set_slug(conn, id, &free_slug(conn, id, &name)?)?;
	}
	Ok(())
}

/// A slug an admin gives a user, cleaned up like an entry's, or what is
/// wrong with it.
pub(crate) fn check_slug(
	conn: &Connection,
	id: i64,
	slug: &str,
) -> Result<std::result::Result<String, FieldError>> {
	let slug = slugify(slug);
	if slug.is_empty() {
		return Ok(Err(FieldError::new("slug", "is required")));
	}
	if slug_taken(conn, id, &slug)? {
		return Ok(Err(FieldError::new("slug", "Another user has this slug")));
	}
	Ok(Ok(slug))
}

/// The values of the user fields cleaned up, or what is wrong with them
/// (by path under `fields.`); nothing to keep without user fields. `by`
/// is who saves, the relations are the ones they may see.
pub(crate) fn check_fields(
	conn: &Connection,
	schema: &Schema,
	fields: &Value,
	old: &Value,
	by: i64,
) -> Result<Value> {
	let Some(comp) = schema.get(USER) else {
		return Ok(Value::Object(Default::default()));
	};
	let in_fields = |errors: Vec<FieldError>| {
		let errors = errors
			.into_iter()
			.map(|e| FieldError::new(format!("fields.{}", e.path), e.message));
		Error::Invalid(errors.collect())
	};
	let mut fields = validate(schema, comp, fields).map_err(in_fields)?;
	let before = links_of(schema, comp, old);
	let errors =
		check_relations(conn, schema, USER, &mut fields, Some(by), &before)?;
	if !errors.is_empty() {
		return Err(in_fields(errors));
	}
	Ok(fields)
}

pub(crate) fn set_fields(
	conn: &Connection,
	id: i64,
	fields: &Value,
) -> Result<()> {
	match conn.execute(
		"UPDATE users SET fields = ?1 WHERE id = ?2",
		params![fields.to_string(), id],
	)? {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

/// Which public users to list. A user is public in a site while they
/// are the author of an entry that is live there, in one of `public`.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct PublicQuery {
	pub site: String,
	/// the sections whose entries make users public, `None` any
	pub public: Option<Vec<String>>,
	/// only the authors of live entries of this section too
	pub has_entries: Option<String>,
	pub ids: Option<Vec<i64>>,
	pub slug: Option<String>,
	/// by name (the slug without one), else by id
	pub by_id: bool,
	pub desc: bool,
	pub limit: Option<i64>,
	pub offset: i64,
}

impl PublicQuery {
	/// The `WHERE` and its values.
	fn condition(&self) -> (String, Vec<SqlValue>) {
		let mut conds = vec!["u.slug <> ''".to_string()];
		let mut values = vec![];
		// a live entry of the site in the sections, all if `None`
		let mut live = |sections: Option<&[String]>,
		                values: &mut Vec<SqlValue>| {
			let mut sql = format!(
				"EXISTS (SELECT 1 FROM entries e
				JOIN entry_sites s ON s.entry_id = e.id
				WHERE e.author_id = u.id AND e.deleted_at IS NULL
				AND s.site = ? AND s.published = 1 AND {LIVE_DATES}"
			);
			values.push(SqlValue::Text(self.site.clone()));
			if let Some(sections) = sections {
				sql +=
					&format!(" AND e.section IN ({})", marks(sections.len()));
				values.extend(sections.iter().cloned().map(SqlValue::Text));
			}
			conds.push(sql + ")");
		};
		live(self.public.as_deref(), &mut values);
		if let Some(s) = &self.has_entries {
			live(Some(std::slice::from_ref(s)), &mut values);
		}
		if let Some(ids) = &self.ids {
			conds.push(format!("u.id IN ({})", marks(ids.len())));
			values.extend(ids.iter().map(|i| SqlValue::Integer(*i)));
		}
		if let Some(slug) = &self.slug {
			conds.push("u.slug = ?".into());
			values.push(SqlValue::Text(slug.clone()));
		}
		(conds.join(" AND "), values)
	}

	/// Whether nobody can match: no section makes users public.
	fn nobody(&self) -> bool {
		self.public.as_ref().is_some_and(|p| p.is_empty())
			|| self.ids.as_ref().is_some_and(|i| i.is_empty())
	}
}

/// The public users of a query.
pub(crate) fn public(
	conn: &Connection,
	q: &PublicQuery,
) -> Result<Vec<Profile>> {
	if q.nobody() {
		return Ok(vec![]);
	}
	let (cond, mut values) = q.condition();
	let order = match q.by_id {
		true => "u.id",
		false => {
			"CASE u.name WHEN '' THEN u.slug ELSE u.name END \
			COLLATE NOCASE"
		}
	};
	let dir = if q.desc { "DESC" } else { "ASC" };
	values.push(SqlValue::Integer(q.limit.unwrap_or(-1)));
	values.push(SqlValue::Integer(q.offset));
	let sql = format!(
		"SELECT u.id, u.name, u.slug, u.fields FROM users u WHERE {cond}
		ORDER BY {order} {dir}, u.id {dir} LIMIT ? OFFSET ?"
	);
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query_map(params_from_iter(values), |r| {
		Ok(Profile {
			id: r.get(0)?,
			name: r.get(1)?,
			slug: r.get(2)?,
			fields: serde_json::from_str(&r.get::<_, String>(3)?)
				.unwrap_or_default(),
		})
	})?;
	Ok(rows.collect::<rusqlite::Result<_>>()?)
}

/// The number of public users of a query, without its limit and offset.
pub(crate) fn public_count(conn: &Connection, q: &PublicQuery) -> Result<i64> {
	if q.nobody() {
		return Ok(0);
	}
	let (cond, values) = q.condition();
	let sql = format!("SELECT COUNT(*) FROM users u WHERE {cond}");
	Ok(conn.query_row(&sql, params_from_iter(values), |r| r.get(0))?)
}

/// The slug of a user without a name, `user-<id>`: a name they set later
/// gives them one of it.
pub(crate) fn rename_placeholder(
	conn: &Connection,
	id: i64,
	name: &str,
) -> Result<()> {
	let slug: Option<String> = conn
		.query_row("SELECT slug FROM users WHERE id = ?1", [id], |r| r.get(0))
		.optional()?;
	if slug == Some(format!("user-{id}")) {
		set_slug(conn, id, &free_slug(conn, id, name)?)?;
	}
	Ok(())
}

/// The uris of the pages of the public users of a site (see
/// [`Schema::user_pages`](crate::Schema::user_pages)), by name, and how
/// many there are in all; none without user pages. For sitemaps.
pub fn page_uris(
	conn: &Connection,
	schema: &Schema,
	site: &str,
	limit: i64,
	offset: i64,
) -> Result<(Vec<String>, i64)> {
	let Some(pages) = &schema.user_pages else {
		return Ok((vec![], 0));
	};
	let q = PublicQuery {
		site: site.into(),
		public: pages.section.clone().map(|s| vec![s]),
		limit: Some(limit),
		offset,
		..Default::default()
	};
	let uris = public(conn, &q)?
		.iter()
		.map(|p| pages.render_uri(site, &p.slug))
		.collect();
	Ok((uris, public_count(conn, &q)?))
}
