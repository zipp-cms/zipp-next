use std::io::Seek;

use axum::extract::multipart::Field;
use axum::http::StatusCode;
use tower::ServiceExt;
use tower_http::services::ServeFile;

use super::api::*;
use super::*;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::{AssetAccess, Delete, Edit, View};
use crate::entries::query::MAX_IDS;
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
	req: axum::extract::Request,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	if asset.filename != filename {
		return Err(Error::NotFound);
	}
	// the edited image is a transform of its own size
	if asset.edits.is_some() && asset.kind == "image" && p.original.is_none() {
		let t = Transform::new("_edited");
		return transformed(&state, asset, "_edited", t, None, req).await;
	}
	let path = asset_dir(&state.files, id).join(&asset.filename);
	serve_file(path, &asset.mime, req).await
}

/// A file from the disk with ranges, a video plays and seeks without
/// loading the whole file, and its date: a cache asks again after a
/// while, a deleted file goes from it.
async fn serve_file(
	path: PathBuf,
	mime: &str,
	req: axum::extract::Request,
) -> Result<Response> {
	let parsed = mime.parse().map_err(|_| Error::NotFound)?;
	let resp = ServeFile::new_with_mime(path, &parsed)
		.oneshot(req)
		.await
		.map_err(|e| Error::Internal(e.to_string()))?;
	if resp.status() == StatusCode::NOT_FOUND {
		return Err(Error::NotFound);
	}
	let (mut parts, body) = resp.into_parts();
	for (name, value) in file_headers(mime) {
		parts.headers.insert(name, value.parse().unwrap());
	}
	Ok(Response::from_parts(parts, axum::body::Body::new(body)))
}

pub(super) async fn serve_transform(
	State(state): State<AppState>,
	Path((id, name, filename)): Path<(i64, String, String)>,
	req: axum::extract::Request,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	let t = transform_by_name(&state, &name).ok_or(Error::NotFound)?;
	if asset.kind != "image" {
		return Err(Error::NotFound);
	}
	// the image's name, or its name with the extension of another type
	let format = match filename == asset.filename {
		true => None,
		false => Some(
			Format::ALL
				.into_iter()
				.find(|f| asset.format_filename(*f) == filename)
				.ok_or(Error::NotFound)?,
		),
	};
	transformed(&state, asset, &name, t, format, req).await
}

/// A transform of an image, generated on the first request, in `format`
/// or the type of the image.
async fn transformed(
	state: &AppState,
	asset: Asset,
	name: &str,
	t: Transform,
	format: Option<Format>,
	req: axum::extract::Request,
) -> Result<Response> {
	let id = asset.id;
	let dir = asset_dir(&state.files, id);
	let original = dir.join(&asset.filename);
	let (filename, mime) = match format {
		Some(f) => (asset.format_filename(f), f.mime()),
		None => (asset.filename.clone(), asset.output_mime()),
	};
	let out = dir.join(TRANSFORMS).join(format!("{name}-{filename}"));
	if !out.exists() {
		let _permit = GENERATING.acquire().await.unwrap();
		// not made already while this one waited
		if !out.exists() {
			let (a, o, m) = (asset.clone(), out.clone(), mime.to_string());
			tokio::task::spawn_blocking(move || {
				generate(&a, &original, &t, &m, &o)
			})
			.await??;
		}
	}

	serve_file(out, mime, req).await
}

/// Files are served from the studio's origin: nothing may run as a page
/// there. Images, video, audio and pdf show in the browser, everything
/// else (svg too) downloads. Caches keep them five minutes, then ask
/// with the date (`Last-Modified`): one in the trash isn't shown long.
pub(super) fn file_headers(mime: &str) -> [(header::HeaderName, String); 5] {
	let inline = (mime.starts_with("image/") && mime != "image/svg+xml")
		|| mime.starts_with("video/")
		|| mime.starts_with("audio/")
		|| mime == "application/pdf";
	[
		(header::CONTENT_TYPE, mime.to_string()),
		(header::CACHE_CONTROL, "public, max-age=300".into()),
		(header::X_CONTENT_TYPE_OPTIONS, "nosniff".into()),
		(header::CONTENT_SECURITY_POLICY, "sandbox".into()),
		(
			header::CONTENT_DISPOSITION,
			if inline { "inline" } else { "attachment" }.into(),
		),
	]
}

/// Transforms generated at once, each decodes a whole image.
pub(super) static GENERATING: tokio::sync::Semaphore =
	tokio::sync::Semaphore::const_new(2);

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

/// `1,2,3`, at most [`MAX_IDS`] of them.
fn id_list(ids: &str) -> Result<Vec<i64>> {
	let ids: Vec<i64> = ids.split(',').filter_map(|i| i.parse().ok()).collect();
	if ids.len() > MAX_IDS {
		return Err(Error::Request(format!("more than {MAX_IDS} ids")));
	}
	Ok(ids)
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
		(None, Some(ids)) => (ZipOf::Assets(id_list(&ids)?), "assets".into()),
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
		let ids = id_list(&ids)?;
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

/// The files, under `/assets`.
pub fn file_routes() -> Router<AppState> {
	Router::new()
		.route("/assets/{id}/{filename}", get(serve_original))
		.route("/assets/{id}/{transform}/{filename}", get(serve_transform))
}
