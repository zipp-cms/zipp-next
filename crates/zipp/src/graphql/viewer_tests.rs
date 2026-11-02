//! What a viewer sees: drafts with a session or a preview token, offline
//! sites, the scope of a token, the limit of a request.

use serde_json::json;

use super::tests::{run, viewer};
use super::*;
use crate::entries::save;
use crate::fixtures::site;
use crate::testing::input;

#[tokio::test]
async fn preview_shows_the_draft() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let doc = {
		let conn = db.conn();
		let mut e = input("pages", "page", "Live", json!({ "intro": "old" }));
		e.published = false;
		let e = save(&conn, &s, None, e, None).unwrap();
		let sd = crate::drafts::SiteDoc {
			ty: "page".into(),
			dates: Default::default(),
			title: "Draft".into(),
			slug: "live".into(),
			published: false,
			fields: json!({ "intro": "new" }),
		};
		crate::drafts::put_draft(&conn, &s, e.id, "de", sd, None, None)
			.unwrap();
		let admin =
			crate::users::create(&conn, "a@test.ch", "", "", None, "").unwrap();
		let token = crate::drafts::create_token(&conn, e.id, admin).unwrap();
		crate::drafts::preview(&conn, &s, &token).unwrap()
	};

	let req = async_graphql::Request::new(
		r#"{ entry(uri: "live") { title ... on page_Entry { intro } } }"#,
	)
	.data(Viewer {
		preview: doc,
		..viewer(true)
	});
	let data = gql.execute(req).await.data.into_json().unwrap();
	assert_eq!(data["entry"], json!({ "title": "Draft", "intro": "new" }));
}

#[tokio::test]
async fn site_preview_shows_every_draft() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let preview = {
		let conn = db.conn();
		let draft = |title: &str, published: bool| crate::drafts::SiteDoc {
			ty: "page".into(),
			dates: Default::default(),
			title: title.into(),
			slug: title.to_lowercase(),
			published,
			fields: json!({}),
		};
		// new, only its draft publishes it
		let mut e = input("pages", "page", "New", json!({}));
		e.published = false;
		let new = save(&conn, &s, None, e, None).unwrap();
		crate::drafts::put_draft(
			&conn,
			&s,
			new.id,
			"de",
			draft("New", true),
			None,
			None,
		)
		.unwrap();
		// live, its draft takes it down
		let old = save(
			&conn,
			&s,
			None,
			input("pages", "page", "Old", json!({})),
			None,
		)
		.unwrap();
		crate::drafts::put_draft(
			&conn,
			&s,
			old.id,
			"de",
			draft("Old", false),
			None,
			None,
		)
		.unwrap();
		save(
			&conn,
			&s,
			None,
			input("pages", "page", "Same", json!({})),
			None,
		)
		.unwrap();
		let admin =
			crate::users::create(&conn, "a@test.ch", "", "", None, "").unwrap();
		let token = crate::drafts::create_site_token(&conn, admin).unwrap();
		crate::drafts::preview(&conn, &s, &token).unwrap()
	};

	let titles = |preview| {
		let req = async_graphql::Request::new(
			r#"{ entries(section: "pages", order: "title") { title } }"#,
		)
		.data(Viewer {
			preview,
			..viewer(true)
		});
		let gql = gql.clone();
		async move { gql.execute(req).await.data.into_json().unwrap() }
	};
	assert_eq!(
		titles(None).await["entries"],
		json!([{ "title": "Old" }, { "title": "Same" }])
	);
	assert_eq!(
		titles(preview).await["entries"],
		json!([{ "title": "New" }, { "title": "Same" }])
	);
}

