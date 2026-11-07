//! `POST /forms/<handle>`: a submission of the website, checked and
//! mailed to the recipients of the form's settings.

use std::sync::Arc;

use serde::Deserialize;
use serde_json::{Map, Value, json};
use zipp::AppState;
use zipp::axum::Router;
use zipp::axum::body::Bytes;
use zipp::axum::extract::{Path, Query, State};
use zipp::axum::http::StatusCode;
use zipp::axum::response::{IntoResponse, Response};
use zipp::axum::routing::post;
use zipp::client::ClientAddr;
use zipp::content::valid_email;
use zipp::db::{MINUTE_MS, now};
use zipp::entries::single_fields;
use zipp::mail::Mail;

use crate::{FORM_SETTINGS, Form, FormFieldKind, Forms};

/// Submissions of an address in [`WINDOW_MS`] before it has to wait. Only
/// per address: a cap of a form from all of them let a few addresses
/// block it for everyone.
const MAX_PER_ADDRESS: usize = 5;
const WINDOW_MS: i64 = 10 * MINUTE_MS;

#[derive(Deserialize)]
struct SiteParam {
	/// the site of the page, for the message after sending
	site: Option<String>,
}

pub(crate) fn routes(forms: Arc<Forms>) -> Router<AppState> {
	Router::new().route(
		"/forms/{handle}",
		post(move |state, path, query, client, body| {
			submit(forms.clone(), state, path, query, client, body)
		}),
	)
}

fn answer(status: StatusCode, body: Value) -> Response {
	(status, zipp::axum::Json(body)).into_response()
}

fn failure(status: StatusCode, message: &str) -> Response {
	answer(status, json!({ "success": false, "message": message }))
}

/// The visitor gets a short reason, the log the whole one.
fn unavailable(why: &str) -> Response {
	tracing::warn!("forms: {why}");
	let message = "The form can't send right now, please try again later";
	failure(StatusCode::SERVICE_UNAVAILABLE, message)
}

async fn submit(
	forms: Arc<Forms>,
	State(state): State<AppState>,
	Path(handle): Path<String>,
	Query(q): Query<SiteParam>,
	client: ClientAddr,
	body: Bytes,
) -> Response {
	let Some(form) = forms.get(&handle) else {
		return StatusCode::NOT_FOUND.into_response();
	};
	let Ok(values) = serde_json::from_slice::<Map<String, Value>>(&body) else {
		let message = "a JSON object of the fields";
		return failure(StatusCode::BAD_REQUEST, message);
	};
	let site = state
		.schema
		.site_or_primary(q.site.filter(|s| state.schema.get_site(s).is_some()));
	let settings = match settings(&state, &site, &handle) {
		Ok(s) => s,
		Err(e) => return unavailable(&e),
	};
	let success = settings
		.success
		.clone()
		.unwrap_or_else(|| "Thank you, your message is sent.".into());
	let ok = answer(
		StatusCode::OK,
		json!({ "success": true, "message": success }),
	);

	// a bot filled what people don't see: it looks sent
	let filled = |v: Option<&Value>| {
		v.and_then(Value::as_str).is_some_and(|s| !s.is_empty())
	};
	if filled(values.get(&form.honeypot)) {
		return ok;
	}
	let fields = match check(form, &values) {
		Ok(f) => f,
		Err(errors) => {
			return answer(
				StatusCode::UNPROCESSABLE_ENTITY,
				json!({ "success": false, "errors": errors }),
			);
		}
	};
	if !forms.may_send(client) {
		let message = "Too many messages, please try again in a few minutes";
		return failure(StatusCode::TOO_MANY_REQUESTS, message);
	}
	if settings.to.is_empty() {
		return unavailable(&format!("{handle} has no recipient in Forms"));
	}

	let subject = settings
		.subject
		.unwrap_or_else(|| format!("New message: {}", form.label));
	let text = mail_text(form, &fields);
	let reply = form
		.fields
		.iter()
		.position(|f| f.reply_to)
		.map(|i| fields[i].as_str())
		.filter(|a| !a.is_empty());
	// one mail to all: a failing one doesn't leave the others sent
	let mut mail = Mail::new(&settings.to.join(", "), &subject, &text);
	if let Some(r) = reply {
		mail = mail.reply_to(r);
	}
	if let Err(e) = state.mail.send(mail).await {
		return unavailable(&format!("{handle}: {e}"));
	}
	ok
}

/// What editors set for a form in a site.
struct Settings {
	to: Vec<String>,
	subject: Option<String>,
	success: Option<String>,
}

fn settings(
	state: &AppState,
	site: &str,
	form: &str,
) -> Result<Settings, String> {
	let conn = state.db.conn();
	let fields = single_fields(&conn, &state.schema, FORM_SETTINGS, site)
		.map_err(|e| e.to_string())?;
	let text = |key: &str| {
		fields[format!("{form}{key}")]
			.as_str()
			.map(str::trim)
			.filter(|s| !s.is_empty())
			.map(String::from)
	};
	let to = text("To").unwrap_or_default();
	Ok(Settings {
		to: to
			.split([',', ';', '\n'])
			.map(str::trim)
			.filter(|a| !a.is_empty())
			.map(String::from)
			.collect(),
		subject: text("Subject"),
		success: text("Success"),
	})
}

/// The values of the form's fields in their order, trimmed; or what is
/// wrong, by field. Fields the form doesn't have are left out.
pub(crate) fn check(
	form: &Form,
	values: &Map<String, Value>,
) -> Result<Vec<String>, Vec<Value>> {
	let mut errors = vec![];
	let mut out = vec![];
	for f in &form.fields {
		let v = match values.get(&f.handle) {
			None | Some(Value::Null) => String::new(),
			Some(Value::String(s)) => s.trim().to_string(),
			Some(Value::Number(n)) => n.to_string(),
			Some(Value::Bool(b)) => b.to_string(),
			Some(_) => {
				errors.push(error(&f.handle, "Text"));
				continue;
			}
		};
		if v.is_empty() && f.required {
			errors.push(error(&f.handle, "Required"));
		} else if v.chars().count() > f.max {
			errors.push(error(
				&f.handle,
				&format!("At most {} characters", f.max),
			));
		} else if !v.is_empty()
			&& f.kind == FormFieldKind::Email
			&& !valid_email(&v)
		{
			errors.push(error(&f.handle, "An email address"));
		} else if f.kind != FormFieldKind::Multiline && v.contains(['\n', '\r'])
		{
			errors.push(error(&f.handle, "One line"));
		}
		out.push(v);
	}
	match errors.is_empty() {
		true => Ok(out),
		false => Err(errors),
	}
}

fn error(path: &str, message: &str) -> Value {
	json!({ "path": path, "message": message })
}

/// The lines of the mail: short fields first, the long ones after.
pub(crate) fn mail_text(form: &Form, values: &[String]) -> String {
	let pairs = || {
		form.fields
			.iter()
			.zip(values)
			.filter(|(_, v)| !v.is_empty())
	};
	let mut text = String::new();
	for (f, v) in pairs().filter(|(f, _)| f.kind != FormFieldKind::Multiline) {
		text += &format!("{}: {v}\n", f.label);
	}
	for (f, v) in pairs().filter(|(f, _)| f.kind == FormFieldKind::Multiline) {
		text += &format!("\n{}:\n{v}\n", f.label);
	}
	text
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
