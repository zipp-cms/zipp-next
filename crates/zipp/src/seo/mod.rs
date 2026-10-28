//! Resolves the SEO of a page like the Craft plugin meierlabs-seo: the
//! title, meta tags, links and JSON-LD a crelte frontend renders with the
//! npm package `meierlabs-seo`. Every value is taken from the first place
//! that has one: the entry's SEO field, its entry type, its section, the
//! global settings. The data is loaded by the caller (GraphQL, with the
//! drafts of a preview), this only computes.

use serde_json::{Map, Value, json};

use crate::assets::{Asset, transform_size};
use crate::entries::Entry;
use crate::schema::seo::{ORGANIZATION_TYPES, SEO_SETTINGS, SeoDefaults};
use crate::schema::{FieldKind, Schema, Site, Transform};

pub mod routes;
pub mod sitemap;

pub const DEFAULT_TITLE_PATTERN: &str = "{title} | {siteName}";
pub const DESCRIPTION_LENGTH: usize = 160;
/// The social image (og:image), cropped around the focal point.
pub const SOCIAL_TRANSFORM: &str = "_social";

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
}

fn text(v: &Value) -> Option<String> {
	v.as_str()
		.map(str::trim)
		.filter(|s| !s.is_empty())
		.map(Into::into)
}

fn first_id(v: &Value) -> Option<i64> {
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
	}
}

/// The defaults of an entry, the entry type first.
pub fn defaults<'a>(schema: &'a Schema, entry: &Entry) -> Vec<&'a SeoDefaults> {
	let Some(seo) = &schema.seo else {
		return vec![];
	};
	[seo.types.get(&entry.ty), seo.sections.get(&entry.section)]
		.into_iter()
		.flatten()
		.collect()
}

fn first<'a>(
	defaults: &[&'a SeoDefaults],
	f: impl Fn(&'a SeoDefaults) -> Option<&'a String>,
) -> Option<String> {
	defaults.iter().find_map(|d| f(d)).cloned()
}

/// The value of the entry's SEO field, if its type has one.
pub fn field_data(schema: &Schema, entry: &Entry) -> Value {
	let comp = schema.get(&entry.ty);
	let field =
		comp.and_then(|c| c.fields.iter().find(|f| f.kind == FieldKind::Seo));
	field.map_or(Value::Null, |f| entry.fields[&f.handle].clone())
}

/// A value of the fields at a path like `pageIntro.text`.
fn at<'a>(fields: &'a Value, path: &str) -> &'a Value {
	path.split('.').fold(fields, |v, key| &v[key])
}

/// The text of html: tags out, entities decoded, the words of blocks
/// kept apart (`<p>a</p><p>b</p>` is `a b`, `<em>a</em>,` is `a,`).
fn html_text(html: &str) -> String {
	let mut out = String::new();
	let mut rest = html;
	while let Some(start) = rest.find('<') {
		out.push_str(&rest[..start]);
		let Some(end) = rest[start..].find('>') else {
			break;
		};
		let tag = rest[start + 1..start + end].trim_start_matches('/');
		let name = tag
			.chars()
			.take_while(|c| c.is_ascii_alphanumeric())
			.collect::<String>()
			.to_ascii_lowercase();
		let block = matches!(
			name.as_str(),
			"p" | "div" | "li" | "br" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
		);
		if block {
			out.push(' ');
		}
		rest = &rest[start + end + 1..];
	}
	out.push_str(rest);
	out.replace("&lt;", "<")
		.replace("&gt;", ">")
		.replace("&quot;", "\"")
		.replace("&#39;", "'")
		.replace("&nbsp;", " ")
		.replace("&amp;", "&")
}