#[tokio::test]
async fn urls_of_a_preview_keep_the_token() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let urls = SiteUrls::from([("de".to_string(), "https://x.ch".to_string())]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	save(
		&db.conn(),
		&s,
		None,
		input("pages", "page", "Team", json!({})),
		None,
	)
	.unwrap();

	let url = |preview, token: Option<&str>| {
		let req =
			async_graphql::Request::new(r#"{ entry(uri: "team") { url } }"#)
				.data(Viewer {
					preview,
					token: token.map(Into::into),
					..viewer(true)
				});
		let gql = gql.clone();
		async move {
			gql.execute(req).await.data.into_json().unwrap()["entry"]["url"]
				.clone()
		}
	};
	assert_eq!(url(None, None).await, "https://x.ch/team");
	assert_eq!(
		url(Some(Preview::All(vec![])), Some("t")).await,
		"https://x.ch/team?token=t&x-craft-live-preview=1"
	);
}

#[tokio::test]
async fn hides_drafts_without_a_session() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let mut draft = input("pages", "page", "Draft", json!({}));
	draft.published = false;
	save(&db.conn(), &s, None, draft, None).unwrap();

	let q = r#"{ entry(uri: "draft") { title } }"#;
	assert_eq!(run(&gql, q, true).await["entry"], Value::Null);
	assert_eq!(run(&gql, q, false).await["entry"]["title"], "Draft");
}

#[tokio::test]
async fn offline_sites_only_for_previews() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site().offline("en"));
	s.check().unwrap();
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	save(
		&db.conn(),
		&s,
		None,
		input("pages", "page", "Team", json!({})),
		None,
	)
	.unwrap();

	let q = r#"{ crelteSites { handle } entries(section: "pages", site: "en") { title } }"#;
	let data = run(&gql, q, true).await;
	assert_eq!(data["crelteSites"], json!([{ "handle": "de" }]));
	assert_eq!(data["entries"], json!([]));
	// an admin sees it
	let data = run(&gql, q, false).await;
	assert_eq!(data["entries"], json!([{ "title": "Team" }]));

	// the preview of an entry: the site, but only that entry of it
	let jobs = save(
		&db.conn(),
		&s,
		None,
		input("pages", "page", "Jobs", json!({})),
		None,
	)
	.unwrap();
	let preview = |p| Viewer {
		preview: Some(p),
		..viewer(true)
	};
	let ask = async |v: Viewer| {
		let resp = gql.execute(async_graphql::Request::new(q).data(v)).await;
		assert!(resp.errors.is_empty(), "{:?}", resp.errors);
		resp.data.into_json().unwrap()
	};
	let data = ask(preview(Preview::Entry(jobs.id, Default::default()))).await;
	assert_eq!(data["crelteSites"].as_array().unwrap().len(), 2);
	assert_eq!(data["entries"], json!([{ "title": "Jobs" }]));
	// the preview of all drafts, the studio's, sees the whole site
	let data = ask(preview(Preview::All(vec!["pages".into()]))).await;
	assert_eq!(data["entries"].as_array().unwrap().len(), 2);
	assert_eq!(
		site().offline("de").check().unwrap_err(),
		"the primary site can't be offline"
	);
}

#[test]
fn tokens_are_kept_hashed() {
	let db = Db::memory().unwrap();
	let conn = db.conn();
	let scope = Scope {
		sections: Some(vec!["events".into()]),
		sites: None,
		previews: false,
	};
	let (info, token) =
		tokens::create(&conn, " Website ", &scope, None).unwrap();
	assert_eq!(info.name, "Website");

	// the database has no copy of it
	let hash: String = conn
		.query_row("SELECT hash FROM graphql_tokens", [], |r| r.get(0))
		.unwrap();
	assert_ne!(hash, token);
	assert_eq!(hash.len(), 64);

	let found = tokens::scope_of(&conn, &token).unwrap().unwrap();
	assert!(found.section("events") && !found.section("pages"));
	assert!(found.site("en"));
	assert_eq!(tokens::scope_of(&conn, &hash).unwrap(), None);
	assert!(tokens::get(&conn, info.id).unwrap().last_used_at.is_some());

	// an empty list reads nothing
	let none = Scope {
		sections: Some(vec![]),
		..Scope::ALL
	};
	assert!(!none.section("events"));
	assert!(tokens::check(&site(), "x", &none).is_empty());
}

