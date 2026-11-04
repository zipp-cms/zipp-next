use super::*;

impl Transform {
	pub fn new(handle: &str) -> Self {
		Self {
			handle: handle.into(),
			width: None,
			height: None,
			crop: false,
			format: None,
		}
	}

	pub fn width(mut self, width: u32) -> Self {
		self.width = Some(width);
		self
	}

	pub fn height(mut self, height: u32) -> Self {
		self.height = Some(height);
		self
	}

	/// Fill width and height, cut around the focal point.
	pub fn crop(mut self) -> Self {
		self.crop = true;
		self
	}

	/// Its images in this type, `Format::Webp` for smaller photos. Urls of
	/// other types still work with another extension, see
	/// `Asset::format_url`.
	pub fn format(mut self, format: Format) -> Self {
		self.format = Some(format);
		self
	}
}

impl Section {
	fn new<'a>(
		handle: &str,
		kind: SectionKind,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self {
			handle: handle.into(),
			name: humanize(handle),
			kind,
			types: types.into_iter().map(Into::into).collect(),
			uri: None,
			type_uris: BTreeMap::new(),
			site_uris: BTreeMap::new(),
			uri_if: None,
			max_level: None,
		}
	}

	/// A section with exactly one entry, it is created on start.
	pub fn single(handle: &str, ty: &str) -> Self {
		Self::new(handle, SectionKind::Single, [ty])
	}

	pub fn channel<'a>(
		handle: &str,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self::new(handle, SectionKind::Channel, types)
	}

	pub fn structure<'a>(
		handle: &str,
		types: impl IntoIterator<Item = &'a str>,
	) -> Self {
		Self::new(handle, SectionKind::Structure, types)
	}

	pub fn name(mut self, name: &str) -> Self {
		self.name = name.into();
		self
	}

	pub fn uri(mut self, uri: &str) -> Self {
		self.uri = Some(uri.into());
		self
	}

	/// A uri for the entries of one type, like `""` for the home page.
	pub fn uri_of(mut self, ty: &str, uri: &str) -> Self {
		self.type_uris.insert(ty.into(), uri.into());
		self
	}

	/// A uri in one site instead of the section's, like `offres/{slug}`.
	pub fn uri_in(mut self, site: &str, uri: &str) -> Self {
		self.site_uris.insert(site.into(), uri.into());
		self
	}

	/// Only entries with this boolean field on get a page and a uri.
	pub fn uri_if(mut self, field: &str) -> Self {
		self.uri_if = Some(field.into());
		self
	}

	/// How deep the tree of a structure goes, 1 is only the top.
	pub fn max_level(mut self, level: u32) -> Self {
		self.max_level = Some(level);
		self
	}
}

impl Component {
	fn new(
		handle: &str,
		name: &str,
		entry: bool,
		fields: Vec<Field>,
		part_of: Option<String>,
	) -> Self {
		Self {
			handle: handle.into(),
			name: name.into(),
			entry,
			fields,
			tabs: vec![],
			card: false,
			icon: None,
			title_field: None,
			part_of,
		}
	}
}

/// Makes the fields of inline groups blocks of their own, named after
/// the parent and the field (`event` + `tickets` = `eventTickets`), so
/// content, GraphQL and migrations only know normal groups.
fn hoist_groups(parent: &str, fields: &mut [Field], out: &mut Vec<Component>) {
	for f in fields {
		let Some(mut inner) = f.inline.take() else {
			continue;
		};
		let mut chars = f.handle.chars();
		let first = chars.next().map(|c| c.to_ascii_uppercase());
		let handle =
			format!("{parent}{}{}", first.unwrap_or_default(), chars.as_str());
		f.kind = FieldKind::Group {
			component: handle.clone(),
		};

		let mut nested = vec![];
		hoist_groups(&handle, &mut inner, &mut nested);
		out.push(Component::new(
			&handle,
			&f.label,
			false,
			inner,
			Some(parent.into()),
		));
		out.extend(nested);
	}
}

impl Schema {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn site(mut self, handle: &str, name: &str, language: &str) -> Self {
		self.sites.push(Site::new(handle, name, language));
		self
	}

	/// A site that is not on the website, like a disabled site in Craft:
	/// frontends get none of its entries, only previews and admins do, and
	/// it is not in `crelteSites` and the sitemaps. Editors fill it in.
	pub fn offline(mut self, site: &str) -> Self {
		self.offline.push(site.into());
		self
	}

