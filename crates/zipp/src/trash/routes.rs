use axum::Router;
use axum::extract::State;
use axum::response::Json;

use super::api::*;
use super::*;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::{AppState, ChangeKind};

async fn list_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(_): Req<TrashReq>,
) -> ApiResult<TrashReq> {
	Ok(Json(list(&state.db.conn(), &state.schema, &auth)?))
}

/// Whether the user may delete an entry of the trash, with the children
/// that went with it.
fn check_entry(state: &AppState, auth: &Authed, id: i64) -> Result<()> {
	let conn = state.db.conn();
	let (section, author, deleted_at) = trashed_owner(&conn, id)?;
	let (schema, need) = (&state.schema, Access::Delete);
	auth.check_entry(schema, &section, author, need)?;
	check_tree(&conn, schema, auth, id, &section, need, Some(deleted_at))
}

async fn restore_entry_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<RestoreEntryReq>,
) -> ApiResult<RestoreEntryReq> {
	check_entry(&state, &auth, req.id)?;
	let section = restore_entry(&state.db.conn(), &state.schema, req.id)?;
	state.changed(req.id, &section, ChangeKind::Saved);
	Ok(Json(()))
}

async fn purge_entry_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<PurgeEntryReq>,
) -> ApiResult<PurgeEntryReq> {
	check_entry(&state, &auth, req.id)?;
	purge_entry(&state.db.conn(), req.id)?;
	Ok(Json(()))
}

async fn restore_asset_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<RestoreAssetReq>,
) -> ApiResult<RestoreAssetReq> {
	auth.check_assets(&state.schema, Access::Delete)?;
	restore_asset(&state.db.conn(), req.id)?;
	Ok(Json(()))
}

async fn purge_asset_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<PurgeAssetReq>,
) -> ApiResult<PurgeAssetReq> {
	auth.check_assets(&state.schema, Access::Delete)?;
	purge_asset(&state.db.conn(), &state.files, req.id)?;
	Ok(Json(()))
}

async fn empty_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(_): Req<EmptyReq>,
) -> ApiResult<EmptyReq> {
	let conn = state.db.conn();
	let trash = list(&conn, &state.schema, &auth)?;
	let tx = conn.unchecked_transaction()?;
	for e in trash.entries {
		purge_entry(&tx, e.id)?;
	}
	tx.commit()?;
	// the files go one by one, each with its row
	for a in trash.assets {
		purge_asset(&conn, &state.files, a.asset.id)?;
	}
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<TrashReq, _>(list_route)
		.api::<RestoreEntryReq, _>(restore_entry_route)
		.api::<PurgeEntryReq, _>(purge_entry_route)
		.api::<RestoreAssetReq, _>(restore_asset_route)
		.api::<PurgeAssetReq, _>(purge_asset_route)
		.api::<EmptyReq, _>(empty_route)
}