/// The text of a text, markdown or rich text value.
pub fn plain_text(v: &Value) -> String {
	fn walk(v: &Value, out: &mut String) {
		if let Some(t) = v["text"].as_str() {
			out.push_str(t);
		}
		// buttons and embedded blocks are no text
		let ty = v["type"].as_str().unwrap_or_default();
		if ty == "buttons" || ty == "embed" {
			return;
		}
		for c in v["content"].as_array().into_iter().flatten() {
			walk(c, out);
		}
		if matches!(ty, "paragraph" | "heading" | "listItem") {
			out.push(' ');
		}
	}
	let mut out = String::new();
	match v {
		Value::String(s) => out = html_text(s),
		Value::Object(_) => walk(v, &mut out),
		_ => {}
	}
	out.split_whitespace().collect::<Vec<_>>().join(" ")
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

/// The robots of an entry, also whether it is in the sitemap.
pub fn robots(
	schema: &Schema,
	entry: &Entry,
	globals: &Globals,
	indexing: bool,
) -> String {
	if !indexing {
		return "noindex, nofollow".into();
	}
	let field = field_data(schema, entry);
	text(&field["robots"])
		.or_else(|| first(&defaults(schema, entry), |d| d.robots.as_ref()))
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
	let canonical = text(&field_data(schema, entry)["canonicalUrl"]);
	if canonical.is_some_and(|c| c != url) {
		return false;
	}
	!robots(schema, entry, globals, true).contains("noindex")
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
	let defaults = defaults(page.schema, entry);
	let pattern = first(&defaults, |d| d.title_pattern.as_ref())
		.unwrap_or_else(|| page.globals.title_pattern.clone());
	let section = section_name(page.schema, entry);
	let marker = "\u{1}";
	let rendered =
		render_title(&pattern, marker, &page.globals.site_name, &section);
	let (prefix, suffix) = rendered.split_once(marker).unwrap_or(("", ""));
	let image = mapped_image(page.schema, entry)
		.or(page.globals.image)
		.and_then(|id| (page.asset)(id));

	Some(json!({
		"title": entry.title,
		"titlePrefix": prefix,
		"titleSuffix": suffix,
		"description": mapped_description(page.schema, entry)
			.or_else(|| first(&defaults, |d| d.description.as_ref()))
			.or(page.globals.description.clone()),
		"image": image.map(|a| image_data(page, &a, true)),
		"robots": first(&defaults, |d| d.robots.as_ref())
			.unwrap_or(page.globals.robots.clone()),
		"canonicalUrl": e.url,
		"url": e.url,
		"siteName": page.globals.site_name,
	}))
}

fn section_name(schema: &Schema, entry: &Entry) -> String {
	schema
		.get_section(&entry.section)
		.map(|s| s.name.clone())
		.unwrap_or_default()
}

fn mapped_description(schema: &Schema, entry: &Entry) -> Option<String> {
	let path =
		first(&defaults(schema, entry), |d| d.description_from.as_ref())?;
	let value = at(&entry.fields, &path);
	// markdown as its text, not its marks
	let markdown = schema
		.get(&entry.ty)
		.and_then(|c| schema.field_at(c, &path))
		.is_some_and(|f| f.kind == FieldKind::Markdown);
	let text = match (markdown, value.as_str()) {
		(true, Some(md)) => {
			plain_text(&Value::from(crate::graphql::markdown_html(md)))
		}
		// plain text has no tags to take out
		(false, Some(t)) => t.split_whitespace().collect::<Vec<_>>().join(" "),
		_ => plain_text(value),
	};
	(!text.is_empty()).then(|| truncate(&text, DESCRIPTION_LENGTH))
}

fn mapped_image(schema: &Schema, entry: &Entry) -> Option<i64> {
	let path = first(&defaults(schema, entry), |d| d.image_from.as_ref())?;
	first_id(at(&entry.fields, &path))
}

/// The resolved SEO as the plugin's `seoJson`.
pub fn resolve(page: &Page) -> Value {
	let g = &page.globals;
	let asset = |id: Option<i64>| id.and_then(|id| (page.asset)(id));

	let (title, description, image, social, url, canonical, robots) =
		match &page.entry {
			Some(e) => {
				let entry = e.entry;
				let field = field_data(page.schema, entry);
				let defaults = defaults(page.schema, entry);
				let pattern = first(&defaults, |d| d.title_pattern.as_ref())
					.unwrap_or_else(|| g.title_pattern.clone());
				let title = render_title(
					&pattern,
					&text(&field["title"])
						.unwrap_or_else(|| entry.title.clone()),
					&g.site_name,
					&section_name(page.schema, entry),
				);
				let description = text(&field["description"])
					.or_else(|| mapped_description(page.schema, entry))
					.or_else(|| first(&defaults, |d| d.description.as_ref()))
					.or(g.description.clone());
				let image = asset(
					first_id(&field["image"])
						.or_else(|| mapped_image(page.schema, entry))
						.or(g.image),
				);
				let social = (
					text(&field["socialTitle"]).unwrap_or(title.clone()),
					text(&field["socialDescription"]).or(description.clone()),
					asset(first_id(&field["socialImage"])).or(image.clone()),
				);
				let canonical = text(&field["canonicalUrl"]).or(e.url.clone());
				let robots = robots(page.schema, entry, g, page.indexing);
				(
					title,
					description,
					image,
					social,
					e.url.clone(),
					canonical,
					robots,
				)
			}
			None => {
				let image = asset(g.image);
				let social =
					(g.site_name.clone(), g.description.clone(), image.clone());
				let robots = match page.indexing {
					true => g.robots.clone(),
					false => "noindex, nofollow".into(),
				};
				let url = Some(page.site_url.clone());
				(
					g.site_name.clone(),
					g.description.clone(),
					image,
					social,
					url.clone(),
					url,
					robots,
				)
			}
		};
	let (social_title, social_description, social_image) = social;
	let locale = page.site.language.replace('-', "_");
	let alternates =
		page.entry.as_ref().map_or(vec![], |e| e.alternates.clone());
	let image_json = image.as_ref().map(|a| image_data(page, a, true));
	let social_json = social_image.as_ref().map(|a| image_data(page, a, true));

	// meta tags
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
	add(Some("description"), None, description.clone());
	add(Some("robots"), None, Some(robots.clone()));
	add(None, Some("og:type"), Some("website".into()));
	add(None, Some("og:site_name"), Some(g.site_name.clone()));
	add(None, Some("og:title"), Some(social_title.clone()));
	add(None, Some("og:description"), social_description.clone());
	add(None, Some("og:url"), canonical.clone());
	add(None, Some("og:locale"), Some(locale.clone()));
	for (lang, _) in &alternates {
		if lang != "x-default" && *lang != page.site.language {
			add(
				None,
				Some("og:locale:alternate"),
				Some(lang.replace('-', "_")),
			);
		}
	}
	if let Some(img) = &social_json {
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
			if social_json.is_some() {
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

	// links
	let mut links = vec![];
	if let Some(c) = &canonical {
		links.push(json!({ "rel": "canonical", "href": c, "hreflang": null }));
	}
	for (lang, href) in &alternates {
		links.push(
			json!({ "rel": "alternate", "href": href, "hreflang": lang }),
		);
	}

	let json_ld = json_ld(
		page,
		&title,
		description.as_deref(),
		image.as_ref(),
		url.as_deref(),
	);

	json!({
		"title": title,
		"description": description,
		"canonical": canonical,
		"robots": robots,
		"siteName": g.site_name,
		"locale": locale,
		"image": image_json,
		"socialTitle": social_title,
		"socialDescription": social_description,
		"socialImage": social_json,
		"alternates": alternates.iter()
			.map(|(l, u)| json!({ "hreflang": l, "url": u }))
			.collect::<Vec<_>>(),
		"meta": meta,
		"links": links,
		"jsonLd": serde_json::to_string(&json_ld).unwrap(),
	})
}

/// An image with its absolute url, the social size if `crop`.
fn image_data(page: &Page, a: &Asset, crop: bool) -> Value {
	let base = page.public_url.trim_end_matches('/');
	let (url, size) = match (crop && a.kind == "image", a.width.zip(a.height)) {
		(true, Some((w, h))) => (
			a.url(Some(SOCIAL_TRANSFORM)),
			Some(transform_size(w, h, &social_transform())),
		),
		_ => (a.url(None), a.width.zip(a.height)),
	};
	json!({
		"url": format!("{base}{url}"),
		"width": size.map(|s| s.0),
		"height": size.map(|s| s.1),
		"alt": if a.alt.is_empty() { a.filename.clone() } else { a.alt.clone() },
		"mimeType": a.mime,
	})
}

fn image_object(page: &Page, a: Option<&Asset>) -> Option<Value> {
	let d = image_data(page, a?, false);
	Some(without_nulls(json!({
		"@type": "ImageObject",
		"url": d["url"],
		"width": d["width"],
		"height": d["height"],
		"caption": d["alt"],
	})))
}

fn without_nulls(v: Value) -> Value {
	match v {
		Value::Object(o) => Value::Object(
			o.into_iter()
				.filter(|(_, v)| !v.is_null() && v != "" && *v != json!([]))
				.collect::<Map<_, _>>(),
		),
		v => v,
	}
}

/// Organization, WebSite, WebPage and BreadcrumbList.
fn json_ld(
	page: &Page,
	title: &str,
	description: Option<&str>,
	image: Option<&Asset>,
	url: Option<&str>,
) -> Value {
	let base = page.site_url.trim_end_matches('/');
	let org = &page.globals.organization;
	let s = |k: &str| text(&org[k]).map(Value::from).unwrap_or(Value::Null);
	let asset = |k: &str| first_id(&org[k]).and_then(|id| (page.asset)(id));
	let mut graph = vec![];

	let mut org_id = None;
	if let Some(name) = text(&org["name"]) {
		let ty = text(&org["type"]).unwrap_or_else(|| "Organization".into());
		let person = ty == "Person";
		let local = ORGANIZATION_TYPES.iter().any(|(t, l)| *t == ty && *l);
		let address = without_nulls(json!({
			"streetAddress": s("street"),
			"postalCode": s("postalCode"),
			"addressLocality": s("locality"),
			"addressRegion": s("region"),
			"addressCountry": s("country"),
		}));
		let address = match address.as_object().is_some_and(|a| !a.is_empty()) {
			true => {
				let mut a = json!({ "@type": "PostalAddress" });
				a.as_object_mut()
					.unwrap()
					.extend(address.as_object().unwrap().clone());
				a
			}
			false => Value::Null,
		};
		let geo = match (
			local,
			org["latitude"].as_f64(),
			org["longitude"].as_f64(),
		) {
			(true, Some(lat), Some(lon)) => {
				json!({ "@type": "GeoCoordinates", "latitude": lat, "longitude": lon })
			}
			_ => Value::Null,
		};
		let hours: Vec<Value> = org["openingHours"]
			.as_array()
			.into_iter()
			.flatten()
			.map(|b| &b["fields"])
			.map(|f| {
				json!({
					"@type": "OpeningHoursSpecification",
					"dayOfWeek": f["day"],
					"opens": f["opens"],
					"closes": f["closes"],
				})
			})
			.collect();
		let same_as: Vec<&str> = org["sameAs"]
			.as_str()
			.unwrap_or_default()
			.lines()
			.map(str::trim)
			.filter(|l| !l.is_empty())
			.collect();
		let id = format!("{base}#organization");
		graph.push(without_nulls(json!({
			"@type": ty,
			"@id": id,
			"name": name,
			"alternateName": s("alternateName"),
			"legalName": if person { Value::Null } else { s("legalName") },
			"description": s("description"),
			"url": text(&org["url"]).unwrap_or_else(|| base.to_string()),
			"logo": if person { None } else { image_object(page, asset("logo").as_ref()) },
			"image": image_object(page, asset("image").as_ref()),
			"email": s("email"),
			"telephone": s("telephone"),
			"address": address,
			"geo": geo,
			"openingHoursSpecification": if local && !hours.is_empty() { json!(hours) } else { Value::Null },
			"priceRange": if local { s("priceRange") } else { Value::Null },
			"sameAs": same_as,
		})));
		org_id = Some(id);
	}

	let creator = text(&org["creatorName"]).map(|name| {
		without_nulls(json!({
			"@type": text(&org["creatorType"]).unwrap_or_else(|| "Organization".into()),
			"name": name,
			"url": s("creatorUrl"),
		}))
	});
	let website_id = format!("{base}#website");
	graph.push(without_nulls(json!({
		"@type": "WebSite",
		"@id": website_id,
		"url": base,
		"name": page.globals.site_name,
		"inLanguage": page.site.language,
		"publisher": org_id.map(|id| json!({ "@id": id })),
		"creator": creator,
	})));

	if let (Some(e), Some(url)) = (&page.entry, url) {
		let crumbs = &e.breadcrumbs;
		graph.push(without_nulls(json!({
			"@type": "WebPage",
			"@id": format!("{url}#webpage"),
			"url": url,
			"name": title,
			"description": description,
			"inLanguage": page.site.language,
			"isPartOf": { "@id": website_id },
			"primaryImageOfPage": image_object(page, image),
			"datePublished": iso(e.entry.created_at),
			"dateModified": iso(e.entry.updated_at),
			"breadcrumb": if crumbs.is_empty() { Value::Null } else { json!({ "@id": format!("{url}#breadcrumb") }) },
		})));
		if !crumbs.is_empty() {
			let items: Vec<Value> = crumbs
				.iter()
				.enumerate()
				.map(|(i, (name, item))| {
					json!({
						"@type": "ListItem",
						"position": i + 1,
						"name": name,
						"item": item,
					})
				})
				.collect();
			graph.push(json!({
				"@type": "BreadcrumbList",
				"@id": format!("{url}#breadcrumb"),
				"itemListElement": items,
			}));
		}
	}

	json!({ "@context": "https://schema.org", "@graph": graph })
}

pub use crate::db::iso;

/// Whether the schema has SEO, then [`SEO_SETTINGS`] exists.
pub fn enabled(schema: &Schema) -> bool {
	schema.seo.is_some() && schema.get(SEO_SETTINGS).is_some()
}

#[cfg(test)]
mod tests;
