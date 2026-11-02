//! The JSON that `craft/export.php` writes.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Export {
	pub sites: Vec<CraftSite>,
	#[serde(default, deserialize_with = "map_or_list")]
	pub sections: BTreeMap<String, CraftSection>,
	/// group, site, categories
	#[serde(default, deserialize_with = "map_or_list")]
	pub categories: BTreeMap<String, BTreeMap<String, Vec<Element>>>,
	/// set, site, its fields
	#[serde(default, deserialize_with = "map_or_list")]
	pub globals: BTreeMap<String, BTreeMap<String, Global>>,
	#[serde(default)]
	pub assets: Vec<CraftAsset>,
}

#[derive(Debug, Deserialize)]
pub struct CraftSite {
	pub handle: String,
}

#[derive(Debug, Deserialize)]
pub struct CraftSection {
	/// the elements per site, a structure in its order
	#[serde(deserialize_with = "map_or_list")]
	pub sites: BTreeMap<String, Vec<Element>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
	pub id: i64,
	#[serde(rename = "type")]
	pub ty: Option<String>,
	pub title: Option<String>,
	pub slug: Option<String>,
	pub enabled: bool,
	pub parent_id: Option<i64>,
	pub post_date: Option<String>,
	pub expiry_date: Option<String>,
	pub date_created: Option<String>,
	#[serde(default)]
	pub fields: Value,
}

#[derive(Debug, Deserialize)]
pub struct Global {
	#[serde(default)]
	pub fields: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CraftAsset {
	pub id: i64,
	pub volume: String,
	pub path: String,
	pub title: Option<String>,
	/// of the primary site
	pub alt: Option<String>,
	/// per Craft site, from exports that have it
	#[serde(default, deserialize_with = "map_or_list")]
	pub alts: BTreeMap<String, String>,
	pub focal_point: Option<(f64, f64)>,
}

/// PHP writes an empty map as `[]`.
fn map_or_list<'de, D, T>(
	d: D,
) -> std::result::Result<BTreeMap<String, T>, D::Error>
where
	D: serde::Deserializer<'de>,
	T: Deserialize<'de>,
{
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum Either<T> {
		Map(BTreeMap<String, T>),
		List(#[allow(dead_code)] Vec<Value>),
	}
	Ok(match Either::deserialize(d)? {
		Either::Map(m) => m,
		Either::List(_) => BTreeMap::new(),
	})
}
