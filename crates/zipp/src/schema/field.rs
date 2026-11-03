use super::*;

/// The extensions of videos browsers play, see [`Field::videos`].
pub const VIDEOS: &[&str] = &["mp4", "webm", "mov"];

/// An option of a dropdown: the value that is stored and its label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
	pub value: String,
	pub label: String,
}

pub(super) fn choices<'a>(
	options: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> Vec<Choice> {
	options
		.into_iter()
		.map(|(value, label)| Choice {
			value: value.into(),
			label: label.into(),
		})
		.collect()
}

/// What a link field can point to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum LinkKind {
	Entry,
	Url,
	Email,
	Phone,
	Asset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
	pub handle: String,
	pub label: String,
	#[serde(default)]
	pub required: bool,
	/// Indexed fields are copied to `entry_values`, so entries can be
	/// filtered and sorted by them.
	#[serde(default)]
	pub indexed: bool,
	/// Translatable fields have a value per site, the others are the same
	/// in all sites.
	#[serde(default)]
	pub translatable: bool,
	/// What an editor should know about the field, under its label in
	/// the studio, markdown; only looks like the label, no migration
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/// What a new entry or block starts with, see [`Field::default`].
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub default: Option<Value>,
	/// How much of a row the field takes in the studio, in percent: 25,
	/// 50, 75 or 100; only layout, no migration
	#[serde(default = "full_width", skip_serializing_if = "is_full_width")]
	pub width: u8,
	/// Shown in the studio only when this holds, see [`Field::when`];
	/// only layout, no migration
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub when: Option<Condition>,
	#[serde(flatten)]
	pub kind: FieldKind,
	/// The fields of a [`Field::group_of`] until the schema makes them a
	/// component of their own.
	#[serde(skip)]
	pub(crate) inline: Option<Vec<Field>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum FieldKind {
	Text {
		max: Option<u32>,
		multiline: bool,
	},
	/// A number between `min` and `max`, with at most `decimals` places
	/// (none: any); the studio shows `unit` after the input.
	Number {
		min: Option<f64>,
		max: Option<f64>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		decimals: Option<u32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		unit: Option<String>,
	},
	Boolean,
	/// An email address, `name@host.tld`.
	Email,
	/// A day, `2026-11-14`, with `time` and a time of it,
	/// `2026-11-14T20:00`: as the editor sees it, no time zone; it sorts
	/// as text.
	Date {
		time: bool,
	},
	/// Ids of entries of the given types.
	Relation {
		types: Vec<String>,
		max: Option<u32>,
		/// only entries of these sections, none: all, see
		/// [`Field::sections`]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		sections: Vec<String>,
	},
	/// Ids of assets, with `images` only images, with `extensions` only
	/// files that end so; new ones go into `folder` (a path, `Photos/
	/// Events`, made when missing), the picker opens there.
	Assets {
		images: bool,
		max: Option<u32>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		folder: Option<String>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		extensions: Vec<String>,
	},
	/// One link to an entry, a url, an email, a phone number or an asset,
	/// with an optional label. External urls and files open in a new tab
	/// unless `new_tab` is off; a link can override it.
	Link {
		to: Vec<LinkKind>,
		/// whether a link can have its own label
		labels: bool,
		#[serde(rename = "newTab")]
		new_tab: bool,
		/// only entries of these sections, none: all, see
		/// [`Field::sections`]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		sections: Vec<String>,
	},
	/// Text with formatting, stored as TipTap's JSON. Bold and italic are
	/// always there; headings of the given levels, lists, links, rows of
	/// buttons, blocks of the given types and styles only if the field
	/// says so.
	RichText {
		headings: Vec<u8>,
		lists: bool,
		links: bool,
		buttons: bool,
		blocks: Vec<String>,
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		styles: Vec<Style>,
		/// only entries of these sections, none: all, see
		/// [`Field::sections`]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		sections: Vec<String>,
	},
	/// The fields of another component (a block or an entry type), kept
	/// together under one name, like zandi's `pageIntro`. The value is an
	/// object, not a list like blocks.
	Group {
		component: String,
	},
	/// Text in Markdown, GraphQL renders it to html too.
	Markdown,
	/// Some of the options, the value is the list of their values.
	Checkboxes {
		options: Vec<Choice>,
	},
	/// Rows of the columns, each row an object of the cells' values.
	Table {
		columns: Vec<Column>,
	},
	/// A menu: links with their own label and links under them, up to
	/// `depth` levels.
	Navigation {
		to: Vec<LinkKind>,
		depth: u32,
		/// only entries of these sections, none: all, see
		/// [`Field::sections`]
		#[serde(default, skip_serializing_if = "Vec::is_empty")]
		sections: Vec<String>,
	},
	/// Texts of the frontend by key, `{ "nav.home": "Home" }`; the keys
	/// and the texts of the primary site come from the code, see
	/// [`Schema::translations`](crate::Schema::translations).
	Texts {
		keys: Vec<(String, String)>,
	},
	/// One of some options, the value is the option's value. The studio
	/// shows up to three as buttons, more as a select.
	Dropdown {
		options: Vec<Choice>,
	},
	/// A list of blocks of the given types.
	Blocks {
		types: Vec<String>,
		min: Option<u32>,
		max: Option<u32>,
	},
	/// Text for editors in the field layout, markdown from the code: a
	/// how-to or a warning. It holds no value, no migration when it
	/// changes.
	Note {
		text: String,
	},
	/// A field type of another crate, registered in the schema; the
	/// options are its own.
	Custom {
		#[serde(rename = "type")]
		ty: String,
		#[serde(default)]
		options: Value,
	},
}

