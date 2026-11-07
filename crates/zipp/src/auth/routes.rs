use axum::extract::State;
use axum::http::HeaderMap;
use axum::{Json, Router};

use super::api::*;
use super::links::{
	self, get_link, link_mail, link_url, login_as_session, new_link,
	reset_target, take_link,
};
use super::sessions::{end_site_session, new_site_session};
use super::site::{bearer, new_site_link, site_link_mail, site_url};
use super::{
	Session, SiteUser, end_session, hash_password, login, new_session,
	renew_session, role_of, set_password_hash, studio_role, token_of,
};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::client::ClientAddr;
use crate::content::FieldError;
use crate::error::{Error, Result};
use crate::users::profile::{check_fields, set_fields};
use crate::users::{password_error, set_name, user};

/// A user of a role without the studio, see `Role::no_studio`: an error
/// after the password was right, the login page says why.
fn check_studio(state: &AppState, user_id: i64) -> Result<()> {
	let role = role_of(&state.db.conn(), user_id)?;
	match studio_role(&state.schema, role.as_deref()) {
		true => Ok(()),
		false => Err(Error::Request(
			"This account has no access to the studio, log in on the website"
				.into(),
		)),
	}
}

async fn login_route(
	State(state): State<AppState>,
	client: ClientAddr,
	Req(req): Req<LoginReq>,
) -> ApiResult<LoginReq> {
	let user_id = login(
		&state.db,
		&state.logins,
		&client.network().to_string(),
		&req.email,
		&req.password,
	)
	.await?;
	check_studio(&state, user_id)?;
	let conn = state.db.conn();
	let session = new_session(&conn, user_id)?;
	Ok(Json(authenticated(&conn, session)?))
}

fn authenticated(
	conn: &rusqlite::Connection,
	s: Session,
) -> crate::error::Result<Authenticated> {
	Ok(Authenticated {
		user: user(conn, s.user_id)?,
		session: SessionInfo {
			impersonated_by: match s.impersonated_by {
				Some(by) => Some(user(conn, by)?.email),
				None => None,
			},
			token: s.token,
			expires_at: s.expires_at,
		},
	})
}

async fn session_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	Req(_): Req<SessionReq>,
) -> ApiResult<SessionReq> {
	let token = token_of(&headers)?.ok_or(Error::MissingSessionToken)?;
	let conn = state.db.conn();
	let session = renew_session(&conn, token, false)?;
	// a role that lost the studio is logged out of it
	let role = role_of(&conn, session.user_id)?;
	if !studio_role(&state.schema, role.as_deref()) {
		return Err(Error::InvalidSessionToken);
	}
	Ok(Json(authenticated(&conn, session)?))
}

async fn logout_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	Req(_): Req<LogoutReq>,
) -> ApiResult<LogoutReq> {
	if let Some(token) = token_of(&headers)? {
		end_session(&state.db.conn(), token)?;
	}
	Ok(Json(()))
}

async fn forgot_route(
	State(state): State<AppState>,
	Req(req): Req<ForgotReq>,
) -> ApiResult<ForgotReq> {
	if !state.mail.configured() {
		return Err(Error::Request(
			"This site sends no mails: ask an admin for a new password".into(),
		));
	}
	// before the lookup, so the answer is the same for every email
	link_url(&state.public_url, links::Kind::Reset, "")?;
	send_reset(state, &req.email, None)?;
	Ok(Json(()))
}

/// Mails a reset link if there is a user with the email: to the studio,
/// for a member of a role without it to the website (`site` picks its
/// url), like an admin's link on the Users page. Without that url no
/// mail, the answer is the same either way.
fn send_reset(state: AppState, email: &str, site: Option<&str>) -> Result<()> {
	let conn = state.db.conn();
	let Some((id, email)) = reset_target(&conn, email)? else {
		return Ok(());
	};
	let kind = links::Kind::Reset;
	let role = role_of(&conn, id)?;
	let mail = if studio_role(&state.schema, role.as_deref()) {
		if state.public_url.is_empty() {
			return Ok(());
		}
		let (url, _) = new_link(&conn, &state.public_url, id, kind, None)?;
		link_mail(&state.public_url, kind, &email, &url, None)
	} else {
		let Ok(site) = site_url(&state.schema, &state.site_urls, site) else {
			return Ok(());
		};
		let (url, _) = new_site_link(&conn, &site, id, kind, None)?;
		site_link_mail(&site, kind, &email, &url, None)
	};
	drop(conn);
	// in the background: the answer takes as long for unknown emails
	tokio::spawn(async move {
		if let Err(e) = state.mail.send(mail).await {
			tracing::warn!("password reset: {e}");
		}
	});
	Ok(())
}

