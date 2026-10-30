use super::api::*;
use super::*;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{AssetAccess, Delete, Edit, View};
use crate::schema::Access;

pub(super) fn transform_by_name(
	state: &AppState,
	name: &str,
) -> Option<Transform> {
	match name {
		"_thumb" => Some(Transform::new("_thumb").width(400).height(400)),
		_ => state.schema.transform_named(name),
	}
}

#[derive(Debug, Deserialize)]
pub(super) struct OriginalParam {
	/// the file as it was uploaded, without the edits
	original: Option<String>,
}

pub(super) async fn serve_original(
	State(state): State<AppState>,
	Path((id, filename)): Path<(i64, String)>,
	Query(p): Query<OriginalParam>,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	if asset.filename != filename {
		return Err(Error::NotFound);
	}
	// the edited image is a transform of its own size
	if asset.edits.is_some() && asset.kind == "image" && p.original.is_none() {
		return transformed(
			&state,
			asset,
			"_edited",
			Transform::new("_edited"),
		)
		.await;
	}
	let path = state.files.join(id.to_string()).join(&asset.filename);
	let data = fs::read(path).map_err(|_| Error::NotFound)?;
	Ok(file_response(&asset.mime, data))
}

pub(super) async fn serve_transform(
	State(state): State<AppState>,
	Path((id, name, filename)): Path<(i64, String, String)>,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	let t = transform_by_name(&state, &name).ok_or(Error::NotFound)?;
	if asset.filename != filename || asset.kind != "image" {
		return Err(Error::NotFound);
	}
	transformed(&state, asset, &name, t).await
}

/// A transform of an image, generated on the first request.
async fn transformed(
	state: &AppState,
	asset: Asset,
	name: &str,
	t: Transform,
) -> Result<Response> {
	let id = asset.id;
	let dir = state.files.join(id.to_string());
	let original = dir.join(&asset.filename);
	let out = dir.join("t").join(format!("{name}-{}", asset.filename));
	if !out.exists() {
		let _permit = GENERATING.acquire().await.unwrap();
		let (a, o) = (asset.clone(), out.clone());
		tokio::task::spawn_blocking(move || generate(&a, &original, &t, &o))
			.await
			.map_err(|e| Error::Internal(e.to_string()))??;
	}

	let data = fs::read(out).map_err(|e| Error::Internal(e.to_string()))?;
	Ok(file_response(asset.output_mime(), data))
}

/// Files are served from the studio's origin: nothing may run as a page
/// there. Images, video, audio and pdf show in the browser, everything
/// else (svg too) downloads.
pub(super) fn file_response(mime: &str, data: Vec<u8>) -> Response {
	let inline = (mime.starts_with("image/") && mime != "image/svg+xml")
		|| mime.starts_with("video/")
		|| mime.starts_with("audio/")
		|| mime == "application/pdf";
	(
		[
			(header::CONTENT_TYPE, mime.to_string()),
			(header::CACHE_CONTROL, "public, max-age=86400".into()),
			(header::X_CONTENT_TYPE_OPTIONS, "nosniff".into()),
			(header::CONTENT_SECURITY_POLICY, "sandbox".into()),
			(
				header::CONTENT_DISPOSITION,
				if inline { "inline" } else { "attachment" }.into(),
			),
		],
		data,
	)
		.into_response()
}

/// Transforms generated at once, each decodes a whole image.
pub(super) static GENERATING: tokio::sync::Semaphore =
	tokio::sync::Semaphore::const_new(2);

#[derive(Deserialize)]
pub(super) struct UploadParams {
	/// the folder the files go into, the top without
	folder: Option<i64>,
}

/// `POST /assets?folder=<id>`, multipart: the files of the upload.
pub(super) async fn upload(
	State(state): State<AppState>,
	AssetAccess { auth, .. }: AssetAccess<Edit>,
	Query(params): Query<UploadParams>,
	mut multipart: Multipart,
) -> Result<Json<Vec<Asset>>> {
	// the folder is there, before a file is stored
	if let Some(f) = params.folder {
		move_assets(&state.db.conn(), &[], Some(f))?;
	}
	let mut assets = vec![];
	while let Some(field) = multipart
		.next_field()
		.await
		.map_err(|e| Error::Request(e.to_string()))?
	{
		let filename = field.file_name().unwrap_or("file").to_string();
		let data = field
			.bytes()
			.await
			.map_err(|e| Error::Request(e.to_string()))?;
		let conn = state.db.conn();
		let asset =
			store(&conn, &state.files, &filename, &data, Some(auth.user_id))?;
		move_assets(&conn, &[asset.id], params.folder)?;
		assets.push(get_asset(&conn, asset.id)?);
	}
	Ok(Json(assets))
}

