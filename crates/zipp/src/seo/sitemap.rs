//! Sitemaps and robots.txt under the paths of the Craft plugin's actions,
//! so `seoRoutes` of the npm package `meierlabs-seo` proxies them from
//! the frontend unchanged: `/sitemap.xml`, `/sitemap-<site>-<section>-
//! <page>.xml` and `/robots.txt` of the frontend's domain.

use axum::Router;
use axum::extract::{Query as Params, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use rusqlite::Connection;
use serde::Deserialize;

use super::{Globals, globals, in_sitemap};
use crate::AppState;
use crate::entries::{Entry, list};
use crate::error::{Error, Result};
use crate::query::Query;
use crate::schema::seo::SEO_SETTINGS;
use crate::schema::{Schema, Site};

/// The published entries of a site, `None` filters nothing more.
fn entries(
	conn: &Connection,
	schema: &Schema,
	site: &str,
	extra: &[(&str, String)],
	limit: i64,
	offset: i64,
) -> Result<Vec<Entry>> {
	let mut params = vec![("site".to_string(), site.to_string())];
	params.extend(extra.iter().map(|(k, v)| (k.to_string(), v.clone())));
	let mut q = Query::parse(schema, &params).map_err(Error::Request)?;
	q.published_only = true;
	q.limit = limit;
	q.offset = offset;
	Ok(list(conn, &q)?.0)
}

/// The global settings of a site as published.
pub fn load_globals(
	conn: &Connection,
	schema: &Schema,
	site: &Site,
) -> Result<Globals> {
	let fields = |s: &str| -> Result<serde_json::Value> {
		let section = [("section", SEO_SETTINGS.to_string())];
		let e = entries(conn, schema, s, &section, 1, 0)?;
		Ok(e.into_iter().next().map(|e| e.fields).unwrap_or_default())
	};
	let primary = schema.primary_site().handle.clone();
	Ok(globals(site, &fields(&site.handle)?, &fields(&primary)?))
}

fn origin(url: &str) -> &str {
	let after = url.find("://").map_or(0, |i| i + 3);
	match url[after..].find('/') {
		Some(i) => &url[..after + i],
		None => url,
	}
}

/// The sites on the frontend's domain, all if none is.
fn sites_for<'a>(state: &'a AppState, origin_: &str) -> Vec<&'a Site> {
	let with_url: Vec<&Site> = state
		.schema
		.sites
		.iter()
		.filter(|s| state.site_urls.contains_key(&s.handle))
		.filter(|s| !state.schema.is_offline(&s.handle))
		.collect();
	let matching: Vec<&Site> = with_url
		.iter()
		.copied()
		.filter(|s| {
			origin(&state.site_urls[&s.handle]) == origin_.trim_end_matches('/')
		})
		.collect();
	if matching.is_empty() {
		with_url
	} else {
		matching
	}
}

/// Sections with urls and the sitemap on.
fn sections(schema: &Schema) -> Vec<&str> {
	let seo = schema.seo.as_ref();
	schema
		.sections
		.iter()
		.filter(|s| s.uri.is_some() || !s.type_uris.is_empty())
		.filter(|s| {
			seo.and_then(|seo| seo.sections.get(&s.handle))
				.is_none_or(|d| d.sitemap)
		})
		.map(|s| s.handle.as_str())
		.collect()
}

fn page_size(schema: &Schema) -> i64 {
	schema.seo.as_ref().map_or(2000, |s| s.sitemap_page_size) as i64
}

fn escape(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
		.replace('\'', "&apos;")
}

fn xml(body: String) -> Response {
	(
		[
			(header::CONTENT_TYPE, "application/xml; charset=utf-8"),
			(header::CACHE_CONTROL, "public, max-age=3600"),
		],
		body,
	)
		.into_response()
}

#[derive(Debug, Deserialize)]
struct OriginParam {
	origin: Option<String>,
}

/// The frontend's origin, the primary site's without one.
fn frontend(state: &AppState, p: &OriginParam) -> String {
	let given = p.origin.as_deref().map(|o| o.trim_end_matches('/'));
	let valid = given.filter(|o| {
		(o.starts_with("https://") || o.starts_with("http://"))
			&& origin(o) == *o
	});
	match valid {
		Some(o) => o.to_string(),
		None => {
			let primary = &state.schema.primary_site().handle;
			let url = state.site_urls.get(primary).cloned().unwrap_or_default();
			origin(&url).to_string()
		}
	}
}

async fn index(
	State(state): State<AppState>,
	Params(p): Params<OriginParam>,
) -> Result<Response> {
	let origin_ = frontend(&state, &p);
	let conn = state.db.conn();
	let size = page_size(&state.schema);
	let mut out = String::from(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		<sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n",
	);
	for site in sites_for(&state, &origin_) {
		for section in sections(&state.schema) {
			let all = entries(
				&conn,
				&state.schema,
				&site.handle,
				&[("section", section.into()), ("order", "updated".into())],
				1_000_000,
				0,
			)?;
			let with_uri: Vec<&Entry> =
				all.iter().filter(|e| e.uri.is_some()).collect();
			let pages = (with_uri.len() as i64 + size - 1) / size;
			let last = with_uri.iter().map(|e| e.updated_at).max();
			for page in 1..=pages {
				out += &format!(
					"\t<sitemap>\n\t\t<loc>{}</loc>\n",
					escape(&format!(
						"{origin_}/sitemap-{}-{section}-{page}.xml",
						site.handle
					))
				);
				if let Some(last) = last {
					out += &format!(
						"\t\t<lastmod>{}</lastmod>\n",
						super::iso(last)
					);
				}
				out += "\t</sitemap>\n";
			}
		}
	}
	out += "</sitemapindex>\n";
	Ok(xml(out))
}

