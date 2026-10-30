use axum::extract::State;
use axum::{Json, Router};

use super::api::*;
use super::{check, check_role, create, delete, get, list, set_role, update};
use crate::AppState;
use crate::api::{ApiResult, ApiRouter, Req};
use crate::auth::links::{LinkKind, create_link, link_mail, link_url};
use crate::auth::{
	Admin, Authed, end_sessions, hash_password, password_matches,
	set_password_hash,
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
	let errors = check(&state.db.conn(), None, &req.email, password)?;
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}
	let hash = match password {
		Some(_) => hash_password(req.password).await?,
		None => String::new(),
	};
	let conn = state.db.conn();
	let id = create(&conn, &req.email, &req.name, &hash, req.role.as_deref())?;
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

	let password = req.password.as_deref().filter(|p| !p.is_empty());
	let mut errors =
		check(&state.db.conn(), Some(req.id), &req.email, password)?;
	// a new password of oneself needs the current one, a session alone
	// could be someone else at an open laptop
	if own && password.is_some() {
		let given = req.current_password.as_deref().unwrap_or_default();
		if !password_matches(&state.db, req.id, given).await? {
			errors.push(FieldError {
				path: "currentPassword".into(),
				message: "That isn't your password".into(),
			});
		}
	}
	if !errors.is_empty() {
		return Err(Error::Invalid(errors));
	}

	let hash = match password {
		Some(p) => Some(hash_password(p.into()).await?),
		None => None,
	};
	let conn = state.db.conn();
	// all of it or nothing
	let tx = conn.unchecked_transaction()?;
	update(&tx, req.id, &req.email, &req.name)?;
	if !own {
		set_role(&tx, req.id, req.role.as_deref())?;
	}
	if let Some(hash) = hash {
		// the own session stays when the others end
		let keep = own.then_some(auth.token.as_str());
		set_password_hash(&tx, req.id, &hash, keep)?;
	}
	tx.commit()?;
	Ok(Json(get(&conn, req.id)?))
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
	let (kind, token, expires_at, to, by) = {
		let conn = state.db.conn();
		let account = get(&conn, req.id)?;
		let kind = match account.invited {
			true => LinkKind::Invite,
			false => LinkKind::Reset,
		};
		// without the url of the cms there is no link to make
		link_url(&state.public_url, kind, "")?;
		let (token, expires_at) =
			create_link(&conn, req.id, kind, Some(auth.user_id))?;
		let by = get(&conn, auth.user_id)?.user;
		let by = if by.name.is_empty() {
			by.email
		} else {
			by.name
		};
		(kind, token, expires_at, account.user.email, by)
	};
	let url = link_url(&state.public_url, kind, &token)?;
	if req.send {
		let mail = link_mail(&state.public_url, kind, &to, &url, Some(&by));
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
	if get(&conn, req.id)?.user.role.is_none() {
		return Err(Error::Request("you can't log in as an admin".into()));
	}
	let kind = LinkKind::LoginAs;
	link_url(&state.public_url, kind, "")?;
	let (token, expires_at) =
		create_link(&conn, req.id, kind, Some(auth.user_id))?;
	Ok(Json(LinkResp {
		kind,
		url: link_url(&state.public_url, kind, &token)?,
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
