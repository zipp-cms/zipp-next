//! Logins, sessions and what a user may do. A handler says who may call
//! it with its extractor: [`Authed`] any user, [`Admin`], [`EntryAccess`]
//! and [`AssetAccess`] a level of a role, [`AiAccess`] the AI chat.

use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::{Mutex, OnceLock, PoisonError};

use axum::extract::{FromRef, FromRequestParts, RawPathParams};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use rusqlite::{Connection, OptionalExtension, params};

use crate::AppState;
use crate::db::{Db, now, random_token};
use crate::error::{Error, Result};
use crate::schema::{Access, Role, Schema};

pub mod api;
#[cfg(test)]
mod api_tests;
mod routes;
#[cfg(test)]
mod tests;

pub use routes::routes;

/// bcrypt's work, small in tests so they stay fast.
const COST: u32 = if cfg!(test) { 4 } else { bcrypt::DEFAULT_COST };
/// How long a session lasts without the studio being used.
const SESSION_DURATION_MS: i64 = 1000 * 60 * 60 * 24 * 30;
/// A session with less left is renewed when the studio opens.
const RENEW_BELOW_MS: i64 = 1000 * 60 * 60 * 24 * 10;
/// Wrong passwords of an email before it has to wait.
const MAX_FAILS: u32 = 10;
const FAIL_WINDOW_MS: i64 = 1000 * 60 * 15;

/// Failed logins per email: count and the time of the first one.
static FAILS: Mutex<BTreeMap<String, (u32, i64)>> = Mutex::new(BTreeMap::new());

/// A hash to verify against for unknown emails, so they take as long.
static DUMMY_HASH: OnceLock<String> = OnceLock::new();

/// Extractor that only succeeds with a valid `session-token` header.
#[non_exhaustive]
pub struct Authed {
	pub user_id: i64,
	/// `None` is an admin
	pub role: Option<String>,
	/// the session of the request
	pub token: String,
}

impl Authed {
	pub fn admin(&self) -> bool {
		self.role.is_none()
	}

	/// An admin, or the user `id` themself.
	pub fn check_self(&self, id: i64) -> Result<()> {
		allowed(self.admin() || self.user_id == id)
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

	/// [`Authed::can`] as an error, for a section that comes with the
	/// request.
	pub fn check(
		&self,
		schema: &Schema,
		section: &str,
		need: Access,
	) -> Result<()> {
		allowed(self.can(schema, section, need))
	}

	/// [`Authed::can_assets`] as an error, where no extractor checks it.
	pub fn check_assets(&self, schema: &Schema, need: Access) -> Result<()> {
		allowed(self.can_assets(schema, need))
	}
}

fn allowed(ok: bool) -> Result<()> {
	match ok {
		true => Ok(()),
		false => Err(Error::Forbidden),
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
		let token =
			token_of(&parts.headers)?.ok_or(Error::MissingSessionToken)?;
		let Some(user_id) = session(&conn, &parts.headers)? else {
			return Err(Error::MissingSessionToken);
		};
		let role = role_of(&conn, user_id)?;
		Ok(Self {
			user_id,
			role,
			token: token.into(),
		})
	}
}

/// The levels of [`Access`] as types, for the extractors.
pub trait Level: Send + Sync + 'static {
	const ACCESS: Access;
}

pub struct View;
pub struct Edit;
pub struct Publish;
pub struct Delete;

impl Level for View {
	const ACCESS: Access = Access::View;
}
impl Level for Edit {
	const ACCESS: Access = Access::Edit;
}
impl Level for Publish {
	const ACCESS: Access = Access::Publish;
}
impl Level for Delete {
	const ACCESS: Access = Access::Delete;
}

/// A user without a role.
pub struct Admin(pub Authed);

impl FromRequestParts<AppState> for Admin {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		allowed(auth.admin())?;
		Ok(Self(auth))
	}
}

/// A user who may do `L` in the section of the entry `{id}` of the path.
pub struct EntryAccess<L> {
	pub auth: Authed,
	pub id: i64,
	pub section: String,
	level: PhantomData<L>,
}

