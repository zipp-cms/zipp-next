use super::*;

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
		assets.push(store(
			&conn,
			&state.files,
			&filename,
			&data,
			Some(auth.user_id),
		)?);
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
	/// a new name, unchanged when missing
	filename: Option<String>,
	/// the image editor's, none resets to the original
	#[serde(default)]
	edits: Option<Edits>,
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
	let edits = req.edits.filter(|e| !e.is_empty());
	if let Some(e) = &edits {
		if old.kind != "image" {
			return Err(Error::Request("only images can be edited".into()));
		}
		e.check().map_err(Error::Request)?;
		let (ow, oh) = (
			old.original_width.unwrap_or(1),
			old.original_height.unwrap_or(1),
		);
		let (w, h) = e.size(ow, oh);
		// a crop past the image is made before a resize makes it smaller
		let crop = Edits {
			resize: None,
			canvas: None,
			..e.clone()
		};
		let (cw, ch) = crop.size(ow, oh);
		if u64::from(w) * u64::from(h) > MAX_PIXELS
			|| u64::from(cw) * u64::from(ch) > MAX_PIXELS
		{
			return Err(Error::Request(
				"the edited image is bigger than 40 megapixels".into(),
			));
		}
	}
	let dir = state.files.join(id.to_string());
	// a new name: the file moves, its transforms are made again
	let filename = match req.filename.as_deref().map(clean_filename) {
		Some(name) if name != old.filename => {
			if dir.join(&name).exists() {
				return Err(Error::Request(format!("{name} exists already")));
			}
			fs::rename(dir.join(&old.filename), dir.join(&name))
				.map_err(|e| Error::Internal(e.to_string()))?;
			name
		}
		_ => old.filename.clone(),
	};
	let json = edits.as_ref().map(|e| serde_json::to_string(e).unwrap());
	conn.execute(
		"UPDATE assets SET title = IFNULL(?1, title), alt = ?2, focal_x = ?3,
		focal_y = ?4, filename = ?5, edits = ?6 WHERE id = ?7",
		params![
			req.title.as_deref().map(str::trim),
			req.alt,
			fx,
			fy,
			filename,
			json,
			id
		],
	)?;
	if (old.focal_x, old.focal_y) != (fx, fy)
		|| old.edits != edits
		|| old.filename != filename
	{
		let _ = fs::remove_dir_all(dir.join("t"));
	}
	Ok(Json(get_asset(&conn, id)?))
}

/// Another file for the asset, like Craft's replace file: where it is
/// used stays, the edits and the focal point start new.
pub(super) async fn replace(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
	mut multipart: Multipart,
) -> Result<Json<Asset>> {
	auth.check_assets(&state.schema, crate::schema::Access::Edit)?;
	let old = get_asset(&state.db.conn(), id)?;
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
	let up = inspect(&name, &data)?;

	let dir = state.files.join(id.to_string());
	let internal = |e: std::io::Error| Error::Internal(e.to_string());
	let tmp = dir.join(format!(".replace{}", rand::random::<u32>()));
	fs::write(&tmp, &data).map_err(internal)?;
	if old.filename != up.filename {
		let _ = fs::remove_file(dir.join(&old.filename));
	}
	fs::rename(&tmp, dir.join(&up.filename)).map_err(internal)?;
	let _ = fs::remove_dir_all(dir.join("t"));

	let conn = state.db.conn();
	conn.execute(
		"UPDATE assets SET filename = ?1, kind = ?2, mime = ?3, size = ?4,
		width = ?5, height = ?6, edits = NULL, focal_x = 0.5, focal_y = 0.5
		WHERE id = ?7",
		params![
			up.filename,
			up.kind,
			up.mime,
			data.len(),
			up.width,
			up.height,
			id
		],
	)?;
	Ok(Json(get_asset(&conn, id)?))
}

/// An entry that uses an asset.
#[derive(Debug, Serialize)]
pub(super) struct Usage {
	id: i64,
	section: String,
	#[serde(rename = "type")]
	ty: String,
	title: String,
	/// the sites it is used in
	sites: Vec<String>,
}

/// The entries that use an asset, as published.
pub(super) async fn usage(
	State(state): State<AppState>,
	auth: Authed,
	Path(id): Path<i64>,
) -> Result<Json<Vec<Usage>>> {
	auth.check_assets(&state.schema, crate::schema::Access::View)?;
	let conn = state.db.conn();
	let primary = &state.schema.primary_site().handle;
	let mut stmt = conn.prepare(
		"SELECT e.id, e.section, e.type, group_concat(DISTINCT r.site),
		(SELECT title FROM entry_sites s WHERE s.entry_id = e.id
			ORDER BY s.site = ?2 DESC LIMIT 1)
		FROM asset_relations r JOIN entries e ON e.id = r.entry_id
		WHERE r.asset_id = ?1 GROUP BY e.id ORDER BY e.id",
	)?;
	let found = stmt
		.query_map(params![id, primary], |r| {
			let sites: String = r.get(3)?;
			Ok(Usage {
				id: r.get(0)?,
				section: r.get(1)?,
				ty: r.get(2)?,
				sites: sites.split(',').map(String::from).collect(),
				title: r.get(4)?,
			})
		})?
		.collect::<rusqlite::Result<Vec<_>>>()?;
	// only what the role may see
	Ok(Json(
		found
			.into_iter()
			.filter(|u| {
				auth.can(&state.schema, &u.section, crate::schema::Access::View)
			})
			.collect(),
	))
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
		.route(
			"/assets/{id}/file",
			post(replace).layer(DefaultBodyLimit::max(200 * 1024 * 1024)),
		)
		.route("/assets/{id}/usage", get(usage))
}

/// The files, under `/assets`.
pub fn file_routes() -> Router<AppState> {
	Router::new()
		.route("/assets/{id}/{filename}", get(serve_original))
		.route("/assets/{id}/{transform}/{filename}", get(serve_transform))
}
