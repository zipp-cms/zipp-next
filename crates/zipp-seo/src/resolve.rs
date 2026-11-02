//! Resolves the SEO of a page like the Craft plugin meierlabs-seo: the
//! title, meta tags, links and JSON-LD a crelte frontend renders with the
//! npm package `meierlabs-seo`. Every value is taken from the first place
//! that has one: the entry's SEO field, its entry type (the settings, then
//! the code), its section (the same), the global settings. The data is
//! loaded by the caller (GraphQL, with the drafts of a preview), this only
//! computes; the JSON-LD is in `json_ld.rs`.

use std::collections::BTreeMap;

use serde_json::{Value, json};

use crate::json_ld::{json_ld, script_json};
use crate::{Seo, SeoDefaults};
use zipp::assets::{Asset, transform_size};
use zipp::content::{markdown_text, richtext};
use zipp::entries::Entry;
use zipp::schema::{FieldKind, Schema, Site, Transform};

pub const DEFAULT_TITLE_PATTERN: &str = "{title} | {siteName}";
pub const DESCRIPTION_LENGTH: usize = 160;
/// The social image (og:image), cropped around the focal point.
pub const SOCIAL_TRANSFORM: &str = "seoSocial";

pub fn social_transform() -> Transform {
	Transform::new(SOCIAL_TRANSFORM)
		.width(1200)
		.height(630)
		.crop()
}

/// The global settings of a site, with the primary site's where the
/// site has none (but not the description and robots.txt).
#[derive(Debug, Clone, Default)]
pub struct Globals {
	pub site_name: String,
	pub title_pattern: String,
	pub description: Option<String>,
	pub image: Option<i64>,
	pub robots: String,
	pub twitter_handle: Option<String>,
	pub robots_txt: Option<String>,
	/// the fields of the organization, for JSON-LD
	pub organization: Value,
	/// by `section:<handle>` and `type:<handle>`, what editors set
	pub overrides: BTreeMap<String, Level>,
}

/// SEO values of a section or an entry type: the editors' in the settings
/// or the code's [`SeoDefaults`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Level {
	pub title_pattern: Option<String>,
	pub description: Option<String>,
	pub robots: Option<String>,
	pub image: Option<i64>,
	/// code only: the fields title, description and image come from
	pub title_from: Option<String>,
	pub description_from: Option<String>,
	pub image_from: Option<String>,
	pub sitemap: Option<bool>,
}

impl From<&SeoDefaults> for Level {
	fn from(d: &SeoDefaults) -> Self {
		Self {
			title_pattern: d.title_pattern.clone(),
			description: d.description.clone(),
			robots: d.robots.clone(),
			image: None,
			title_from: d.title_from.clone(),
			description_from: d.description_from.clone(),
			image_from: d.image_from.clone(),
			// only leaving out counts, so settings of a section win over
			// an entry type that sets something else in code
			sitemap: (!d.sitemap).then_some(false),
		}
	}
}

/// The blocks of the overrides by target.
fn by_target(v: &Value) -> BTreeMap<String, &Value> {
	let blocks = v["overrides"].as_array().map_or(&[][..], |a| a);
	blocks
		.iter()
		.filter_map(|b| {
			let f = &b["fields"];
			Some((text(&f["appliesTo"])?, f))
		})
		.collect()
}

/// The overrides of the settings by target, the site's with the primary
/// site's where the site has none (but not the description).
fn overrides(own: &Value, primary: &Value) -> BTreeMap<String, Level> {
	let (own, primary) = (by_target(own), by_target(primary));
	let none = Value::Null;
	let mut out = BTreeMap::new();
	for target in own.keys().chain(primary.keys()) {
		let o = own.get(target).copied().unwrap_or(&none);
		let p = primary.get(target).copied().unwrap_or(&none);
		let inherited = |k: &str| text(&o[k]).or_else(|| text(&p[k]));
		let sitemap = inherited("sitemap").map(|s| s == "yes");
		out.insert(
			target.clone(),
			Level {
				title_pattern: inherited("titlePattern"),
				description: text(&o["description"]),
				robots: inherited("robots"),
				image: first_id(&o["image"]).or_else(|| first_id(&p["image"])),
				sitemap,
				..Default::default()
			},
		);
	}
	out
}