impl<L: Level> FromRequestParts<AppState> for EntryAccess<L> {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		let params = RawPathParams::from_request_parts(parts, state)
			.await
			.map_err(|e| Error::Request(e.to_string()))?;
		let id = params
			.iter()
			.find(|(k, _)| *k == "id")
			.and_then(|(_, v)| v.parse().ok())
			.ok_or(Error::NotFound)?;

		let section = crate::entries::section_of(&state.db.conn(), id)?;
		auth.check(&state.schema, &section, L::ACCESS)?;
		Ok(Self {
			auth,
			id,
			section,
			level: PhantomData,
		})
	}
}

/// A user who may do `L` with assets.
pub struct AssetAccess<L> {
	pub auth: Authed,
	level: PhantomData<L>,
}

impl<L: Level> FromRequestParts<AppState> for AssetAccess<L> {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		allowed(auth.can_assets(&state.schema, L::ACCESS))?;
		Ok(Self {
			auth,
			level: PhantomData,
		})
	}
}

/// A user who may use the AI chat; it sees every draft.
pub struct AiAccess(pub Authed);

impl FromRequestParts<AppState> for AiAccess {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		allowed(auth.can_ai(&state.schema))?;
		Ok(Self(auth))
	}
}

/// The user of a request, `None` without a token. Public routes use it to
/// show unpublished entries to editors.
pub fn session(conn: &Connection, headers: &HeaderMap) -> Result<Option<i64>> {
	let Some(token) = token_of(headers)? else {
		return Ok(None);
	};

	match get_session(conn, token)? {
		Some(s) if s.expires_at > now() => Ok(Some(s.user_id)),
		_ => Err(Error::InvalidSessionToken),
	}
}

/// The token of a request, its `session-token` header.
pub(crate) fn token_of(headers: &HeaderMap) -> Result<Option<&str>> {
	match headers.get("session-token") {
		Some(t) => t.to_str().map(Some).map_err(|_| Error::InvalidSessionToken),
		None => Ok(None),
	}
}

/// A session of the `sessions` table.
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
	pub token: String,
	pub user_id: i64,
	/// in milliseconds
	pub expires_at: i64,
}

fn get_session(conn: &Connection, token: &str) -> Result<Option<Session>> {
	Ok(conn
		.query_row(
			"SELECT token, user_id, expires_at FROM sessions WHERE token = ?1",
			[token],
			|r| {
				Ok(Session {
					token: r.get(0)?,
					user_id: r.get(1)?,
					expires_at: r.get(2)?,
				})
			},
		)
		.optional()?)
}

/// The valid session of a token, renewed when less than [`RENEW_BELOW_MS`]
/// are left: who uses the studio stays logged in.
pub fn renew_session(conn: &Connection, token: &str) -> Result<Session> {
	let mut s = get_session(conn, token)?
		.filter(|s| s.expires_at > now())
		.ok_or(Error::InvalidSessionToken)?;
	if s.expires_at - now() < RENEW_BELOW_MS {
		s.expires_at = now() + SESSION_DURATION_MS;
		conn.execute(
			"UPDATE sessions SET expires_at = ?1 WHERE token = ?2",
			params![s.expires_at, token],
		)?;
	}
	Ok(s)
}

/// The role of a user, `None` is an admin.
pub fn role_of(conn: &Connection, user_id: i64) -> Result<Option<String>> {
	Ok(conn.query_row(
		"SELECT role FROM users WHERE id = ?1",
		[user_id],
		|r| r.get(0),
	)?)
}

/// Adds a user or sets a new password and role, `None` is an admin.
pub fn add_user(
	conn: &Connection,
	email: &str,
	password: &str,
	role: Option<&str>,
) -> Result<()> {
	let hash = bcrypt::hash(password, COST)
		.map_err(|e| Error::Internal(e.to_string()))?;
	let id: Option<i64> = conn
		.query_row("SELECT id FROM users WHERE email = ?1", [email], |r| {
			r.get(0)
		})
		.optional()?;
	let id = match id {
		Some(id) => {
			crate::users::set_role(conn, id, role)?;
			id
		}
		None => crate::users::create(conn, email, "", &hash, role)?,
	};
	// a new password logs out everywhere
	set_password_hash(conn, id, &hash, None)
}

