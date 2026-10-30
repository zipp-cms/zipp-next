use super::api::*;
use super::*;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{AiAccess, Edit, EntryAccess, Publish, View};

async fn get_route(
	State(state): State<AppState>,
	_access: EntryAccess<View>,
	Req(req): Req<GetReq>,
) -> ApiResult<GetReq> {
	let conn = state.db.conn();
	let draft = get_draft(&conn, req.id)?;
	let site = req
		.site
		.unwrap_or_else(|| state.schema.primary_site().handle.clone());
	let relations = match &draft {
		Some(d) => draft_refs(&conn, &state.schema, d, &site)?,
		None => Default::default(),
	};
	Ok(Json(DraftResp { draft, relations }))
}

async fn put_route(
	State(state): State<AppState>,
	access: EntryAccess<Edit>,
	Req(req): Req<PutReq>,
) -> ApiResult<PutReq> {
	let d = put_draft(
		&state.db.conn(),
		&state.schema,
		req.id,
		&req.site,
		req.content,
		req.base,
		Some(access.auth.user_id),
	)?;
	Ok(Json(d))
}

async fn discard_route(
	State(state): State<AppState>,
	_access: EntryAccess<Edit>,
	Req(req): Req<DiscardReq>,
) -> ApiResult<DiscardReq> {
	discard(&state.db.conn(), req.id)?;
	Ok(Json(()))
}

async fn publish_route(
	State(state): State<AppState>,
	access: EntryAccess<Publish>,
	Req(req): Req<PublishReq>,
) -> ApiResult<PublishReq> {
	let user = Some(access.auth.user_id);
	publish(&state.db.conn(), &state.schema, req.id, user)?;
	state.changed(req.id, &access.section, crate::ChangeKind::Published);
	Ok(Json(()))
}

async fn revisions_route(
	State(state): State<AppState>,
	_access: EntryAccess<View>,
	Req(req): Req<RevisionsReq>,
) -> ApiResult<RevisionsReq> {
	Ok(Json(revisions(&state.db.conn(), req.id)?))
}

async fn revision_route(
	State(state): State<AppState>,
	_access: EntryAccess<View>,
	Req(req): Req<RevisionReq>,
) -> ApiResult<RevisionReq> {
	Ok(Json(get_revision(&state.db.conn(), req.id, req.rev)?))
}

async fn restore_route(
	State(state): State<AppState>,
	access: EntryAccess<Edit>,
	Req(req): Req<RestoreReq>,
) -> ApiResult<RestoreReq> {
	let user = Some(access.auth.user_id);
	Ok(Json(restore(&state.db.conn(), req.id, req.rev, user)?))
}

async fn preview_route(
	State(state): State<AppState>,
	_access: EntryAccess<View>,
	Req(req): Req<PreviewReq>,
) -> ApiResult<PreviewReq> {
	let token = create_token(&state.db.conn(), req.id)?;
	Ok(Json(TokenResp { token }))
}

/// Every draft of every section: the AI page's.
async fn site_preview_route(
	State(state): State<AppState>,
	_access: AiAccess,
	Req(_): Req<SitePreviewReq>,
) -> ApiResult<SitePreviewReq> {
	let token = create_site_token(&state.db.conn())?;
	Ok(Json(TokenResp { token }))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<GetReq, _>(get_route)
		.api::<PutReq, _>(put_route)
		.api::<DiscardReq, _>(discard_route)
		.api::<PublishReq, _>(publish_route)
		.api::<RevisionsReq, _>(revisions_route)
		.api::<RevisionReq, _>(revision_route)
		.api::<RestoreReq, _>(restore_route)
		.api::<PreviewReq, _>(preview_route)
		.api::<SitePreviewReq, _>(site_preview_route)
}
