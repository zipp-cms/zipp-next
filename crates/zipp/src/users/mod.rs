//! The users of the studio, managed by admins. Roles are code, see
//! [`Role`](crate::Role); a user without one is an admin.

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

use crate::content::FieldError;
use crate::db::now;
use crate::error::{Error, Result};
use crate::schema::Schema;

pub mod api;
#[cfg(test)]
mod api_tests;
mod routes;

pub use routes::routes;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
	pub id: i64,
	pub email: String,
	/// empty when it isn't set
	pub name: String,
	pub role: Option<String>,
	/// of the studio, one of [`LANGUAGES`]; empty is the browser's
	pub language: String,
}

/// The languages the studio is translated to.
pub const LANGUAGES: &[&str] = &["en", "de"];

/// A user with what the studio shows about the account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
	#[serde(flatten)]
	pub user: User,
	/// in milliseconds, like the times below
	pub created_at: i64,
	pub last_login_at: Option<i64>,
	/// sessions that haven't ended: browsers it is logged in
	pub sessions: i64,
	/// invited, no password yet: the invite link sets it
	pub invited: bool,
}

const ACCOUNT: &str = "SELECT id, email, name, role, language, created_at,
	last_login_at,
	(SELECT COUNT(*) FROM sessions s
		WHERE s.user_id = users.id AND s.expires_at > ?1),
	password_hash = ''
	FROM users";

fn account(r: &rusqlite::Row) -> rusqlite::Result<Account> {
	Ok(Account {
		user: User {
			id: r.get(0)?,
			email: r.get(1)?,
			name: r.get(2)?,
			role: r.get(3)?,
			language: r.get(4)?,
		},
		created_at: r.get(5)?,
		last_login_at: r.get(6)?,
		sessions: r.get(7)?,
		invited: r.get(8)?,
	})
}

/// All users, by name or email.
pub fn list(conn: &Connection) -> Result<Vec<Account>> {
	let sql = format!(
		"{ACCOUNT} ORDER BY CASE name WHEN '' THEN email ELSE name END"
	);
	let mut stmt = conn.prepare(&sql)?;
	let users = stmt
		.query_map([now()], account)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(users)
}

pub fn get(conn: &Connection, id: i64) -> Result<Account> {
	let sql = format!("{ACCOUNT} WHERE id = ?2");
	Ok(conn.query_row(&sql, params![now(), id], account)?)
}

pub fn user(conn: &Connection, id: i64) -> Result<User> {
	Ok(get(conn, id)?.user)
}

/// What is wrong with an email, language and new password, by field. The
/// email is another user's, `id` is the user's own (`None` for a new one).
pub fn check(
	conn: &Connection,
	id: Option<i64>,
	email: &str,
	language: &str,
	password: Option<&str>,
) -> Result<Vec<FieldError>> {
	let mut errors = vec![];
	let mut error = |path: &str, message: &str| {
		errors.push(FieldError {
			path: path.into(),
			message: message.into(),
		})
	};
	let taken: bool = conn.query_row(
		"SELECT EXISTS (SELECT 1 FROM users WHERE email = ?1 AND id IS NOT ?2)",
		params![email, id],
		|r| r.get(0),
	)?;
	if !email.contains('@') {
		error("email", "An email address");
	} else if taken {
		error("email", "Another user has this email");
	}
	if !language.is_empty() && !LANGUAGES.contains(&language) {
		error("language", "A language of the studio");
	}
	if let Some(e) = password.and_then(password_error) {
		error("password", e);
	}
	Ok(errors)
}

/// What is wrong with a new password, the studio's and the cli's rule.
pub fn password_error(password: &str) -> Option<&'static str> {
	(password.chars().count() < 8).then_some("At least 8 characters")
}

/// A new user with a password's hash, see `auth::hash_password`; an
/// empty one is invited.
pub fn create(
	conn: &Connection,
	email: &str,
	name: &str,
	hash: &str,
	role: Option<&str>,
	language: &str,
) -> Result<i64> {
	conn.execute(
		"INSERT INTO users (email, name, password_hash, role, language,
			created_at)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
		params![email, name.trim(), hash, role, language, now()],
	)?;
	Ok(conn.last_insert_rowid())
}

pub fn update(
	conn: &Connection,
	id: i64,
	email: &str,
	name: &str,
	language: &str,
) -> Result<()> {
	let n = conn.execute(
		"UPDATE users SET email = ?1, name = ?2, language = ?3 WHERE id = ?4",
		params![email, name.trim(), language, id],
	)?;
	match n {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

/// A role of the schema, `None` is an admin.
pub fn check_role(schema: &Schema, role: Option<&str>) -> Result<()> {
	match role {
		Some(r) if schema.get_role(r).is_none() => {
			Err(Error::Request(format!("unknown role {r}")))
		}
		_ => Ok(()),
	}
}

pub fn set_role(conn: &Connection, id: i64, role: Option<&str>) -> Result<()> {
	let n = conn.execute(
		"UPDATE users SET role = ?1 WHERE id = ?2",
		params![role, id],
	)?;
	match n {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

pub fn delete(conn: &Connection, id: i64) -> Result<()> {
	conn.execute("DELETE FROM sessions WHERE user_id = ?1", [id])?;
	match conn.execute("DELETE FROM users WHERE id = ?1", [id])? {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}
