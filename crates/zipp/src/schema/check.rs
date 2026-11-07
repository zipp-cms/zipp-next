use super::*;

impl Schema {
	/// Finds mistakes a compiler can't: unknown or duplicate handles and
	/// settings that don't fit the field.
	pub fn check(&self) -> Result<(), String> {
		self.check_sites()?;
		self.check_sections()?;
		self.check_names()?;
		self.check_components()?;
		self.check_nav()?;
		self.check_offline()?;
		for p in &self.plugins.0 {
			p.check(self).map_err(|e| format!("{}: {e}", p.name()))?;
		}
		self.check_roles()?;
		self.check_docs()?;
		self.check_transforms()?;
		self.check_timezone()
	}

	fn check_timezone(&self) -> Result<(), String> {
		match &self.timezone {
			Some(tz) if jiff::tz::TimeZone::get(tz).is_err() => {
				Err(format!("timezone {tz:?} is unknown"))
			}
			_ => Ok(()),
		}
	}

	fn check_sites(&self) -> Result<(), String> {
		if self.sites.is_empty() {
			return Err("a schema needs at least one site".into());
		}
		for site in &self.sites {
			check_handle(&site.handle)?;
		}
		if let Some(s) = first_twice(&self.sites, |a, b| a.handle == b.handle) {
			return Err(format!("site {} exists twice", s.handle));
		}
		Ok(())
	}

	fn check_sections(&self) -> Result<(), String> {
		for section in &self.sections {
			check_handle(&section.handle)?;
		}
		if let Some(s) =
			first_twice(&self.sections, |a, b| a.handle == b.handle)
		{
			return Err(format!("section {} exists twice", s.handle));
		}
		for section in &self.sections {
			self.check_section(section)
				.map_err(|e| format!("section {}: {e}", section.handle))?;
		}
		Ok(())
	}

	/// Code finds its plugins and field types by name.
	fn check_names(&self) -> Result<(), String> {
		let plugins: Vec<&str> =
			self.plugins.0.iter().map(|p| p.name()).collect();
		let types: Vec<&str> =
			self.field_types.0.iter().map(|t| t.name()).collect();
		for names in [&plugins, &types] {
			if let Some(n) = first_twice(names, |a, b| a == b) {
				return Err(format!("two plugins or field types {n}"));
			}
		}
		Ok(())
	}

	fn check_components(&self) -> Result<(), String> {
		for comp in &self.components {
			check_handle(&comp.handle)?;
		}
		if let Some(comp) =
			first_twice(&self.components, |a, b| a.handle == b.handle)
		{
			// the handle of an inline group is made, say where from
			let other = self.get(&comp.handle);
			let made = [Some(comp), other]
				.into_iter()
				.flatten()
				.find_map(|c| c.part_of.as_ref());
			return Err(match made {
				Some(p) => format!(
					"component {} exists twice, an inline group of {p} \
					makes it",
					comp.handle
				),
				None => format!("component {} exists twice", comp.handle),
			});
		}
		for comp in &self.components {
			self.check_component(comp)?;
		}

		for comp in self.components.iter().filter(|c| c.entry) {
			if !self.sections.iter().any(|s| s.types.contains(&comp.handle)) {
				return Err(format!(
					"entry type {} is in no section",
					comp.handle
				));
			}
		}
		Ok(())
	}

