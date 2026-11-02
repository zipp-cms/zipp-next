use axum::Router;
use axum::extract::State;
use axum::response::Json;

use super::api::*;
use super::*;
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};

/// Any user, they find what their role may view.
async fn search_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<SearchReq>,
) -> ApiResult<SearchReq> {
	let schema = &state.schema;
	let site = schema.site_or_primary(req.site);
	let found = search(&state.db.conn(), schema, &auth, &site, &req.q)?;
	Ok(Json(found))
}

pub fn routes() -> Router<AppState> {
	Router::new().api::<SearchReq, _>(search_route)
}
