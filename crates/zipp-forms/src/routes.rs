//! `POST /forms/<handle>`: a submission of the website, checked and
//! mailed to the recipients of the form's settings.

use std::net::IpAddr;
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
use zipp::db::now;
use zipp::mail::Mail;

use crate::{FORM_SETTINGS, Form, FormFieldKind, Forms};

/// Submissions of an address in [`WINDOW_MS`] before it has to wait.
const MAX_PER_ADDRESS: usize = 5;
/// Submissions of a form in [`WINDOW_MS`], from all addresses: many
/// addresses can't send a flood of mails either. It also keeps the
/// limits small, only what is counted is kept.
const MAX_PER_FORM: usize = 50;
const WINDOW_MS: i64 = 1000 * 60 * 10;

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

/// The visitor gets a short reason, the log the whole one.
fn unavailable(why: &str) -> Response {
	tracing::warn!("forms: {why}");
	let message = "The form can't send right now, please try again later";
	answer(
		StatusCode::SERVICE_UNAVAILABLE,
		json!({ "success": false, "message": message }),
	)
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
		return answer(
			StatusCode::BAD_REQUEST,
			json!({ "success": false, "message": message }),
		);
	};
	let site = q
		.site
		.filter(|s| state.schema.get_site(s).is_some())
		.unwrap_or_else(|| state.schema.primary_site().handle.clone());
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
	if !forms.may_send(&handle, client) {
		let message = "Too many messages, please try again in a few minutes";
		return answer(
			StatusCode::TOO_MANY_REQUESTS,
			json!({ "success": false, "message": message }),
		);
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
	let params = [
		("section".to_string(), FORM_SETTINGS.to_string()),
		("site".to_string(), site.to_string()),
	];
	let q = zipp::query::Query::parse(&state.schema, &params)?;
	let found = zipp::entries::select(&state.db.conn(), &q)
		.map_err(|e| e.to_string())?;
	let fields = found.first().map(|e| e.fields.clone()).unwrap_or_default();
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

/// What a submission counts for.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Limit {
	Address(IpAddr),
	Form(String),
}

impl Forms {
	/// Whether an address may send the form now; counts it.
	fn may_send(&self, form: &str, client: ClientAddr) -> bool {
		let now = now();
		let mut limits = self.limits.lock().unwrap_or_else(|e| e.into_inner());
		limits.retain(|_, times| {
			times.retain(|t| now - *t < WINDOW_MS);
			!times.is_empty()
		});
		let address = Limit::Address(client.network());
		let form = Limit::Form(form.into());
		let count = |l: &Limit| limits.get(l).map_or(0, Vec::len);
		if count(&address) >= MAX_PER_ADDRESS || count(&form) >= MAX_PER_FORM {
			return false;
		}
		limits.entry(address).or_default().push(now);
		limits.entry(form).or_default().push(now);
		true
	}
}
