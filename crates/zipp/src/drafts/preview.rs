use super::*;

pub fn create_token(conn: &Connection, id: i64) -> Result<String> {
	get_entry_exists(conn, id)?;
	insert_token(conn, Some(id))
}

/// A token that shows every draft.
pub fn create_site_token(conn: &Connection) -> Result<String> {
	insert_token(conn, None)
}

pub(super) fn insert_token(
	conn: &Connection,
	id: Option<i64>,
) -> Result<String> {
	let n: u128 = rand::rng().random();
	let token = format!("{n:032x}");
	conn.execute(
		"INSERT INTO preview_tokens (token, entry_id, expires_at)
		VALUES (?1, ?2, ?3)",
		params![token, id, now() + TOKEN_DURATION_MS],
	)?;
	conn.execute("DELETE FROM preview_tokens WHERE expires_at < ?1", [now()])?;
	Ok(token)
}

pub(super) fn get_entry_exists(conn: &Connection, id: i64) -> Result<()> {
	conn.query_row("SELECT id FROM entries WHERE id = ?1", [id], |_| Ok(()))?;
	Ok(())
}

/// What a preview token shows.
#[derive(Debug, Clone)]
pub enum Preview {
	/// one entry and its draft, the live content if there is none
	Entry(i64, Doc),
	/// every draft
	All,
}

/// What a valid token shows.
pub fn preview(conn: &Connection, token: &str) -> Result<Option<Preview>> {
	let id: Option<Option<i64>> = conn
		.query_row(
			"SELECT entry_id FROM preview_tokens
			WHERE token = ?1 AND expires_at > ?2",
			params![token, now()],
			|r| r.get(0),
		)
		.optional()?;
	let id = match id {
		None => return Ok(None),
		Some(None) => return Ok(Some(Preview::All)),
		Some(Some(id)) => id,
	};
	let doc = match get_draft(conn, id)? {
		Some(d) => d.doc,
		None => live_doc(conn, id)?,
	};
	Ok(Some(Preview::Entry(id, doc)))
}
