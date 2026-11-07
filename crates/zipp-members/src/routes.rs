//! The handlers of [`api`](crate::api): who (a [`SiteUser`]), whether
//! the entry is theirs and in a section of the plugin, then zipp's
//! functions as the user, which check the role.

use std::sync::Arc;

use serde_json::Value;
use zipp::AppState;
use zipp::api::{ApiResult, ApiRouter, Req};
use zipp::assets::{UploadRules, upload_as};
use zipp::auth::{Authed, SiteUser};
use zipp::axum::extract::{DefaultBodyLimit, Multipart, State};
use zipp::axum::routing::post;
use zipp::axum::{Extension, Json, Router};
use zipp::db::{MINUTE_MS, now};
use zipp::entries::query::Query;
use zipp::entries::{
	EntryInput, WithDraft, create_as, delete_as, get_as, list_as, update_as,
};
use zipp::error::{Error, Result};

use crate::Members;
use crate::api::*;

/// Changes and uploads of a user in [`WINDOW_MS`] before they wait: a
/// member can't fill the database.
pub(crate) const MAX_WRITES: usize = 60;
const WINDOW_MS: i64 = 10 * MINUTE_MS;

/// Room for the rest of a multipart body next to the image.
const MULTIPART_SLACK: u64 = 64 * 1024;

pub(crate) fn routes(members: Arc<Members>) -> Router<AppState> {
	let mut router = Router::new()
		.api::<ListReq, _>(list)
		.api::<GetReq, _>(get)
		.api::<CreateReq, _>(create)
		.api::<UpdateReq, _>(update)
		.api::<DeleteReq, _>(delete);
	if let Some(images) = &members.images {
		let limit = images.max_bytes.saturating_add(MULTIPART_SLACK);
		let limit = DefaultBodyLimit::max(limit as usize);
		router = router.route("/members/assets", post(upload).layer(limit));
	}
	router.layer(Extension(members))
}

impl Members {
	fn check_section(&self, section: &str) -> Result<()> {
		match self.sections.iter().any(|s| s == section) {
			true => Ok(()),
			false => Err(Error::Forbidden),
		}
	}

	/// The user's entry of a section of the plugin, with its draft.
	fn own(
		&self,
		state: &AppState,
		auth: &Authed,
		id: i64,
		site: Option<String>,
	) -> Result<WithDraft> {
		let e = get_as(state, auth, id, site)?;
		self.check_section(&e.entry.section)?;
		match e.entry.author_id == Some(auth.user_id) {
			true => Ok(e),
			false => Err(Error::Forbidden),
		}
	}

	/// Whether the user may change something now; counts it.
	fn may_write(&self, user: i64) -> Result<()> {
		let now = now();
		let mut writes = self.writes.lock().unwrap_or_else(|e| e.into_inner());
		writes.retain(|_, times| {
			times.retain(|t| now - *t < WINDOW_MS);
			!times.is_empty()
		});
		let times = writes.entry(user).or_default();
		if times.len() >= MAX_WRITES {
			return Err(Error::Request(
				"Too many changes, please try again in a few minutes".into(),
			));
		}
		times.push(now);
		Ok(())
	}
}

fn member_entry(state: &AppState, e: WithDraft) -> MemberEntry {
	let WithDraft { entry, draft, .. } = e;
	let url = entry
		.uri
		.as_ref()
		.and_then(|uri| zipp::page_url(&state.site_urls, &entry.site, uri));
	let has_draft = draft.is_some();
	let (ty, title, slug, fields) = match draft {
		Some(d) => (d.ty, d.title, d.slug, d.fields),
		None => (entry.ty, entry.title, entry.slug, entry.fields),
	};
	MemberEntry {
		id: entry.id,
		section: entry.section,
		ty,
		site: entry.site,
		title,
		slug,
		uri: entry.uri,
		url,
		published: entry.published,
		post_date: entry.dates.post_date,
		fields,
		has_draft,
	}
}

