//! The favicon of a site for the studio's sidebar: the studio can't read
//! another origin's page, so the cms fetches it from the site's url (of
//! `--site-url`, never one of the request), only from that host (with or without `www.`).

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use axum::body::Bytes;
use axum::http::HeaderValue;
use reqwest::{Client, Response, Url, header, redirect};

/// How long a site's icon, or that it has none, is kept.
const KEEP: Duration = Duration::from_secs(60 * 60);
/// The part of a page read for its links, they are in the head.
const HTML_MAX: usize = 512 * 1024;
const ICON_MAX: usize = 256 * 1024;
/// The size the sidebar shows at most, in pixels.
const LARGEST: u32 = 64;

#[derive(Debug, Clone)]
pub(crate) struct Favicon {
	/// its `Content-Type`, an `image/*`
	pub mime: HeaderValue,
	pub data: Bytes,
}

/// The favicons of the sites, fetched on the first ask and kept an hour;
/// a site without one isn't asked again either.
pub(crate) struct Favicons {
	client: Client,
	kept: Mutex<HashMap<String, (Instant, Option<Favicon>)>>,
}

impl Default for Favicons {
	fn default() -> Self {
		// redirects only on the same host, like the links
		let policy = redirect::Policy::custom(|a| {
			let first = &a.previous()[0];
			match a.previous().len() < 5 && same_host(a.url(), first) {
				true => a.follow(),
				false => a.stop(),
			}
		});
		let client = Client::builder()
			.timeout(Duration::from_secs(3))
			.redirect(policy)
			.user_agent("zipp")
			.build()
			.expect("a http client");
		Self {
			client,
			kept: Default::default(),
		}
	}
}

impl Favicons {
	/// The favicon of the site at `url`, none when it has none or isn't
	/// reachable (offline, behind a login).
	pub async fn get(&self, site: &str, url: &str) -> Option<Favicon> {
		if let Some((at, icon)) = self.kept.lock().unwrap().get(site)
			&& at.elapsed() < KEEP
		{
			return icon.clone();
		}

		let icon = fetch(&self.client, url).await;
		let kept = (Instant::now(), icon.clone());
		self.kept.lock().unwrap().insert(site.into(), kept);
		icon
	}
}

async fn fetch(client: &Client, url: &str) -> Option<Favicon> {
	let site = Url::parse(url).ok()?;
	if !web(&site) {
		return None;
	}

	// the page, its links resolved against where it ended up
	// a redirect to another host stops at the redirect
	let page = client.get(site.clone()).send().await.ok()?;
	if !page.status().is_success() {
		return None;
	}
	let base = page.url().clone();
	let html = read(page, HTML_MAX).await?;
	let html = String::from_utf8_lossy(&html);
	let links: Vec<_> = links(&html)
		.into_iter()
		.filter_map(|l| {
			let href = base.join(&l.href).ok()?;
			(web(&href) && same_host(&href, &site)).then_some(Link {
				href: href.into(),
				..l
			})
		})
		.collect();
	let icon = match best(&links) {
		Some(l) => Url::parse(&l.href).ok()?,
		None => site.join("/favicon.ico").ok()?,
	};

	let resp = client.get(icon).send().await.ok()?;
	if !resp.status().is_success() {
		return None;
	}
	let mime = resp.headers().get(header::CONTENT_TYPE)?.to_str().ok()?;
	let mime = image_type(mime)?;
	if resp.content_length().is_some_and(|l| l > ICON_MAX as u64) {
		return None;
	}
	let data = read(resp, ICON_MAX + 1).await?;
	if data.len() > ICON_MAX {
		return None;
	}
	Some(Favicon {
		mime: HeaderValue::from_static(mime),
		data: data.into(),
	})
}

/// The first `max` bytes of a body.
async fn read(mut resp: Response, max: usize) -> Option<Vec<u8>> {
	let mut data = vec![];
	while data.len() < max {
		let Some(chunk) = resp.chunk().await.ok()? else {
			break;
		};
		data.extend_from_slice(&chunk);
	}
	data.truncate(max);
	Some(data)
}

/// The image type of a `Content-Type`, one the studio shows; never the
/// header as it came, a list like `image/png, text/html` is html.
pub(crate) fn image_type(header: &str) -> Option<&'static str> {
	let mime = header.split(';').next()?.trim().to_ascii_lowercase();
	[
		"image/png",
		"image/x-icon",
		"image/vnd.microsoft.icon",
		"image/gif",
		"image/jpeg",
		"image/webp",
		"image/avif",
		"image/svg+xml",
	]
	.into_iter()
	.find(|t| *t == mime)
}

