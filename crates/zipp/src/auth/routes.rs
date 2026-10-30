use axum::extract::State;
use axum::http::HeaderMap;
use axum::{Json, Router};

use super::api::*;
use super::{Authed, end_session, login, new_session};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::error::Result;

async fn login_route(
	State(state): State<AppState>,
	Req(req): Req<LoginReq>,
) -> ApiResult<LoginReq> {
	let user_id = login(&state.db, &req.email, &req.password).await?;
	let token = new_session(&state.db.conn(), user_id)?;
	Ok(Json(LoginResp { token }))
}

async fn logout_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	Req(_): Req<LogoutReq>,
) -> ApiResult<LogoutReq> {
	if let Some(token) = headers.get("session-token") {
		let token = token.to_str().unwrap_or_default();
		end_session(&state.db.conn(), token)?;
	}
	Ok(Json(()))
}

async fn check_route(_auth: Authed, Req(_): Req<CheckReq>) -> Result<Json<()>> {
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<LoginReq, _>(login_route)
		.api::<LogoutReq, _>(logout_route)
		.api::<CheckReq, _>(check_route)
}
