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

/// The section of an entry of the trash, if the user may delete in it.
fn entry_section(state: &AppState, auth: &Authed, id: i64) -> Result<String> {
	let section = trashed_section(&state.db.conn(), id)?;
	auth.check(&state.schema, &section, Access::Delete)?;
	Ok(section)
}

async fn restore_entry_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<RestoreEntryReq>,
) -> ApiResult<RestoreEntryReq> {
	entry_section(&state, &auth, req.id)?;
	let section = restore_entry(&state.db.conn(), &state.schema, req.id)?;
	state.changed(req.id, &section, ChangeKind::Saved);
	Ok(Json(()))
}

async fn purge_entry_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<PurgeEntryReq>,
) -> ApiResult<PurgeEntryReq> {
	entry_section(&state, &auth, req.id)?;
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