	fn check_component(&self, comp: &Component) -> Result<(), String> {
		if comp.handle == ASSET && comp.entry {
			return Err(format!(
				"{ASSET} is the handle of the asset fields, not of an entry type"
			));
		}

		for tab in &comp.tabs {
			if tab.name.trim().is_empty() {
				return Err(format!("{}: a tab needs a name", comp.handle));
			}
		}
		if let Some(tab) = first_twice(&comp.tabs, |a, b| a.name == b.name) {
			return Err(format!(
				"{}: tab {} exists twice",
				comp.handle, tab.name
			));
		}
		for tab in &comp.tabs {
			let at = format!("{}: tab {}", comp.handle, tab.name);
			if tab.fields.is_empty() {
				return Err(format!("{at} has no fields"));
			}
			if let Some(c) = &tab.when {
				self.check_condition(comp, c, &tab.fields)
					.map_err(|e| format!("{at}: {e}"))?;
			}
		}

		for field in &comp.fields {
			check_handle(&field.handle).map_err(|e| {
				format!("{}.{}: {e}", comp.handle, field.handle)
			})?;
		}
		if let Some(f) = first_twice(&comp.fields, |a, b| a.handle == b.handle)
		{
			return Err(format!(
				"field {}.{} exists twice",
				comp.handle, f.handle
			));
		}
		let reserved = match comp.entry {
			true => crate::graphql::ENTRY_FIELDS,
			false if comp.handle == ASSET => crate::graphql::ASSET_FIELDS,
			false => crate::graphql::BLOCK_FIELDS,
		};
		for field in &comp.fields {
			let at = format!("{}.{}", comp.handle, field.handle);
			if reserved.contains(&field.handle.as_str()) {
				return Err(format!(
					"field {at}: {} is built in",
					field.handle
				));
			}
			self.check_field(comp, field)
				.map_err(|e| format!("{at}: {e}"))?;
			if let Some(c) = &field.when {
				let of = [field.handle.clone()];
				self.check_condition(comp, c, &of)
					.map_err(|e| format!("{at}: {e}"))?;
			}
		}

		if let Some(t) = &comp.title_field {
			check_title_field(comp, t)
				.map_err(|e| format!("{}: title {t}: {e}", comp.handle))?;
		}
		Self::check_cycles(comp).map_err(|e| format!("{}: {e}", comp.handle))
	}

	fn check_nav(&self) -> Result<(), String> {
		if self.nav.is_empty() {
			return Ok(());
		}
		for section in &self.sections {
			let n = self
				.nav
				.iter()
				.filter(|g| g.sections.contains(&section.handle))
				.count();
			if n != 1 {
				return Err(format!(
					"section {} is in {n} nav groups, it needs one",
					section.handle
				));
			}
		}
		for g in &self.nav {
			// docs are pages of the studio, not sections
			let unknown = g.sections.iter().find(|s| {
				self.get_section(s).is_none()
					&& !self.docs.iter().any(|d| d.handle == **s)
			});
			if let Some(s) = unknown {
				return Err(format!("nav {}: unknown section {s}", g.name));
			}
		}
		Ok(())
	}

	fn check_offline(&self) -> Result<(), String> {
		if let Some(s) =
			self.offline.iter().find(|s| self.get_site(s).is_none())
		{
			return Err(format!("offline: unknown site {s}"));
		}
		if self.is_offline(&self.primary_site().handle) {
			return Err("the primary site can't be offline".into());
		}
		Ok(())
	}

	fn check_roles(&self) -> Result<(), String> {
		for r in &self.roles {
			check_handle(&r.handle)?;
		}
		let twice = self.roles.iter().find(|r| r.handle == "admin");
		if let Some(r) =
			twice.or(first_twice(&self.roles, |a, b| a.handle == b.handle))
		{
			return Err(format!("role {} exists twice", r.handle));
		}
		for r in &self.roles {
			if let Some(s) =
				r.sections.keys().find(|s| self.get_section(s).is_none())
			{
				return Err(format!("role {}: unknown section {s}", r.handle));
			}
		}
		Ok(())
	}

	fn check_docs(&self) -> Result<(), String> {
		for d in &self.docs {
			check_handle(&d.handle)?;
		}
		let twice = self
			.docs
			.iter()
			.find(|d| self.get_section(&d.handle).is_some());
		if let Some(d) =
			twice.or(first_twice(&self.docs, |a, b| a.handle == b.handle))
		{
			return Err(format!("doc {} exists twice", d.handle));
		}
		Ok(())
	}

