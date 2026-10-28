use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use axum::routing::post;
use axum::{Json, Router};
use rand::Rng;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::db::{Db, now};
use crate::error::{Error, Result};

const SESSION_DURATION_MS: i64 = 1000 * 60 * 60 * 24 * 30;

/// Extractor that only succeeds with a valid `session-token` header.
pub struct Authed {
	pub user_id: i64,
}

impl<S> FromRequestParts<S> for Authed
where
	S: Send + Sync,
	Db: FromRef<S>,
{
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
		let db = Db::from_ref(state);
		match session(&db.conn(), &parts.headers)? {
			Some(user_id) => Ok(Self { user_id }),
			None => Err(Error::MissingSessionToken),
		}
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

pub fn add_user(conn: &Connection, email: &str, password: &str) -> Result<()> {
	let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)
		.map_err(|e| Error::Internal(e.to_string()))?;
	conn.execute(
		"INSERT INTO users (email, password_hash, created_at)
		VALUES (?1, ?2, ?3)
		ON CONFLICT (email) DO UPDATE SET password_hash = ?2",
		params![email, hash, now()],
	)?;
	Ok(())
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
	let conn = db.conn();
	let user: Option<(i64, String)> = conn
		.query_row(
			"SELECT id, password_hash FROM users WHERE email = ?1",
			[&req.email],
			|r| Ok((r.get(0)?, r.get(1)?)),
		)
		.optional()?;
	let Some((user_id, hash)) = user else {
		return Err(Error::LoginIncorrect);
	};
	if !bcrypt::verify(&req.password, &hash).unwrap_or(false) {
		return Err(Error::LoginIncorrect);
	}

	let n: u128 = rand::rng().random();
	let token = format!("{n:032x}");
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
