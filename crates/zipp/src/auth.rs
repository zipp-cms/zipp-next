use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use axum::routing::post;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::db::{Db, now, random_token};
use crate::error::{Error, Result};
use crate::schema::{Access, Role, Schema};

const SESSION_DURATION_MS: i64 = 1000 * 60 * 60 * 24 * 30;
/// Wrong passwords of an email before it has to wait.
const MAX_FAILS: u32 = 10;
const FAIL_WINDOW_MS: i64 = 1000 * 60 * 15;

/// Failed logins per email: count and the time of the first one.
static FAILS: Mutex<BTreeMap<String, (u32, i64)>> = Mutex::new(BTreeMap::new());

/// A hash to verify against for unknown emails, so they take as long.
static DUMMY_HASH: OnceLock<String> = OnceLock::new();

/// Extractor that only succeeds with a valid `session-token` header.
pub struct Authed {
	pub user_id: i64,
	/// `None` is an admin
	pub role: Option<String>,
}

impl Authed {
	pub fn admin(&self) -> bool {
		self.role.is_none()
	}

	fn role<'a>(&self, schema: &'a Schema) -> Option<&'a Role> {
		schema.get_role(self.role.as_deref()?)
	}

	/// Whether the user may do `need` in a section.
	pub fn can(&self, schema: &Schema, section: &str, need: Access) -> bool {
		self.admin()
			|| self
				.role(schema)
				.and_then(|r| r.sections.get(section))
				.is_some_and(|a| *a >= need)
	}

	pub fn can_assets(&self, schema: &Schema, need: Access) -> bool {
		self.admin()
			|| self
				.role(schema)
				.and_then(|r| r.assets)
				.is_some_and(|a| a >= need)
	}

	pub fn can_ai(&self, schema: &Schema) -> bool {
		self.admin() || self.role(schema).is_some_and(|r| r.ai)
	}

	/// [`Authed::can`] as an error.
	pub fn check(
		&self,
		schema: &Schema,
		section: &str,
		need: Access,
	) -> Result<()> {
		match self.can(schema, section, need) {
			true => Ok(()),
			false => Err(Error::Forbidden),
		}
	}

	pub fn check_assets(&self, schema: &Schema, need: Access) -> Result<()> {
		match self.can_assets(schema, need) {
			true => Ok(()),
			false => Err(Error::Forbidden),
		}
	}

	pub fn check_admin(&self) -> Result<()> {
		match self.admin() {
			true => Ok(()),
			false => Err(Error::Forbidden),
		}
	}
}

impl<S> FromRequestParts<S> for Authed
where
	S: Send + Sync,
	Db: FromRef<S>,
{
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
		let db = Db::from_ref(state);
		let conn = db.conn();
		let Some(user_id) = session(&conn, &parts.headers)? else {
			return Err(Error::MissingSessionToken);
		};
		let role = conn.query_row(
			"SELECT role FROM users WHERE id = ?1",
			[user_id],
			|r| r.get(0),
		)?;
		Ok(Self { user_id, role })
	}
}

/// The user of a request, `None` without a token. Public routes use it to
/// show unpublished entries to editors.
pub fn session(conn: &Connection, headers: &HeaderMap) -> Result<Option<i64>> {
	let Some(token) = headers.get("session-token") else {
		return Ok(None);
	};
	let token = token.to_str().map_err(|_| Error::InvalidSessionToken)?;

	let row: Option<(i64, i64)> = conn
		.query_row(
			"SELECT user_id, created_at FROM sessions WHERE token = ?1",
			[token],
			|r| Ok((r.get(0)?, r.get(1)?)),
		)
		.optional()?;
	match row {
		Some((user_id, created)) if created + SESSION_DURATION_MS > now() => {
			Ok(Some(user_id))
		}
		_ => Err(Error::InvalidSessionToken),
	}
}

/// Adds a user or sets a new password and role, `None` is an admin.
pub fn add_user(
	conn: &Connection,
	email: &str,
	password: &str,
	role: Option<&str>,
) -> Result<()> {
	let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
		.map_err(|e| Error::Internal(e.to_string()))?;
	conn.execute(
		"INSERT INTO users (email, password_hash, role, created_at)
		VALUES (?1, ?2, ?3, ?4)
		ON CONFLICT (email) DO UPDATE SET password_hash = ?2, role = ?3",
		params![email, hash, role, now()],
	)?;
	// a new password logs out everywhere
	conn.execute(
		"DELETE FROM sessions WHERE user_id =
		(SELECT id FROM users WHERE email = ?1)",
		[email],
	)?;
	Ok(())
}