#[tokio::test]
async fn localized_lists_the_live_versions_a_viewer_may_see() {
	let db = Db::memory().unwrap();
	let s = Arc::new(
		site()
			.site("fr", "Français", "fr")
			.site("it", "Italiano", "it")
			.offline("it"),
	);
	s.check().unwrap();
	let urls = SiteUrls::from([
		("de".to_string(), "https://x.ch".to_string()),
		("en".to_string(), "https://x.ch/en".to_string()),
		("fr".to_string(), "https://x.ch/fr".to_string()),
	]);
	let gql = build(s.clone(), db.clone(), String::new(), urls).unwrap();
	let id = {
		let conn = db.conn();
		let about = save(
			&conn,
			&s,
			None,
			input("pages", "page", "Über uns", json!({})),
			None,
		)
		.unwrap();
		let mut en = input("pages", "page", "About us", json!({}));
		en.site = Some("en".into());
		save(&conn, &s, Some(about.id), en, None).unwrap();
		// not published in French
		let mut fr = input("pages", "page", "À propos", json!({}));
		fr.site = Some("fr".into());
		fr.published = false;
		save(&conn, &s, Some(about.id), fr, None).unwrap();
		about.id
	};

	let q = format!(
		r#"{{ entry(id: {id}) {{ localized {{
			siteId siteHandle language title uri url }} }} }}"#
	);
	let seen = |published_only, preview, sites: Option<Vec<&str>>| Viewer {
		preview,
		scope: Scope {
			sites: sites.map(|l| l.into_iter().map(Into::into).collect()),
			..Scope::ALL
		},
		..viewer(published_only)
	};
	let sites = async |v: Viewer| {
		let req = async_graphql::Request::new(q.clone()).data(v);
		let resp = gql.execute(req).await;
		assert!(resp.errors.is_empty(), "{:?}", resp.errors);
		let data = resp.data.into_json().unwrap();
		let list = data["entry"]["localized"].as_array().unwrap().clone();
		list.iter()
			.map(|e| e["siteHandle"].as_str().unwrap().to_string())
			.collect::<Vec<_>>()
	};

	// a frontend: not the unpublished French one nor the offline site
	let data = run(&gql, &q, true).await;
	assert_eq!(
		data["entry"]["localized"],
		json!([{
			"siteId": 2,
			"siteHandle": "en",
			"language": "en",
			"title": "About us",
			"uri": "about-us",
			"url": "https://x.ch/en/about-us",
		}])
	);

	// a token of German and French reads no English
	assert_eq!(
		sites(seen(true, None, Some(vec!["de", "fr"]))).await,
		Vec::<String>::new()
	);
	// admins see every version
	assert_eq!(sites(seen(false, None, None)).await, ["en", "fr", "it"]);
	// the preview of the entry shows it in every site, offline ones too
	let preview = Preview::Entry(id, Default::default());
	assert_eq!(
		sites(seen(true, Some(preview), None)).await,
		["en", "fr", "it"]
	);
	// but not the sites out of the token's scope
	let preview = Preview::Entry(id, Default::default());
	assert_eq!(
		sites(seen(true, Some(preview), Some(vec!["de", "en"]))).await,
		["en"]
	);
}

#[tokio::test]
async fn a_request_loads_a_limited_number_of_entries() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	{
		let conn = db.conn();
		for i in 0..60 {
			let page = input("pages", "page", &format!("P{i}"), json!({}));
			save(&conn, &s, None, page, None).unwrap();
		}
	}

	// 61 for the list and its query, one query per entry for the children
	let one = "entries(section: \"pages\", limit: 1000) { id children { id } }";
	let data = run(&gql, &format!("{{ {one} }}"), true).await;
	assert_eq!(data["entries"].as_array().unwrap().len(), 60);

	// aliases repeat it past the budget: an error, not a stall
	let many: String = (0..100).map(|i| format!("a{i}: {one}\n")).collect();
	let req =
		async_graphql::Request::new(format!("{{ {many} }}")).data(viewer(true));
	let resp = gql.execute(req).await;
	assert!(
		resp.errors
			.iter()
			.any(|e| e.message.contains("more than 10000")),
		"{:?}",
		resp.errors
	);
}
