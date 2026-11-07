//! Forms of the website, like Craft's Contact Form, as a plugin: the
//! fields are code, a submission is checked and mailed, nothing is
//! stored.
//!
//! ```ignore
//! Schema::new()
//!     // ...
//!     .plugin(Forms::new().form(
//!         Form::new("contact", "Contact")
//!             .field(FormField::text("name").required())
//!             .field(FormField::email("email").required().reply_to())
//!             .field(FormField::multiline("message").required()),
//!     ))
//! ```
//!
//! The website posts JSON to `POST /forms/<handle>?site=<site>` (the
//! crelte server proxies it): `{ "name": "Anna", ... }`. A filled
//! honeypot field (`website`, invisible to people) is answered like a
//! success and dropped; an address may send a few per ten minutes. The
//! answer is `{ "success": true, "message": "…" }`, with invalid fields
//! `422 { "success": false, "errors": [{ "path", "message" }] }`.
//!
//! Editors set the recipients, the subject and the message after sending
//! per form in the single `formSettings` ("Forms"): `contactTo` in all
//! sites, `contactSubject` and `contactSuccess` per site.

use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use zipp::schema::humanize;
use zipp::{Field, Plugin, Schema, Section};

mod routes;
#[cfg(test)]
mod tests;

/// The single with the settings of every form.
pub const FORM_SETTINGS: &str = "formSettings";

/// The fields of a form in [`FORM_SETTINGS`]: `contactTo` (all sites),
/// `contactSubject` and `contactSuccess` (per site).
fn settings_fields(form: &Form) -> Vec<Field> {
	let h = &form.handle;
	vec![
		Field::text(&format!("{h}To"))
			.label(&format!("{}: send to", form.label))
			.instructions(
				"One or more addresses, separated by commas. Without one the form can't send.",
			),
		Field::text(&format!("{h}Subject"))
			.translatable()
			.label(&format!("{}: subject", form.label))
			.instructions(&format!("Empty: *New message: {}*", form.label)),
		Field::text(&format!("{h}Success"))
			.multiline()
			.translatable()
			.label(&format!("{}: message after sending", form.label))
			.instructions("Empty: *Thank you, your message is sent.*"),
	]
}

/// The forms of a site, see the crate.
#[derive(Debug, Clone, Default)]
pub struct Forms {
	pub forms: Vec<Form>,
	/// the times of submissions per address, see [`routes`]
	limits: Arc<Mutex<HashMap<std::net::IpAddr, Vec<i64>>>>,
}

/// A form: its fields in the order of the mail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Form {
	pub handle: String,
	pub label: String,
	pub fields: Vec<FormField>,
	/// a field people leave empty, bots fill it
	pub honeypot: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FormField {
	pub handle: String,
	pub label: String,
	pub kind: FormFieldKind,
	pub required: bool,
	/// answers to the mail go to this address
	pub reply_to: bool,
	/// characters
	pub max: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum FormFieldKind {
	Text,
	/// several lines, at the end of the mail
	Multiline,
	Email,
}

impl Forms {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn form(mut self, form: Form) -> Self {
		self.forms.push(form);
		self
	}

	pub fn get(&self, handle: &str) -> Option<&Form> {
		self.forms.iter().find(|f| f.handle == handle)
	}
}

impl Form {
	pub fn new(handle: &str, label: &str) -> Self {
		Self {
			handle: handle.into(),
			label: label.into(),
			fields: vec![],
			honeypot: "website".into(),
		}
	}

	pub fn field(mut self, field: FormField) -> Self {
		self.fields.push(field);
		self
	}

	/// Another name of the honeypot field than `website`.
	pub fn honeypot(mut self, handle: &str) -> Self {
		self.honeypot = handle.into();
		self
	}
}

impl FormField {
	fn new(handle: &str, kind: FormFieldKind, max: usize) -> Self {
		Self {
			handle: handle.into(),
			label: humanize(handle),
			kind,
			required: false,
			reply_to: false,
			max,
		}
	}

	pub fn text(handle: &str) -> Self {
		Self::new(handle, FormFieldKind::Text, 200)
	}

	pub fn multiline(handle: &str) -> Self {
		Self::new(handle, FormFieldKind::Multiline, 5000)
	}

	pub fn email(handle: &str) -> Self {
		Self::new(handle, FormFieldKind::Email, 200)
	}

	/// Its name in the mail, else from the handle.
	pub fn label(mut self, label: &str) -> Self {
		self.label = label.into();
		self
	}

	pub fn required(mut self) -> Self {
		self.required = true;
		self
	}

	/// Of an email field: answers to the mail go to it.
	pub fn reply_to(mut self) -> Self {
		self.reply_to = true;
		self
	}

	pub fn max(mut self, chars: usize) -> Self {
		self.max = chars;
		self
	}
}

impl Plugin for Forms {
	fn name(&self) -> &'static str {
		"forms"
	}

	/// The single `formSettings`: the mail settings of each form, a tab
	/// each when there are several.
	fn schema(&self, schema: Schema) -> Schema {
		let schema = schema.section(
			Section::single(FORM_SETTINGS, FORM_SETTINGS).name("Forms"),
		);
		match self.forms.as_slice() {
			[form] => {
				schema.entry(FORM_SETTINGS, "Forms", settings_fields(form))
			}
			forms => schema.entry_tabs(
				FORM_SETTINGS,
				"Forms",
				forms
					.iter()
					.map(|f| (f.label.as_str(), settings_fields(f)))
					.collect::<Vec<_>>(),
			),
		}
	}

	fn check(&self, _schema: &Schema) -> Result<(), String> {
		let mut handles = BTreeSet::new();
		for form in &self.forms {
			let h = &form.handle;
			if !handles.insert(h) {
				return Err(format!("forms: the form {h} is there twice"));
			}
			if form.fields.is_empty() {
				return Err(format!("forms: the form {h} has no fields"));
			}
			let mut fields = BTreeSet::new();
			for f in &form.fields {
				if !fields.insert(&f.handle) || f.handle == form.honeypot {
					return Err(format!(
						"forms: {h} has the field {} twice (or as the honeypot)",
						f.handle
					));
				}
				if f.reply_to && f.kind != FormFieldKind::Email {
					return Err(format!(
						"forms: {h}.{} replies to an address, it needs to be \
						an email field",
						f.handle
					));
				}
			}
			if form.fields.iter().filter(|f| f.reply_to).count() > 1 {
				return Err(format!("forms: {h} replies to two fields"));
			}
		}
		Ok(())
	}

	fn routes(&self, _schema: &Schema) -> zipp::axum::Router<zipp::AppState> {
		routes::routes(Arc::new(self.clone()))
	}
}
