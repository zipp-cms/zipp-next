use super::*;

#[derive(Debug, Serialize)]
pub(super) struct DraftResp {
	draft: Option<Draft>,
}

pub(super) async fn get_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<DraftResp>> {
	let draft = get_draft(&state.db.conn(), id)?;
	Ok(Json(DraftResp { draft }))
}

#[derive(Debug, Deserialize)]
pub(super) struct PutReq {
	site: String,
	/// the version of the draft the editor knows
	base: Option<i64>,
	#[serde(flatten)]
	content: SiteDoc,
}

pub(super) async fn put_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	Json(req): Json<PutReq>,
) -> Result<Json<Draft>> {
	let conn = state.db.conn();
	let d = put_draft(
		&conn,
		&state.schema,
		id,
		&req.site,
		req.content,
		req.base,
		Some(auth.user_id),
	)?;
	Ok(Json(d))
}

pub(super) async fn delete_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	discard(&state.db.conn(), id)
}

pub(super) async fn publish_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	let section = {
		let conn = state.db.conn();
		publish(&conn, &state.schema, id, Some(auth.user_id))?;
		crate::entries::section_of(&conn, id)?
	};
	state.changed(id, &section, crate::ChangeKind::Published);
	Ok(())
}

pub(super) async fn revisions_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<Vec<Revision>>> {
	Ok(Json(revisions(&state.db.conn(), id)?))
}

pub(super) async fn revision_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path((id, rev)): Path<(i64, i64)>,
) -> Result<Json<Doc>> {
	Ok(Json(get_revision(&state.db.conn(), id, rev)?))
}

pub(super) async fn restore_route(
	State(state): State<AppState>,
	auth: Authed,
	Path((id, rev)): Path<(i64, i64)>,
) -> Result<Json<Draft>> {
	let conn = state.db.conn();
	Ok(Json(restore(&conn, id, rev, Some(auth.user_id))?))
}

#[derive(Debug, Serialize)]
pub(super) struct TokenResp {
	token: String,
}

pub(super) async fn site_preview_route(
	State(state): State<AppState>,
	_auth: Authed,
) -> Result<Json<TokenResp>> {
	let token = create_site_token(&state.db.conn())?;
	Ok(Json(TokenResp { token }))
}

pub(super) async fn preview_route(
	State(state): State<AppState>,
	_auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<TokenResp>> {
	let token = create_token(&state.db.conn(), id)?;
	Ok(Json(TokenResp { token }))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route(
			"/entries/{id}/draft",
			get(get_route).put(put_route).delete(delete_route),
		)
		.route("/entries/{id}/publish", post(publish_route))
		.route("/entries/{id}/revisions", get(revisions_route))
		.route("/entries/{id}/revisions/{rev}", get(revision_route))
		.route("/entries/{id}/revisions/{rev}/restore", post(restore_route))
		.route("/entries/{id}/preview", post(preview_route))
		.route("/preview", post(site_preview_route))
}
