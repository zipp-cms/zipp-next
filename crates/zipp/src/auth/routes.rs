use axum::extract::State;
use axum::http::HeaderMap;
use axum::{Json, Router};

use super::api::*;
use super::{
	Session, end_session, login, new_session, renew_session, token_of,
};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::error::Error;
use crate::users::user;

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

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<LoginReq, _>(login_route)
		.api::<LogoutReq, _>(logout_route)
		.api::<SessionReq, _>(session_route)
}