	fn check_transforms(&self) -> Result<(), String> {
		for t in &self.transforms {
			// only a name in urls, so also Craft's `land_w400`; `_thumb`
			// and other zipp's own start with an underscore
			let mut chars = t.handle.chars();
			let ok = chars.next().is_some_and(|c| c.is_ascii_lowercase())
				&& chars.all(|c| c.is_ascii_alphanumeric() || c == '_');
			if !ok {
				return Err(format!(
					"transform {:?}: letters, digits and _, starting with a \
					small letter",
					t.handle
				));
			}
		}
		if let Some(t) =
			first_twice(&self.transforms, |a, b| a.handle == b.handle)
		{
			return Err(format!("transform {} exists twice", t.handle));
		}
		for t in &self.transforms {
			if t.width.is_none() && t.height.is_none() {
				return Err(format!("transform {} needs a size", t.handle));
			}
			if t.crop && (t.width.is_none() || t.height.is_none()) {
				return Err(format!(
					"transform {} crops, it needs both sizes",
					t.handle
				));
			}
		}
		let zero = |s: Option<u32>| s == Some(0);
		if self.image_sizes.iter().any(|(w, h)| zero(*w) || zero(*h)) {
			return Err("image sizes can't be 0".into());
		}
		Ok(())
	}

	fn check_section(&self, section: &Section) -> Result<(), String> {
		if section.types.is_empty() {
			return Err("needs at least one entry type".into());
		}
		for t in &section.types {
			if self.entry_type(t).is_none() {
				return Err(format!("{t} is not an entry type"));
			}
		}

		for (ty, uri) in &section.type_uris {
			if !section.types.contains(ty) {
				return Err(format!("uri of {ty}: not a type of the section"));
			}
			known_placeholders(uri)?;
		}

		for (site, uri) in &section.site_uris {
			if self.get_site(site).is_none() {
				return Err(format!("uri in {site}: not a site"));
			}
			if section.uri.is_none() {
				return Err(format!("uri in {site}: the section has no uri"));
			}
			known_placeholders(uri)?;
		}
		if let Some(f) = &section.uri_if {
			for t in &section.types {
				let field = self
					.entry_type(t)
					.and_then(|c| c.fields.iter().find(|x| &x.handle == f));
				if !field.is_some_and(|x| x.kind == FieldKind::Boolean) {
					return Err(format!("uri if {f}: {t} has no boolean {f}"));
				}
			}
		}

		match section.max_level {
			Some(0) => return Err("the max level is at least 1".into()),
			Some(_) if section.kind != SectionKind::Structure => {
				return Err("only structures have a max level".into());
			}
			_ => {}
		}

		let uri = section.uri.as_deref().unwrap_or_default();
		match section.kind {
			SectionKind::Single if section.types.len() > 1 => {
				Err("a single has one entry type".into())
			}
			SectionKind::Single if uri.contains('{') => {
				Err("the uri of a single can't have placeholders".into())
			}
			SectionKind::Channel if uri.contains("{parent.uri}") => {
				Err("only structures have {parent.uri}".into())
			}
			SectionKind::Channel | SectionKind::Structure
				if section.uri.is_some() && !uri.contains("{slug}") =>
			{
				Err("the uri needs {slug}".into())
			}
			_ => known_placeholders(uri),
		}
	}

	fn check_field(
		&self,
		comp: &Component,
		field: &Field,
	) -> Result<(), String> {
		self.check_kind(comp, &field.kind)?;
		self.check_limit(&field.kind)?;

		if ![25, 50, 75, 100].contains(&field.width) {
			return Err("the width is 25, 50, 75 or 100".into());
		}
		if matches!(field.kind, FieldKind::Note { .. })
			&& (field.required || field.indexed || field.translatable)
		{
			return Err("a note holds no value, it can't be required, \
				indexed or translatable"
				.into());
		}

		// uploads start empty, they know no schema
		if comp.handle == ASSET && field.default.is_some() {
			return Err("asset fields have no defaults".into());
		}
		crate::content::check_default(self, field)
			.map_err(|e| format!("default: {e}"))?;

		if field.translatable && !comp.entry {
			return Err("only fields of entries can be translatable, blocks \
				follow their field"
				.into());
		}

		if field.indexed {
			if !comp.entry {
				return Err("only fields of entries can be indexed".into());
			}
			if self.index_kind(&field.kind).is_none() {
				return Err(
					"only text, email, number, boolean and dropdowns can be indexed"
						.into(),
				);
			}
		}

		Ok(())
	}