async fn list(
	State(state): State<AppState>,
	Extension(members): Extension<Arc<Members>>,
	user: SiteUser,
	Req(req): Req<ListReq>,
) -> ApiResult<ListReq> {
	members.check_section(&req.section)?;
	let mut params = vec![
		("section".to_string(), req.section),
		("author".into(), user.id.to_string()),
		("order".into(), "-created".into()),
		("offset".into(), req.offset.to_string()),
	];
	params.extend(req.site.map(|s| ("site".into(), s)));
	let q = Query::parse(&state.schema, &params).map_err(Error::Request)?;

	let auth = Authed::of_site_user(&user);
	let (entries, total) = list_as(&state, &auth, q)?;
	let entries = entries
		.into_iter()
		.map(|e| member_entry(&state, e))
		.collect();
	Ok(Json(ListResp { entries, total }))
}

async fn get(
	State(state): State<AppState>,
	Extension(members): Extension<Arc<Members>>,
	user: SiteUser,
	Req(req): Req<GetReq>,
) -> ApiResult<GetReq> {
	let auth = Authed::of_site_user(&user);
	let e = members.own(&state, &auth, req.id, req.site)?;
	Ok(Json(member_entry(&state, e)))
}

async fn create(
	State(state): State<AppState>,
	Extension(members): Extension<Arc<Members>>,
	user: SiteUser,
	Req(req): Req<CreateReq>,
) -> ApiResult<CreateReq> {
	members.check_section(&req.section)?;
	members.may_write(user.id)?;
	let auth = Authed::of_site_user(&user);
	let mut input =
		EntryInput::new(&req.section, &req.ty, &req.title, req.fields);
	input.site = req.site;
	input.published = req.published;

	let entry = create_as(&state, &auth, input)?;
	let e = get_as(&state, &auth, entry.id, Some(entry.site))?;
	Ok(Json(member_entry(&state, e)))
}

async fn update(
	State(state): State<AppState>,
	Extension(members): Extension<Arc<Members>>,
	user: SiteUser,
	Req(req): Req<UpdateReq>,
) -> ApiResult<UpdateReq> {
	let auth = Authed::of_site_user(&user);
	let WithDraft { entry, draft, .. } =
		members.own(&state, &auth, req.id, req.site)?;
	members.may_write(user.id)?;

	// what the request doesn't change stays as it is, in the draft if
	// there is one
	let (ty, slug, published, dates, mut fields) = match draft {
		Some(d) => (d.ty, d.slug, d.published, d.dates, d.fields),
		None => (
			entry.ty,
			entry.slug,
			entry.published,
			entry.dates,
			entry.fields,
		),
	};
	let Value::Object(sent) = req.fields else {
		let msg = "fields: an object of the fields";
		return Err(Error::Request(msg.into()));
	};
	match &mut fields {
		Value::Object(f) => f.extend(sent),
		f => *f = Value::Object(sent),
	}
	let mut input = EntryInput::new(&entry.section, &ty, &req.title, fields);
	input.site = Some(entry.site.clone());
	input.slug = slug;
	input.published = req.published.unwrap_or(published);
	input.dates = dates;

	let saved = update_as(&state, &auth, req.id, input)?;
	let e = get_as(&state, &auth, req.id, Some(entry.site))?;
	Ok(Json(UpdateResp {
		saved,
		entry: member_entry(&state, e),
	}))
}

async fn delete(
	State(state): State<AppState>,
	Extension(members): Extension<Arc<Members>>,
	user: SiteUser,
	Req(req): Req<DeleteReq>,
) -> ApiResult<DeleteReq> {
	let auth = Authed::of_site_user(&user);
	members.own(&state, &auth, req.id, None)?;
	members.may_write(user.id)?;
	delete_as(&state, &auth, req.id)?;
	Ok(Json(()))
}

/// `POST /members/assets`, multipart: an image of the user.
async fn upload(
	State(state): State<AppState>,
	Extension(members): Extension<Arc<Members>>,
	user: SiteUser,
	mut multipart: Multipart,
) -> Result<Json<Uploaded>> {
	let Some(images) = &members.images else {
		return Err(Error::NotFound);
	};
	members.may_write(user.id)?;
	let folder = images.folder.replace("{user}", &user.id.to_string());
	let rules = UploadRules::new(images.max_bytes)
		.images_only()
		.folder(&folder);

	let auth = Authed::of_site_user(&user);
	let asset = upload_as(&state, &auth, &mut multipart, &rules).await?;
	let base = state.public_url.trim_end_matches('/');
	Ok(Json(Uploaded {
		id: asset.id,
		url: format!("{base}{}", asset.url(None)),
		width: asset.width,
		height: asset.height,
	}))
}