/// Where a field keeps components of its own, see [`FieldKind::holds`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Holds<'a> {
	Nothing,
	/// a list of blocks, `{ "id", "type", "fields" }`
	Blocks,
	/// blocks embedded in a rich text document
	RichText,
	/// the fields of this component
	Group(&'a str),
}

impl FieldKind {
	/// Where a value of this kind keeps components: the one place that
	/// knows it, content walks, relations and migrations go by it. Every
	/// kind is named, a new one has to say.
	pub fn holds(&self) -> Holds<'_> {
		match self {
			Self::Blocks { .. } => Holds::Blocks,
			Self::RichText { .. } => Holds::RichText,
			Self::Group { component } => Holds::Group(component),
			Self::Text { .. }
			| Self::Number { .. }
			| Self::Boolean
			| Self::Email
			| Self::Relation { .. }
			| Self::Assets { .. }
			| Self::Link { .. }
			| Self::Markdown
			| Self::Date { .. }
			| Self::Checkboxes { .. }
			| Self::Table { .. }
			| Self::Navigation { .. }
			| Self::Texts { .. }
			| Self::Dropdown { .. }
			| Self::Note { .. }
			| Self::Custom { .. } => Holds::Nothing,
		}
	}

	/// Whether the field has a value in the content; a note only shows
	/// its text.
	pub fn has_value(&self) -> bool {
		!matches!(self, Self::Note { .. })
	}
}

fn full_width() -> u8 {
	100
}

fn is_full_width(width: &u8) -> bool {
	*width == 100
}

impl Field {
	pub(super) fn new(handle: &str, kind: FieldKind) -> Self {
		Self {
			handle: handle.into(),
			label: humanize(handle),
			required: false,
			indexed: false,
			translatable: false,
			instructions: None,
			default: None,
			width: 100,
			when: None,
			kind,
			inline: None,
		}
	}

	pub fn text(handle: &str) -> Self {
		Self::new(
			handle,
			FieldKind::Text {
				max: None,
				multiline: false,
			},
		)
	}

	pub fn number(handle: &str) -> Self {
		Self::new(
			handle,
			FieldKind::Number {
				min: None,
				max: None,
				decimals: None,
				unit: None,
			},
		)
	}

	pub fn boolean(handle: &str) -> Self {
		Self::new(handle, FieldKind::Boolean)
	}

	/// One email address, the server refuses anything else.
	pub fn email(handle: &str) -> Self {
		Self::new(handle, FieldKind::Email)
	}