	pub fn section(mut self, section: Section) -> Self {
		self.sections.push(section);
		self
	}

	/// A group of sections in the top bar of the studio. Once there is one,
	/// every section has to be in exactly one group.
	pub fn nav<'a>(
		mut self,
		name: &str,
		sections: impl IntoIterator<Item = &'a str>,
	) -> Self {
		self.nav.push(NavGroup {
			name: name.into(),
			sections: sections.into_iter().map(Into::into).collect(),
			icon: None,
		});
		self
	}

	/// The icon of a group of the studio's navigation, after its `nav`:
	/// `.nav_icon("Content", "folder-open")`.
	pub fn nav_icon(mut self, group: &str, icon: impl Into<Icon>) -> Self {
		let Some(g) = self.nav.iter_mut().find(|g| g.name == group) else {
			panic!("nav_icon: there is no group {group}");
		};
		g.icon = Some(icon.into());
		self
	}

	/// The icon of an entry type or a block, after it:
	/// `.icon("event", "calendar")` or
	/// `.icon("logo", Icon::svg(include_str!("logo.svg")))`.
	pub fn icon(mut self, component: &str, icon: impl Into<Icon>) -> Self {
		let Some(c) = self.get_mut(component) else {
			panic!("icon: there is no entry type or block {component}");
		};
		c.icon = Some(icon.into());
		self
	}

	/// The field that names a block in the studio, after it, like Craft
	/// shows a nested entry by a field: `.block_title("feedback",
	/// "author")` shows `Feedback · Anna Muster` instead of only the type.
	pub fn block_title(mut self, block: &str, field: &str) -> Self {
		let Some(c) = self.get_mut(block).filter(|c| !c.entry) else {
			panic!("block_title: there is no block {block}");
		};
		c.title_field = Some(field.into());
		self
	}

	/// Shows the fields of a tab in one card with the labels on the left,
	/// like a group, after the type: `.tab_card("article", "Intro")`. For
	/// tabs of several short fields, a long column of them reads badly.
	/// Not for blocks, their fields are in a card already.
	pub fn tab_card(mut self, component: &str, tab: &str) -> Self {
		let t = self
			.get_mut(component)
			.filter(|c| c.entry)
			.and_then(|c| c.tabs.iter_mut().find(|t| t.name == tab));
		let Some(t) = t else {
			panic!(
				"tab_card: there is no entry type {component} with a tab {tab}"
			);
		};
		t.card = true;
		self
	}

	/// [`Schema::tab_card`] for every tab of an entry type, or its one
	/// page when it has none: `.entry_card("event")`.
	pub fn entry_card(mut self, component: &str) -> Self {
		let Some(c) = self.get_mut(component).filter(|c| c.entry) else {
			panic!("entry_card: there is no entry type {component}");
		};
		c.card = true;
		self
	}

	/// Registers a field type of another crate, fields use it with
	/// [`Field::custom`].
	pub fn field_type(mut self, ty: impl FieldType) -> Self {
		self.field_types.0.push(std::sync::Arc::new(ty));
		self
	}

	/// A page of markdown in the studio, read only, e.g.
	/// `.doc("readme", "Readme", include_str!("../readme.md"))`. Put its
	/// handle into a nav group like a section.
	pub fn doc(mut self, handle: &str, name: &str, markdown: &str) -> Self {
		self.docs.push(DocPage {
			handle: handle.into(),
			name: name.into(),
			markdown: markdown.into(),
		});
		self
	}

	/// Texts of the frontend in a namespace, the key and its text in the
	/// primary site; editors translate them. They are a single
	/// `translations` with a texts field per namespace, so they have
	/// drafts like any entry; put `translations` into a nav group.
	pub fn translations<'a>(
		mut self,
		namespace: &str,
		keys: impl IntoIterator<Item = (&'a str, &'a str)>,
	) -> Self {
		if self.get(TRANSLATIONS).is_none() {
			self.sections.push(
				Section::single(TRANSLATIONS, TRANSLATIONS)
					.name("Translations"),
			);
			self = self.component(TRANSLATIONS, "Translations", true, []);
		}
		let field = Field::texts(namespace, keys);
		self.get_mut(TRANSLATIONS).unwrap().fields.push(field);
		self
	}

	/// Fields of every asset beside title and alt, like a photographer or
	/// a licence: `.asset_fields([Field::text("credit")])`. They are a
	/// component `asset`, so validation and migrations are the ones of
	/// blocks; the values are the same in all sites, GraphQL has them on
	/// `Asset`.
	pub fn asset_fields(self, fields: impl IntoIterator<Item = Field>) -> Self {
		self.component(ASSET, "Asset", false, fields)
	}

	/// A role of studio users, see [`Role`].
	pub fn role(mut self, role: Role) -> Self {
		self.roles.push(role);
		self
	}

	pub fn transform(mut self, transform: Transform) -> Self {
		self.transforms.push(transform);
		self
	}

	/// Widths images can be resized to without a named transform, like
	/// Craft's `url(width: 800)`: only these, so nobody can make the
	/// server render any size.
	pub fn image_widths(
		mut self,
		widths: impl IntoIterator<Item = u32>,
	) -> Self {
		self.image_sizes
			.extend(widths.into_iter().map(|w| (Some(w), None)));
		self
	}

	/// Heights images can be resized to, like Craft's `url(height:
	/// 800)`, see [`Schema::image_widths`].
	pub fn image_heights(
		mut self,
		heights: impl IntoIterator<Item = u32>,
	) -> Self {
		self.image_sizes
			.extend(heights.into_iter().map(|h| (None, Some(h))));
		self
	}

	/// Width and height images can be cropped to around their focal
	/// point, like Craft's `url(width: 800, height: 450)`, see
	/// [`Schema::image_widths`].
	pub fn image_sizes(
		mut self,
		sizes: impl IntoIterator<Item = (u32, u32)>,
	) -> Self {
		self.image_sizes
			.extend(sizes.into_iter().map(|(w, h)| (Some(w), Some(h))));
		self
	}

	/// The time zone dates are formatted in, like Craft's time zone of
	/// the system: `Europe/Zurich`. Without one it is UTC.
	pub fn timezone(mut self, timezone: &str) -> Self {
		self.timezone = Some(timezone.into());
		self
	}

	pub fn entry(
		self,
		handle: &str,
		name: &str,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.component(handle, name, true, fields)
	}

	/// An entry type with its fields in tabs, like Craft's field layout:
	/// `[("Content", vec![...]), ("SEO", vec![...])]`.
	pub fn entry_tabs<'a>(
		self,
		handle: &str,
		name: &str,
		tabs: impl IntoIterator<Item = (&'a str, Vec<Field>)>,
	) -> Self {
		self.component_tabs(handle, name, true, tabs)
	}

	pub fn block(
		self,
		handle: &str,
		name: &str,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		self.component(handle, name, false, fields)
	}

	/// A block with its fields in tabs like [`Schema::entry_tabs`], for a
	/// big block: `[("Content", vec![...]), ("Layout", vec![...])]`. The
	/// studio shows them as tabs inside its card.
	pub fn block_tabs<'a>(
		self,
		handle: &str,
		name: &str,
		tabs: impl IntoIterator<Item = (&'a str, Vec<Field>)>,
	) -> Self {
		self.component_tabs(handle, name, false, tabs)
	}

	fn component_tabs<'a>(
		mut self,
		handle: &str,
		name: &str,
		entry: bool,
		tabs: impl IntoIterator<Item = (&'a str, Vec<Field>)>,
	) -> Self {
		let mut fields = vec![];
		let mut layout = vec![];
		for (tab, tab_fields) in tabs {
			layout.push(Tab {
				name: tab.into(),
				fields: tab_fields.iter().map(|f| f.handle.clone()).collect(),
				when: None,
				card: false,
			});
			fields.extend(tab_fields);
		}
		self = self.component(handle, name, entry, fields);
		self.get_mut(handle).unwrap().tabs = layout;
		self
	}

	fn component(
		mut self,
		handle: &str,
		name: &str,
		entry: bool,
		fields: impl IntoIterator<Item = Field>,
	) -> Self {
		let mut fields: Vec<Field> = fields.into_iter().collect();
		let mut parts = vec![];
		hoist_groups(handle, &mut fields, &mut parts);
		self.components
			.push(Component::new(handle, name, entry, fields, None));
		self.components.extend(parts);
		self
	}
}
