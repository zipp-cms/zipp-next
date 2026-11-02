//! Change requests: the content model is code, so an editor who needs
//! something else (a field, a block, a section) writes it on a page of
//! Setup. Admins see the list and tick what is done.

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use crate::content::FieldError;
use crate::db::now;
use crate::error::{Error, Result};
use crate::mail::Mail;

pub mod api;
#[cfg(test)]
mod api_tests;
mod routes;

pub(crate) use routes::routes;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRequest {
	pub id: i64,
	/// the path of the Setup page it was written on
	pub page: String,
	/// what that page shows, like `Entry types › Page`
	pub context: String,
	pub text: String,
	/// the name or email of who wrote it, `None` once deleted
	pub user: Option<String>,
	/// in milliseconds
	pub created_at: i64,
	pub done: bool,
}

const REQUEST: &str = "SELECT r.id, r.page, r.context, r.text,
	CASE u.name WHEN '' THEN u.email ELSE u.name END, r.created_at, r.done
	FROM change_requests r LEFT JOIN users u ON u.id = r.user_id";

fn request_row(r: &rusqlite::Row) -> rusqlite::Result<ChangeRequest> {
	Ok(ChangeRequest {
		id: r.get(0)?,
		page: r.get(1)?,
		context: r.get(2)?,
		text: r.get(3)?,
		user: r.get(4)?,
		created_at: r.get(5)?,
		done: r.get(6)?,
	})
}

/// All requests, the open ones first, the newest first.
pub fn list(conn: &Connection) -> Result<Vec<ChangeRequest>> {
	let sql =
		format!("{REQUEST} ORDER BY r.done, r.created_at DESC, r.id DESC");
	let mut stmt = conn.prepare(&sql)?;
	let all = stmt
		.query_map([], request_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(all)
}

pub fn create(
	conn: &Connection,
	user_id: i64,
	page: &str,
	context: &str,
	text: &str,
) -> Result<ChangeRequest> {
	if !studio_path(page) {
		return Err(Error::Invalid(vec![FieldError {
			path: "page".into(),
			message: "Not a page of the studio".into(),
		}]));
	}
	if text.trim().is_empty() {
		return Err(Error::Invalid(vec![FieldError {
			path: "text".into(),
			message: "Write what you need".into(),
		}]));
	}
	conn.execute(
		"INSERT INTO change_requests (user_id, page, context, text, created_at)
		VALUES (?1, ?2, ?3, ?4, ?5)",
		params![user_id, page, context, text.trim(), now()],
	)?;
	let sql = format!("{REQUEST} WHERE r.id = ?1");
	Ok(conn.query_row(&sql, [conn.last_insert_rowid()], request_row)?)
}

/// Whether `page` is a path of the studio. Admins open it as a link and
/// the mail appends it to the studio's url, so anything else (a
/// `javascript:` url, `@other.host`) could lead them elsewhere.
fn studio_path(page: &str) -> bool {
	page.starts_with("/admin/")
		&& page.len() <= 500
		&& !page.chars().any(|c| c.is_whitespace() || c.is_control())
		&& !page.contains('\\')
}

pub fn set_done(conn: &Connection, id: i64, done: bool) -> Result<()> {
	let n = conn.execute(
		"UPDATE change_requests SET done = ?2 WHERE id = ?1",
		params![id, done],
	)?;
	if n == 0 {
		return Err(Error::NotFound);
	}
	Ok(())
}

/// The emails of the admins, they get a mail of each request.
pub fn admin_emails(conn: &Connection) -> Result<Vec<String>> {
	let mut stmt = conn.prepare(
		"SELECT email FROM users WHERE role IS NULL AND password_hash != ''",
	)?;
	let emails = stmt
		.query_map([], |r| r.get(0))?
		.collect::<rusqlite::Result<_>>()?;
	Ok(emails)
}

/// The mail to an admin about a request, `url` the studio's.
pub fn mail(to: &str, r: &ChangeRequest, url: &str) -> Mail {
	let who = r.user.as_deref().unwrap_or("Someone");
	let text = format!(
		"{who} asks for a change on {} ({url}{}):\n\n{}\n\n\
		All requests: {url}/admin/setup/requests",
		r.context, r.page, r.text
	);
	Mail::new(to, &format!("A change request: {}", r.context), &text)
}
