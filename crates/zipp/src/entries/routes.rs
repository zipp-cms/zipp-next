use super::api::*;
use super::*;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{Admin, Delete, EntryAccess, Publish, View};

/// Only the related entries the role sees: the chips of the others would
/// show their titles and pages.
pub(crate) fn viewable_refs(
	auth: &Authed,
	schema: &Schema,
	refs: &mut BTreeMap<i64, EntryRef>,
) {
	refs.retain(|_, r| {
		auth.can_entry(schema, &r.section, r.author_id, Access::View)
	});
}

async fn list_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(ListReq(params)): Req<ListReq>,
) -> ApiResult<ListReq> {
	let mut q = Query::parse(&state.schema, &params).map_err(Error::Request)?;
	restrict(&auth, &state.schema, &mut q)?;
	let conn = state.db.conn();
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

/// The parts of a list `a,b`; a trailing comma adds nothing, like in
/// `parse_ids`.
fn parts(list: &str) -> impl Iterator<Item = &str> {
	list.split(',').filter(|s| !s.is_empty())
}

async fn choices_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<ChoicesReq>,
) -> ApiResult<ChoicesReq> {
	let schema = &state.schema;
	let types: Vec<&str> = parts(&req.types).collect();
	if let Some(t) = types.iter().find(|t| schema.entry_type(t).is_none()) {
		return Err(Error::Request(format!("unknown entry type {t}")));
	}
	let only: Vec<&str> = req.sections.iter().flat_map(|s| parts(s)).collect();
	if let Some(s) = only.iter().find(|s| schema.get_section(s).is_none()) {
		return Err(Error::Request(format!("unknown section {s}")));
	}
	let site = schema.site_or_primary(req.site);

	// the sections that have one of the types, of those the role sees
	let sections: Vec<&str> = schema
		.sections
		.iter()
		.filter(|s| s.types.iter().any(|t| types.contains(&t.as_str())))
		.filter(|s| only.is_empty() || only.contains(&s.handle.as_str()))
		.filter(|s| auth.can_list(schema, &s.handle))
		.map(|s| s.handle.as_str())
		.collect();
	let own = auth.own_only(schema);
	let conn = state.db.conn();
	let search = req.search.as_deref().map(str::trim).unwrap_or_default();
	Ok(Json(match search {
		"" => {
			let offset = req.offset.unwrap_or(0).max(0);
			choices(&conn, schema, &types, &sections, &site, offset, own)?
		}
		q => search_choices(&conn, schema, &types, &sections, &site, q, own)?,
	}))
}

async fn get_route(
	State(state): State<AppState>,
	access: EntryAccess<View>,
	Req(req): Req<GetReq>,
) -> ApiResult<GetReq> {
	let conn = state.db.conn();
	let site = state.schema.site_or_primary(req.site);

	let entry = get_entry(&conn, req.id, &site)?;
	let mut relations = refs(&conn, &[req.id], &site, false)?;
	viewable_refs(&access.auth, &state.schema, &mut relations);
	let ids: Vec<i64> = entry.author_id.into_iter().collect();
	let author = authors(&conn, &ids)?.into_values().next();

	Ok(Json(EntryResp {
		entry,
		relations,
		author,
	}))
}

async fn create_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(CreateReq(input)): Req<CreateReq>,
) -> ApiResult<CreateReq> {
	Ok(Json(create_as(&state, &auth, input)?))
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

async fn author_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(req): Req<AuthorReq>,
) -> ApiResult<AuthorReq> {
	set_author(&state.db.conn(), req.id, req.author_id)?;
	Ok(Json(()))
}

async fn delete_route(
	State(state): State<AppState>,
	access: EntryAccess<Delete>,
	Req(req): Req<DeleteReq>,
) -> ApiResult<DeleteReq> {
	let user = Some(access.auth.user_id);
	trash(&state, req.id, &access.section, user)?;
	Ok(Json(()))
}

pub(crate) fn routes() -> Router<AppState> {
	Router::new()
		.api::<ListReq, _>(list_route)
		.api::<ChoicesReq, _>(choices_route)
		.api::<GetReq, _>(get_route)
		.api::<CreateReq, _>(create_route)
		.api::<UpdateReq, _>(update_route)
		.api::<MoveReq, _>(move_route)
		.api::<AuthorReq, _>(author_route)
		.api::<DeleteReq, _>(delete_route)
}
