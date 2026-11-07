use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::{Query as QueryParams, State};
use axum::http::HeaderMap;
use axum::response::Response;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;

use super::api::*;
use super::tokens::{self, check, create, delete, list};
use super::{Scope, Viewer};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{Admin, role_of, session};
use crate::drafts::preview;
use crate::error::{Error, Result};

async fn tokens_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(_): Req<TokensReq>,
) -> ApiResult<TokensReq> {
	Ok(Json(TokensResp {
		tokens: list(&state.db.conn())?,
		schemas: state.schema.graphql_schemas.clone(),
		env_token: state.graphql_token.is_some(),
	}))
}

async fn create_token_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(req): Req<CreateTokenReq>,
) -> ApiResult<CreateTokenReq> {
	let errors = check(&state.schema, &req.name, &req.schema);
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}
	let conn = state.db.conn();
	let (info, token) = create(&conn, &req.name, &req.schema, req.expires_at)?;
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

#[derive(Debug, Deserialize)]
struct TokenParam {
	/// a preview token, crelte passes the one of the page on
	token: Option<String>,
}

async fn graphql(
	State(state): State<AppState>,
	headers: HeaderMap,
	QueryParams(p): QueryParams<TokenParam>,
	req: GraphQLRequest,
) -> Result<GraphQLResponse> {
	let (published_only, preview, scope) = {
		let conn = state.db.conn();
		// only admins see every draft and unpublished entry, a role only
		// some sections: it reads GraphQL like a frontend
		let editor = match session(&conn, &headers)? {
			Some(user) => role_of(&conn, user)?.is_none(),
			None => false,
		};
		// private like Craft's: frontends send a token, the studio its
		// session; development without the token of serve is open
		let open = state.dev && state.graphql_token.is_none();
		// GraphiQL's schema picker: an admin reads as a frontend
		let picked = if editor {
			schema_header(&headers)
		} else {
			None
		};
		let scope = match (editor, bearer(&headers)) {
			(true, _) => match picked {
				Some(h) => state
					.schema
					.get_graphql_schema(h)
					.map(Scope::of)
					.ok_or(Error::MissingEndpointToken)?,
				None => Scope::ALL,
			},
			// a wrong token fails in development too, like on the server
			(false, Some(t)) => token_scope(&state, &conn, t)?
				.ok_or(Error::MissingEndpointToken)?,
			(false, None) if open => Scope::ALL,
			(false, None) => return Err(Error::MissingEndpointToken),
		};
		let preview = match &p.token {
			Some(t) if scope.previews => preview(&conn, &state.schema, t)?,
			_ => None,
		};
		(!editor || picked.is_some(), preview, scope)
	};
	let token = preview.as_ref().and(p.token);
	let req = req.into_inner().data(Viewer {
		published_only,
		preview,
		token,
		indexing: state.indexing,
		scope,
		loaded: Default::default(),
	});
	Ok(state.gql.execute(req).await.into())
}

/// The token of a request, `Authorization: Bearer <token>`.
fn bearer(headers: &HeaderMap) -> Option<&str> {
	headers
		.get(axum::http::header::AUTHORIZATION)
		.and_then(|v| v.to_str().ok())
		.and_then(|v| v.strip_prefix("Bearer "))
}

/// The GraphQL schema GraphiQL picked, `x-zipp-schema: <handle>`.
fn schema_header(headers: &HeaderMap) -> Option<&str> {
	headers.get("x-zipp-schema").and_then(|v| v.to_str().ok())
}

/// What a token may read: the one of `serve` everything, the ones of the
/// studio their schema's; `None` for an unknown or expired one.
fn token_scope(
	state: &AppState,
	conn: &rusqlite::Connection,
	sent: &str,
) -> Result<Option<Scope>> {
	if let Some(token) = &state.graphql_token {
		// as long whatever matches
		let same = sent.len() == token.len()
			&& sent
				.bytes()
				.zip(token.bytes())
				.fold(0, |acc, (a, b)| acc | (a ^ b))
				== 0;
		if same {
			return Ok(Some(Scope::ALL));
		}
	}
	tokens::scope_of(conn, &state.schema, sent)
}

/// The playground, for admins and in development: a page of the studio's
/// build, see [`crate::studio`]. Its queries send the session of the
/// studio, so an admin sees drafts like in the studio; without one it goes
/// to the login, except in development where GraphQL is open.
async fn graphiql(State(state): State<AppState>) -> Response {
	crate::studio::graphiql(&state)
}

pub(crate) fn routes() -> Router<AppState> {
	Router::new()
		.route(
			"/graphql",
			get(graphiql)
				.post(graphql)
				// async-graphql reads the body itself, past axum's limit
				.layer(tower_http::limit::RequestBodyLimitLayer::new(
					256 * 1024,
				)),
		)
		.api::<TokensReq, _>(tokens_route)
		.api::<CreateTokenReq, _>(create_token_route)
		.api::<DeleteTokenReq, _>(delete_token_route)
}