/// The user of a login, after the password was checked; bcrypt takes a
/// while: not with the database locked, not on the async workers, and as
/// long for unknown emails.
pub async fn login(db: &Db, email: &str, password: &str) -> Result<i64> {
	if !may_try(email) {
		return Err(Error::Request(
			"Too many wrong passwords, try again in 15 minutes".into(),
		));
	}
	let user: Option<(i64, String)> = db
		.conn()
		.query_row(
			"SELECT id, password_hash FROM users WHERE email = ?1",
			[email],
			|r| Ok((r.get(0)?, r.get(1)?)),
		)
		.optional()?;

	let known = user.is_some();
	let (user_id, hash) = user.unwrap_or_else(|| {
		let dummy = DUMMY_HASH
			.get_or_init(|| bcrypt::hash("not a password", COST).unwrap());
		(0, dummy.clone())
	});
	let ok = verify(password, hash).await?;
	if !ok || !known {
		failed(email);
		return Err(Error::LoginIncorrect);
	}
	Ok(user_id)
}

/// A new session of a user, a login. Ended sessions go.
pub fn new_session(conn: &Connection, user_id: i64) -> Result<Session> {
	conn.execute("DELETE FROM sessions WHERE expires_at < ?1", [now()])?;
	conn.execute(
		"UPDATE users SET last_login_at = ?1 WHERE id = ?2",
		params![now(), user_id],
	)?;
	let session = Session {
		token: random_token(),
		user_id,
		expires_at: now() + SESSION_DURATION_MS,
	};
	conn.execute(
		"INSERT INTO sessions (token, user_id, created_at, expires_at)
		VALUES (?1, ?2, ?3, ?4)",
		params![session.token, user_id, now(), session.expires_at],
	)?;
	Ok(session)
}

pub fn end_session(conn: &Connection, token: &str) -> Result<()> {
	conn.execute("DELETE FROM sessions WHERE token = ?1", [token])?;
	Ok(())
}

/// Logs a user out everywhere, except in the session `keep`.
pub fn end_sessions(
	conn: &Connection,
	user_id: i64,
	keep: Option<&str>,
) -> Result<()> {
	conn.execute(
		"DELETE FROM sessions WHERE user_id = ?1 AND token IS NOT ?2",
		params![user_id, keep],
	)?;
	Ok(())
}

/// The bcrypt hash of a password; it takes a while, not on the async
/// workers and not with the database locked.
pub async fn hash_password(password: String) -> Result<String> {
	tokio::task::spawn_blocking(move || bcrypt::hash(password, COST))
		.await?
		.map_err(|e| Error::Internal(e.to_string()))
}

/// Whether a password has the hash; like hashing, on a blocking thread.
async fn verify(password: &str, hash: String) -> Result<bool> {
	let password = password.to_string();
	let ok = tokio::task::spawn_blocking(move || {
		bcrypt::verify(password, &hash).unwrap_or(false)
	})
	.await?;
	Ok(ok)
}

/// Whether the password is the one of the user, see [`hash_password`].
pub async fn password_matches(
	db: &Db,
	user_id: i64,
	password: &str,
) -> Result<bool> {
	let hash: String = db.conn().query_row(
		"SELECT password_hash FROM users WHERE id = ?1",
		[user_id],
		|r| r.get(0),
	)?;
	verify(password, hash).await
}

/// A new password: the user is logged out everywhere, except in `keep`.
pub fn set_password_hash(
	conn: &Connection,
	user_id: i64,
	hash: &str,
	keep: Option<&str>,
) -> Result<()> {
	conn.execute(
		"UPDATE users SET password_hash = ?1 WHERE id = ?2",
		params![hash, user_id],
	)?;
	end_sessions(conn, user_id, keep)
}

/// Whether the email may try again, see [`MAX_FAILS`].
fn may_try(email: &str) -> bool {
	let mut fails = FAILS.lock().unwrap_or_else(PoisonError::into_inner);
	fails.retain(|_, (_, first)| *first + FAIL_WINDOW_MS > now());
	let email = email.to_lowercase();
	fails.get(&email).is_none_or(|(n, _)| *n < MAX_FAILS)
}

/// A wrong password, the email in any case counts as one.
fn failed(email: &str) {
	let mut fails = FAILS.lock().unwrap_or_else(PoisonError::into_inner);
	fails.entry(email.to_lowercase()).or_insert((0, now())).0 += 1;
}
