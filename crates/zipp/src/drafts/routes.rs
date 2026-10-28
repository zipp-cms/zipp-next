use super::*;

use crate::schema::Access;

/// The role of the user may do `access` in the entry's section.
fn need(
	state: &AppState,
	auth: &Authed,
	id: i64,
	access: Access,
) -> Result<()> {
	let section = crate::entries::section_of(&state.db.conn(), id)?;
	auth.check(&state.schema, &section, access)
}

#[derive(Debug, Serialize)]
pub(super) struct DraftResp {
	draft: Option<Draft>,
}

pub(super) async fn get_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<DraftResp>> {
	need(&state, &auth, id, Access::View)?;
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
	need(&state, &auth, id, Access::Edit)?;
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
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	need(&state, &auth, id, Access::Edit)?;
	discard(&state.db.conn(), id)
}

pub(super) async fn publish_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	need(&state, &auth, id, Access::Publish)?;
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
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<Vec<Revision>>> {
	need(&state, &auth, id, Access::View)?;
	Ok(Json(revisions(&state.db.conn(), id)?))
}

pub(super) async fn revision_route(
	State(state): State<AppState>,
	auth: Authed,
	Path((id, rev)): Path<(i64, i64)>,
) -> Result<Json<Doc>> {
	need(&state, &auth, id, Access::View)?;
	Ok(Json(get_revision(&state.db.conn(), id, rev)?))
}

pub(super) async fn restore_route(
	State(state): State<AppState>,
	auth: Authed,
	Path((id, rev)): Path<(i64, i64)>,
) -> Result<Json<Draft>> {
	need(&state, &auth, id, Access::Edit)?;
	let conn = state.db.conn();
	Ok(Json(restore(&conn, id, rev, Some(auth.user_id))?))
}

#[derive(Debug, Serialize)]
pub(super) struct TokenResp {
	token: String,
}

pub(super) async fn site_preview_route(
	State(state): State<AppState>,
	auth: Authed,
) -> Result<Json<TokenResp>> {
	// every draft of every section: the AI page's
	if !auth.can_ai(&state.schema) {
		return Err(Error::Forbidden);
	}
	let token = create_site_token(&state.db.conn())?;
	Ok(Json(TokenResp { token }))
}

pub(super) async fn preview_route(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<TokenResp>> {
	need(&state, &auth, id, Access::View)?;
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
