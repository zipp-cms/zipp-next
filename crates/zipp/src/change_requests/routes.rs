use axum::Router;
use axum::extract::State;
use axum::response::Json;

use super::api::*;
use super::*;
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{Admin, Authed};

async fn list_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(_): Req<ListReq>,
) -> ApiResult<ListReq> {
	Ok(Json(list(&state.db.conn())?))
}

async fn create_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<CreateReq>,
) -> ApiResult<CreateReq> {
	// they come from Setup
	if !auth.can_setup(&state.schema) {
		return Err(Error::Forbidden);
	}
	let (created, admins) = {
		let conn = state.db.conn();
		let created =
			create(&conn, auth.user_id, &req.page, &req.context, &req.text)?;
		(created, admin_emails(&conn)?)
	};

	// stored either way, a mail that fails is only logged
	if state.mail.configured() {
		for to in admins {
			let mail = mail(&to, &created, &state.public_url);
			if let Err(e) = state.mail.send(mail).await {
				tracing::warn!("the change request mail: {e}");
			}
		}
	}
	Ok(Json(created))
}

async fn done_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(req): Req<DoneReq>,
) -> ApiResult<DoneReq> {
	set_done(&state.db.conn(), req.id, req.done)?;
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<ListReq, _>(list_route)
		.api::<CreateReq, _>(create_route)
		.api::<DoneReq, _>(done_route)
}
