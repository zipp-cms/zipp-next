//! The JSON-LD of a page: the organization of the settings, the website,
//! the page and its breadcrumbs as one schema.org graph.

use serde_json::{Map, Value, json};

use crate::ORGANIZATION_TYPES;
use crate::resolve::{Page, first_id, image_data, text};
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
				.filter(|(_, v)| !v.is_null() && v != "" && *v != json!([]))
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
			"datePublished": iso(e.entry.dates.post_date.unwrap_or(e.entry.created_at)),
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
