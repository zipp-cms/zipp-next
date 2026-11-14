//! The routes of the website, which the frontend proxies (with the
//! visitor's address in `X-Forwarded-For`, see docs/forms.md):
//!
//! - `GET /forms/{id}?site=de`: the [`Form`] as it is live in the site,
//!   NOT_FOUND if it isn't
//! - `POST /forms/{id}?site=de`, JSON of the values by handle: stored,
//!   `{ "success": true, "message": "…" }`; invalid values `422 {
//!   "success": false, "errors": [{ "path", "code", "message" }] }`, too
//!   many from an address `429`. A filled honeypot looks sent and is
//!   dropped.
//! - `POST /forms/{id}?site=de&page=0`: checks only the values of that
//!   page of a form with several, before the visitor goes on: `{
//!   "success": true }` or the errors as above, nothing is stored or
//!   counted.
//!
//! And the studio's, see [`api`](crate::api).

use std::sync::Arc;

use serde::Deserialize;
use serde_json::{Map, Value, json};
use zipp::api::{ApiResult, ApiRouter, Req};
use zipp::auth::Authed;
use zipp::axum::body::Bytes;
use zipp::axum::extract::{Path, Query, State};
use zipp::axum::http::StatusCode;
use zipp::axum::response::{IntoResponse, Response};
use zipp::axum::routing::get;
use zipp::axum::{Json, Router};
use zipp::client::ClientAddr;
use zipp::db::{MINUTE_MS, now};
use zipp::entries::list;
use zipp::entries::query::Query as EntryQuery;
use zipp::error::{Error, Result};
use zipp::{Access, AppState};

use crate::api::*;
use crate::form::Form;
use crate::{FORM, FORMS, Forms, submissions};

/// Submissions of an address in [`WINDOW_MS`] before it has to wait. Only
/// per address: a cap of a form from all of them let a few addresses
/// block it for everyone.
const MAX_PER_ADDRESS: usize = 5;
const WINDOW_MS: i64 = 10 * MINUTE_MS;

#[derive(Deserialize)]
struct SiteParam {
	/// default the primary site
	site: Option<String>,
	/// only check this page, see the module
	page: Option<usize>,
}

pub(crate) fn website(forms: Arc<Forms>) -> Router<AppState> {
	let get_forms = forms.clone();
	Router::new().route(
		"/forms/{id}",
		get(move |state, path, query| {
			form(get_forms.clone(), state, path, query)
		})
		.post(move |state, path, query, client, body| {
			submit(forms.clone(), state, path, query, client, body)
		}),
	)
}

pub(crate) fn studio() -> Router<AppState> {
	Router::new()
		.api::<ListReq, _>(list_route)
		.api::<DeleteReq, _>(delete_route)
}

/// The form `id` as it is live in the site.
fn live_form(
	forms: &Forms,
	state: &AppState,
	id: i64,
	site: Option<String>,
) -> Result<(Form, String)> {
	let site = site
		.filter(|s| state.schema.get_site(s).is_some())
		.unwrap_or_else(|| state.schema.primary_site().handle.clone());
	let id = id.to_string();
	let params = [
		("section", FORMS),
		("type", FORM),
		("site", site.as_str()),
		("ids", id.as_str()),
	];
	let q = EntryQuery::published(&state.schema, &params)
		.map_err(Error::Request)?;
	let (found, _) = list(&state.db.conn(), &q)?;
	let entry = found.into_iter().next().ok_or(Error::NotFound)?;
	Ok((Form::of_entry(&entry, &forms.honeypot), site))
}

async fn form(
	forms: Arc<Forms>,
	State(state): State<AppState>,
	Path(id): Path<i64>,
	Query(q): Query<SiteParam>,
) -> Result<Json<Form>> {
	let (form, _) = live_form(&forms, &state, id, q.site)?;
	Ok(Json(form))
}

fn answer(status: StatusCode, body: Value) -> Response {
	(status, Json(body)).into_response()
}

fn failure(status: StatusCode, message: &str) -> Response {
	answer(status, json!({ "success": false, "message": message }))
}

async fn submit(
	forms: Arc<Forms>,
	State(state): State<AppState>,
	Path(id): Path<i64>,
	Query(q): Query<SiteParam>,
	client: ClientAddr,
	body: Bytes,
) -> Response {
	let (form, site) = match live_form(&forms, &state, id, q.site) {
		Ok(f) => f,
		Err(e) => return e.into_response(),
	};
	let Ok(values) = serde_json::from_slice::<Map<String, Value>>(&body) else {
		let message = "a JSON object of the fields";
		return failure(StatusCode::BAD_REQUEST, message);
	};
	let invalid = |errors| {
		answer(
			StatusCode::UNPROCESSABLE_ENTITY,
			json!({ "success": false, "errors": errors }),
		)
	};

	// a step: only its values, the visitor goes on
	if let Some(page) = q.page {
		if page >= form.pages.len() {
			return failure(StatusCode::BAD_REQUEST, "no such page");
		}
		return match form.check(&values, Some(page)) {
			Ok(_) => answer(StatusCode::OK, json!({ "success": true })),
			Err(errors) => invalid(errors),
		};
	}

	let ok = answer(
		StatusCode::OK,
		json!({ "success": true, "message": form.success_message }),
	);
	// a bot filled what people don't see: it looks sent
	if form.is_spam(&values) {
		return ok;
	}
	let answers = match form.check(&values, None) {
		Ok(a) => a,
		Err(errors) => return invalid(errors),
	};
	if !forms.may_send(client) {
		let message = "Too many messages, please try again in a few minutes";
		return failure(StatusCode::TOO_MANY_REQUESTS, message);
	}

	let conn = state.db.conn();
	match submissions::insert(&conn, form.id, &site, &answers) {
		Ok(_) => ok,
		Err(e) => e.into_response(),
	}
}

impl Forms {
	/// Whether an address may send a form now; counts it.
	fn may_send(&self, client: ClientAddr) -> bool {
		let now = now();
		let mut limits = self.limits.lock().unwrap_or_else(|e| e.into_inner());
		limits.retain(|_, times| {
			times.retain(|t| now - *t < WINDOW_MS);
			!times.is_empty()
		});
		let times = limits.entry(client.network()).or_default();
		if times.len() >= MAX_PER_ADDRESS {
			return false;
		}
		times.push(now);
		true
	}
}

async fn list_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<ListReq>,
) -> ApiResult<ListReq> {
	auth.check(&state.schema, FORMS, Access::View)?;
	let site = state.schema.site_or_primary(req.site);
	let (submissions, total) =
		submissions::list(&state.db.conn(), &site, req.form, req.offset)?;
	Ok(Json(ListResp { submissions, total }))
}

async fn delete_route(
	State(state): State<AppState>,
	auth: Authed,
	Req(req): Req<DeleteReq>,
) -> ApiResult<DeleteReq> {
	auth.check(&state.schema, FORMS, Access::Delete)?;
	submissions::delete(&state.db.conn(), req.id)?;
	Ok(Json(()))
}