async fn link_route(
	State(state): State<AppState>,
	Req(req): Req<LinkReq>,
) -> ApiResult<LinkReq> {
	let conn = state.db.conn();
	let link = get_link(&conn, &req.token)?;
	let u = user(&conn, link.user_id)?;
	Ok(Json(LinkInfo {
		kind: link.kind,
		email: u.email,
		name: u.name,
		expires_at: link.expires_at,
	}))
}

async fn set_password_route(
	State(state): State<AppState>,
	Req(req): Req<SetPasswordReq>,
) -> ApiResult<SetPasswordReq> {
	let kinds = [links::Kind::Invite, links::Kind::Reset];
	let link = get_link(&state.db.conn(), &req.token)?;
	if !kinds.contains(&link.kind) {
		return Err(links::gone());
	}
	// the website sets the password of a member, see `site_password_route`
	check_studio(&state, link.user_id)?;
	// a wrong password keeps the link
	if let Some(message) = password_error(&req.password) {
		return Err(Error::Invalid(vec![FieldError::new("password", message)]));
	}
	let hash = hash_password(req.password).await?;
	let conn = state.db.conn();
	let tx = conn.unchecked_transaction()?;
	let link = take_link(&tx, &req.token, &kinds)?;
	// logged out everywhere else
	set_password_hash(&tx, link.user_id, &hash, None)?;
	if let Some(name) = req.name.filter(|n| !n.trim().is_empty()) {
		set_name(&tx, link.user_id, &name)?;
	}
	let session = new_session(&tx, link.user_id)?;
	tx.commit()?;
	Ok(Json(authenticated(&conn, session)?))
}

async fn login_as_route(
	State(state): State<AppState>,
	Req(req): Req<LoginAsReq>,
) -> ApiResult<LoginAsReq> {
	let conn = state.db.conn();
	let link = take_link(&conn, &req.token, &[links::Kind::LoginAs])?;
	// still an admin, and the user still none
	let by = link.created_by.ok_or(Error::Forbidden)?;
	let admin = role_of(&conn, by)?.is_none();
	let role = role_of(&conn, link.user_id)?;
	if !admin || role.is_none() || !studio_role(&state.schema, role.as_deref())
	{
		return Err(Error::Forbidden);
	}
	let session = login_as_session(&conn, link.user_id, by)?;
	Ok(Json(authenticated(&conn, session)?))
}

pub(crate) fn routes() -> Router<AppState> {
	Router::new()
		.api::<LoginReq, _>(login_route)
		.api::<LogoutReq, _>(logout_route)
		.api::<SessionReq, _>(session_route)
		.api::<ForgotReq, _>(forgot_route)
		.api::<LinkReq, _>(link_route)
		.api::<SetPasswordReq, _>(set_password_route)
		.api::<LoginAsReq, _>(login_as_route)
}

fn site_authenticated(
	conn: &rusqlite::Connection,
	s: Session,
) -> Result<SiteAuthenticated> {
	Ok(SiteAuthenticated {
		user: SiteUser::get(conn, s.user_id)?,
		session: SiteSession {
			token: s.token,
			expires_at: s.expires_at,
		},
	})
}

async fn site_login_route(
	State(state): State<AppState>,
	client: ClientAddr,
	Req(req): Req<SiteLoginReq>,
) -> ApiResult<SiteLoginReq> {
	// the visitor's address: the frontend is a trusted proxy
	let user_id = login(
		&state.db,
		&state.logins,
		&client.network().to_string(),
		&req.email,
		&req.password,
	)
	.await?;
	let conn = state.db.conn();
	let session = new_site_session(&conn, user_id)?;
	Ok(Json(site_authenticated(&conn, session)?))
}

