use super::*;
use crate::db::token_hash;
use crate::schema::Access;

/// A token of `user_id` that shows the draft of an entry.
pub(crate) fn create_token(
	conn: &Connection,
	id: i64,
	user_id: i64,
) -> Result<String> {
	insert_token(conn, Some(id), user_id)
}

/// A token that shows every draft `user_id` may see.
pub(crate) fn create_site_token(
	conn: &Connection,
	user_id: i64,
) -> Result<String> {
	insert_token(conn, None, user_id)
}

fn insert_token(
	conn: &Connection,
	id: Option<i64>,
	user_id: i64,
) -> Result<String> {
	let token = crate::db::random_token();
	conn.execute(
		"INSERT INTO preview_tokens (hash, entry_id, user_id, expires_at)
		VALUES (?1, ?2, ?3, ?4)",
		params![token_hash(&token), id, user_id, now() + TOKEN_DURATION_MS],
	)?;
	conn.execute("DELETE FROM preview_tokens WHERE expires_at < ?1", [now()])?;
	Ok(token)
}

/// What a preview token shows.
#[derive(Debug, Clone)]
pub(crate) enum Preview {
	/// one entry and its draft, the live content if there is none
	Entry(i64, Doc),
	/// every draft of these sections, the ones its user may view (now,
	/// not when the token was made); the others show live
	All(Vec<String>),
}

/// What a valid token shows, as its user may see it now.
pub(crate) fn preview(
	conn: &Connection,
	schema: &Schema,
	token: &str,
) -> Result<Option<Preview>> {
	let found: Option<(Option<i64>, i64, Option<String>)> = conn
		.query_row(
			"SELECT p.entry_id, p.user_id, u.role FROM preview_tokens p
			JOIN users u ON u.id = p.user_id
			WHERE p.hash = ?1 AND p.expires_at > ?2",
			params![token_hash(token), now()],
			|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
		)
		.optional()?;
	let (id, user_id, role) = match found {
		None => return Ok(None),
		Some((None, _, role)) => {
			let role = role.as_deref().map(|r| schema.get_role(r));
			let sections = schema
				.sections
				.iter()
				.filter(|s| match role {
					// an admin
					None => true,
					Some(role) => role
						.and_then(|r| r.sections.get(&s.handle))
						.is_some_and(|a| *a >= Access::View),
				})
				.map(|s| s.handle.clone())
				.collect();
			return Ok(Some(Preview::All(sections)));
		}
		Some((Some(id), user_id, role)) => (id, user_id, role),
	};
	// a user who doesn't see the entry anymore shows it no more
	let auth = crate::auth::Authed {
		user_id,
		role,
		token: String::new(),
	};
	// nor one in the trash
	let (section, author) = match crate::entries::owner_of(conn, id) {
		Err(Error::NotFound) => return Ok(None),
		owner => owner?,
	};
	if !auth.can_entry(schema, &section, author, Access::View) {
		return Ok(None);
	}
	let (doc, _) = draft_or_live(conn, id)?;
	Ok(Some(Preview::Entry(id, doc)))
}
