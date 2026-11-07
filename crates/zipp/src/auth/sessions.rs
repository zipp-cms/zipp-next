//! Sessions and passwords: logins, the sessions of the `sessions` table
//! and bcrypt.

use std::sync::OnceLock;

use rusqlite::{Connection, OptionalExtension, params};

use super::links;
use super::throttle::{self, Throttle};
use crate::db::{DAY_MS, Db, now, random_token, token_hash};
use crate::error::{Error, Result};

/// bcrypt's work, small in tests so they stay fast.
const COST: u32 = if cfg!(test) { 4 } else { bcrypt::DEFAULT_COST };
/// How long a session lasts without the studio being used.
const SESSION_DURATION_MS: i64 = 30 * DAY_MS;
/// A session with less left is renewed when the studio opens.
const RENEW_BELOW_MS: i64 = 10 * DAY_MS;
/// A hash to verify against for unknown emails, so they take as long.
static DUMMY_HASH: OnceLock<String> = OnceLock::new();

/// A session of the `sessions` table, which keeps the hash of the token.
#[derive(Debug, Clone, PartialEq)]
pub struct Session {
	pub token: String,
	pub user_id: i64,
	/// in milliseconds
	pub expires_at: i64,
	/// the admin logged in as the user, see `auth::links`
	pub impersonated_by: Option<i64>,
}

/// A session of the studio or, with `site`, of the website: a token is
/// only the one or the other.
pub(super) fn get_session(
	conn: &Connection,
	token: &str,
	site: bool,
) -> Result<Option<Session>> {
	Ok(conn
		.query_row(
			"SELECT user_id, expires_at, impersonated_by FROM sessions
			WHERE hash = ?1 AND site = ?2",
			params![token_hash(token), site],
			|r| {
				Ok(Session {
					token: token.into(),
					user_id: r.get(0)?,
					expires_at: r.get(1)?,
					impersonated_by: r.get(2)?,
				})
			},
		)
		.optional()?)
}

/// The valid session of a token, renewed when less than `RENEW_BELOW_MS`
/// are left: who uses the studio stays logged in. `site` like
/// [`get_session`].
pub(crate) fn renew_session(
	conn: &Connection,
	token: &str,
	site: bool,
) -> Result<Session> {
	let mut s = get_session(conn, token, site)?
		.filter(|s| s.expires_at > now())
		.ok_or(Error::InvalidSessionToken)?;
	// logging in as someone ends when it ends, a short session too
	if s.expires_at - now() < RENEW_BELOW_MS && s.impersonated_by.is_none() {
		let expires_at = now() + SESSION_DURATION_MS;
		let n = conn.execute(
			"UPDATE sessions SET expires_at = ?1 WHERE hash = ?2 AND renews",
			params![expires_at, token_hash(token)],
		)?;
		if n > 0 {
			s.expires_at = expires_at;
		}
	}
	Ok(s)
}

/// Adds a user or sets a new password and role, `None` is an admin.
pub(crate) fn add_user(
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
		None => crate::users::create(conn, email, "", &hash, role, "")?,
	};
	// a new password logs out everywhere
	set_password_hash(conn, id, &hash, None)
}

/// The user of a login, after the password was checked; bcrypt takes a
/// while: not with the database locked, not on the async workers, and as
/// long for unknown emails. `address` is who tries, see [`Throttle`].
pub(crate) async fn login(
	db: &Db,
	throttle: &Throttle,
	address: &str,
	email: &str,
	password: &str,
) -> Result<i64> {
	if email.len() > throttle::MAX_EMAIL_LEN {
		return Err(Error::LoginIncorrect);
	}
	let lower = email.to_lowercase();
	if !throttle.attempt(address, &lower, now()) {
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
		return Err(Error::LoginIncorrect);
	}
	throttle.succeeded(address, &lower);
	Ok(user_id)
}

/// A new session of a user, a login.
pub(crate) fn new_session(conn: &Connection, user_id: i64) -> Result<Session> {
	conn.execute(
		"UPDATE users SET last_login_at = ?1 WHERE id = ?2",
		params![now(), user_id],
	)?;
	insert_session(conn, user_id, SESSION_DURATION_MS, true, None, false)
}

/// A new session of a user on the website, a login there; it lasts and
/// renews like one of the studio but never opens it.
pub(crate) fn new_site_session(
	conn: &Connection,
	user_id: i64,
) -> Result<Session> {
	conn.execute(
		"UPDATE users SET last_login_at = ?1 WHERE id = ?2",
		params![now(), user_id],
	)?;
	insert_session(conn, user_id, SESSION_DURATION_MS, true, None, true)
}

/// A session of a user for a tool that acts as them, like the coding
/// agent of a run: it isn't a login and ends after `ms`, never renewed,
/// so it doesn't outlive what it is for.
pub fn short_session(
	conn: &Connection,
	user_id: i64,
	ms: i64,
) -> Result<Session> {
	insert_session(conn, user_id, ms, false, None, false)
}

/// Stores a new session that ends in `ms`. Ended sessions go.
pub(super) fn insert_session(
	conn: &Connection,
	user_id: i64,
	ms: i64,
	renews: bool,
	impersonated_by: Option<i64>,
	site: bool,
) -> Result<Session> {
	conn.execute("DELETE FROM sessions WHERE expires_at < ?1", [now()])?;
	let session = Session {
		token: random_token(),
		user_id,
		expires_at: now() + ms,
		impersonated_by,
	};
	conn.execute(
		"INSERT INTO sessions
		(hash, user_id, created_at, expires_at, renews, impersonated_by,
			site)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
		params![
			token_hash(&session.token),
			user_id,
			now(),
			session.expires_at,
			renews,
			impersonated_by,
			site
		],
	)?;
	Ok(session)
}

/// Ends a session of the studio.
pub fn end_session(conn: &Connection, token: &str) -> Result<()> {
	conn.execute(
		"DELETE FROM sessions WHERE hash = ?1 AND NOT site",
		[token_hash(token)],
	)?;
	Ok(())
}

/// Ends a session of the website, a logout there.
pub(crate) fn end_site_session(conn: &Connection, token: &str) -> Result<()> {
	conn.execute(
		"DELETE FROM sessions WHERE hash = ?1 AND site",
		[token_hash(token)],
	)?;
	Ok(())
}

/// Logs a user out everywhere, except in the session `keep`.
pub(crate) fn end_sessions(
	conn: &Connection,
	user_id: i64,
	keep: Option<&str>,
) -> Result<()> {
	conn.execute(
		"DELETE FROM sessions WHERE user_id = ?1 AND hash IS NOT ?2",
		params![user_id, keep.map(token_hash)],
	)?;
	Ok(())
}

/// The bcrypt hash of a password; it takes a while, not on the async
/// workers and not with the database locked.
pub(crate) async fn hash_password(password: String) -> Result<String> {
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
pub(crate) async fn password_matches(
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

/// A new password: the user is logged out everywhere, except in `keep`,
/// and links that set one end.
pub(crate) fn set_password_hash(
	conn: &Connection,
	user_id: i64,
	hash: &str,
	keep: Option<&str>,
) -> Result<()> {
	conn.execute(
		"UPDATE users SET password_hash = ?1 WHERE id = ?2",
		params![hash, user_id],
	)?;
	links::end_password_links(conn, user_id)?;
	end_sessions(conn, user_id, keep)
}