	pub fn relation<'a>(
		handle: &str,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self::new(
			handle,
			FieldKind::Relation {
				types: types.into_iter().map(Into::into).collect(),
				max: None,
				sections: vec![],
			},
		)
	}

	/// A link to one of `to`, with a label, external ones in a new tab.
	pub fn link(handle: &str, to: impl IntoIterator<Item = LinkKind>) -> Self {
		Self::new(
			handle,
			FieldKind::Link {
				to: to.into_iter().collect(),
				labels: true,
				new_tab: true,
				sections: vec![],
			},
		)
	}

	/// A link without its own label, the frontend shows its default.
	pub fn without_label(mut self) -> Self {
		match &mut self.kind {
			FieldKind::Link { labels, .. } => *labels = false,
			_ => panic!("{}: without_label is only for links", self.handle),
		}
		self
	}

	/// External links open in the same tab too.
	pub fn same_tab(mut self) -> Self {
		match &mut self.kind {
			FieldKind::Link { new_tab, .. } => *new_tab = false,
			_ => panic!("{}: same_tab is only for links", self.handle),
		}
		self
	}

	/// Text with bold and italic, more with `headings`, `lists`, `links`,
	/// `buttons`, `embed` and `styles`.
	pub fn rich_text(handle: &str) -> Self {
		Self::new(
			handle,
			FieldKind::RichText {
				headings: vec![],
				lists: false,
				links: false,
				buttons: false,
				blocks: vec![],
				styles: vec![],
				sections: vec![],
			},
		)
	}

	fn rich(mut self, f: impl FnOnce(&mut FieldKind)) -> Self {
		if !matches!(self.kind, FieldKind::RichText { .. }) {
			panic!("{}: only rich text has this", self.handle);
		}
		f(&mut self.kind);
		self
	}

	/// Headings of these levels, like `[2, 3]`.
	pub fn headings(self, levels: impl IntoIterator<Item = u8>) -> Self {
		self.rich(|k| {
			if let FieldKind::RichText { headings, .. } = k {
				*headings = levels.into_iter().collect();
			}
		})
	}

	pub fn lists(self) -> Self {
		self.rich(|k| {
			if let FieldKind::RichText { lists, .. } = k {
				*lists = true;
			}
		})
	}

	/// Links in the text, to entries, urls, emails and phone numbers.
	pub fn links(self) -> Self {
		self.rich(|k| {
			if let FieldKind::RichText { links, .. } = k {
				*links = true;
			}
		})
	}

	/// Rows of buttons in the text.
	pub fn buttons(self) -> Self {
		self.rich(|k| {
			if let FieldKind::RichText { buttons, .. } = k {
				*buttons = true;
			}
		})
	}

	/// Blocks of these types in the text, like an image or a contact box.
	pub fn embed<'a>(self, types: impl IntoIterator<Item = &'a str>) -> Self {
		self.rich(|k| {
			if let FieldKind::RichText { blocks, .. } = k {
				*blocks = types.into_iter().map(Into::into).collect();
			}
		})
	}

	/// Styles editors can give the text, classes for the frontend's CSS:
	/// `[Style::block("lead", "Lead"), Style::inline("highlight",
	/// "Highlight")]`.
	pub fn styles(self, list: impl IntoIterator<Item = Style>) -> Self {
		self.rich(|k| {
			if let FieldKind::RichText { styles, .. } = k {
				*styles = list.into_iter().collect();
			}
		})
	}

	/// The fields of `component` under one name.
	pub fn group(handle: &str, component: &str) -> Self {
		Self::new(
			handle,
			FieldKind::Group {
				component: component.into(),
			},
		)
	}

	/// A group of fields of its own, for fields that belong together but
	/// are only used here: `Field::group_of("tickets", [Field::number(
	/// "price"), Field::boolean("soldOut")])`. The schema makes them a
	/// block `eventTickets` of the entry type `event`, its GraphQL type.
	pub fn group_of(
		handle: &str,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		let mut field = Self::group(handle, "");
		field.inline = Some(fields.into_iter().collect());
		field
	}

	pub fn markdown(handle: &str) -> Self {
		Self::new(handle, FieldKind::Markdown)
	}

	/// A day, `.time()` with a time of it.
	pub fn date(handle: &str) -> Self {
		Self::new(handle, FieldKind::Date { time: false })
	}

	/// A date field with a time too.
	pub fn time(mut self) -> Self {
		match &mut self.kind {
			FieldKind::Date { time } => *time = true,
			_ => panic!("{}: time is only for dates", self.handle),
		}
		self
	}

	pub(crate) fn texts<'a>(
		handle: &str,
		keys: impl IntoIterator<Item = (&'a str, &'a str)>,
	) -> Self {
		let keys = keys
			.into_iter()
			.map(|(k, d)| (k.to_string(), d.to_string()))
			.collect();
		Self::new(handle, FieldKind::Texts { keys }).translatable()
	}

	/// One of the options, given as value and label.
	pub fn dropdown<'a>(
		handle: &str,
		options: impl IntoIterator<Item = (&'a str, &'a str)>,
	) -> Self {
		Self::new(
			handle,
			FieldKind::Dropdown {
				options: choices(options),
			},
		)
	}

	/// Some of the options, given as value and label.
	pub fn checkboxes<'a>(
		handle: &str,
		options: impl IntoIterator<Item = (&'a str, &'a str)>,
	) -> Self {
		Self::new(
			handle,
			FieldKind::Checkboxes {
				options: choices(options),
			},
		)
	}

	/// Rows of the columns, like Craft's table field.
	pub fn table(
		handle: &str,
		columns: impl IntoIterator<Item = Column>,
	) -> Self {
		let columns = columns.into_iter().collect();
		Self::new(handle, FieldKind::Table { columns })
	}

	/// A menu of links to the given kinds, two levels deep unless
	/// [`Field::depth`] says otherwise.
	pub fn navigation(
		handle: &str,
		to: impl IntoIterator<Item = LinkKind>,
	) -> Self {
		let to = to.into_iter().collect();
		Self::new(
			handle,
			FieldKind::Navigation {
				to,
				depth: 2,
				sections: vec![],
			},
		)
	}

	/// Levels of a navigation.
	pub fn depth(mut self, levels: u32) -> Self {
		if let FieldKind::Navigation { depth, .. } = &mut self.kind {
			*depth = levels;
		}
		self
	}

	/// Text for editors between the fields, markdown: `Field::note("howto",
	/// "**Bilder** im Querformat")`. It stores nothing, the handle places
	/// it in tabs.
	pub fn note(handle: &str, text: &str) -> Self {
		Self::new(handle, FieldKind::Note { text: text.into() })
	}

	/// A field of a type of another crate, it has to be registered with
	/// [`Schema::field_type`]. Such crates usually have a builder of their
	/// own that calls this.
	pub fn custom(handle: &str, ty: &str, options: Value) -> Self {
		Self::new(
			handle,
			FieldKind::Custom {
				ty: ty.into(),
				options,
			},
		)
	}

	/// Any files.
	pub fn assets(handle: &str) -> Self {
		Self::new(
			handle,
			FieldKind::Assets {
				images: false,
				max: None,
				folder: None,
				extensions: vec![],
			},
		)
	}

	/// Only images.
	pub fn images(handle: &str) -> Self {
		Self::new(
			handle,
			FieldKind::Assets {
				images: true,
				max: None,
				folder: None,
				extensions: vec![],
			},
		)
	}

	/// Only videos, the files of [`VIDEOS`].
	pub fn videos(handle: &str) -> Self {
		Self::assets(handle).extensions(VIDEOS.iter().copied())
	}

	pub fn blocks<'a>(
		handle: &str,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self::new(
			handle,
			FieldKind::Blocks {
				types: types.into_iter().map(Into::into).collect(),
				min: None,
				max: None,
			},
		)
	}

	pub fn label(mut self, label: &str) -> Self {
		self.label = label.into();
		self
	}

	/// A hint for editors under the label, markdown:
	/// `.instructions("Shown as the **teaser** on the home page.")`
	pub fn instructions(mut self, text: &str) -> Self {
		self.instructions = Some(text.into());
		self
	}

	/// What a new entry or block starts with, in the studio and through
	/// the api: `.default(true)`, `.default("Mehr")`, a dropdown's value.
	/// Existing entries get it when a migration adds the field.
	pub fn default(mut self, value: impl Into<Value>) -> Self {
		self.default = Some(value.into());
		self
	}

	/// How much of a row the field takes in the editor, 25, 50, 75 or 100
	/// percent: `.width(50)` puts two fields next to each other. Narrow
	/// screens show every field full width.
	pub fn width(mut self, percent: u8) -> Self {
		self.width = percent;
		self
	}

	/// Shows the field only when another field of the same entry type or
	/// block has a value: `.when(Condition::on("hasPage"))`,
	/// `.when(Condition::is("genre", "concert"))`. Hidden, it keeps its
	/// value and isn't required.
	pub fn when(mut self, condition: Condition) -> Self {
		self.when = Some(condition);
		self
	}

	pub fn required(mut self) -> Self {
		self.required = true;
		self
	}

	pub fn indexed(mut self) -> Self {
		self.indexed = true;
		self
	}

	pub fn translatable(mut self) -> Self {
		self.translatable = true;
		self
	}

	pub fn multiline(mut self) -> Self {
		match &mut self.kind {
			FieldKind::Text { multiline, .. } => *multiline = true,
			_ => panic!("{}: multiline is only for text", self.handle),
		}
		self
	}

	/// Number range.
	pub fn range(mut self, from: f64, to: f64) -> Self {
		match &mut self.kind {
			FieldKind::Number { min, max, .. } => {
				*min = Some(from);
				*max = Some(to);
			}
			_ => panic!("{}: range is only for numbers", self.handle),
		}
		self
	}

	/// Most places after the point of a number, `0` for whole numbers.
	pub fn decimals(mut self, n: u32) -> Self {
		match &mut self.kind {
			FieldKind::Number { decimals, .. } => *decimals = Some(n),
			_ => panic!("{}: decimals are only for numbers", self.handle),
		}
		self
	}

	/// What a number counts, `CHF` or `s`, shown after its input.
	pub fn unit(mut self, unit: &str) -> Self {
		match &mut self.kind {
			FieldKind::Number { unit: u, .. } => *u = Some(unit.into()),
			_ => panic!("{}: unit is only for numbers", self.handle),
		}
		self
	}

	/// Least number of blocks.
	pub fn min(mut self, n: u32) -> Self {
		match &mut self.kind {
			FieldKind::Blocks { min, .. } => *min = Some(n),
			_ => panic!("{}: min is only for blocks", self.handle),
		}
		self
	}

	/// Where new files of an asset field go and its picker opens: a path
	/// of folders, `Photos/Events`, made when missing.
	pub fn folder(mut self, path: &str) -> Self {
		match &mut self.kind {
			FieldKind::Assets { folder, .. } => {
				*folder = Some(path.trim_matches('/').into())
			}
			_ => panic!("{}: folder is only for assets", self.handle),
		}
		self
	}

	/// Only files with these extensions in an asset field: `["pdf"]`.
	pub fn extensions<'a>(
		mut self,
		allowed: impl IntoIterator<Item = &'a str>,
	) -> Self {
		match &mut self.kind {
			FieldKind::Assets { extensions, .. } => {
				*extensions = allowed
					.into_iter()
					.map(|e| e.trim_start_matches('.').to_lowercase())
					.collect()
			}
			_ => panic!("{}: extensions are only for assets", self.handle),
		}
		self
	}

	/// Entries only of these sections, for a relation and the links to
	/// entries of a link, navigation or rich text: `["pages", "news"]`.
	pub fn sections<'a>(
		mut self,
		handles: impl IntoIterator<Item = &'a str>,
	) -> Self {
		match &mut self.kind {
			FieldKind::Relation { sections, .. }
			| FieldKind::Link { sections, .. }
			| FieldKind::Navigation { sections, .. }
			| FieldKind::RichText { sections, .. } => {
				*sections = handles.into_iter().map(Into::into).collect()
			}
			_ => panic!(
				"{}: sections are for relations, links, navigations and \
				rich text",
				self.handle
			),
		}
		self
	}

	/// Most characters of a text, entries of a relation or blocks.
	pub fn max(mut self, n: u32) -> Self {
		match &mut self.kind {
			FieldKind::Text { max, .. }
			| FieldKind::Relation { max, .. }
			| FieldKind::Assets { max, .. }
			| FieldKind::Blocks { max, .. } => *max = Some(n),
			_ => panic!("{}: max is not for numbers, use range", self.handle),
		}
		self
	}
}

/// A label for a handle: `eventDate` becomes `Event date`, `first_name`
/// `First name`.
pub fn humanize(handle: &str) -> String {
	let mut s = String::new();
	for (i, c) in handle.chars().enumerate() {
		match (i, c) {
			(0, c) => s.extend(c.to_uppercase()),
			(_, c) if c.is_uppercase() => {
				s.push(' ');
				s.extend(c.to_lowercase());
			}
			(_, '_') => s.push(' '),
			(_, c) => s.push(c),
		}
	}
	s
}