pub(crate) fn text(v: &Value) -> Option<String> {
	v.as_str()
		.map(str::trim)
		.filter(|s| !s.is_empty())
		.map(Into::into)
}

pub(crate) fn first_id(v: &Value) -> Option<i64> {
	v.as_array()?.iter().find_map(Value::as_i64)
}

/// `own` and `primary` are the fields of `seoSettings` in the site and the
/// primary site.
pub fn globals(site: &Site, own: &Value, primary: &Value) -> Globals {
	let inherited = |k: &str| text(&own[k]).or_else(|| text(&primary[k]));
	Globals {
		site_name: inherited("siteName").unwrap_or_else(|| site.name.clone()),
		title_pattern: inherited("titlePattern")
			.unwrap_or_else(|| DEFAULT_TITLE_PATTERN.into()),
		description: text(&own["description"]),
		image: first_id(&own["image"]).or_else(|| first_id(&primary["image"])),
		robots: inherited("robots").unwrap_or_else(|| "index, follow".into()),
		twitter_handle: inherited("twitterHandle"),
		robots_txt: text(&own["robotsTxt"]),
		organization: if own["organization"].is_object() {
			own["organization"].clone()
		} else {
			primary["organization"].clone()
		},
		overrides: overrides(own, primary),
	}
}

/// The levels of an entry, most specific first: its entry type in the
/// settings and in code, then its section.
pub fn defaults(
	schema: &Schema,
	globals: &Globals,
	entry: &Entry,
) -> Vec<Level> {
	levels(schema, globals, Some(&entry.ty), &entry.section)
}

pub(crate) fn levels(
	schema: &Schema,
	globals: &Globals,
	ty: Option<&str>,
	section: &str,
) -> Vec<Level> {
	let seo = Seo::of(schema);
	let set = |k: String| globals.overrides.get(&k).cloned();
	let code = |d: Option<&SeoDefaults>| d.map(Level::from);
	let mut out = vec![];
	if let Some(ty) = ty {
		out.extend(set(format!("type:{ty}")));
		out.extend(code(seo.types.get(ty)));
	}
	out.extend(set(format!("section:{section}")));
	out.extend(code(seo.sections.get(section)));
	out
}

pub(crate) fn first(
	defaults: &[Level],
	f: impl Fn(&Level) -> Option<&String>,
) -> Option<String> {
	defaults.iter().find_map(f).cloned()
}

/// Whether a section belongs into the sitemap: not left out, or one of
/// its entry types is put in.
pub fn section_in_sitemap(
	schema: &Schema,
	globals: &Globals,
	section: &str,
) -> bool {
	let of = |l: Vec<Level>| l.iter().find_map(|l| l.sitemap);
	let types = schema
		.get_section(section)
		.map_or(vec![], |s| s.types.clone());
	of(levels(schema, globals, None, section)) != Some(false)
		|| types.iter().any(|t| {
			of(levels(schema, globals, Some(t), section)) == Some(true)
		})
}

/// The value of the entry's SEO field, if its type has one.
pub fn field_data(schema: &Schema, entry: &Entry) -> Value {
	let comp = schema.get(&entry.ty);
	let field =
		comp.and_then(|c| c.fields.iter().find(|f| crate::plugin::is_seo(f)));
	field.map_or(Value::Null, |f| entry.fields[&f.handle].clone())
}

/// A value of the fields at a path like `pageIntro.text`.
fn at<'a>(fields: &'a Value, path: &str) -> &'a Value {
	path.split('.').fold(fields, |v, key| &v[key])
}

pub fn truncate(text: &str, length: usize) -> String {
	if text.chars().count() <= length {
		return text.into();
	}
	let mut cut: String = text.chars().take(length - 1).collect();
	if let Some(space) = cut.rfind(' ')
		&& cut[..space].chars().count() > length / 2
	{
		cut.truncate(space);
	}
	format!(
		"{}…",
		cut.trim_end_matches([' ', ',', '.', ';', ':', '-', '–'])
	)
}

