//! The website's search: `search`, `searchCount` and `entries(search:)`.

use serde_json::json;

use super::tests::{run, viewer};
use super::*;
use crate::entries::save;
use crate::fixtures::site;
use crate::testing::input;

/// A site with a few pages and an event, in German; the event is in
/// English too.
fn searched() -> GqlSchema {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let conn = db.conn();
	let page = |title: &str, intro: &str, published: bool| {
		let mut e = input("pages", "page", title, json!({ "intro": intro }));
		e.published = published;
		save(&conn, &s, None, e, None).unwrap()
	};
	page("Konzert im Hof", "Musik im Sommer.", true);
	page("Lesungen", "Nach der Lesung gibt es ein Konzert.", true);
	page("Über uns", "Ein Verein in Zürich, <seit> 2019.", true);
	page("Geheim", "Ein Konzert, noch nicht veröffentlicht.", false);
	let event = save(
		&conn,
		&s,
		None,
		input("events", "event", "Konzert in Bern", json!({})),
		None,
	)
	.unwrap();
	let mut en = input("events", "event", "Concert in Bern", json!({}));
	en.site = Some("en".into());
	save(&conn, &s, Some(event.id), en, None).unwrap();
	drop(conn);
	build(s, db, String::new(), SiteUrls::new()).unwrap()
}

/// The titles of the hits of a search.
async fn titles(gql: &GqlSchema, args: &str) -> Vec<String> {
	let q = format!("{{ search({args}) {{ entry {{ title }} }} }}");
	let data = run(gql, &q, true).await;
	data["search"]
		.as_array()
		.unwrap()
		.iter()
		.map(|h| h["entry"]["title"].as_str().unwrap().to_string())
		.collect()
}

#[tokio::test]
async fn finds_the_words_titles_first() {
	let gql = searched();
	// in the title before in the texts, the unpublished one not
	let mut found = titles(&gql, r#"query: "konzert""#).await;
	assert_eq!(found.pop().unwrap(), "Lesungen");
	found.sort();
	assert_eq!(found, ["Konzert im Hof", "Konzert in Bern"]);
	// all the words
	assert_eq!(
		titles(&gql, r#"query: "konzert bern""#).await,
		["Konzert in Bern"]
	);
	// the start of a word, without accents and umlauts
	assert_eq!(
		titles(&gql, r#"query: "konz hof""#).await,
		["Konzert im Hof"]
	);
	assert_eq!(titles(&gql, r#"query: "zurich""#).await, ["Über uns"]);
	assert_eq!(titles(&gql, r#"query: "uber""#).await, ["Über uns"]);
	assert_eq!(titles(&gql, r#"query: "ÜBER""#).await, ["Über uns"]);
}

#[tokio::test]
async fn filters_by_site_and_section() {
	let gql = searched();
	assert_eq!(
		titles(&gql, r#"query: "konzert", section: "events""#).await,
		["Konzert in Bern"]
	);
	assert_eq!(
		titles(
			&gql,
			r#"query: "konzert", type: "page", limit: 1, offset: 1"#
		)
		.await,
		["Lesungen"]
	);
	assert_eq!(
		titles(&gql, r#"query: "concert", site: "en""#).await,
		["Concert in Bern"]
	);
	assert!(titles(&gql, r#"query: "concert""#).await.is_empty());
	assert!(
		titles(&gql, r#"query: "concert", siteId: 1"#)
			.await
			.is_empty()
	);
}

#[tokio::test]
async fn unpublished_entries_only_with_a_session() {
	let gql = searched();
	let q = r#"{ search(query: "geheim") { entry { title } } }"#;
	assert_eq!(run(&gql, q, true).await["search"], json!([]));
	assert_eq!(
		run(&gql, q, false).await["search"],
		json!([{ "entry": { "title": "Geheim" } }])
	);
}

#[tokio::test]
async fn a_token_finds_only_what_it_reads() {
	let gql = searched();
	let q = r#"{ search(query: "konzert") { entry { title } }
		searchCount(query: "konzert") }"#;
	let req = async_graphql::Request::new(q).data(Viewer {
		scope: Scope {
			sections: Some(vec!["events".into()]),
			..Scope::ALL
		},
		..viewer(true)
	});
	let data = gql.execute(req).await.data.into_json().unwrap();
	assert_eq!(
		data,
		json!({
			"search": [{ "entry": { "title": "Konzert in Bern" } }],
			"searchCount": 1,
		})
	);
}

#[tokio::test]
async fn hits_have_an_excerpt_and_a_score() {
	let gql = searched();
	let data = run(
		&gql,
		r#"{
			lesung: search(query: "konzert lesung", type: "page") {
				excerpt excerptHtml
			}
			verein: search(query: "verein") { excerptHtml score }
			title: search(query: "hof") { excerpt }
			event: search(query: "bern") { excerpt }
		}"#,
		true,
	)
	.await;
	assert_eq!(
		data["lesung"],
		json!([{
			"excerpt": "Nach der Lesung gibt es ein Konzert.",
			"excerptHtml": "Nach der <mark>Lesung</mark> gibt es ein <mark>Konzert</mark>.",
		}])
	);
	// the texts are escaped for html
	let verein = &data["verein"][0];
	assert_eq!(
		verein["excerptHtml"],
		"Ein <mark>Verein</mark> in Zürich, &lt;seit&gt; 2019."
	);
	assert!(verein["score"].as_f64().unwrap() > 0.0);
	// only the title matched: the start of the texts; none without texts
	assert_eq!(data["title"][0]["excerpt"], "Musik im Sommer.");
	assert_eq!(data["event"][0]["excerpt"], Value::Null);
}

#[tokio::test]
async fn counts_and_entries_search_the_same() {
	let gql = searched();
	let data = run(
		&gql,
		r#"{
			searchCount(query: "konzert", limit: 1)
			entryCount(search: "konzert")
			entries(search: "konz", type: "page") { title }
			ordered: entries(search: "konzert", orderBy: "title") { title }
		}"#,
		true,
	)
	.await;
	assert_eq!(data["searchCount"], 3);
	assert_eq!(data["entryCount"], 3);
	assert_eq!(
		data["entries"],
		json!([{ "title": "Konzert im Hof" }, { "title": "Lesungen" }])
	);
	assert_eq!(
		data["ordered"],
		json!([
			{ "title": "Konzert im Hof" },
			{ "title": "Konzert in Bern" },
			{ "title": "Lesungen" },
		])
	);
}

#[tokio::test]
async fn the_syntax_of_fts_is_only_words() {
	let gql = searched();
	// operators, quotes, columns and stars are words or nothing
	for q in [
		r#"konzert OR geheim"#,
		r#"\"konzert"#,
		r#"title: konzert"#,
		r#"NEAR(konzert bern)"#,
		r#"* - ^ ( )"#,
		r#"konzert*"#,
	] {
		let query = format!(r#"{{ search(query: "{q}") {{ score }} }}"#);
		let req = async_graphql::Request::new(query).data(viewer(true));
		let resp = gql.execute(req).await;
		assert!(resp.errors.is_empty(), "{q}: {:?}", resp.errors);
	}
	// OR is a word, not an operator: nothing has it
	assert!(
		titles(&gql, r#"query: "konzert OR geheim""#)
			.await
			.is_empty()
	);
	assert!(titles(&gql, r#"query: "-""#).await.is_empty());
	assert!(titles(&gql, r#"query: """#).await.is_empty());
}