/// `POST /assets/{id}/file`, multipart: another file for the asset.
pub(super) async fn replace_route(
	State(state): State<AppState>,
	_access: AssetAccess<Edit>,
	Path(id): Path<i64>,
	mut multipart: Multipart,
) -> Result<Json<Asset>> {
	get_asset(&state.db.conn(), id)?;
	let field = multipart
		.next_field()
		.await
		.map_err(|e| Error::Request(e.to_string()))?
		.ok_or_else(|| Error::Request("no file".into()))?;
	let name = field.file_name().unwrap_or("file").to_string();
	let data = field
		.bytes()
		.await
		.map_err(|e| Error::Request(e.to_string()))?;
	let asset = replace(&state.db.conn(), &state.files, id, &name, &data)?;
	Ok(Json(asset))
}

pub(super) async fn list_route(
	State(state): State<AppState>,
	_access: AssetAccess<View>,
	Req(req): Req<ListReq>,
) -> ApiResult<ListReq> {
	let conn = state.db.conn();
	if let Some(ids) = req.ids {
		let ids: Vec<i64> =
			ids.split(',').filter_map(|i| i.parse().ok()).collect();
		return Ok(Json(get_many(&conn, &ids)?));
	}
	let folder = match req.folder.as_deref() {
		None | Some("") => InFolder::All,
		Some("top") => InFolder::Folder(None),
		Some(id) => InFolder::Folder(Some(
			id.parse()
				.map_err(|_| Error::Request("folder: an id or top".into()))?,
		)),
	};
	let search_for = req.search.unwrap_or_default();
	Ok(Json(search(
		&conn,
		&search_for,
		req.kind.as_deref(),
		folder,
	)?))
}

pub(super) async fn update_route(
	State(state): State<AppState>,
	_access: AssetAccess<Edit>,
	Req(req): Req<UpdateReq>,
) -> ApiResult<UpdateReq> {
	let conn = state.db.conn();
	Ok(Json(update(&conn, &state.files, req.id, req.change)?))
}

pub(super) async fn usage_route(
	State(state): State<AppState>,
	AssetAccess { auth, .. }: AssetAccess<View>,
	Req(req): Req<UsageReq>,
) -> ApiResult<UsageReq> {
	let primary = &state.schema.primary_site().handle;
	let found = usage(&state.db.conn(), primary, req.id)?;
	// only what the role may see
	Ok(Json(
		found
			.into_iter()
			.filter(|u| auth.can(&state.schema, &u.section, Access::View))
			.collect(),
	))
}

pub(super) async fn delete_route(
	State(state): State<AppState>,
	_access: AssetAccess<Delete>,
	Req(req): Req<DeleteReq>,
) -> ApiResult<DeleteReq> {
	delete(&state.db.conn(), &state.files, req.id)?;
	Ok(Json(()))
}

pub(super) async fn folders_route(
	State(state): State<AppState>,
	_access: AssetAccess<View>,
	Req(_): Req<FoldersReq>,
) -> ApiResult<FoldersReq> {
	Ok(Json(folders(&state.db.conn())?))
}

pub(super) async fn create_folder_route(
	State(state): State<AppState>,
	_access: AssetAccess<Edit>,
	Req(req): Req<CreateFolderReq>,
) -> ApiResult<CreateFolderReq> {
	let conn = state.db.conn();
	Ok(Json(create_folder(&conn, req.parent_id, &req.name)?))
}

pub(super) async fn update_folder_route(
	State(state): State<AppState>,
	_access: AssetAccess<Edit>,
	Req(req): Req<UpdateFolderReq>,
) -> ApiResult<UpdateFolderReq> {
	let conn = state.db.conn();
	Ok(Json(update_folder(
		&conn,
		req.id,
		req.parent_id,
		&req.name,
	)?))
}

pub(super) async fn delete_folder_route(
	State(state): State<AppState>,
	_access: AssetAccess<Delete>,
	Req(req): Req<DeleteFolderReq>,
) -> ApiResult<DeleteFolderReq> {
	delete_folder(&state.db.conn(), req.id)?;
	Ok(Json(()))
}

pub(super) async fn move_route(
	State(state): State<AppState>,
	_access: AssetAccess<Edit>,
	Req(req): Req<MoveReq>,
) -> ApiResult<MoveReq> {
	move_assets(&state.db.conn(), &req.ids, req.folder_id)?;
	Ok(Json(()))
}

/// The api, under `/api`.
pub fn api_routes() -> Router<AppState> {
	let big = || DefaultBodyLimit::max(200 * 1024 * 1024);
	Router::new()
		.route("/assets", post(upload).layer(big()))
		.route("/assets/{id}/file", post(replace_route).layer(big()))
		.api::<ListReq, _>(list_route)
		.api::<UpdateReq, _>(update_route)
		.api::<DeleteReq, _>(delete_route)
		.api::<UsageReq, _>(usage_route)
		.api::<FoldersReq, _>(folders_route)
		.api::<CreateFolderReq, _>(create_folder_route)
		.api::<UpdateFolderReq, _>(update_folder_route)
		.api::<DeleteFolderReq, _>(delete_folder_route)
		.api::<MoveReq, _>(move_route)
}

/// The files, under `/assets`.
pub fn file_routes() -> Router<AppState> {
	Router::new()
		.route("/assets/{id}/{filename}", get(serve_original))
		.route("/assets/{id}/{transform}/{filename}", get(serve_transform))
}
