use super::api::*;
use super::*;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{Delete, EntryAccess, Publish, View};

async fn list_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(ListReq(params)): Req<ListReq>,
) -> ApiResult<ListReq> {
	let conn = state.db.conn();
	let q = Query::parse(&state.schema, &params).map_err(Error::Request)?;
	if let Some(section) = &q.section {
		auth.check(&state.schema, section, Access::View)?;
	}

	let (mut entries, total) = list(&conn, &q)?;
	// a search over all sections only finds what the role sees
	entries.retain(|e| auth.can(&state.schema, &e.section, Access::View));
	let ids: Vec<i64> = entries.iter().map(|e| e.id).collect();
	let relations = refs(&conn, &ids, &q.site, q.published_only)?;
	let published = published_per_site(&conn, &ids)?;
	let drafts = with_drafts(&conn, &ids)?;

	Ok(Json(ListResp {
		entries,
		total,
		relations,
		published,
		drafts,
	}))
}

async fn get_route(
	State(state): State<AppState>,
	_access: EntryAccess<View>,
	Req(req): Req<GetReq>,
) -> ApiResult<GetReq> {
	let conn = state.db.conn();
	let site = req
		.site
		.unwrap_or_else(|| state.schema.primary_site().handle.clone());

	let entry = get_entry(&conn, req.id, &site)?;
	let relations = refs(&conn, &[req.id], &site, false)?;

	Ok(Json(EntryResp { entry, relations }))
}

async fn create_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(CreateReq(mut input)): Req<CreateReq>,
) -> ApiResult<CreateReq> {
	auth.check(&state.schema, &input.section, Access::Edit)?;
	// published only by who may publish
	input.published &= auth.can(&state.schema, &input.section, Access::Publish);
	let entry = {
		let conn = state.db.conn();
		let entry = save(&conn, &state.schema, None, input)?;
		record_revision(&conn, entry.id, Some(auth.user_id))?;
		entry
	};
	state.changed(entry.id, &entry.section, ChangeKind::Saved);
	Ok(Json(entry))
}

/// Live at once, like publishing.
async fn update_route(
	State(state): State<AppState>,
	access: EntryAccess<Publish>,
	Req(req): Req<UpdateReq>,
) -> ApiResult<UpdateReq> {
	let entry = {
		let conn = state.db.conn();
		let entry = save(&conn, &state.schema, Some(req.id), req.input)?;
		record_revision(&conn, req.id, Some(access.auth.user_id))?;
		entry
	};
	state.changed(req.id, &entry.section, ChangeKind::Saved);
	Ok(Json(entry))
}

async fn move_route(
	State(state): State<AppState>,
	access: EntryAccess<Publish>,
	Req(req): Req<MoveReq>,
) -> ApiResult<MoveReq> {
	let conn = state.db.conn();
	move_entry(&conn, &state.schema, req.id, req.parent_id, req.index)?;
	drop(conn);
	state.changed(req.id, &access.section, ChangeKind::Moved);
	Ok(Json(()))
}

async fn delete_route(
	State(state): State<AppState>,
	access: EntryAccess<Delete>,
	Req(req): Req<DeleteReq>,
) -> ApiResult<DeleteReq> {
	delete(&state.db.conn(), &state.schema, req.id)?;
	state.changed(req.id, &access.section, ChangeKind::Deleted);
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<ListReq, _>(list_route)
		.api::<GetReq, _>(get_route)
		.api::<CreateReq, _>(create_route)
		.api::<UpdateReq, _>(update_route)
		.api::<MoveReq, _>(move_route)
		.api::<DeleteReq, _>(delete_route)
}