fn web(url: &Url) -> bool {
	matches!(url.scheme(), "http" | "https")
}

/// Whether `a` is on the site of `b`: the same host and port, https
/// when `b` is.
pub(crate) fn same_host(a: &Url, b: &Url) -> bool {
	// x.ch and www.x.ch are one site, a redirect between them is common
	let bare = |u: &Url| {
		u.host_str()
			.map(|h| h.strip_prefix("www.").unwrap_or(h).to_owned())
	};
	// http to https is common too, never back
	let scheme = a.scheme() == b.scheme() || a.scheme() == "https";
	// a port of its own only when both have it, the default one follows
	// the scheme
	let port = a.port() == b.port();
	bare(a).is_some() && bare(a) == bare(b) && scheme && port
}

/// An icon a page links to.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Link {
	pub href: String,
	pub svg: bool,
	/// its width, the largest of `sizes`
	pub size: Option<u32>,
}

/// The icons of `<link rel="icon">`, `"shortcut icon"` and
/// `"apple-touch-icon"` in a page.
pub(crate) fn links(html: &str) -> Vec<Link> {
	let lower = html.to_ascii_lowercase();
	let mut links = vec![];
	let mut rest = 0;
	while let Some(start) = lower[rest..].find("<link") {
		let start = rest + start + "<link".len();
		let end = lower[start..].find('>').map_or(html.len(), |e| start + e);
		rest = end;

		let attrs = attributes(&html[start..end]);
		let attr = |name: &str| {
			attrs
				.iter()
				.find(|(n, _)| n == name)
				.map(|(_, v)| v.as_str())
		};
		let rel = attr("rel").unwrap_or_default().to_ascii_lowercase();
		let apple = rel
			.split_ascii_whitespace()
			.any(|r| r.starts_with("apple-touch-icon"));
		let icon = rel.split_ascii_whitespace().any(|r| r == "icon");
		let Some(href) = attr("href").filter(|h| !h.is_empty()) else {
			continue;
		};
		if !icon && !apple {
			continue;
		}

		let path = href.split(['?', '#']).next().unwrap_or_default();
		let svg = attr("type").is_some_and(|t| t.contains("svg"))
			|| path.to_ascii_lowercase().ends_with(".svg");
		let size = attr("sizes")
			.unwrap_or_default()
			.split_ascii_whitespace()
			.filter_map(|s| {
				s.to_ascii_lowercase().split('x').next()?.parse().ok()
			})
			.max()
			// what Apple asks for without sizes
			.or(apple.then_some(180));
		links.push(Link {
			href: href.replace("&amp;", "&"),
			svg,
			size,
		});
	}
	links
}

/// The attributes of a tag, names in lower case.
fn attributes(tag: &str) -> Vec<(String, String)> {
	let mut attrs = vec![];
	let mut s = tag.trim_start_matches('/');
	loop {
		s = s.trim_start_matches(|c: char| c.is_whitespace() || c == '/');
		let name_end = s
			.find(|c: char| c.is_whitespace() || c == '=' || c == '/')
			.unwrap_or(s.len());
		if name_end == 0 {
			return attrs;
		}
		let name = s[..name_end].to_ascii_lowercase();
		s = s[name_end..].trim_start();

		let Some(after) = s.strip_prefix('=') else {
			attrs.push((name, String::new()));
			continue;
		};
		let after = after.trim_start();
		let (value, rest) = match after.chars().next() {
			Some(q @ ('"' | '\'')) => {
				let inner = &after[1..];
				let end = inner.find(q).unwrap_or(inner.len());
				(&inner[..end], inner.get(end + 1..).unwrap_or_default())
			}
			_ => {
				let end =
					after.find(char::is_whitespace).unwrap_or(after.len());
				(&after[..end], &after[end..])
			}
		};
		attrs.push((name, value.trim().to_string()));
		s = rest;
	}
}

/// The icon the sidebar shows best: an svg, else the largest up to 64px,
/// else one without sizes, else the smallest bigger one; the first of
/// equals.
pub(crate) fn best(links: &[Link]) -> Option<&Link> {
	let rank = |l: &Link| match (l.svg, l.size) {
		(true, _) => (3, 0),
		(false, Some(s)) if s <= LARGEST => (2, s as i64),
		(false, None) => (1, 0),
		(false, Some(s)) => (0, -(s as i64)),
	};
	links.iter().rev().max_by_key(|l| rank(l))
}
