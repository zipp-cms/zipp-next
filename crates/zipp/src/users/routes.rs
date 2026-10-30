use axum::extract::State;
use axum::{Json, Router};

use super::api::*;
use super::{check_role, delete, list, set_role};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{Admin, add_user};
use crate::error::Error;

async fn list_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(_): Req<ListReq>,
) -> ApiResult<ListReq> {
	Ok(Json(list(&state.db.conn())?))
}

async fn create_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(req): Req<CreateReq>,
) -> ApiResult<CreateReq> {
	check_role(&state.schema, req.role.as_deref())?;
	if !req.email.contains('@') || req.password.chars().count() < 8 {
		return Err(Error::Request(
			"an email and a password of at least 8 characters".into(),
		));
	}
	// bcrypt takes a while, not with the database locked
	let db = state.db.clone();
	tokio::task::spawn_blocking(move || {
		add_user(&db.conn(), &req.email, &req.password, req.role.as_deref())
	})
	.await
	.map_err(|e| Error::Internal(e.to_string()))??;
	Ok(Json(()))
}

async fn role_route(
	State(state): State<AppState>,
	Admin(auth): Admin,
	Req(req): Req<RoleReq>,
) -> ApiResult<RoleReq> {
	check_role(&state.schema, req.role.as_deref())?;
	if req.id == auth.user_id {
		return Err(Error::Request("you can't change your own role".into()));
	}
	set_role(&state.db.conn(), req.id, req.role.as_deref())?;
	Ok(Json(()))
}

async fn delete_route(
	State(state): State<AppState>,
	Admin(auth): Admin,
	Req(req): Req<DeleteReq>,
) -> ApiResult<DeleteReq> {
	if req.id == auth.user_id {
		return Err(Error::Request("you can't delete yourself".into()));
	}
	delete(&state.db.conn(), req.id)?;
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<ListReq, _>(list_route)
		.api::<CreateReq, _>(create_route)
		.api::<RoleReq, _>(role_route)
		.api::<DeleteReq, _>(delete_route)
}
