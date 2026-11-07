use axum::extract::State;
use axum::{Json, Router};

use super::api::*;
use super::profile::{check_fields, check_slug, set_fields, set_slug};
use super::{
	check, check_role, create, delete, email_mail, get, list, set_role, update,
};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::links::{self, end_password_links, link_mail, new_link};
use crate::auth::site::{new_site_link, site_link_mail, site_url};
use crate::auth::{
	Admin, Authed, end_sessions, hash_password, password_matches,
	set_password_hash, studio_role,
};
use crate::content::FieldError;
use crate::error::Error;

async fn list_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(_): Req<ListReq>,
) -> ApiResult<ListReq> {
	Ok(Json(list(&state.db.conn())?))
}

async fn get_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<GetReq>,
) -> ApiResult<GetReq> {
	auth.check_self(req.id)?;
	Ok(Json(get(&state.db.conn(), req.id)?))
}

async fn create_route(
	State(state): State<AppState>,
	_admin: Admin,
	Req(req): Req<CreateReq>,
) -> ApiResult<CreateReq> {
	check_role(&state.schema, req.role.as_deref())?;
	// without a password the user is invited
	let password = Some(req.password.as_str()).filter(|p| !p.is_empty());
	let errors =
		check(&state.db.conn(), None, &req.email, &req.language, password)?;
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}
	let hash = match password {
		Some(_) => hash_password(req.password).await?,
		None => String::new(),
	};
	let conn = state.db.conn();
	let role = req.role.as_deref();
	let id = create(&conn, &req.email, &req.name, &hash, role, &req.language)?;
	Ok(Json(get(&conn, id)?))
}