/// Whether the email may try again, see [`MAX_FAILS`].
fn may_try(email: &str) -> bool {
	let mut fails = FAILS.lock().unwrap();
	fails.retain(|_, (_, first)| *first + FAIL_WINDOW_MS > now());
	fails.get(email).is_none_or(|(n, _)| *n < MAX_FAILS)
}

fn failed(email: &str) {
	let mut fails = FAILS.lock().unwrap();
	fails.entry(email.to_string()).or_insert((0, now())).0 += 1;
}

#[derive(Debug, Deserialize)]
struct LoginReq {
	email: String,
	password: String,
}

#[derive(Debug, Serialize)]
struct LoginResp {
	token: String,
}

async fn login(
	State(db): State<Db>,
	Json(req): Json<LoginReq>,
) -> Result<Json<LoginResp>> {
	if !may_try(&req.email) {
		return Err(Error::Request(
			"Too many wrong passwords, try again in 15 minutes".into(),
		));
	}
	let user: Option<(i64, String)> = db
		.conn()
		.query_row(
			"SELECT id, password_hash FROM users WHERE email = ?1",
			[&req.email],
			|r| Ok((r.get(0)?, r.get(1)?)),
		)
		.optional()?;

	// bcrypt takes a while: not with the database locked, not on the
	// async workers, and as long for unknown emails
	let known = user.is_some();
	let (user_id, hash) = user.unwrap_or_else(|| {
		let dummy = DUMMY_HASH.get_or_init(|| {
			bcrypt::hash("not a password", bcrypt::DEFAULT_COST).unwrap()
		});
		(0, dummy.clone())
	});
	let password = req.password.clone();
	let ok = tokio::task::spawn_blocking(move || {
		bcrypt::verify(password, &hash).unwrap_or(false)
	})
	.await
	.map_err(|e| Error::Internal(e.to_string()))?;
	if !ok || !known {
		failed(&req.email);
		return Err(Error::LoginIncorrect);
	}

	let token = random_token();
	let conn = db.conn();
	conn.execute(
		"DELETE FROM sessions WHERE created_at < ?1",
		[now() - SESSION_DURATION_MS],
	)?;
	conn.execute(
		"INSERT INTO sessions (token, user_id, created_at) VALUES (?1, ?2, ?3)",
		params![token, user_id, now()],
	)?;

	Ok(Json(LoginResp { token }))
}

async fn logout(State(db): State<Db>, headers: HeaderMap) -> Result<()> {
	if let Some(token) = headers.get("session-token") {
		let token = token.to_str().unwrap_or_default();
		db.conn()
			.execute("DELETE FROM sessions WHERE token = ?1", [token])?;
	}
	Ok(())
}

async fn check(_auth: Authed) {}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/login", post(login))
		.route("/logout", post(logout))
		.route("/check", post(check))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::schema::tests::site;

	#[test]
	fn roles_allow_levels() {
		let s = site().role(
			Role::new("editor", "Editor")
				.section("events", Access::Publish)
				.section("pages", Access::View)
				.assets(Access::Edit),
		);
		s.check().unwrap();
		let user = |role: Option<&str>| Authed {
			user_id: 1,
			role: role.map(Into::into),
		};

		let editor = user(Some("editor"));
		assert!(editor.can(&s, "events", Access::Edit));
		assert!(editor.can(&s, "events", Access::Publish));
		assert!(!editor.can(&s, "events", Access::Delete));
		assert!(editor.can(&s, "pages", Access::View));
		assert!(!editor.can(&s, "pages", Access::Edit));
		assert!(!editor.can(&s, "home", Access::View));
		assert!(editor.can_assets(&s, Access::Edit));
		assert!(!editor.can_assets(&s, Access::Delete));
		assert!(!editor.can_ai(&s));

		// an admin can everything, a role that is gone nothing
		assert!(user(None).can(&s, "home", Access::Delete));
		assert!(!user(Some("gone")).can(&s, "events", Access::View));

		let bad =
			site().role(Role::new("x", "X").section("nope", Access::View));
		assert!(bad.check().unwrap_err().contains("unknown section nope"));
	}
}