	/// The settings of a field's kind.
	fn check_kind(
		&self,
		comp: &Component,
		kind: &FieldKind,
	) -> Result<(), String> {
		match kind {
			FieldKind::Relation { types, .. }
			| FieldKind::Blocks { types, .. }
				if types.is_empty() =>
			{
				Err("needs at least one type".into())
			}
			FieldKind::Relation { types, .. } => self.check_types(types, true),
			FieldKind::Blocks { types, .. } => self.check_types(types, false),
			FieldKind::RichText {
				headings,
				styles,
				blocks,
				..
			} => {
				if headings.iter().any(|h| !(1..=6).contains(h)) {
					return Err("headings go from 1 to 6".into());
				}
				for style in styles {
					// a class of css the frontend can select
					let mut chars = style.class.chars();
					let ok =
						chars.next().is_some_and(|c| c.is_ascii_alphabetic())
							&& chars.all(|c| {
								c.is_ascii_alphanumeric() || "-_".contains(c)
							});
					if !ok {
						return Err(format!(
							"style {:?} must be a class like lead or text-large",
							style.class
						));
					}
				}
				if let Some(s) = first_twice(styles, |a, b| a.class == b.class)
				{
					return Err(format!("style {} exists twice", s.class));
				}
				self.check_types(blocks, false)?;
				check_toolbar(kind)
			}
			// more places than an f64 keeps exactly
			FieldKind::Number {
				decimals: Some(d), ..
			} if *d > 10 => Err("decimals go from 0 to 10".into()),
			FieldKind::Link { to, .. } if to.is_empty() => {
				Err("a link needs something to point to".into())
			}
			FieldKind::Texts { keys } => {
				match first_twice(keys, |(a, _), (b, _)| a == b) {
					Some((key, _)) => Err(format!("text {key} exists twice")),
					None => Ok(()),
				}
			}
			FieldKind::Dropdown { options }
			| FieldKind::Checkboxes { options } => {
				if options.is_empty() {
					return Err("it needs options".into());
				}
				if options.iter().any(|o| o.value.is_empty()) {
					return Err("an option needs a value".into());
				}
				match first_twice(options, |a, b| a.value == b.value) {
					Some(o) => Err(format!("option {} exists twice", o.value)),
					None => Ok(()),
				}
			}
			FieldKind::ButtonGroup { options } => {
				if options.is_empty() {
					return Err("it needs options".into());
				}
				if options.iter().any(|o| o.value.is_empty()) {
					return Err("an option needs a value".into());
				}
				if let Some(o) = first_twice(options, |a, b| a.value == b.value)
				{
					return Err(format!("option {} exists twice", o.value));
				}
				// the studio and websites put it into CSS
				let bad = options
					.iter()
					.find(|o| o.color.as_ref().is_some_and(|c| !is_color(c)));
				match bad {
					Some(o) => {
						Err(format!("option {}: the color is #rrggbb", o.value))
					}
					None => Ok(()),
				}
			}
			FieldKind::Assets {
				folder, extensions, ..
			} => {
				// the folders are names, the api refuses a `/` in them
				if folder
					.as_deref()
					.is_some_and(|f| f.split('/').any(|n| n.trim().is_empty()))
				{
					return Err("a folder is a path like Photos/Events".into());
				}
				// the studio fills them in from the entry, see field.rs
				let rest = folder.as_deref().map(|f| {
					f.replace("{slug}", "")
						.replace("{section}", "")
						.replace("{type}", "")
				});
				if rest.is_some_and(|r| r.contains(['{', '}'])) {
					return Err("a folder knows only {slug}, {section} and \
						{type}"
						.into());
				}
				if extensions.iter().any(|e| {
					e.is_empty() || !e.chars().all(|c| c.is_alphanumeric())
				}) {
					return Err("extensions are like pdf or jpg".into());
				}
				Ok(())
			}
			FieldKind::Table { columns } => {
				if columns.is_empty() {
					return Err("a table needs columns".into());
				}
				for c in columns {
					check_handle(&c.handle)?;
				}
				if let Some(c) =
					first_twice(columns, |a, b| a.handle == b.handle)
				{
					return Err(format!("column {} exists twice", c.handle));
				}
				for c in columns {
					// a cell is checked like the field of its kind
					self.check_field(comp, &c.as_field())
						.map_err(|e| format!("column {}: {e}", c.handle))?;
				}
				Ok(())
			}
			FieldKind::Navigation { to, depth, .. }
				if to.is_empty() || *depth == 0 =>
			{
				Err("a navigation needs link kinds and a depth".into())
			}
			FieldKind::Custom { ty, options } => {
				let Some(t) = self.field_types.get(ty) else {
					return Err(format!(
						"unknown field type {ty}, register it"
					));
				};
				t.check(options)
			}
			FieldKind::Group { component } => {
				if self.get(component).is_none() {
					return Err(format!("unknown component {component}"));
				}
				// its defaults would never end
				if self.group_holds(component, &comp.handle) {
					return Err("a group can't hold its own component".into());
				}
				if component == ASSET {
					return Err(asset_holds_no_content());
				}
				Ok(())
			}
			FieldKind::Note { text } if text.trim().is_empty() => {
				Err("a note needs a text".into())
			}
			_ => Ok(()),
		}
	}