async fn update_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<UpdateReq>,
) -> ApiResult<UpdateReq> {
	auth.check_self(req.id)?;
	let own = auth.user_id == req.id;
	let current = get(&state.db.conn(), req.id)?;
	if own && req.role != current.user.role {
		return Err(Error::Request("you can't change your own role".into()));
	}
	check_role(&state.schema, req.role.as_deref())?;
	// the address of a page, like an entry's slug; the one it has stays
	// (and a placeholder can follow a new name)
	let new_slug = req.slug.as_ref().filter(|s| **s != current.user.slug);
	if !auth.admin() && new_slug.is_some() {
		return Err(Error::Forbidden);
	}

	let password = req.password.as_deref().filter(|p| !p.is_empty());
	let mut errors = check(
		&state.db.conn(),
		Some(req.id),
		&req.email,
		&req.language,
		password,
	)?;
	// a new password or email of oneself needs the current password, a
	// session alone could be someone else at an open laptop (with the
	// email they'd reset the password)
	let new_email = !req.email.trim().eq_ignore_ascii_case(&current.user.email);
	if own && (password.is_some() || new_email) {
		let given = req.current_password.as_deref().unwrap_or_default();
		if !password_matches(&state.db, req.id, given).await? {
			errors.push(FieldError::new(
				"currentPassword",
				"That isn't your password",
			));
		}
	}
	let slug = match new_slug {
		Some(s) => match check_slug(&state.db.conn(), req.id, s)? {
			Ok(s) => Some(s),
			Err(e) => {
				errors.push(e);
				None
			}
		},
		None => None,
	};
	let fields = match &req.fields {
		Some(f) => {
			let conn = state.db.conn();
			let old = &current.user.fields;
			match check_fields(&conn, &state.schema, f, old, auth.user_id) {
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

	let hash = match password {
		Some(p) => Some(hash_password(p.into()).await?),
		None => None,
	};
	let account = {
		let conn = state.db.conn();
		// all of it or nothing
		let tx = conn.unchecked_transaction()?;
		update(&tx, req.id, &req.email, &req.name, &req.language)?;
		if let Some(slug) = &slug {
			set_slug(&tx, req.id, slug)?;
		}
		if let Some(fields) = &fields {
			set_fields(&tx, req.id, fields)?;
		}
		// a link sent to the old address sets no password anymore
		if new_email {
			end_password_links(&tx, req.id)?;
		}
		if !own {
			set_role(&tx, req.id, req.role.as_deref())?;
		}
		if let Some(hash) = hash {
			// the own session stays when the others end
			let keep = own.then_some(auth.token.as_str());
			set_password_hash(&tx, req.id, &hash, keep)?;
		}
		tx.commit()?;
		get(&conn, req.id)?
	};

	// the old address hears of it, a mail that fails is only logged
	if new_email && state.mail.configured() {
		let mail = email_mail(&current.user.email, &account.user.email);
		if let Err(e) = state.mail.send(mail).await {
			tracing::warn!("the mail of a new email: {e}");
		}
	}
	Ok(Json(account))
}

async fn delete_route(
	State(state): State<AppState>,
	Admin(auth): Admin,
	Req(req): Req<DeleteReq>,
) -> ApiResult<DeleteReq> {
	if req.id == auth.user_id {
		return Err(Error::Request("you can't delete yourself".into()));
	}
	delete(&state.db.conn(), req.id)?;
	Ok(Json(()))
}

async fn end_sessions_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<EndSessionsReq>,
) -> ApiResult<EndSessionsReq> {
	auth.check_self(req.id)?;
	let keep = (auth.user_id == req.id).then_some(auth.token.as_str());
	end_sessions(&state.db.conn(), req.id, keep)?;
	Ok(Json(()))
}

async fn password_link_route(
	State(state): State<AppState>,
	Admin(auth): Admin,
	Req(req): Req<PasswordLinkReq>,
) -> ApiResult<PasswordLinkReq> {
	let (kind, url, expires_at, mail) = {
		let conn = state.db.conn();
		let account = get(&conn, req.id)?;
		let kind = match account.invited {
			true => links::Kind::Invite,
			false => links::Kind::Reset,
		};
		let by = get(&conn, auth.user_id)?.user;
		let by_name = if by.name.is_empty() {
			by.email
		} else {
			by.name
		};
		let to = &account.user.email;
		let by = Some(auth.user_id);
		// a member without the studio sets it on the website
		let role = account.user.role.as_deref();
		if studio_role(&state.schema, role) {
			let (url, expires_at) =
				new_link(&conn, &state.public_url, req.id, kind, by)?;
			let mail =
				link_mail(&state.public_url, kind, to, &url, Some(&by_name));
			(kind, url, expires_at, mail)
		} else {
			let site = site_url(&state.schema, &state.site_urls, None)?;
			let (url, expires_at) =
				new_site_link(&conn, &site, req.id, kind, by)?;
			let mail = site_link_mail(&site, kind, to, &url, Some(&by_name));
			(kind, url, expires_at, mail)
		}
	};
	if req.send {
		state.mail.send(mail).await?;
	}
	Ok(Json(LinkResp {
		kind,
		url,
		expires_at,
		sent: req.send,
	}))
}

async fn login_as_route(
	State(state): State<AppState>,
	Admin(auth): Admin,
	Req(req): Req<LoginAsLinkReq>,
) -> ApiResult<LoginAsLinkReq> {
	let conn = state.db.conn();
	// an admin sees everything already
	let role = get(&conn, req.id)?.user.role;
	if role.is_none() {
		return Err(Error::Request("you can't log in as an admin".into()));
	}
	if !studio_role(&state.schema, role.as_deref()) {
		return Err(Error::Request(
			"this user has no access to the studio".into(),
		));
	}
	let kind = links::Kind::LoginAs;
	let by = Some(auth.user_id);
	let (url, expires_at) =
		new_link(&conn, &state.public_url, req.id, kind, by)?;
	Ok(Json(LinkResp {
		kind,
		url,
		expires_at,
		sent: false,
	}))
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.api::<ListReq, _>(list_route)
		.api::<GetReq, _>(get_route)
		.api::<CreateReq, _>(create_route)
		.api::<UpdateReq, _>(update_route)
		.api::<DeleteReq, _>(delete_route)
		.api::<EndSessionsReq, _>(end_sessions_route)
		.api::<PasswordLinkReq, _>(password_link_route)
		.api::<LoginAsLinkReq, _>(login_as_route)
}
