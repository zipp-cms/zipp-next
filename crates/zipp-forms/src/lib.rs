//! Forms of the website for zipp, like Craft's Formie: editors build
//! them in the studio, the website shows them and sends what visitors
//! fill in, the studio lists the submissions.
//!
//! ```ignore
//! Schema::new()
//!     // ...
//!     .plugin(Forms::new())
//!     // a page picks one of the forms
//!     .entry("page", "Page", [form_field("form")])
//! ```
//!
//! A form is an entry of the section `forms`: its fields are blocks
//! (single line text, multi-line text, email, phone) with a label, a
//! handle, required, a placeholder and instructions, in each site of
//! their own (the block list is translatable), a page break block
//! starts another page (a step), plus the labels of the buttons and the
//! message after sending. The website gets a form by its
//! id and sends to it, see [`routes`]; the studio's page Submissions
//! shows what came in, for who may view `forms`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use zipp::{Field, Plugin, PluginPage, PluginStudio, Schema, Section};

pub mod api;
#[cfg(test)]
mod api_tests;
pub mod form;
mod routes;
pub mod submissions;
#[cfg(test)]
mod tests;

/// The section of the forms.
pub const FORMS: &str = "forms";
/// The entry type of a form.
pub const FORM: &str = "form";

/// The blocks of the fields of a form, with their names.
pub const FIELD_BLOCKS: &[(&str, &str)] = &[
	("formText", "Single line text"),
	("formTextarea", "Multi-line text"),
	("formEmail", "Email"),
	("formPhone", "Phone"),
];

/// The forms of a site, see the crate.
#[derive(Debug, Clone)]
pub struct Forms {
	/// the field people don't see and bots fill, see [`Forms::honeypot`]
	pub honeypot: String,
	/// the times of submissions per address, see [`routes`]
	limits: Arc<Mutex<HashMap<std::net::IpAddr, Vec<i64>>>>,
}

impl Default for Forms {
	fn default() -> Self {
		Self {
			honeypot: "website".into(),
			limits: Default::default(),
		}
	}
}

impl Forms {
	pub fn new() -> Self {
		Self::default()
	}

	/// Another name of the honeypot field than `website`; a form with a
	/// field of the name gets a `_` after it.
	pub fn honeypot(mut self, handle: &str) -> Self {
		self.honeypot = handle.into();
		self
	}
}

/// A field that picks one of the forms, for a page or a block.
pub fn form_field(handle: &str) -> Field {
	Field::relation(handle, [FORM]).sections([FORMS]).max(1)
}

/// The fields every field block has.
fn block_fields() -> Vec<Field> {
	vec![
		Field::text("label").required(),
		Field::text("handle").instructions(
			"The name of the value for the website and in the submissions, \
			like `email`. Empty: from the label.",
		),
		Field::boolean("required"),
		Field::text("placeholder"),
		Field::text("instructions").instructions("Shown under the label."),
	]
}

impl Plugin for Forms {
	fn name(&self) -> &'static str {
		"forms"
	}

	/// The section `forms` with its entry type and the field blocks.
	fn schema(&self, schema: Schema) -> Schema {
		let mut schema = schema
			.section(Section::channel(FORMS, [FORM]).name("Forms"))
			.entry(
				FORM,
				"Form",
				[
					Field::blocks(
						"fields",
						FIELD_BLOCKS
							.iter()
							.map(|(h, _)| *h)
							.chain([form::PAGE_BREAK]),
					)
					.translatable()
					.min(1),
					Field::text("submitLabel")
						.label("Button")
						.translatable()
						.instructions("Empty: *Send*"),
					Field::text("nextLabel")
						.label("Next page")
						.translatable()
						.width(50)
						.instructions("With pages. Empty: *Next*"),
					Field::text("backLabel")
						.label("Page before")
						.translatable()
						.width(50)
						.instructions("With pages. Empty: *Back*"),
					Field::text("successMessage")
						.label("Message after sending")
						.multiline()
						.translatable()
						.instructions(
							"Empty: *Thank you, your message is sent.*",
						),
				],
			)
			.icon(FORM, "clipboard-list");
		let icons = ["type", "align-left", "mail", "phone"];
		for ((handle, name), icon) in FIELD_BLOCKS.iter().zip(icons) {
			schema = schema
				.block(handle, name, block_fields())
				.block_title(handle, "label")
				.icon(handle, icon);
		}
		let page = Field::text("label").instructions(
			"The title of the page that starts here, a step of the form.",
		);
		schema
			.block(form::PAGE_BREAK, "Page break", [page])
			.block_title(form::PAGE_BREAK, "label")
			.icon(form::PAGE_BREAK, "separator-horizontal")
	}

	fn tables(&self) -> &'static [&'static str] {
		submissions::TABLES
	}

	fn routes(&self, _schema: &Schema) -> zipp::axum::Router<zipp::AppState> {
		routes::website(Arc::new(self.clone()))
	}

	fn api_routes(
		&self,
		_schema: &Schema,
	) -> zipp::axum::Router<zipp::AppState> {
		routes::studio()
	}

	fn studio(&self) -> Option<PluginStudio> {
		let page = PluginPage::new("submissions", "Submissions")
			.icon("inbox")
			.section(FORMS);
		let js = include_str!("../assets/forms.js");
		Some(PluginStudio::new(js).page(page))
	}
}