	/// The sections a field takes entries of, see [`Field::sections`].
	fn check_limit(&self, kind: &FieldKind) -> Result<(), String> {
		let (sections, links) = match kind {
			FieldKind::Relation { sections, .. } => (sections, false),
			FieldKind::Link { sections, to, .. }
			| FieldKind::Navigation { sections, to, .. } => {
				if !sections.is_empty() && !to.contains(&LinkKind::Entry) {
					return Err("sections are for links to entries".into());
				}
				(sections, true)
			}
			FieldKind::RichText {
				sections,
				links,
				buttons,
				..
			} => {
				if !sections.is_empty() && !links && !buttons {
					return Err("sections are for links and buttons".into());
				}
				(sections, true)
			}
			_ => return Ok(()),
		};

		for handle in sections {
			let Some(s) = self.get_section(handle) else {
				return Err(format!("unknown section {handle}"));
			};
			// links go to pages, a section without them gives none
			if links && s.uri.is_none() && s.type_uris.is_empty() {
				return Err(format!("section {handle} has no pages"));
			}
		}
		if let Some(s) = first_twice(sections, |a, b| a == b) {
			return Err(format!("section {s} is there twice"));
		}

		// every type and every section must give something to pick
		if let FieldKind::Relation { types, .. } = kind
			&& !sections.is_empty()
		{
			let of = |s: &String| &self.get_section(s).unwrap().types;
			if let Some(t) = types
				.iter()
				.find(|t| !sections.iter().any(|s| of(s).contains(t)))
			{
				return Err(format!("{t} is in none of the sections"));
			}
			if let Some(s) = sections
				.iter()
				.find(|s| !of(s).iter().any(|t| types.contains(t)))
			{
				return Err(format!("section {s} has none of the types"));
			}
		}
		Ok(())
	}

	/// The components a relation (`entry`) or blocks field holds.
	fn check_types(&self, types: &[String], entry: bool) -> Result<(), String> {
		// the asset fields only live on assets
		if types.iter().any(|t| t == ASSET) {
			return Err(asset_holds_no_content());
		}
		for t in types {
			match self.get(t) {
				Some(c) if c.entry == entry => {}
				Some(_) if entry => return Err(format!("{t} is not an entry")),
				Some(_) => return Err(format!("{t} is not a block")),
				None => return Err(format!("unknown component {t}")),
			}
		}
		Ok(())
	}

	/// Whether the group of `component` holds `target`, also through the
	/// groups inside it.
	fn group_holds(&self, component: &str, target: &str) -> bool {
		let mut todo = vec![component];
		let mut seen = vec![];
		while let Some(c) = todo.pop() {
			if c == target {
				return true;
			}
			if seen.contains(&c) {
				continue;
			}
			seen.push(c);
			for f in self.get(c).into_iter().flat_map(|c| &c.fields) {
				if let FieldKind::Group { component } = &f.kind {
					todo.push(component);
				}
			}
		}
		false
	}
}