/// The title pattern with `{title}`, `{siteName}` and `{sectionName}`.
pub fn render_title(
	pattern: &str,
	title: &str,
	site_name: &str,
	section_name: &str,
) -> String {
	let rendered = pattern
		.replace("{title}", title)
		.replace("{siteName}", site_name)
		.replace("{sectionName}", section_name);
	rendered.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The robots of an entry with the values of its SEO field.
fn robots(field: &Value, levels: &[Level], globals: &Globals) -> String {
	text(&field["robots"])
		.or_else(|| first(levels, |d| d.robots.as_ref()))
		.unwrap_or_else(|| globals.robots.clone())
}

/// Whether an entry belongs into the sitemap: indexed and not pointing to
/// another canonical url.
pub fn in_sitemap(
	schema: &Schema,
	entry: &Entry,
	url: &str,
	globals: &Globals,
) -> bool {
	let field = field_data(schema, entry);
	let canonical = text(&field["canonicalUrl"]);
	if canonical.is_some_and(|c| c != url) {
		return false;
	}
	let levels = defaults(schema, globals, entry);
	levels.iter().find_map(|l| l.sitemap) != Some(false)
		&& !robots(&field, &levels, globals).contains("noindex")
}

/// The global settings of a site, `fields` loads those of `seoSettings`
/// in a site: its own and the primary site's.
pub fn load_globals<E>(
	schema: &Schema,
	site: &Site,
	fields: impl Fn(&str) -> Result<Value, E>,
) -> Result<Globals, E> {
	let primary = &schema.primary_site().handle;
	Ok(globals(site, &fields(&site.handle)?, &fields(primary)?))
}

/// The hreflang alternates of a page from its url in the sites, with
/// `x-default` the primary site's; none without another language.
pub fn alternates<'a>(
	schema: &Schema,
	urls: impl IntoIterator<Item = (&'a Site, String)>,
) -> Vec<(String, String)> {
	let primary = &schema.primary_site().handle;
	let mut out = vec![];
	for (site, url) in urls {
		out.push((site.language.clone(), url.clone()));
		if &site.handle == primary {
			out.push(("x-default".into(), url));
		}
	}
	if out.len() <= 2 {
		out.clear();
	}
	out
}

/// An entry's page: what only an entry has.
pub struct EntryPage<'a> {
	pub entry: &'a Entry,
	pub url: Option<String>,
	/// (hreflang, url), with `x-default`
	pub alternates: Vec<(String, String)>,
	/// (name, url) from the home page to the entry
	pub breadcrumbs: Vec<(String, String)>,
}

/// What [`resolve`] needs, loaded by the caller.
pub struct Page<'a> {
	pub schema: &'a Schema,
	pub site: &'a Site,
	pub site_url: String,
	/// put before asset urls
	pub public_url: &'a str,
	pub globals: Globals,
	pub entry: Option<EntryPage<'a>>,
	pub asset: &'a dyn Fn(i64) -> Option<Asset>,
	/// outside production everything is noindex
	pub indexing: bool,
}

/// What an entry has without its own SEO field values, the placeholders
/// of the studio: the title is the pattern around `{title}`.
pub fn defaults_for_entry(page: &Page) -> Option<Value> {
	let e = page.entry.as_ref()?;
	let entry = e.entry;
	let v = Values::of_entry(page, e, &Value::Null);
	let defaults = defaults(page.schema, &page.globals, entry);
	let pattern = title_pattern(&defaults, &page.globals);
	let section = section_name(page.schema, entry);
	let marker = "\u{1}";
	let rendered =
		render_title(&pattern, marker, &page.globals.site_name, &section);
	let (prefix, suffix) = rendered.split_once(marker).unwrap_or(("", ""));

	Some(json!({
		"title": mapped_title(&defaults, entry),
		"titlePrefix": prefix,
		"titleSuffix": suffix,
		"description": v.description,
		"image": v.image.map(|a| image_data(page, &a, true)),
		"robots": v.robots,
		"canonicalUrl": e.url,
		"url": e.url,
		"siteName": page.globals.site_name,
	}))
}

/// What a page shows, each from the first place that has it.
struct Values {
	title: String,
	description: Option<String>,
	image: Option<Asset>,
	social_title: String,
	social_description: Option<String>,
	social_image: Option<Asset>,
	url: Option<String>,
	canonical: Option<String>,
	robots: String,
}