#[derive(Debug, Deserialize)]
struct PageParams {
	site: String,
	section: String,
	page: i64,
}

async fn page(
	State(state): State<AppState>,
	Params(p): Params<PageParams>,
) -> Result<Response> {
	let schema = &state.schema;
	let Some(site) = schema
		.get_site(&p.site)
		.filter(|s| !schema.is_offline(&s.handle))
	else {
		return Ok(StatusCode::NOT_FOUND.into_response());
	};
	if p.page < 1 || !sections(schema).contains(&p.section.as_str()) {
		return Ok(StatusCode::NOT_FOUND.into_response());
	}
	let conn = state.db.conn();
	let size = page_size(schema);
	let section = [("section", p.section.clone()), ("order", "created".into())];
	let found: Vec<Entry> =
		entries(&conn, schema, &site.handle, &section, 1_000_000, 0)?
			.into_iter()
			.filter(|e| e.uri.is_some())
			.skip(((p.page - 1) * size) as usize)
			.take(size as usize)
			.collect();
	if found.is_empty() && p.page > 1 {
		return Ok(StatusCode::NOT_FOUND.into_response());
	}

	let globals = load_globals(&conn, schema, site)?;
	// the urls of the entries in every site, for hreflang
	let ids = found
		.iter()
		.map(|e| e.id.to_string())
		.collect::<Vec<_>>()
		.join(",");
	let mut others = vec![];
	for s in &schema.sites {
		if !ids.is_empty() {
			let list = entries(
				&conn,
				schema,
				&s.handle,
				&[("ids", ids.clone())],
				1_000_000,
				0,
			)?;
			others.extend(list.into_iter().map(|e| (s, e)));
		}
	}

	let mut out = String::from(
		"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
		<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" \
		xmlns:xhtml=\"http://www.w3.org/1999/xhtml\">\n",
	);
	let primary = &schema.primary_site().handle;
	for e in &found {
		let Some(url) = crate::page_url(
			&state.site_urls,
			&site.handle,
			e.uri.as_deref().unwrap(),
		) else {
			continue;
		};
		if !in_sitemap(schema, e, &url, &globals) {
			continue;
		}
		out += &format!("\t<url>\n\t\t<loc>{}</loc>\n", escape(&url));
		out +=
			&format!("\t\t<lastmod>{}</lastmod>\n", super::iso(e.updated_at));
		let mut alts = vec![];
		for (s, other) in others.iter().filter(|(_, o)| o.id == e.id) {
			let Some(u) = other.uri.as_deref().and_then(|uri| {
				crate::page_url(&state.site_urls, &s.handle, uri)
			}) else {
				continue;
			};
			alts.push((s.language.clone(), u.clone()));
			if &s.handle == primary {
				alts.push(("x-default".into(), u));
			}
		}
		if alts.len() > 2 {
			for (lang, href) in alts {
				out += &format!(
					"\t\t<xhtml:link rel=\"alternate\" hreflang=\"{}\" href=\"{}\"/>\n",
					escape(&lang),
					escape(&href)
				);
			}
		}
		out += "\t</url>\n";
	}
	out += "</urlset>\n";
	Ok(xml(out))
}

async fn robots(
	State(state): State<AppState>,
	Params(p): Params<OriginParam>,
) -> Result<Response> {
	let origin_ = frontend(&state, &p);
	let mut lines = vec![];
	if !state.indexing {
		lines.push("# not to be indexed: development or --noindex".to_string());
		lines.push("User-agent: *".into());
		lines.push("Disallow: /".into());
	} else {
		let sites = sites_for(&state, &origin_);
		let primary = state.schema.primary_site();
		let site = sites
			.iter()
			.find(|s| s.handle == primary.handle)
			.copied()
			.or(sites.first().copied())
			.unwrap_or(primary);
		let globals = load_globals(&state.db.conn(), &state.schema, site)?;
		match globals.robots_txt {
			Some(custom) => lines.push(custom.trim().to_string()),
			None => {
				lines.push("User-agent: *".into());
				lines.push("Disallow:".into());
			}
		}
		lines.push(String::new());
		lines.push(format!("Sitemap: {origin_}/sitemap.xml"));
	}
	Ok((
		[
			(header::CONTENT_TYPE, "text/plain; charset=utf-8"),
			(header::CACHE_CONTROL, "public, max-age=3600"),
		],
		lines.join("\n") + "\n",
	)
		.into_response())
}

/// Public: the frontend proxies them without a token, like Craft's.
pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/actions/meierlabs-seo/sitemap/index", get(index))
		.route("/actions/meierlabs-seo/sitemap/page", get(page))
		.route("/actions/meierlabs-seo/sitemap/robots", get(robots))
}
