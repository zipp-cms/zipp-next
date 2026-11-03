//! The columns of a table field and the styles of a rich text.

use serde::{Deserialize, Serialize};

use super::field::choices;
use super::{Choice, Field, FieldKind, LinkKind, humanize};

/// A column of a table field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Column {
	pub handle: String,
	pub label: String,
	#[serde(flatten)]
	pub kind: ColumnKind,
}

/// What a cell of a column holds; like the field of the same name, kept
/// small enough for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum ColumnKind {
	Text,
	/// a url of the web or a path of the site
	Url,
	Email,
	/// a number with at most `decimals` places, `unit` after its input
	Number {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		decimals: Option<u32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		unit: Option<String>,
	},
	/// yes or no, no is stored as nothing
	Boolean,
	/// a day, `2026-11-14`
	Date,
	/// one of the options, the value is the option's value
	Dropdown {
		options: Vec<Choice>,
	},
	/// a link to one of `to`, without a label of its own
	Link {
		to: Vec<LinkKind>,
	},
}

impl Column {
	fn new(handle: &str, kind: ColumnKind) -> Self {
		Self {
			handle: handle.into(),
			label: humanize(handle),
			kind,
		}
	}

	pub fn text(handle: &str) -> Self {
		Self::new(handle, ColumnKind::Text)
	}

	pub fn url(handle: &str) -> Self {
		Self::new(handle, ColumnKind::Url)
	}

	pub fn email(handle: &str) -> Self {
		Self::new(handle, ColumnKind::Email)
	}

	/// A number, `.decimals(2)` and `.unit("CHF")` like a number field.
	pub fn number(handle: &str) -> Self {
		Self::new(
			handle,
			ColumnKind::Number {
				decimals: None,
				unit: None,
			},
		)
	}

	/// Yes or no, a switch in the studio.
	pub fn boolean(handle: &str) -> Self {
		Self::new(handle, ColumnKind::Boolean)
	}

	/// A day, `2026-11-14`.
	pub fn date(handle: &str) -> Self {
		Self::new(handle, ColumnKind::Date)
	}

	/// One of the options, given as value and label.
	pub fn dropdown<'a>(
		handle: &str,
		options: impl IntoIterator<Item = (&'a str, &'a str)>,
	) -> Self {
		let options = choices(options);
		Self::new(handle, ColumnKind::Dropdown { options })
	}

	/// A link to one of `to`; the other columns say what it is.
	pub fn link(handle: &str, to: impl IntoIterator<Item = LinkKind>) -> Self {
		let to = to.into_iter().collect();
		Self::new(handle, ColumnKind::Link { to })
	}

	pub fn label(mut self, label: &str) -> Self {
		self.label = label.into();
		self
	}

	/// Most places after the point of a number, `0` for whole numbers.
	pub fn decimals(mut self, n: u32) -> Self {
		match &mut self.kind {
			ColumnKind::Number { decimals, .. } => *decimals = Some(n),
			_ => panic!("{}: decimals are only for numbers", self.handle),
		}
		self
	}

	/// What a number counts, `CHF`, shown after its input.
	pub fn unit(mut self, unit: &str) -> Self {
		match &mut self.kind {
			ColumnKind::Number { unit: u, .. } => *u = Some(unit.into()),
			_ => panic!("{}: unit is only for numbers", self.handle),
		}
		self
	}

	/// The field a cell is checked as, a url is a text with a check of
	/// its own.
	pub(crate) fn as_field(&self) -> Field {
		let kind = match &self.kind {
			ColumnKind::Text | ColumnKind::Url => FieldKind::Text {
				max: None,
				multiline: false,
			},
			ColumnKind::Email => FieldKind::Email,
			ColumnKind::Number { decimals, unit } => FieldKind::Number {
				min: None,
				max: None,
				decimals: *decimals,
				unit: unit.clone(),
			},
			ColumnKind::Boolean => FieldKind::Boolean,
			ColumnKind::Date => FieldKind::Date { time: false },
			ColumnKind::Dropdown { options } => FieldKind::Dropdown {
				options: options.clone(),
			},
			ColumnKind::Link { to } => FieldKind::Link {
				to: to.clone(),
				labels: false,
				new_tab: true,
				sections: vec![],
			},
		};
		Field::new(&self.handle, kind)
	}
}

/// A style of a rich text: a class on a paragraph or heading, or on a
/// part of the text (`inline`); the frontend's CSS gives it its look.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Style {
	pub class: String,
	pub label: String,
	pub inline: bool,
}

impl Style {
	/// A style of a whole paragraph or heading, like a lead.
	pub fn block(class: &str, label: &str) -> Self {
		Self {
			class: class.into(),
			label: label.into(),
			inline: false,
		}
	}

	/// A style of a part of the text, like a highlight.
	pub fn inline(class: &str, label: &str) -> Self {
		Self {
			class: class.into(),
			label: label.into(),
			inline: true,
		}
	}
}
