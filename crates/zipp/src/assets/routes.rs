use super::*;

pub(super) fn transform_by_name(
	state: &AppState,
	name: &str,
) -> Option<Transform> {
	match name {
		"_thumb" => Some(Transform::new("_thumb").width(400).height(400)),
		// the image of a page when it is shared, see seo
		crate::seo::SOCIAL_TRANSFORM => Some(crate::seo::social_transform()),
		_ => state.schema.transform_named(name),
	}
}

pub(super) async fn serve_original(
	State(state): State<AppState>,
	Path((id, filename)): Path<(i64, String)>,
) -> Result<Response> {
	let asset = get_asset(&state.db.conn(), id)?;
	if asset.filename != filename {
		return Err(Error::NotFound);
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
	Ok(file_response(&asset.mime, data))
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

pub(super) async fn upload(
	State(state): State<AppState>,
	auth: Authed,
	mut multipart: Multipart,
) -> Result<Json<Vec<Asset>>> {
	auth.check_assets(&state.schema, crate::schema::Access::Edit)?;
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
		assets.push(store(&conn, &state.files, &filename, &data)?);
	}
	Ok(Json(assets))
}

#[derive(Debug, Deserialize)]
pub(super) struct ListParams {
	search: Option<String>,
	ids: Option<String>,
	kind: Option<String>,
}

pub(super) async fn list(
	State(state): State<AppState>,
	auth: Authed,
	Query(p): Query<ListParams>,
) -> Result<Json<Vec<Asset>>> {
	auth.check_assets(&state.schema, crate::schema::Access::View)?;
	let conn = state.db.conn();
	if let Some(ids) = p.ids {
		let ids: Vec<i64> =
			ids.split(',').filter_map(|i| i.parse().ok()).collect();
		return Ok(Json(get_many(&conn, &ids)?));
	}

	let sql = format!(
		"SELECT {COLUMNS} FROM assets
		WHERE filename LIKE ?1 AND (?2 IS NULL OR kind = ?2)
		ORDER BY id DESC LIMIT 200"
	);
	let search = format!("%{}%", p.search.unwrap_or_default());
	let mut stmt = conn.prepare(&sql)?;
	let assets = stmt
		.query_map(params![search, p.kind], from_row)?
		.collect::<rusqlite::Result<_>>()?;
	Ok(Json(assets))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UpdateReq {
	/// unchanged when missing
	title: Option<String>,
	alt: String,
	focal_x: f64,
	focal_y: f64,
}

pub(super) async fn update(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	Json(req): Json<UpdateReq>,
) -> Result<Json<Asset>> {
	auth.check_assets(&state.schema, crate::schema::Access::Edit)?;
	let conn = state.db.conn();
	let old = get_asset(&conn, id)?;
	let (fx, fy) = (req.focal_x.clamp(0.0, 1.0), req.focal_y.clamp(0.0, 1.0));
	conn.execute(
		"UPDATE assets SET title = IFNULL(?1, title), alt = ?2, focal_x = ?3,
		focal_y = ?4 WHERE id = ?5",
		params![req.title.as_deref().map(str::trim), req.alt, fx, fy, id],
	)?;
	if (old.focal_x, old.focal_y) != (fx, fy) {
		let _ = fs::remove_dir_all(state.files.join(id.to_string()).join("t"));
	}
	Ok(Json(get_asset(&conn, id)?))
}

pub(super) async fn delete(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<()> {
	auth.check_assets(&state.schema, crate::schema::Access::Delete)?;
	let conn = state.db.conn();
	if conn.execute("DELETE FROM assets WHERE id = ?1", [id])? == 0 {
		return Err(Error::NotFound);
	}
	let _ = fs::remove_dir_all(state.files.join(id.to_string()));
	Ok(())
}

/// The api, under `/api`.
pub fn api_routes() -> Router<AppState> {
	Router::new()
		.route(
			"/assets",
			get(list)
				.post(upload)
				.layer(DefaultBodyLimit::max(200 * 1024 * 1024)),
		)
		.route("/assets/{id}", put(update).delete(delete))
}

/// The files, under `/assets`.
pub fn file_routes() -> Router<AppState> {
	Router::new()
		.route("/assets/{id}/{filename}", get(serve_original))
		.route("/assets/{id}/{transform}/{filename}", get(serve_transform))
}
