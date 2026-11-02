use super::api::*;
use super::*;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{Delete, EntryAccess, Publish, View};

/// Only the related entries the role sees: the chips of the others would
/// show their titles and pages.
pub(crate) fn viewable_refs(
	auth: &Authed,
	schema: &Schema,
	refs: &mut BTreeMap<i64, EntryRef>,
) {
	refs.retain(|_, r| auth.can(schema, &r.section, Access::View));
}

async fn list_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(ListReq(params)): Req<ListReq>,
) -> ApiResult<ListReq> {
	let conn = state.db.conn();
	let mut q = Query::parse(&state.schema, &params).map_err(Error::Request)?;
	if let Some(section) = &q.section {
		auth.check(&state.schema, section, Access::View)?;
	}
	// a search over all sections only finds what the role sees, also in
	// the total, or a search would tell whether a hidden entry matches
	if !auth.admin() {
		let sections = state.schema.sections.iter();
		q.sections = Some(
			sections
				.filter(|s| auth.can(&state.schema, &s.handle, Access::View))
				.map(|s| s.handle.clone())
				.collect(),
		);
	}

	let (entries, total) = list(&conn, &q)?;
	let ids: Vec<i64> = entries.iter().map(|e| e.id).collect();
	let mut relations = refs(&conn, &ids, &q.site, q.published_only)?;
	viewable_refs(&auth, &state.schema, &mut relations);
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

async fn choices_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<ChoicesReq>,
) -> ApiResult<ChoicesReq> {
	let schema = &state.schema;
	let types: Vec<&str> = req.types.split(',').collect();
	if let Some(t) = types.iter().find(|t| schema.entry_type(t).is_none()) {
		return Err(Error::Request(format!("unknown entry type {t}")));
	}
	let site = req
		.site
		.unwrap_or_else(|| schema.primary_site().handle.clone());

	// the sections that have one of the types, of those the role sees
	let sections: Vec<&str> = schema
		.sections
		.iter()
		.filter(|s| s.types.iter().any(|t| types.contains(&t.as_str())))
		.filter(|s| auth.can(schema, &s.handle, Access::View))
		.map(|s| s.handle.as_str())
		.collect();
	let entries = choices(&state.db.conn(), schema, &sections, &site)?;
	Ok(Json(ChoicesResp { entries }))
}

async fn get_route(
	State(state): State<AppState>,
	access: EntryAccess<View>,
	Req(req): Req<GetReq>,
) -> ApiResult<GetReq> {
	let conn = state.db.conn();
	let site = req
		.site
		.unwrap_or_else(|| state.schema.primary_site().handle.clone());

	let entry = get_entry(&conn, req.id, &site)?;
	let mut relations = refs(&conn, &[req.id], &site, false)?;
	viewable_refs(&access.auth, &state.schema, &mut relations);

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
		save(&conn, &state.schema, None, input, Some(auth.user_id))?
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
		let user = Some(access.auth.user_id);
		save(&conn, &state.schema, Some(req.id), req.input, user)?
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
	let user = Some(access.auth.user_id);
	crate::trash::trash_entry(&state.db.conn(), &state.schema, req.id, user)?;
	state.changed(req.id, &access.section, ChangeKind::Deleted);
	Ok(Json(()))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<ListReq, _>(list_route)
		.api::<ChoicesReq, _>(choices_route)
		.api::<GetReq, _>(get_route)
		.api::<CreateReq, _>(create_route)
		.api::<UpdateReq, _>(update_route)
		.api::<MoveReq, _>(move_route)
		.api::<DeleteReq, _>(delete_route)
}
