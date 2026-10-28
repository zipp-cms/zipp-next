use super::*;

impl Transform {
	pub fn new(handle: &str) -> Self {
		Self {
			handle: handle.into(),
			width: None,
			height: None,
			crop: false,
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

	/// The uri of an entry in a site, `None` if it has no page. `fields`
	/// are its values in the site, for [`Section::uri_if`].
	pub fn render_uri(
		&self,
		ty: &str,
		slug: &str,
		parent: Option<&str>,
		site: &str,
		fields: &serde_json::Value,
	) -> Option<String> {
		if let Some(f) = &self.uri_if
			&& fields[f] != true
		{
			return None;
		}
		let format = self
			.type_uris
			.get(ty)
			.or(self.site_uris.get(site))
			.or(self.uri.as_ref())?;
		let uri = match parent {
			Some(p) => format.replace("{parent.uri}", p),
			None => format
				.replace("{parent.uri}/", "")
				.replace("{parent.uri}", ""),
		};
		Some(uri.replace("{slug}", slug).trim_matches('/').into())
	}
}
