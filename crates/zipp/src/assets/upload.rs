//! Uploads: the files of a multipart request, and uploads of plugins as
//! a user ([`upload_as`]).

use axum::extract::Multipart;
use axum::extract::multipart::Field;

use super::*;
use crate::AppState;
use crate::auth::Authed;
use crate::schema::Access;

/// A file of a multipart request into a temporary file, a chunk at a
/// time: uploads of 200 MB don't wait in memory.
pub(super) async fn spool(
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
pub(super) async fn first_file(
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

/// What an upload of [`upload_as`] may be.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UploadRules {
	pub max_bytes: u64,
	pub images_only: bool,
	/// the folder it goes into, like `Members/4`, made when it is
	/// missing; `None` the top
	pub folder: Option<String>,
}

impl UploadRules {
	/// Any file up to `max_bytes`, on the top. The route needs a body
	/// limit above it (`DefaultBodyLimit`), axum's is 2 MB.
	pub fn new(max_bytes: u64) -> Self {
		Self {
			max_bytes,
			images_only: false,
			folder: None,
		}
	}

	pub fn images_only(mut self) -> Self {
		self.images_only = true;
		self
	}

	pub fn folder(mut self, path: &str) -> Self {
		self.folder = Some(path.into());
		self
	}
}

/// Stores the first file of a multipart request as the user, who needs
/// to edit assets, like an upload in the studio.
pub async fn upload_as(
	state: &AppState,
	auth: &Authed,
	multipart: &mut Multipart,
	rules: &UploadRules,
) -> Result<Asset> {
	auth.check_assets(&state.schema, Access::Edit)?;
	let (name, tmp) = first_file(&state.files, multipart).await?;

	if tmp.as_file().metadata()?.len() > rules.max_bytes {
		let mb = rules.max_bytes as f64 / (1024.0 * 1024.0);
		return Err(Error::Request(format!(
			"the file is too big, at most {mb:.1} MB"
		)));
	}
	if rules.images_only && inspect(&name, tmp.path())?.kind != "image" {
		return Err(Error::Request("only images".into()));
	}

	let conn = state.db.conn();
	let folder = match &rules.folder {
		Some(path) => Some(folder_at(&conn, path)?),
		None => None,
	};
	let asset =
		store_file(&conn, &state.files, &name, tmp, Some(auth.user_id))?;
	move_assets(&conn, &[asset.id], folder)?;
	get_asset(&conn, asset.id)
}
