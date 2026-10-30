use axum::extract::State;
use axum::http::HeaderMap;
use axum::{Json, Router};

use rusqlite::{OptionalExtension, params};

use super::api::*;
use super::links::{
	LinkKind, create_link, get_link, link_mail, link_url, login_as_session,
	take_link,
};
use super::{
	Session, end_session, hash_password, login, new_session, renew_session,
	role_of, set_password_hash, token_of,
};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::content::FieldError;
use crate::db::now;
use crate::error::Error;
use crate::users::{password_error, user};

async fn login_route(
	State(state): State<AppState>,
	Req(req): Req<LoginReq>,
) -> ApiResult<LoginReq> {
	let user_id = login(&state.db, &req.email, &req.password).await?;
	let conn = state.db.conn();
	let session = new_session(&conn, user_id)?;
	Ok(Json(authenticated(&conn, session)?))
}

fn authenticated(
	conn: &rusqlite::Connection,
	s: Session,
) -> crate::error::Result<Authenticated> {
	Ok(Authenticated {
		user: user(conn, s.user_id)?,
		session: SessionInfo {
			impersonated_by: match s.impersonated_by {
				Some(by) => Some(user(conn, by)?.email),
				None => None,
			},
			token: s.token,
			expires_at: s.expires_at,
		},
	})
}

async fn session_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	Req(_): Req<SessionReq>,
) -> ApiResult<SessionReq> {
	let token = token_of(&headers)?.ok_or(Error::MissingSessionToken)?;
	let conn = state.db.conn();
	let session = renew_session(&conn, token)?;
	Ok(Json(authenticated(&conn, session)?))
}

async fn logout_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	Req(_): Req<LogoutReq>,
) -> ApiResult<LogoutReq> {
	if let Some(token) = token_of(&headers)? {
		end_session(&state.db.conn(), token)?;
	}
	Ok(Json(()))
}

/// A reset mail at most every few minutes, the form sends no flood.
const FORGOT_EVERY_MS: i64 = 1000 * 60 * 2;

async fn forgot_route(
	State(state): State<AppState>,
	Req(req): Req<ForgotReq>,
) -> ApiResult<ForgotReq> {
	if !state.mail.configured() {
		return Err(Error::Request(
			"This site sends no mails: ask an admin for a new password".into(),
		));
	}
	let conn = state.db.conn();
	let user: Option<(i64, String)> = conn
		.query_row(
			"SELECT id, email FROM users WHERE email = ?1 AND NOT EXISTS (
				SELECT 1 FROM user_links WHERE user_id = users.id
				AND kind = 'reset' AND created_at > ?2)",
			params![req.email.trim(), now() - FORGOT_EVERY_MS],
			|r| Ok((r.get(0)?, r.get(1)?)),
		)
		.optional()?;
	let Some((id, email)) = user else {
		return Ok(Json(()));
	};
	let (token, _) = create_link(&conn, id, LinkKind::Reset, None)?;
	drop(conn);
	let url = link_url(&state.public_url, LinkKind::Reset, &token)?;
	let mail =
		link_mail(&state.public_url, LinkKind::Reset, &email, &url, None);
	// in the background: the answer takes as long for unknown emails
	tokio::spawn(async move {
		if let Err(e) = state.mail.send(mail).await {
			tracing::warn!("password reset: {e}");
		}
	});
	Ok(Json(()))
}

async fn link_route(
	State(state): State<AppState>,
	Req(req): Req<LinkReq>,
) -> ApiResult<LinkReq> {
	let conn = state.db.conn();
	let link = get_link(&conn, &req.token)?;
	let u = user(&conn, link.user_id)?;
	Ok(Json(LinkInfo {
		kind: link.kind,
		email: u.email,
		name: u.name,
		expires_at: link.expires_at,
	}))
}

async fn set_password_route(
	State(state): State<AppState>,
	Req(req): Req<SetPasswordReq>,
) -> ApiResult<SetPasswordReq> {
	let kinds = [LinkKind::Invite, LinkKind::Reset];
	let link = get_link(&state.db.conn(), &req.token)?;
	if !kinds.contains(&link.kind) {
		return Err(Error::Request("This link is used or expired".into()));
	}
	// a wrong password keeps the link
	if let Some(message) = password_error(&req.password) {
		return Err(Error::Invalid(vec![FieldError {
			path: "password".into(),
			message: message.into(),
		}]));
	}
	let hash = hash_password(req.password).await?;
	let conn = state.db.conn();
	let tx = conn.unchecked_transaction()?;
	let link = take_link(&tx, &req.token, &kinds)?;
	// logged out everywhere else
	set_password_hash(&tx, link.user_id, &hash, None)?;
	if let Some(name) = req.name.filter(|n| !n.trim().is_empty()) {
		tx.execute(
			"UPDATE users SET name = ?1 WHERE id = ?2",
			params![name.trim(), link.user_id],
		)?;
	}
	let session = new_session(&tx, link.user_id)?;
	tx.commit()?;
	Ok(Json(authenticated(&conn, session)?))
}

async fn login_as_route(
	State(state): State<AppState>,
	Req(req): Req<LoginAsReq>,
) -> ApiResult<LoginAsReq> {
	let conn = state.db.conn();
	let link = take_link(&conn, &req.token, &[LinkKind::LoginAs])?;
	// still an admin, and the user still none
	let by = link.created_by.ok_or(Error::Forbidden)?;
	let admin = role_of(&conn, by)?.is_none();
	if !admin || role_of(&conn, link.user_id)?.is_none() {
		return Err(Error::Forbidden);
	}
	let session = login_as_session(&conn, link.user_id, by)?;
	Ok(Json(authenticated(&conn, session)?))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<LoginReq, _>(login_route)
		.api::<LogoutReq, _>(logout_route)
		.api::<SessionReq, _>(session_route)
		.api::<ForgotReq, _>(forgot_route)
		.api::<LinkReq, _>(link_route)
		.api::<SetPasswordReq, _>(set_password_route)
		.api::<LoginAsReq, _>(login_as_route)
}
