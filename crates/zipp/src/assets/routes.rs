use std::io::Seek;

use axum::Json;
use axum::Router;
use axum::extract::multipart::Field;
use axum::extract::{DefaultBodyLimit, Multipart, Path, Query, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};

use super::api::*;
use super::*;
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{AssetAccess, Delete, Edit, View};
use crate::entries::query::parse_ids;
use crate::schema::Access;

#[derive(Deserialize)]
pub(super) struct UploadParams {
	/// the folder the files go into, the top without
	folder: Option<i64>,
}

/// A file of a multipart request into a temporary file, a chunk at a
/// time: uploads of 200 MB don't wait in memory.
async fn spool(
	files: &FsPath,
	mut field: Field<'_>,
) -> Result<tempfile::NamedTempFile> {
	use tokio::io::AsyncWriteExt;
	let tmp = temp_file(files)?;
	let mut out = tokio::fs::File::from_std(tmp.reopen()?);
	while let Some(chunk) = field
		.chunk()
		.await
		.map_err(|e| Error::Request(e.to_string()))?
	{
		out.write_all(&chunk).await?;
	}
	out.flush().await?;
	Ok(tmp)
}

/// The first file of a multipart request.
async fn first_file(
	files: &FsPath,
	multipart: &mut Multipart,
) -> Result<(String, tempfile::NamedTempFile)> {
	let field = multipart
		.next_field()
		.await
		.map_err(|e| Error::Request(e.to_string()))?
		.ok_or_else(|| Error::Request("no file".into()))?;
	let name = field.file_name().unwrap_or("file").to_string();
	Ok((name, spool(files, field).await?))
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
		check_folder(&state.db.conn(), Some(f))?;
	}
	let mut assets = vec![];
	while let Some(field) = multipart
		.next_field()
		.await
		.map_err(|e| Error::Request(e.to_string()))?
	{
		let filename = field.file_name().unwrap_or("file").to_string();
		let tmp = spool(&state.files, field).await?;
		let conn = state.db.conn();
		let user = Some(auth.user_id);
		let asset = store_file(&conn, &state.files, &filename, tmp, user)?;
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
	let (name, tmp) = first_file(&state.files, &mut multipart).await?;
	let asset = replace(&state.db.conn(), &state.files, id, &name, tmp)?;
	Ok(Json(asset))
}

/// `POST /assets/unzip?folder=<id>`, multipart: a zip, its folders
/// become folders in the folder, its files assets. It is unpacked before
/// the database is taken.
pub(super) async fn unzip_route(
	State(state): State<AppState>,
	AssetAccess { auth, .. }: AssetAccess<Edit>,
	Query(params): Query<UploadParams>,
	mut multipart: Multipart,
) -> Result<Json<Unzipped>> {
	let (_, tmp) = first_file(&state.files, &mut multipart).await?;
	let user = Some(auth.user_id);
	let unzipped = tokio::task::spawn_blocking(move || {
		let zip = read_zip(&state.files, tmp.as_file(), &Limits::default())?;
		let conn = state.db.conn();
		unzip(&conn, &state.files, zip, params.folder, user)
	})
	.await??;
	Ok(Json(unzipped))
}

#[derive(Deserialize)]
pub(super) struct ZipParams {
	/// `1,2,3`
	ids: Option<String>,
	folder: Option<i64>,
}

/// `GET /assets/zip?ids=1,2,3` or `?folder=<id>`: a zip of the assets,
/// or of the folder with its subfolders. It is made in a file without a
/// name, gone once sent, so a big one takes no memory.
pub(super) async fn zip_route(
	State(state): State<AppState>,
	_access: AssetAccess<View>,
	Query(p): Query<ZipParams>,
) -> Result<Response> {
	let (of, name) = match (p.folder, p.ids) {
		(Some(id), _) => {
			(ZipOf::Folder(id), folder(&state.db.conn(), id)?.name)
		}
		(None, Some(ids)) => (
			ZipOf::Assets(parse_ids("ids", &ids).map_err(Error::Request)?),
			"assets".into(),
		),
		(None, None) => {
			return Err(Error::Request("ids or a folder".into()));
		}
	};
	let entries = zip_files(&state.db.conn(), &of)?;
	let files = state.files.clone();
	let zip = tokio::task::spawn_blocking(move || -> Result<fs::File> {
		let mut zip =
			write_zip(&files, &entries, tempfile::tempfile_in(&files)?)?;
		zip.rewind()?;
		Ok(zip)
	})
	.await??;

	let len = zip.metadata()?.len();
	let body = axum::body::Body::from_stream(
		tokio_util::io::ReaderStream::new(tokio::fs::File::from_std(zip)),
	);
	let filename = format!("{}.zip", clean_filename(&name));
	Ok((
		[
			(header::CONTENT_TYPE, "application/zip".to_string()),
			(header::CONTENT_LENGTH, len.to_string()),
			(
				header::CONTENT_DISPOSITION,
				format!("attachment; filename=\"{filename}\""),
			),
		],
		body,
	)
		.into_response())
}

pub(super) async fn list_route(
	State(state): State<AppState>,
	_access: AssetAccess<View>,
	Req(req): Req<ListReq>,
) -> ApiResult<ListReq> {
	let conn = state.db.conn();
	if let Some(ids) = req.ids {
		let ids = parse_ids("ids", &ids).map_err(Error::Request)?;
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
	let extensions: Vec<String> = req
		.extensions
		.iter()
		.flat_map(|e| e.split(','))
		.map(|e| e.trim().trim_start_matches('.').to_lowercase())
		.filter(|e| !e.is_empty())
		.collect();
	Ok(Json(search(
		&conn,
		&search_for,
		req.kind.as_deref(),
		folder,
		&extensions,
	)?))
}

pub(super) async fn update_route(
	State(state): State<AppState>,
	_access: AssetAccess<Edit>,
	Req(req): Req<UpdateReq>,
) -> ApiResult<UpdateReq> {
	let conn = state.db.conn();
	Ok(Json(update(
		&conn,
		&state.files,
		&state.schema,
		req.id,
		req.change,
	)?))
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
	access: AssetAccess<Delete>,
	Req(req): Req<DeleteReq>,
) -> ApiResult<DeleteReq> {
	let user = Some(access.auth.user_id);
	crate::trash::trash_asset(&state.db.conn(), req.id, user)?;
	Ok(Json(()))
}

pub(super) async fn folders_route(
	State(state): State<AppState>,
	_access: AssetAccess<View>,
	Req(_): Req<FoldersReq>,
) -> ApiResult<FoldersReq> {
	let conn = state.db.conn();
	Ok(Json(Folders {
		folders: folders(&conn)?,
		top: top_count(&conn)?,
	}))
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
	let big = || DefaultBodyLimit::max(MAX_UPLOAD);
	Router::new()
		.route("/assets", post(upload).layer(big()))
		.route("/assets/unzip", post(unzip_route).layer(big()))
		.route("/assets/zip", get(zip_route))
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