impl Values {
	/// An entry's: `field` are the values of its SEO field (null for the
	/// placeholders), then the fields the code maps, the levels, the
	/// globals.
	fn of_entry(page: &Page, e: &EntryPage, field: &Value) -> Self {
		let (g, entry) = (&page.globals, e.entry);
		let defaults = defaults(page.schema, g, entry);
		let title = render_title(
			&title_pattern(&defaults, g),
			&text(&field["title"])
				.unwrap_or_else(|| mapped_title(&defaults, entry)),
			&g.site_name,
			&section_name(page.schema, entry),
		);
		let description = text(&field["description"])
			.or_else(|| mapped_description(page.schema, &defaults, entry))
			.or_else(|| first(&defaults, |d| d.description.as_ref()))
			.or(g.description.clone());
		let image = first_id(&field["image"])
			.or_else(|| mapped_image(&defaults, entry))
			.or_else(|| defaults.iter().find_map(|d| d.image))
			.or(g.image)
			.and_then(|id| (page.asset)(id));
		let social_image = first_id(&field["socialImage"])
			.and_then(|id| (page.asset)(id))
			.or(image.clone());
		Values {
			social_title: text(&field["socialTitle"]).unwrap_or(title.clone()),
			social_description: text(&field["socialDescription"])
				.or(description.clone()),
			social_image,
			title,
			description,
			image,
			url: e.url.clone(),
			canonical: text(&field["canonicalUrl"]).or(e.url.clone()),
			robots: robots(field, &defaults, g),
		}
	}

	/// A page without an entry: the site's.
	fn of_site(page: &Page) -> Self {
		let g = &page.globals;
		let image = g.image.and_then(|id| (page.asset)(id));
		let url = Some(page.site_url.clone());
		Values {
			title: g.site_name.clone(),
			description: g.description.clone(),
			image: image.clone(),
			social_title: g.site_name.clone(),
			social_description: g.description.clone(),
			social_image: image,
			url: url.clone(),
			canonical: url,
			robots: g.robots.clone(),
		}
	}
}

fn title_pattern(levels: &[Level], globals: &Globals) -> String {
	first(levels, |d| d.title_pattern.as_ref())
		.unwrap_or_else(|| globals.title_pattern.clone())
}

fn section_name(schema: &Schema, entry: &Entry) -> String {
	schema
		.get_section(&entry.section)
		.map(|s| s.name.clone())
		.unwrap_or_default()
}

fn mapped_description(
	schema: &Schema,
	defaults: &[Level],
	entry: &Entry,
) -> Option<String> {
	let path = first(defaults, |d| d.description_from.as_ref())?;
	let value = at(&entry.fields, &path);
	// markdown as its text, not its marks
	let markdown = schema
		.get(&entry.ty)
		.and_then(|c| schema.field_at(c, &path))
		.is_some_and(|f| f.kind == FieldKind::Markdown);
	let text = text_of(value, markdown);
	(!text.is_empty()).then(|| truncate(&text, DESCRIPTION_LENGTH))
}

