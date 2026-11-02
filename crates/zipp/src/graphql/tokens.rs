//! Tokens of frontends, like Craft's GraphQL schemas: each reads some
//! sections and sites, maybe drafts with preview tokens. Admins make them
//! in the studio; the token of `serve --graphql-token` reads everything.

use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

use crate::content::FieldError;
use crate::db::{now, random_token, token_hash};
use crate::error::{Error, Result};
use crate::schema::Schema;

/// What a request may read in GraphQL.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scope {
	/// handles of sections, `None` all
	pub sections: Option<Vec<String>>,
	/// handles of sites, `None` all
	pub sites: Option<Vec<String>>,
	/// whether preview tokens show drafts
	pub previews: bool,
}

impl Scope {
	/// Everything: admins, the token of `serve` and development.
	pub const ALL: Scope = Scope {
		sections: None,
		sites: None,
		previews: true,
	};

	pub fn section(&self, handle: &str) -> bool {
		self.sections
			.as_ref()
			.is_none_or(|l| l.iter().any(|s| s == handle))
	}

	pub fn site(&self, handle: &str) -> bool {
		self.sites
			.as_ref()
			.is_none_or(|l| l.iter().any(|s| s == handle))
	}
}

/// A token as the studio lists it, without the token itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphqlToken {
	pub id: i64,
	pub name: String,
	#[serde(flatten)]
	pub scope: Scope,
	/// in milliseconds, like the times below
	pub created_at: i64,
	pub expires_at: Option<i64>,
	pub last_used_at: Option<i64>,
}

const TOKEN: &str = "SELECT id, name, sections, sites, previews, created_at,
	expires_at, last_used_at FROM graphql_tokens";

fn token_row(r: &rusqlite::Row) -> rusqlite::Result<GraphqlToken> {
	let list = |i: usize| -> rusqlite::Result<Option<Vec<String>>> {
		let json: Option<String> = r.get(i)?;
		Ok(json.and_then(|j| serde_json::from_str(&j).ok()))
	};
	Ok(GraphqlToken {
		id: r.get(0)?,
		name: r.get(1)?,
		scope: Scope {
			sections: list(2)?,
			sites: list(3)?,
			previews: r.get(4)?,
		},
		created_at: r.get(5)?,
		expires_at: r.get(6)?,
		last_used_at: r.get(7)?,
	})
}

/// All tokens, by name.
pub fn list(conn: &Connection) -> Result<Vec<GraphqlToken>> {
	let mut stmt = conn.prepare(&format!("{TOKEN} ORDER BY name, id"))?;
	let tokens = stmt
		.query_map([], token_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(tokens)
}

pub fn get(conn: &Connection, id: i64) -> Result<GraphqlToken> {
	let sql = format!("{TOKEN} WHERE id = ?1");
	Ok(conn.query_row(&sql, [id], token_row)?)
}

/// The mistakes of a new token: a name, sections and sites the schema has.
pub fn check(schema: &Schema, name: &str, scope: &Scope) -> Vec<FieldError> {
	let mut errors = vec![];
	let mut error = |path: &str, message: String| {
		errors.push(FieldError::new(path, message))
	};
	if name.trim().is_empty() {
		error("name", "A token needs a name".into());
	}
	for s in scope.sections.iter().flatten() {
		if schema.get_section(s).is_none() {
			error("sections", format!("There is no section {s}"));
		}
	}
	for s in scope.sites.iter().flatten() {
		if schema.get_site(s).is_none() {
			error("sites", format!("There is no site {s}"));
		}
	}
	errors
}

/// A new token, and the token itself: it shows only now.
pub fn create(
	conn: &Connection,
	name: &str,
	scope: &Scope,
	expires_at: Option<i64>,
) -> Result<(GraphqlToken, String)> {
	let token = random_token();
	let json = |l: &Option<Vec<String>>| {
		l.as_ref().map(|l| serde_json::to_string(l).unwrap())
	};
	conn.execute(
		"INSERT INTO graphql_tokens
		(name, hash, sections, sites, previews, created_at, expires_at)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
		params![
			name.trim(),
			token_hash(&token),
			json(&scope.sections),
			json(&scope.sites),
			scope.previews,
			now(),
			expires_at,
		],
	)?;
	Ok((get(conn, conn.last_insert_rowid())?, token))
}

/// Revokes a token, frontends that send it get 401.
pub fn delete(conn: &Connection, id: i64) -> Result<()> {
	match conn.execute("DELETE FROM graphql_tokens WHERE id = ?1", [id])? {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

/// The scope of a token that hasn't expired, and it was used now.
pub fn scope_of(conn: &Connection, token: &str) -> Result<Option<Scope>> {
	let sql = format!("{TOKEN} WHERE hash = ?1");
	let Some(t) = conn
		.query_row(&sql, [token_hash(token)], token_row)
		.optional()?
	else {
		return Ok(None);
	};
	if t.expires_at.is_some_and(|e| e <= now()) {
		return Ok(None);
	}
	// a write per minute is enough to see whether a token is in use
	conn.execute(
		"UPDATE graphql_tokens SET last_used_at = ?1
		WHERE id = ?2 AND (last_used_at IS NULL OR last_used_at < ?1 - 60000)",
		params![now(), t.id],
	)?;
	Ok(Some(t.scope))
}
