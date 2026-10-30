use axum::extract::State;
use axum::http::HeaderMap;
use axum::{Json, Router};

use super::api::*;
use super::{check, check_role, create, delete, get, list, set_role, update};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{
	Admin, Authed, end_sessions, hash_password, password_matches,
	set_password_hash,
};
use crate::content::FieldError;
use crate::error::{Error, Result};

async fn list_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(_): Req<ListReq>,
) -> ApiResult<ListReq> {
	Ok(Json(list(&state.db.conn())?))
}

/// An admin, or the user themself.
fn admin_or_self(auth: &Authed, id: i64) -> Result<()> {
	match auth.admin() || auth.user_id == id {
		true => Ok(()),
		false => Err(Error::Forbidden),
	}
}

/// The session of the request, it stays when the others end.
fn own_token(headers: &HeaderMap) -> Option<&str> {
	headers.get("session-token").and_then(|t| t.to_str().ok())
}

async fn get_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<GetReq>,
) -> ApiResult<GetReq> {
	admin_or_self(&auth, req.id)?;
	Ok(Json(get(&state.db.conn(), req.id)?))
}

async fn create_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(req): Req<CreateReq>,
) -> ApiResult<CreateReq> {
	check_role(&state.schema, req.role.as_deref())?;
	let errors =
		check(&state.db.conn(), None, &req.email, Some(&req.password))?;
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}
	let hash = hash_password(req.password).await?;
	let conn = state.db.conn();
	let id = create(&conn, &req.email, &req.name, &hash, req.role.as_deref())?;
	Ok(Json(get(&conn, id)?))
}

async fn update_route(
	State(state): State<AppState>,
	auth: Authed,
	headers: HeaderMap,
	Req(req): Req<UpdateReq>,
) -> ApiResult<UpdateReq> {
	admin_or_self(&auth, req.id)?;
	let own = auth.user_id == req.id;
	let current = get(&state.db.conn(), req.id)?;
	if own && req.role != current.user.role {
		return Err(Error::Request("you can't change your own role".into()));
	}
	check_role(&state.schema, req.role.as_deref())?;

	let password = req.password.as_deref().filter(|p| !p.is_empty());
	let mut errors =
		check(&state.db.conn(), Some(req.id), &req.email, password)?;
	// a new password of oneself needs the current one, a session alone
	// could be someone else at an open laptop
	if own && password.is_some() {
		let given = req.current_password.as_deref().unwrap_or_default();
		if !password_matches(&state.db, req.id, given).await? {
			errors.push(FieldError {
				path: "currentPassword".into(),
				message: "That isn't your password".into(),
			});
		}
	}
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}

	let hash = match password {
		Some(p) => Some(hash_password(p.into()).await?),
		None => None,
	};
	let conn = state.db.conn();
	update(&conn, req.id, &req.email, &req.name)?;
	if !own {
		set_role(&conn, req.id, req.role.as_deref())?;
	}
	if let Some(hash) = hash {
		let keep = if own { own_token(&headers) } else { None };
		set_password_hash(&conn, req.id, &hash, keep)?;
	}
	Ok(Json(get(&conn, req.id)?))
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

async fn end_sessions_route(
	State(state): State<AppState>,
	auth: Authed,
	headers: HeaderMap,
	Req(req): Req<EndSessionsReq>,
) -> ApiResult<EndSessionsReq> {
	admin_or_self(&auth, req.id)?;
	let keep = own_token(&headers).filter(|_| auth.user_id == req.id);
	end_sessions(&state.db.conn(), req.id, keep)?;
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<ListReq, _>(list_route)
		.api::<GetReq, _>(get_route)
		.api::<CreateReq, _>(create_route)
		.api::<UpdateReq, _>(update_route)
		.api::<DeleteReq, _>(delete_route)
		.api::<EndSessionsReq, _>(end_sessions_route)
}
