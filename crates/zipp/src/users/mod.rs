//! The users of the studio, managed by admins. Roles are code, see
//! [`Role`](crate::Role); a user without one is an admin.

use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};

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
	pub role: Option<String>,
}

pub fn list(conn: &Connection) -> Result<Vec<User>> {
	let mut stmt =
		conn.prepare("SELECT id, email, role FROM users ORDER BY email")?;
	let users = stmt
		.query_map([], |r| {
			Ok(User {
				id: r.get(0)?,
				email: r.get(1)?,
				role: r.get(2)?,
			})
		})?
		.collect::<rusqlite::Result<_>>()?;
	Ok(users)
}

pub fn email_of(conn: &Connection, id: i64) -> Result<String> {
	Ok(
		conn.query_row("SELECT email FROM users WHERE id = ?1", [id], |r| {
			r.get(0)
		})?,
	)
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
