use axum::Router;
use axum::extract::State;
use axum::response::Json;

use super::api::*;
use super::*;
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::Admin;

async fn status_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(_): Req<StatusReq>,
) -> ApiResult<StatusReq> {
	let m = &state.mail;
	Ok(Json(MailStatus {
		configured: m.configured(),
		server: m.server().map(String::from),
		from: m.from(),
	}))
}

async fn test_route(
	State(state): State<AppState>,
	Admin(auth): Admin,
	Req(_): Req<TestReq>,
) -> ApiResult<TestReq> {
	let email = crate::users::get(&state.db.conn(), auth.user_id)?
		.user
		.email;
	state.mail.send(test_mail(&email)).await?;
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<StatusReq, _>(status_route)
		.api::<TestReq, _>(test_route)
}
