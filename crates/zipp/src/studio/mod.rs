//! The studio, built from `studio/` of the repository into
//! `crates/zipp/studio` (`npm run build`), so a site needs no npm.

pub mod api;
#[cfg(test)]
mod api_tests;
mod routes;

pub use routes::{api_routes, routes};

use std::collections::BTreeMap;

use crate::schema::{Icon, Schema};

#[derive(rust_embed::Embed)]
#[folder = "studio/"]
struct Files;

/// The Lucide icons the schema names (entry types, blocks, nav groups),
/// by name: from the build of the studio, which has all of them in
/// `_studio/lucide-<version>/`. Gathered once at the start, they come with
/// the schema; the studio fetches only names that aren't here.
pub fn schema_icons(schema: &Schema) -> BTreeMap<String, String> {
	let names = schema
		.components
		.iter()
		.filter_map(|c| c.icon.as_ref())
		.chain(schema.nav.iter().filter_map(|g| g.icon.as_ref()))
		.filter_map(|i| match i {
			Icon::Lucide(name) => Some(name),
			Icon::Svg(_) => None,
		});
	let Some(dir) = Files::iter().find_map(|p| {
		let (dir, _) = p.strip_prefix("_studio/lucide-")?.split_once('/')?;
		Some(format!("_studio/lucide-{dir}"))
	}) else {
		return BTreeMap::new();
	};

	let mut icons = BTreeMap::new();
	for name in names {
		let Some(file) = Files::get(&format!("{dir}/{name}.svg")) else {
			continue;
		};
		let svg = String::from_utf8_lossy(&file.data);
		// the license comment is in the files, not needed in the schema
		let svg = match svg.find("<svg") {
			Some(start) => svg[start..].trim().to_string(),
			None => continue,
		};
		icons.insert(name.clone(), svg);
	}
	icons
}