async fn site_me_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	Req(_): Req<SiteMeReq>,
) -> ApiResult<SiteMeReq> {
	let token = bearer(&headers)?.ok_or(Error::MissingSessionToken)?;
	let conn = state.db.conn();
	let session = renew_session(&conn, token, true)?;
	Ok(Json(site_authenticated(&conn, session)?))
}

async fn site_profile_route(
	State(state): State<AppState>,
	user: SiteUser,
	Req(req): Req<SiteProfileReq>,
) -> ApiResult<SiteProfileReq> {
	let mut errors = vec![];
	let name = req.name.as_deref().map(str::trim);
	if name == Some("") {
		errors.push(FieldError::new("name", "is required"));
	}
	// the fields it sends replace theirs, the others stay
	let fields = match req.fields {
		Some(sent) => {
			let mut all = user.fields.as_object().cloned().unwrap_or_default();
			all.extend(sent);
			let all = serde_json::Value::Object(all);
			let conn = state.db.conn();
			match check_fields(
				&conn,
				&state.schema,
				&all,
				&user.fields,
				user.id,
			) {
				Ok(f) => Some(f),
				Err(Error::Invalid(e)) => {
					errors.extend(e);
					None
				}
				Err(e) => return Err(e),
			}
		}
		None => None,
	};
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}

	let conn = state.db.conn();
	let tx = conn.unchecked_transaction()?;
	if let Some(name) = name {
		set_name(&tx, user.id, name)?;
	}
	if let Some(fields) = &fields {
		set_fields(&tx, user.id, fields)?;
	}
	tx.commit()?;
	Ok(Json(SiteUser::get(&conn, user.id)?))
}

async fn site_logout_route(
	State(state): State<AppState>,
	headers: HeaderMap,
	Req(_): Req<SiteLogoutReq>,
) -> ApiResult<SiteLogoutReq> {
	if let Some(token) = bearer(&headers)? {
		end_site_session(&state.db.conn(), token)?;
	}
	Ok(Json(()))
}

async fn site_reset_route(
	State(state): State<AppState>,
	Req(req): Req<SiteResetReq>,
) -> ApiResult<SiteResetReq> {
	if !state.mail.configured() {
		return Err(Error::Request(
			"This site sends no mails: ask an admin for a new password".into(),
		));
	}
	// before the lookup, so the answer is the same for every email
	site_url(&state.schema, &state.site_urls, req.site.as_deref())?;
	send_reset(state, &req.email, req.site.as_deref())?;
	Ok(Json(()))
}

async fn site_password_route(
	State(state): State<AppState>,
	Req(req): Req<SitePasswordReq>,
) -> ApiResult<SitePasswordReq> {
	let kinds = [links::Kind::Invite, links::Kind::Reset];
	let link = get_link(&state.db.conn(), &req.token)?;
	if !kinds.contains(&link.kind) {
		return Err(links::gone());
	}
	// a wrong password keeps the link
	if let Some(message) = password_error(&req.password) {
		return Err(Error::Invalid(vec![FieldError::new("password", message)]));
	}
	let hash = hash_password(req.password).await?;
	let conn = state.db.conn();
	let tx = conn.unchecked_transaction()?;
	let link = take_link(&tx, &req.token, &kinds)?;
	// logged out everywhere else
	set_password_hash(&tx, link.user_id, &hash, None)?;
	let session = new_site_session(&tx, link.user_id)?;
	tx.commit()?;
	Ok(Json(site_authenticated(&conn, session)?))
}

/// The logins of the website, on the router its frontend forwards to
/// (not under `/api`, which is the studio's).
pub(crate) fn site_routes() -> Router<AppState> {
	Router::new()
		.api::<SiteLoginReq, _>(site_login_route)
		.api::<SiteLogoutReq, _>(site_logout_route)
		.api::<SiteMeReq, _>(site_me_route)
		.api::<SiteProfileReq, _>(site_profile_route)
		.api::<SiteResetReq, _>(site_reset_route)
		.api::<SitePasswordReq, _>(site_password_route)
}
