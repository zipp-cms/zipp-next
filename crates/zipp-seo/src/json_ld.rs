//! The JSON-LD of a page: the organization of the settings, the website,
//! the page and its breadcrumbs as one schema.org graph.

use serde_json::{Map, Value, json};

use crate::ORGANIZATION_TYPES;
use crate::resolve::{EntryPage, Page, first_id, image_data, is_empty, text};
use zipp::assets::Asset;
use zipp::db::iso;

/// JSON for a `<script>` of the page: a title with `</script>` can't end
/// it. The escapes are JSON's own, it reads the same.
pub(crate) fn script_json(v: &Value) -> String {
	serde_json::to_string(v)
		.unwrap()
		.replace('<', "\\u003c")
		.replace('>', "\\u003e")
		.replace('&', "\\u0026")
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
				.filter(|(_, v)| !is_empty(v))
				.collect::<Map<_, _>>(),
		),
		v => v,
	}
}

/// Organization, WebSite, WebPage and BreadcrumbList.
pub(crate) fn json_ld(
	page: &Page,
	title: &str,
	description: Option<&str>,
	image: Option<&Asset>,
	url: Option<&str>,
) -> Value {
	let base = page.site_url.trim_end_matches('/');
	let mut graph = vec![];

	let org = organization(page, base);
	let org_id = org.as_ref().map(|o| o["@id"].clone());
	graph.extend(org);
	graph.push(website(page, base, org_id));

	if let (Some(e), Some(url)) = (&page.entry, url) {
		graph.push(web_page(page, e, base, url, title, description, image));
		if !e.breadcrumbs.is_empty() {
			graph.push(breadcrumb_list(url, &e.breadcrumbs));
		}
	}

	json!({ "@context": "https://schema.org", "@graph": graph })
}

/// The organization (or person) of the settings, none without a name.
fn organization(page: &Page, base: &str) -> Option<Value> {
	let org = &page.globals.organization;
	let s = |k: &str| text_value(&org[k]);
	let asset = |k: &str| first_id(&org[k]).and_then(|id| (page.asset)(id));
	let name = text(&org["name"])?;

	let ty = text(&org["type"]).unwrap_or_else(|| "Organization".into());
	let person = ty == "Person";
	let local = ORGANIZATION_TYPES.iter().any(|(t, l)| *t == ty && *l);
	let address = without_nulls(json!({
		"@type": "PostalAddress",
		"streetAddress": s("street"),
		"postalCode": s("postalCode"),
		"addressLocality": s("locality"),
		"addressRegion": s("region"),
		"addressCountry": s("country"),
	}));
	// more than its type
	let address = match address.as_object().is_some_and(|a| a.len() > 1) {
		true => address,
		false => Value::Null,
	};
	let geo = match (local, org["latitude"].as_f64(), org["longitude"].as_f64())
	{
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

	Some(without_nulls(json!({
		"@type": ty,
		"@id": format!("{base}#organization"),
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
	})))
}

/// The website, published by the organization and made by the creator
/// of the settings.
fn website(page: &Page, base: &str, org_id: Option<Value>) -> Value {
	let org = &page.globals.organization;
	let creator = text(&org["creatorName"]).map(|name| {
		without_nulls(json!({
			"@type": text(&org["creatorType"]).unwrap_or_else(|| "Organization".into()),
			"name": name,
			"url": text_value(&org["creatorUrl"]),
		}))
	});
	without_nulls(json!({
		"@type": "WebSite",
		"@id": format!("{base}#website"),
		"url": base,
		"name": page.globals.site_name,
		"inLanguage": page.site.language,
		"publisher": org_id.map(|id| json!({ "@id": id })),
		"creator": creator,
	}))
}

/// The page of an entry, part of the website.
fn web_page(
	page: &Page,
	e: &EntryPage,
	base: &str,
	url: &str,
	title: &str,
	description: Option<&str>,
	image: Option<&Asset>,
) -> Value {
	let crumbs = &e.breadcrumbs;
	without_nulls(json!({
		"@type": "WebPage",
		"@id": format!("{url}#webpage"),
		"url": url,
		"name": title,
		"description": description,
		"inLanguage": page.site.language,
		"isPartOf": { "@id": format!("{base}#website") },
		"primaryImageOfPage": image_object(page, image),
		"datePublished": iso(e.entry.dates.post_date.unwrap_or(e.entry.created_at)),
		"dateModified": iso(e.entry.updated_at),
		"breadcrumb": if crumbs.is_empty() { Value::Null } else { json!({ "@id": format!("{url}#breadcrumb") }) },
	}))
}

/// The breadcrumbs of a page, (name, url) from the home page on.
fn breadcrumb_list(url: &str, crumbs: &[(String, String)]) -> Value {
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
	json!({
		"@type": "BreadcrumbList",
		"@id": format!("{url}#breadcrumb"),
		"itemListElement": items,
	})
}

/// A text of the settings, null when there is none.
fn text_value(v: &Value) -> Value {
	text(v).map(Value::from).unwrap_or(Value::Null)
}