/// The text of a text, markdown or rich text value on one line.
pub(crate) fn text_of(value: &Value, markdown: bool) -> String {
	let text = match (markdown, value.as_str()) {
		(true, Some(md)) => markdown_text(md),
		(false, Some(t)) => t.into(),
		// a button's label is no description
		_ => richtext::plain_text(value, false),
	};
	text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The text of the field the title comes from, else the entry's title.
fn mapped_title(defaults: &[Level], entry: &Entry) -> String {
	first(defaults, |d| d.title_from.as_ref())
		.and_then(|path| text(at(&entry.fields, &path)))
		.unwrap_or_else(|| entry.title.clone())
}

fn mapped_image(defaults: &[Level], entry: &Entry) -> Option<i64> {
	let path = first(defaults, |d| d.image_from.as_ref())?;
	first_id(at(&entry.fields, &path))
}

/// The resolved SEO as the plugin's `seoJson`.
pub fn resolve(page: &Page) -> Value {
	let mut v = match &page.entry {
		Some(e) => {
			let field = field_data(page.schema, e.entry);
			Values::of_entry(page, e, &field)
		}
		None => Values::of_site(page),
	};
	if !page.indexing {
		v.robots = "noindex, nofollow".into();
	}
	let locale = page.site.language.replace('-', "_");
	let alternates =
		page.entry.as_ref().map_or(vec![], |e| e.alternates.clone());
	let social_image =
		v.social_image.as_ref().map(|a| image_data(page, a, true));

	// links
	let mut links = vec![];
	if let Some(c) = &v.canonical {
		links.push(json!({ "rel": "canonical", "href": c, "hreflang": null }));
	}
	for (lang, href) in &alternates {
		links.push(
			json!({ "rel": "alternate", "href": href, "hreflang": lang }),
		);
	}

	let json_ld = json_ld(
		page,
		&v.title,
		v.description.as_deref(),
		v.image.as_ref(),
		v.url.as_deref(),
	);

	json!({
		"title": v.title,
		"description": v.description,
		"canonical": v.canonical,
		"robots": v.robots,
		"siteName": page.globals.site_name,
		"locale": locale,
		"image": v.image.as_ref().map(|a| image_data(page, a, true)),
		"socialTitle": v.social_title,
		"socialDescription": v.social_description,
		"socialImage": social_image,
		"alternates": alternates.iter()
			.map(|(l, u)| json!({ "hreflang": l, "url": u }))
			.collect::<Vec<_>>(),
		"meta": meta(page, &v, &alternates, social_image.as_ref()),
		"links": links,
		"jsonLd": script_json(&json_ld),
	})
}

/// The meta tags, of what has a value.
fn meta(
	page: &Page,
	v: &Values,
	alternates: &[(String, String)],
	social_image: Option<&Value>,
) -> Vec<Value> {
	let g = &page.globals;
	let mut meta = vec![];
	let mut add = |name: Option<&str>,
	               property: Option<&str>,
	               content: Option<String>| {
		if let Some(content) = content.filter(|c| !c.is_empty()) {
			meta.push(
				json!({ "name": name, "property": property, "content": content }),
			);
		}
	};
	add(Some("description"), None, v.description.clone());
	add(Some("robots"), None, Some(v.robots.clone()));
	add(None, Some("og:type"), Some("website".into()));
	add(None, Some("og:site_name"), Some(g.site_name.clone()));
	add(None, Some("og:title"), Some(v.social_title.clone()));
	add(None, Some("og:description"), v.social_description.clone());
	add(None, Some("og:url"), v.canonical.clone());
	add(
		None,
		Some("og:locale"),
		Some(page.site.language.replace('-', "_")),
	);
	for (lang, _) in alternates {
		if lang != "x-default" && *lang != page.site.language {
			add(
				None,
				Some("og:locale:alternate"),
				Some(lang.replace('-', "_")),
			);
		}
	}
	if let Some(img) = social_image {
		let s = |k: &str| {
			img[k]
				.as_str()
				.map(String::from)
				.or(img[k].as_u64().map(|n| n.to_string()))
		};
		add(None, Some("og:image"), s("url"));
		add(None, Some("og:image:width"), s("width"));
		add(None, Some("og:image:height"), s("height"));
		add(None, Some("og:image:alt"), s("alt"));
	}
	add(
		Some("twitter:card"),
		None,
		Some(
			if social_image.is_some() {
				"summary_large_image"
			} else {
				"summary"
			}
			.into(),
		),
	);
	if let Some(handle) = &g.twitter_handle {
		add(
			Some("twitter:site"),
			None,
			Some(format!("@{}", handle.trim_start_matches('@'))),
		);
	}
	meta
}

/// An image with its absolute url, the social size if `crop`.
pub(crate) fn image_data(page: &Page, a: &Asset, crop: bool) -> Value {
	let base = page.public_url.trim_end_matches('/');
	let (url, size) = match (crop && a.kind == "image", a.width.zip(a.height)) {
		(true, Some((w, h))) => (
			a.url(Some(SOCIAL_TRANSFORM)),
			Some(transform_size(w, h, &social_transform())),
		),
		_ => (a.url(None), a.width.zip(a.height)),
	};
	let primary = &page.schema.primary_site().handle;
	let alt = a.alt_in(&page.site.handle, primary);
	json!({
		"url": format!("{base}{url}"),
		"width": size.map(|s| s.0),
		"height": size.map(|s| s.1),
		"alt": match alt {
			"" => a.filename.as_str(),
			alt => alt,
		},
		"mimeType": a.mime,
	})
}
