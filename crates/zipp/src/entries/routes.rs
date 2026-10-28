use super::*;

#[derive(Debug, Serialize)]
pub(super) struct ListResp {
	entries: Vec<Entry>,
	total: i64,
	relations: BTreeMap<i64, EntryRef>,
	/// whether each entry is published, per site
	published: BTreeMap<i64, BTreeMap<String, bool>>,
	/// the entries with unpublished changes
	drafts: Vec<i64>,
}

pub(super) async fn list_route(
	State(state): State<AppState>,
	auth: Authed,
	QueryParams(params): QueryParams<Vec<(String, String)>>,
) -> Result<Json<ListResp>> {
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

#[derive(Debug, Deserialize)]
pub(super) struct SiteParam {
	site: Option<String>,
}

#[derive(Debug, Serialize)]
pub(super) struct EntryResp {
	entry: Entry,
	relations: BTreeMap<i64, EntryRef>,
}

pub(super) async fn get_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	QueryParams(p): QueryParams<SiteParam>,
) -> Result<Json<EntryResp>> {
	let conn = state.db.conn();
	auth.check(&state.schema, &section_of(&conn, id)?, Access::View)?;
	let site = p
		.site
		.unwrap_or_else(|| state.schema.primary_site().handle.clone());

	let entry = get_entry(&conn, id, &site)?;
	let relations = refs(&conn, &[id], &site, false)?;

	Ok(Json(EntryResp { entry, relations }))
}

pub(super) async fn create_route(
	State(state): State<AppState>,
	auth: Authed,
	Json(mut input): Json<EntryInput>,
) -> Result<Json<Entry>> {
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

/// Saves the live entry directly, without a draft (the api of scripts).
pub(super) async fn update_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	Json(input): Json<EntryInput>,
) -> Result<Json<Entry>> {
	let entry = {
		let conn = state.db.conn();
		// live at once, like publishing
		auth.check(&state.schema, &section_of(&conn, id)?, Access::Publish)?;
		let entry = save(&conn, &state.schema, Some(id), input)?;
		record_revision(&conn, id, Some(auth.user_id))?;
		entry
	};
	state.changed(id, &entry.section, ChangeKind::Saved);
	Ok(Json(entry))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct MoveReq {
	parent_id: Option<i64>,
	index: usize,
}

pub(super) async fn move_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	Json(req): Json<MoveReq>,
) -> Result<()> {
	let section = {
		let conn = state.db.conn();
		auth.check(&state.schema, &section_of(&conn, id)?, Access::Publish)?;
		move_entry(&conn, &state.schema, id, req.parent_id, req.index)?;
		section_of(&conn, id)?
	};
	state.changed(id, &section, ChangeKind::Moved);
	Ok(())
}

pub(super) async fn delete_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	let section = {
		let conn = state.db.conn();
		let section = section_of(&conn, id)?;
		auth.check(&state.schema, &section, Access::Delete)?;
		delete(&conn, &state.schema, id)?;
		section
	};
	state.changed(id, &section, ChangeKind::Deleted);
	Ok(())
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/entries", get(list_route).post(create_route))
		.route(
			"/entries/{id}",
			get(get_route).put(update_route).delete(delete_route),
		)
		.route("/entries/{id}/move", post(move_route))
}
