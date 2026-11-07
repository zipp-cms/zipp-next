use reqwest::Url;

use super::favicon::{Link, best, links, same_host};

fn link(href: &str, svg: bool, size: Option<u32>) -> Link {
	Link {
		href: href.into(),
		svg,
		size,
	}
}

#[test]
fn the_icon_links_of_a_page() {
	let html = r##"<!doctype html><HTML><head>
		<meta charset="utf-8">
		<link rel="stylesheet" href="/main.css">
		<LINK REL="Shortcut Icon" HREF='/favicon.ico'>
		<link rel=icon type="image/png" sizes="16x16 32X32" href=/i/32.png />
		<link href="/i/icon.svg?v=2&amp;x=1" rel="icon">
		<link rel="apple-touch-icon" href="/apple.png">
		<link rel="mask-icon" href="/mask.svg" color="#000">
		<link rel="icon" href="">
		</head><body><link rel="icon" href="/late.png"></body>"##;
	assert_eq!(
		links(html),
		vec![
			link("/favicon.ico", false, None),
			link("/i/32.png", false, Some(32)),
			link("/i/icon.svg?v=2&x=1", true, None),
			// Apple's size without sizes
			link("/apple.png", false, Some(180)),
			link("/late.png", false, None),
		]
	);
	assert!(links("<p>no head</p><link").is_empty());
}

#[test]
fn the_best_icon_for_the_sidebar() {
	let ico = link("/favicon.ico", false, None);
	let small = link("/16.png", false, Some(16));
	let fits = link("/48.png", false, Some(48));
	let big = link("/192.png", false, Some(192));
	let bigger = link("/512.png", false, Some(512));
	let svg = link("/icon.svg", true, None);

	let all = [ico.clone(), small.clone(), fits.clone(), big.clone(), svg];
	assert_eq!(best(&all).unwrap().href, "/icon.svg");
	// the largest up to 64px
	let pngs = [small.clone(), fits.clone(), big.clone()];
	assert_eq!(best(&pngs).unwrap().href, "/48.png");
	// a size it fits over one without
	assert_eq!(best(&[ico.clone(), small]).unwrap().href, "/16.png");
	// one without sizes over a big one, else the smallest big one
	assert_eq!(best(&[big.clone(), ico.clone()]).unwrap().href, ico.href);
	assert_eq!(best(&[bigger, big.clone()]).unwrap().href, big.href);
	// the first of equals
	let other = link("/other.ico", false, None);
	assert_eq!(best(&[ico.clone(), other]).unwrap().href, ico.href);
	assert!(best(&[]).is_none());
}

#[test]
fn www_is_the_same_site() {
	let url = |s: &str| Url::parse(s).unwrap();
	assert!(same_host(&url("https://x.ch/"), &url("https://www.x.ch/a")));
	assert!(same_host(&url("https://www.x.ch/"), &url("https://x.ch/")));
	assert!(!same_host(&url("https://x.ch/"), &url("https://y.ch/")));
	assert!(!same_host(&url("https://x.ch/"), &url("https://cdn.x.ch/")));
}
