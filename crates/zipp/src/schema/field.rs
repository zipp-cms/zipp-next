use super::*;

/// An option of a dropdown: the value that is stored and its label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
	pub value: String,
	pub label: String,
}

fn choices<'a>(
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

/// A column of a table field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Column {
	pub handle: String,
	pub label: String,
	pub kind: ColumnKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ColumnKind {
	Text,
	/// a url of the web or a path of the site
	Url,
	Email,
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

	pub fn label(mut self, label: &str) -> Self {
		self.label = label.into();
		self
	}
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
	#[serde(flatten)]
	pub kind: FieldKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
pub enum FieldKind {
	Text {
		max: Option<u32>,
		multiline: bool,
	},
	Number {
		min: Option<f64>,
		max: Option<f64>,
	},
	Boolean,
	/// Ids of entries of the given types.
	Relation {
		types: Vec<String>,
		max: Option<u32>,
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
	},
	/// Text with formatting, stored as TipTap's JSON. Bold and italic are
	/// always there; headings of the given levels, lists, links, rows of
	/// buttons and blocks of the given types only if the field says so.
	RichText {
		headings: Vec<u8>,
		lists: bool,
		links: bool,
		buttons: bool,
		blocks: Vec<String>,
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
	/// Rows of the columns, each row an object of texts.
	Table {
		columns: Vec<Column>,
	},
	/// A menu: links with their own label and links under them, up to
	/// `depth` levels.
	Navigation {
		to: Vec<LinkKind>,
		depth: u32,
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
			| Self::Relation { .. }
			| Self::Assets { .. }
			| Self::Link { .. }
			| Self::Markdown
			| Self::Checkboxes { .. }
			| Self::Table { .. }
			| Self::Navigation { .. }
			| Self::Texts { .. }
			| Self::Dropdown { .. }
			| Self::Custom { .. } => Holds::Nothing,
		}
	}
}

impl Field {
	fn new(handle: &str, kind: FieldKind) -> Self {
		Self {
			handle: handle.into(),
			label: humanize(handle),
			required: false,
			indexed: false,
			translatable: false,
			kind,
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
			},
		)
	}

	pub fn boolean(handle: &str) -> Self {
		Self::new(handle, FieldKind::Boolean)
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
	/// `buttons` and `embed`.
	pub fn rich_text(handle: &str) -> Self {
		Self::new(
			handle,
			FieldKind::RichText {
				headings: vec![],
				lists: false,
				links: false,
				buttons: false,
				blocks: vec![],
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

	/// The fields of `component` under one name.
	pub fn group(handle: &str, component: &str) -> Self {
		Self::new(
			handle,
			FieldKind::Group {
				component: component.into(),
			},
		)
	}

	pub fn markdown(handle: &str) -> Self {
		Self::new(handle, FieldKind::Markdown)
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
		Self::new(handle, FieldKind::Navigation { to, depth: 2 })
	}

	/// Levels of a navigation.
	pub fn depth(mut self, levels: u32) -> Self {
		if let FieldKind::Navigation { depth, .. } = &mut self.kind {
			*depth = levels;
		}
		self
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
			FieldKind::Number { min, max } => {
				*min = Some(from);
				*max = Some(to);
			}
			_ => panic!("{}: range is only for numbers", self.handle),
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

/// `eventDate` becomes `Event date`.
pub(super) fn humanize(handle: &str) -> String {
	let mut s = String::new();
	for (i, c) in handle.chars().enumerate() {
		if i == 0 {
			s.push(c.to_ascii_uppercase());
		} else if c.is_ascii_uppercase() {
			s.push(' ');
			s.push(c.to_ascii_lowercase());
		} else {
			s.push(c);
		}
	}
	s
}
