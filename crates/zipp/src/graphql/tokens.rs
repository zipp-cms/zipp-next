//! Tokens of frontends, like Craft's: each reads what its GraphQL schema
//! of the code says ([`GraphqlSchema`]). Admins make them in the studio;
//! the token of `serve --graphql-token` reads everything.

use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

use crate::content::FieldError;
use crate::db::{now, random_token, token_hash};
use crate::error::{Error, Result};
use crate::schema::{GraphqlSchema, Schema};

/// What a request may read in GraphQL.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Scope {
	/// handles of sections, `None` all
	pub sections: Option<Vec<String>>,
	/// handles of sites, `None` all
	pub sites: Option<Vec<String>>,
	/// whether it lists assets
	pub assets: bool,
	/// whether preview tokens show drafts
	pub previews: bool,
	/// whether it reads the public profiles of users, like Craft's
	/// schema permission
	pub users: bool,
}

impl Scope {
	/// Everything: admins, the token of `serve` and development.
	pub(crate) const ALL: Scope = Scope {
		sections: None,
		sites: None,
		assets: true,
		previews: true,
		users: true,
	};

	/// What a schema of the code says.
	pub(crate) fn of(s: &GraphqlSchema) -> Scope {
		Scope {
			sections: Some(s.sections.clone()),
			sites: s.sites.clone(),
			assets: s.assets,
			previews: s.previews,
			users: s.users,
		}
	}

	pub(crate) fn section(&self, handle: &str) -> bool {
		self.sections
			.as_ref()
			.is_none_or(|l| l.iter().any(|s| s == handle))
	}

	pub(crate) fn site(&self, handle: &str) -> bool {
		self.sites
			.as_ref()
			.is_none_or(|l| l.iter().any(|s| s == handle))
	}
}

/// A token as the studio lists it, without the token itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GraphqlToken {
	pub id: i64,
	pub name: String,
	/// the handle of its [`GraphqlSchema`], one the code may not have
	/// anymore: it reads nothing then
	pub schema: String,
	/// in milliseconds, like the times below
	pub created_at: i64,
	pub expires_at: Option<i64>,
	pub last_used_at: Option<i64>,
}

const TOKEN: &str = "SELECT id, name, schema, created_at, expires_at,
	last_used_at FROM graphql_tokens";

fn token_row(r: &rusqlite::Row) -> rusqlite::Result<GraphqlToken> {
	Ok(GraphqlToken {
		id: r.get(0)?,
		name: r.get(1)?,
		schema: r.get(2)?,
		created_at: r.get(3)?,
		expires_at: r.get(4)?,
		last_used_at: r.get(5)?,
	})
}

/// All tokens, by name.
pub(crate) fn list(conn: &Connection) -> Result<Vec<GraphqlToken>> {
	let mut stmt = conn.prepare(&format!("{TOKEN} ORDER BY name, id"))?;
	let tokens = stmt
		.query_map([], token_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(tokens)
}

pub(crate) fn get(conn: &Connection, id: i64) -> Result<GraphqlToken> {
	let sql = format!("{TOKEN} WHERE id = ?1");
	Ok(conn.query_row(&sql, [id], token_row)?)
}

/// The mistakes of a new token: a name and a GraphQL schema of the code.
pub(crate) fn check(
	schema: &Schema,
	name: &str,
	graphql_schema: &str,
) -> Vec<FieldError> {
	let mut errors = vec![];
	let mut error = |path: &str, message: String| {
		errors.push(FieldError::new(path, message))
	};
	if name.trim().is_empty() {
		error("name", "A token needs a name".into());
	}
	if schema.get_graphql_schema(graphql_schema).is_none() {
		error("schema", format!("There is no schema {graphql_schema}"));
	}
	errors
}

/// A new token, and the token itself: it shows only now.
pub(crate) fn create(
	conn: &Connection,
	name: &str,
	schema: &str,
	expires_at: Option<i64>,
) -> Result<(GraphqlToken, String)> {
	let token = random_token();
	conn.execute(
		"INSERT INTO graphql_tokens
		(name, hash, schema, created_at, expires_at)
		VALUES (?1, ?2, ?3, ?4, ?5)",
		params![name.trim(), token_hash(&token), schema, now(), expires_at],
	)?;
	Ok((get(conn, conn.last_insert_rowid())?, token))
}

/// Revokes a token, frontends that send it get 401.
pub(crate) fn delete(conn: &Connection, id: i64) -> Result<()> {
	match conn.execute("DELETE FROM graphql_tokens WHERE id = ?1", [id])? {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

/// The scope of a token that hasn't expired, and it was used now; `None`
/// also for one whose schema the code doesn't have anymore.
pub(crate) fn scope_of(
	conn: &Connection,
	schema: &Schema,
	token: &str,
) -> Result<Option<Scope>> {
	let sql = format!("{TOKEN} WHERE hash = ?1");
	let Some(t) = conn
		.query_row(&sql, [token_hash(token)], token_row)
		.optional()?
	else {
		return Ok(None);
	};
	let Some(scope) = scope_in(schema, &t) else {
		return Ok(None);
	};
	// a write per minute is enough to see whether a token is in use
	conn.execute(
		"UPDATE graphql_tokens SET last_used_at = ?1
		WHERE id = ?2 AND (last_used_at IS NULL OR last_used_at < ?1 - 60000)",
		params![now(), t.id],
	)?;
	Ok(Some(scope))
}

/// What a token reads, `None` once it expired or without its schema.
fn scope_in(schema: &Schema, t: &GraphqlToken) -> Option<Scope> {
	if t.expires_at.is_some_and(|e| e <= now()) {
		return None;
	}
	schema.get_graphql_schema(&t.schema).map(Scope::of)
}
