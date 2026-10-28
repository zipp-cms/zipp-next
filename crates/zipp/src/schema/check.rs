use super::*;

impl Schema {
	/// Finds mistakes a compiler can't: unknown or duplicate handles and
	/// settings that don't fit the field.
	pub fn check(&self) -> Result<(), String> {
		if self.sites.is_empty() {
			return Err("a schema needs at least one site".into());
		}
		for (i, site) in self.sites.iter().enumerate() {
			check_handle(&site.handle)?;
			if self.sites[..i].iter().any(|s| s.handle == site.handle) {
				return Err(format!("site {} exists twice", site.handle));
			}
		}

		for (i, section) in self.sections.iter().enumerate() {
			check_handle(&section.handle)?;
			if self.sections[..i]
				.iter()
				.any(|s| s.handle == section.handle)
			{
				return Err(format!("section {} exists twice", section.handle));
			}
			self.check_section(section)
				.map_err(|e| format!("section {}: {e}", section.handle))?;
		}

		for (i, comp) in self.components.iter().enumerate() {
			check_handle(&comp.handle)?;
			if self.components[..i].iter().any(|c| c.handle == comp.handle) {
				return Err(format!("component {} exists twice", comp.handle));
			}

			for (j, field) in comp.fields.iter().enumerate() {
				let at = format!("{}.{}", comp.handle, field.handle);
				check_handle(&field.handle)
					.map_err(|e| format!("{at}: {e}"))?;
				if comp.fields[..j].iter().any(|f| f.handle == field.handle) {
					return Err(format!("field {at} exists twice"));
				}
				let reserved = match comp.entry {
					true => crate::graphql::ENTRY_FIELDS,
					false => crate::graphql::BLOCK_FIELDS,
				};
				if reserved.contains(&field.handle.as_str()) {
					return Err(format!(
						"field {at}: {} is built in",
						field.handle
					));
				}
				self.check_field(comp, field)
					.map_err(|e| format!("{at}: {e}"))?;
			}
		}

		if !self.nav.is_empty() {
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
		}

		for (i, d) in self.docs.iter().enumerate() {
			check_handle(&d.handle)?;
			if self.get_section(&d.handle).is_some()
				|| self.docs[..i].iter().any(|x| x.handle == d.handle)
			{
				return Err(format!("doc {} exists twice", d.handle));
			}
		}

		for (i, t) in self.transforms.iter().enumerate() {
			check_handle(&t.handle)?;
			if self.transforms[..i].iter().any(|x| x.handle == t.handle) {
				return Err(format!("transform {} exists twice", t.handle));
			}
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
			let parts = uri.replace("{slug}", "").replace("{parent.uri}", "");
			if parts.contains(['{', '}']) {
				return Err("the uri knows only {slug} and {parent.uri}".into());
			}
		}

		let uri = section.uri.as_deref().unwrap_or_default();
		let parts = uri.replace("{slug}", "").replace("{parent.uri}", "");
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
			_ if parts.contains(['{', '}']) => {
				Err("the uri knows only {slug} and {parent.uri}".into())
			}
			_ => Ok(()),
		}
	}

	fn check_field(
		&self,
		comp: &Component,
		field: &Field,
	) -> Result<(), String> {
		let (types, entry) = match &field.kind {
			FieldKind::Relation { types, .. } => (types, true),
			FieldKind::Blocks { types, .. } => (types, false),
			FieldKind::RichText { blocks, .. } => (blocks, false),
			_ => (&vec![], false),
		};
		if let FieldKind::RichText { headings, .. } = &field.kind
			&& headings.iter().any(|h| !(1..=6).contains(h))
		{
			return Err("headings go from 1 to 6".into());
		}
		if matches!(
			field.kind,
			FieldKind::Relation { .. } | FieldKind::Blocks { .. }
		) && types.is_empty()
		{
			return Err("needs at least one type".into());
		}
		if let FieldKind::Link { to, .. } = &field.kind
			&& to.is_empty()
		{
			return Err("a link needs something to point to".into());
		}
		if let FieldKind::Texts { keys } = &field.kind {
			for (i, (key, _)) in keys.iter().enumerate() {
				if keys[..i].iter().any(|(k, _)| k == key) {
					return Err(format!("text {key} exists twice"));
				}
			}
		}
		if let FieldKind::Dropdown { options } = &field.kind {
			if options.is_empty() {
				return Err("a dropdown needs options".into());
			}
			for (i, o) in options.iter().enumerate() {
				if o.value.is_empty() {
					return Err("an option needs a value".into());
				}
				if options[..i].iter().any(|x| x.value == o.value) {
					return Err(format!("option {} exists twice", o.value));
				}
			}
		}
		if let FieldKind::Custom { ty, options } = &field.kind {
			let Some(t) = self.field_types.get(ty) else {
				return Err(format!("unknown field type {ty}, register it"));
			};
			t.check(options)?;
		}
		if let FieldKind::Group { component } = &field.kind {
			if component == &comp.handle {
				return Err("a group can't hold its own component".into());
			}
			if self.get(component).is_none() {
				return Err(format!("unknown component {component}"));
			}
		}
		for t in types {
			match self.get(t) {
				Some(c) if c.entry == entry => {}
				Some(_) if entry => return Err(format!("{t} is not an entry")),
				Some(_) => return Err(format!("{t} is not a block")),
				None => return Err(format!("unknown component {t}")),
			}
		}

		if field.translatable && !comp.entry {
			return Err("only fields of entries can be translatable, blocks \
				follow their field"
				.into());
		}

		if field.indexed {
			if !comp.entry {
				return Err("only fields of entries can be indexed".into());
			}
			if !matches!(
				field.kind,
				FieldKind::Text { .. }
					| FieldKind::Number { .. }
					| FieldKind::Boolean
					| FieldKind::Dropdown { .. }
			) && !self.custom_scalar(&field.kind)
			{
				return Err(
					"only text, number, boolean and dropdowns can be indexed"
						.into(),
				);
			}
		}

		Ok(())
	}
}

impl Schema {
	/// Whether a field is of a custom type GraphQL shows as a scalar.
	pub(crate) fn custom_scalar(&self, kind: &FieldKind) -> bool {
		let FieldKind::Custom { ty, .. } = kind else {
			return false;
		};
		self.field_types
			.get(ty)
			.is_some_and(|t| matches!(t.graphql(), GraphqlType::Scalar(_)))
	}
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
