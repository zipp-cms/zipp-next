//! A form as the website gets it, made of its entry in a site, and the
//! check of what a visitor sends.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use zipp::content::valid_email;
use zipp::entries::Entry;

/// What the website needs to show a form, `GET /forms/{id}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
	pub id: i64,
	pub title: String,
	/// one at least, a form without pages is one without a name
	pub pages: Vec<FormPage>,
	pub submit_label: String,
	/// the buttons between the pages
	pub next_label: String,
	pub back_label: String,
	pub success_message: String,
	/// the field people don't see and bots fill: send it empty
	pub honeypot: String,
}

/// A step of a form: the fields of the items of one page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormPage {
	/// its title, empty for a form without pages
	pub label: String,
	pub fields: Vec<FormField>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormField {
	#[serde(rename = "type")]
	pub kind: FieldKind,
	/// the key of its value, unique in the form
	pub handle: String,
	pub label: String,
	pub required: bool,
	pub placeholder: String,
	pub instructions: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FieldKind {
	Text,
	Textarea,
	Email,
	Phone,
}

impl FieldKind {
	fn of_block(ty: &str) -> Option<Self> {
		Some(match ty {
			"formText" => Self::Text,
			"formTextarea" => Self::Textarea,
			"formEmail" => Self::Email,
			"formPhone" => Self::Phone,
			_ => return None,
		})
	}

	/// The most characters of a value.
	fn max(self) -> usize {
		match self {
			Self::Textarea => 10_000,
			Self::Phone => 50,
			Self::Text | Self::Email => 500,
		}
	}
}

/// A value of a submission, with the label its field had then.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Answer {
	pub handle: String,
	pub label: String,
	pub value: String,
}

/// What is wrong with a value: `code` for the website's own text
/// (`required`, `oneLine`, `tooLong`, `email`, `phone`, `text`), the
/// message in English.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldError {
	pub path: String,
	pub code: String,
	pub message: String,
}

impl Form {
	/// The form of an entry of `forms` in its site, its fields on their
	/// pages (see [`FormFields`](crate::FormFields)); switched off blocks
	/// are left out, a handle that is empty or already there is made of
	/// the label.
	pub fn of_entry(entry: &Entry, honeypot: &str) -> Self {
		let text = |v: &Value, key: &str| {
			v[key].as_str().unwrap_or_default().trim().to_string()
		};
		let mut pages: Vec<FormPage> = vec![];
		let items = entry.fields["fields"].as_array();
		for item in items.into_iter().flatten() {
			let block = &item["block"];
			if block["enabled"] == Value::Bool(false) {
				continue;
			}
			let Some(kind) =
				block["type"].as_str().and_then(FieldKind::of_block)
			else {
				continue;
			};
			let f = &block["fields"];
			let label = text(f, "label");
			let mut handle = match text(f, "handle") {
				h if h.is_empty() => handle_of(&label),
				h => h,
			};
			let base = handle.clone();
			let mut n = 1;
			let taken = |h: &str| {
				pages.iter().flat_map(|p| &p.fields).any(|x| x.handle == h)
			};
			while handle.is_empty() || taken(&handle) {
				n += 1;
				handle = format!("{base}{n}");
			}
			let field = FormField {
				kind,
				handle,
				label,
				required: f["required"] == Value::Bool(true),
				placeholder: text(f, "placeholder"),
				instructions: text(f, "instructions"),
			};
			// the items of a page are one after the other
			let page = text(item, "page");
			match pages.last_mut() {
				Some(p) if p.label == page => p.fields.push(field),
				_ => pages.push(FormPage {
					label: page,
					fields: vec![field],
				}),
			}
		}
		if pages.is_empty() {
			pages.push(FormPage {
				label: String::new(),
				fields: vec![],
			});
		}

		let mut honeypot = honeypot.to_string();
		let handles = || pages.iter().flat_map(|p| &p.fields);
		while handles().any(|f| f.handle == honeypot) {
			honeypot.push('_');
		}
		let or = |key: &str, default: &str| match text(&entry.fields, key) {
			t if t.is_empty() => default.to_string(),
			t => t,
		};
		Self {
			id: entry.id,
			title: entry.title.clone(),
			submit_label: or("submitLabel", "Send"),
			next_label: or("nextLabel", "Next"),
			back_label: or("backLabel", "Back"),
			success_message: or(
				"successMessage",
				"Thank you, your message is sent.",
			),
			pages,
			honeypot,
		}
	}

	/// Whether a bot filled the honeypot.
	pub fn is_spam(&self, values: &Map<String, Value>) -> bool {
		values
			.get(&self.honeypot)
			.and_then(Value::as_str)
			.is_some_and(|s| !s.is_empty())
	}

	/// The values of the fields of a page (`Some(i)`) or of the whole
	/// form in their order, trimmed, or what is wrong with them. Keys the
	/// form doesn't have are left out.
	pub fn check(
		&self,
		values: &Map<String, Value>,
		page: Option<usize>,
	) -> Result<Vec<Answer>, Vec<FieldError>> {
		let pages = match page {
			Some(i) => &self.pages[i..=i],
			None => &self.pages[..],
		};
		let mut errors = vec![];
		let mut out = vec![];
		for f in pages.iter().flat_map(|p| &p.fields) {
			let error = |code: &str, message: &str| FieldError {
				path: f.handle.clone(),
				code: code.into(),
				message: message.into(),
			};
			let v = match values.get(&f.handle) {
				None | Some(Value::Null) => String::new(),
				Some(Value::String(s)) => s.trim().to_string(),
				Some(_) => {
					errors.push(error("text", "Text"));
					continue;
				}
			};
			let max = f.kind.max();
			if v.is_empty() && f.required {
				errors.push(error("required", "Required"));
			} else if v.chars().count() > max {
				let message = format!("At most {max} characters");
				errors.push(error("tooLong", &message));
			} else if f.kind != FieldKind::Textarea && v.contains(['\n', '\r'])
			{
				errors.push(error("oneLine", "One line"));
			} else if !v.is_empty()
				&& f.kind == FieldKind::Email
				&& !valid_email(&v)
			{
				errors.push(error("email", "An email address"));
			} else if !v.is_empty() && f.kind == FieldKind::Phone && !phone(&v)
			{
				errors.push(error("phone", "A phone number"));
			}
			out.push(Answer {
				handle: f.handle.clone(),
				label: f.label.clone(),
				value: v,
			});
		}
		match errors.is_empty() {
			true => Ok(out),
			false => Err(errors),
		}
	}
}

/// A phone number as people write it: digits with `+ ( ) - . /` and
/// spaces, at least a few digits.
fn phone(v: &str) -> bool {
	let digits = v.chars().filter(char::is_ascii_digit).count();
	digits >= 3
		&& v.chars()
			.all(|c| c.is_ascii_digit() || " +()-./".contains(c))
}

/// A handle of a label: its words in camelCase, `E-Mail Adresse` is
/// `eMailAdresse`.
pub(crate) fn handle_of(label: &str) -> String {
	let mut handle = String::new();
	for (i, word) in label
		.split(|c: char| !c.is_alphanumeric())
		.filter(|w| !w.is_empty())
		.enumerate()
	{
		let mut chars = word.chars();
		let first = chars.next().unwrap_or_default();
		match i {
			0 => handle.extend(first.to_lowercase()),
			_ => handle.extend(first.to_uppercase()),
		}
		handle.extend(chars.flat_map(char::to_lowercase));
	}
	handle
}