/// The title field of a block is one of its fields that holds a short
/// text, the studio shows it in the block's head.
fn check_title_field(comp: &Component, handle: &str) -> Result<(), String> {
	let Some(field) = comp.fields.iter().find(|f| f.handle == handle) else {
		return Err("the block has no such field".into());
	};
	match field.kind {
		FieldKind::Text { .. }
		| FieldKind::Email
		| FieldKind::Number { .. }
		| FieldKind::Date { .. }
		| FieldKind::Dropdown { .. }
		| FieldKind::ButtonGroup { .. }
		| FieldKind::RichText { .. }
		| FieldKind::Markdown
		| FieldKind::Link { .. }
		| FieldKind::Relation { .. } => Ok(()),
		_ => Err("only text, email, number, date, dropdown, button group, \
			rich text, markdown, link and relation fields give a title"
			.into()),
	}
}

fn is_color(c: &str) -> bool {
	c.len() == 7
		&& c.starts_with('#')
		&& c[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn asset_holds_no_content() -> String {
	format!("{ASSET} holds the asset fields, no content")
}

/// The first item that is the same as one before it, for the handles
/// that exist twice.
fn first_twice<T>(items: &[T], same: impl Fn(&T, &T) -> bool) -> Option<&T> {
	items
		.iter()
		.enumerate()
		.find(|(i, x)| items[..*i].iter().any(|y| same(x, y)))
		.map(|(_, x)| x)
}

/// The tools of a rich text's bar exist, are allowed by the field and
/// are there once; the lines are between tools. Only for a rich text.
fn check_toolbar(kind: &FieldKind) -> Result<(), String> {
	let FieldKind::RichText {
		headings,
		lists,
		links,
		buttons,
		blocks,
		styles,
		toolbar,
		..
	} = kind
	else {
		unreachable!("a toolbar is a rich text's");
	};

	for (i, tool) in toolbar.iter().enumerate() {
		let allowed = match tool.as_str() {
			"|" => {
				if i == 0 || i == toolbar.len() - 1 || toolbar[i - 1] == "|" {
					return Err("a | of the toolbar goes between tools".into());
				}
				continue;
			}
			"format" => !headings.is_empty(),
			"styles" => !styles.is_empty(),
			"bold" | "italic" => true,
			"bulletList" | "orderedList" => *lists,
			"link" => *links,
			"button" => *buttons,
			"insert" => true,
			t => match t.strip_prefix("block:") {
				Some(b) => blocks.iter().any(|x| x == b),
				None => {
					return Err(format!("the toolbar has no tool {t}"));
				}
			},
		};
		if !allowed {
			return Err(format!("the toolbar's {tool} isn't allowed here"));
		}
	}

	let tools: Vec<_> = toolbar.iter().filter(|t| *t != "|").collect();
	if let Some(t) = first_twice(&tools, |a, b| a == b) {
		return Err(format!("the toolbar has {t} twice"));
	}

	// the Insert menu takes the buttons and blocks that have no tool
	let left = (*buttons && !toolbar.iter().any(|t| t == "button"))
		|| blocks
			.iter()
			.any(|b| !toolbar.iter().any(|t| *t == format!("block:{b}")));
	if toolbar.iter().any(|t| t == "insert") && !left {
		return Err("the toolbar's insert has nothing left to insert".into());
	}

	Ok(())
}

/// A uri has no other placeholders than `{slug}` and `{parent.uri}`.
fn known_placeholders(uri: &str) -> Result<(), String> {
	let rest = uri.replace("{slug}", "").replace("{parent.uri}", "");
	if rest.contains(['{', '}']) {
		return Err("the uri knows only {slug} and {parent.uri}".into());
	}
	Ok(())
}

pub(super) fn check_handle(handle: &str) -> Result<(), String> {
	let mut chars = handle.chars();
	let ok = chars.next().is_some_and(|c| c.is_ascii_lowercase())
		&& chars.all(|c| c.is_ascii_alphanumeric());
	if ok {
		Ok(())
	} else {
		Err(format!("handle {handle:?} must be camelCase"))
	}
}
