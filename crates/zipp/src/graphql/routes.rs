use axum::extract::State;
use axum::{Json, Router};

use super::api::*;
use super::tokens::{check, create, delete, list};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::Admin;
use crate::error::Error;

async fn tokens_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(_): Req<TokensReq>,
) -> ApiResult<TokensReq> {
	Ok(Json(TokensResp {
		tokens: list(&state.db.conn())?,
		env_token: state.graphql_token.is_some(),
	}))
}

async fn create_token_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(req): Req<CreateTokenReq>,
) -> ApiResult<CreateTokenReq> {
	let errors = check(&state.schema, &req.name, &req.scope);
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}
	let conn = state.db.conn();
	let (info, token) = create(&conn, &req.name, &req.scope, req.expires_at)?;
	Ok(Json(CreatedToken { info, token }))
}

async fn delete_token_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(req): Req<DeleteTokenReq>,
) -> ApiResult<DeleteTokenReq> {
	delete(&state.db.conn(), req.id)?;
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<TokensReq, _>(tokens_route)
		.api::<CreateTokenReq, _>(create_token_route)
		.api::<DeleteTokenReq, _>(delete_token_route)
}
