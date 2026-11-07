//! The arguments of `entries`, `entryCount` and `children` like Craft's:
//! several sections, `not`, `level`, `relatedTo`, `relatedToEntries`,
//! `orderBy: "score"`, and the slug of blocks.

use serde_json::json;

use super::tests::{run, viewer};
use super::*;
use crate::entries::save;
use crate::fixtures::site;
use crate::testing::input;

/// Pages About (Team, Board) and Jobs, the events Jazz and Rock: Jazz
/// points to Team, Rock to Jobs and isn't published.
fn tree() -> (GqlSchema, Db) {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let conn = db.conn();
	let page = |title: &str, parent: Option<i64>| {
		let mut p = input("pages", "page", title, json!({}));
		p.parent_id = parent;
		save(&conn, &s, None, p, None).unwrap().id
	};
	let about = page("About", None);
	let team = page("Team", Some(about));
	page("Board", Some(about));
	let jobs = page("Jobs", None);
	let event = |title: &str, page: i64, published: bool| {
		let mut e = input("events", "event", title, json!({ "page": [page] }));
		e.published = published;
		save(&conn, &s, None, e, None).unwrap().id
	};
	event("Jazz", team, true);
	event("Rock", jobs, false);
	drop(conn);
	let gql = build(s, db.clone(), String::new(), SiteUrls::new()).unwrap();
	(gql, db)
}

fn titles(v: &Value) -> Vec<&str> {
	let list = v.as_array().unwrap().iter();
	list.map(|e| e["title"].as_str().unwrap()).collect()
}

async fn error(gql: &GqlSchema, query: &str) -> String {
	let req = async_graphql::Request::new(query).data(viewer(true));
	let resp = gql.execute(req).await;
	assert!(!resp.errors.is_empty(), "{query}");
	resp.errors[0].message.clone()
}

#[tokio::test]
async fn several_sections_and_not() {
	let (gql, _) = tree();
	let data = run(
		&gql,
		r#"{
			both: entryCount(section: ["pages", "events"])
			pages: entryCount(section: ["pages"])
			notPages: entries(section: ["not", "pages"], orderBy: "title") {
				title
			}
			notEvents: entryCount(type: ["not", "event"])
			notTwo: entryCount(type: ["not", "event", "page"])
		}"#,
		false,
	)
	.await;
	assert_eq!(data["both"], 6);
	assert_eq!(data["pages"], 4);
	assert_eq!(titles(&data["notPages"]), ["Jazz", "Rock"]);
	assert_eq!(data["notEvents"], 4);
	assert_eq!(data["notTwo"], 0);

	// a section that doesn't exist is an error, not the first one only
	let err = error(&gql, r#"{ entries(section: ["pages", "news"]) { id } }"#);
	assert!(err.await.contains("unknown section news"));
	let err = error(&gql, r#"{ entries(type: ["not", "wiki"]) { id } }"#);
	assert!(err.await.contains("unknown entry type wiki"));
}

#[tokio::test]
async fn levels_of_a_structure() {
	let (gql, _) = tree();
	let data = run(
		&gql,
		r#"{
			top: entries(section: "pages", level: 1) { title }
			second: entries(section: "pages", level: 2) { title }
			third: entryCount(section: "pages", level: 3)
		}"#,
		false,
	)
	.await;
	assert_eq!(titles(&data["top"]), ["About", "Jobs"]);
	assert_eq!(titles(&data["second"]), ["Team", "Board"]);
	assert_eq!(data["third"], 0);

	let err = error(&gql, r#"{ entries(level: 0) { id } }"#).await;
	assert!(err.contains("level must be between"), "{err}");
}

#[tokio::test]
async fn related_to_only_of_entries_it_sees() {
	let (gql, db) = tree();
	let id = |title: &str| -> i64 {
		db.conn()
			.query_row(
				"SELECT entry_id FROM entry_sites WHERE title = ?1",
				[title],
				|r| r.get(0),
			)
			.unwrap()
	};
	let (jazz, rock) = (id("Jazz"), id("Rock"));
	let query = format!(
		r#"{{
		rock: entries(relatedTo: [{rock}]) {{ title }}
		jazz: entries(relatedTo: [{jazz}]) {{ title }}
	}}"#
	);

	// Rock isn't published: what it points to doesn't show
	let data = run(&gql, &query, true).await;
	assert_eq!(titles(&data["rock"]), Vec::<&str>::new());
	assert_eq!(titles(&data["jazz"]), ["Team"]);
	let data = run(&gql, &query, false).await;
	assert_eq!(titles(&data["rock"]), ["Jobs"]);

	// nor of a section the token doesn't read
	let pages = Scope {
		sections: Some(vec!["pages".into()]),
		..Scope::ALL
	};
	let req = async_graphql::Request::new(&query).data(Viewer {
		scope: pages,
		..viewer(true)
	});
	let data = gql.execute(req).await.data.into_json().unwrap();
	assert_eq!(titles(&data["jazz"]), Vec::<&str>::new());
}

#[tokio::test]
async fn related_to_in_both_directions() {
	let (gql, db) = tree();
	let id = |title: &str| -> i64 {
		db.conn()
			.query_row(
				"SELECT entry_id FROM entry_sites WHERE title = ?1",
				[title],
				|r| r.get(0),
			)
			.unwrap()
	};
	let (team, jazz, jobs) = (id("Team"), id("Jazz"), id("Jobs"));
	let data = run(
		&gql,
		&format!(
			r#"{{
			from: entries(relatedTo: [{team}]) {{ title }}
			to: entries(relatedTo: [{jazz}]) {{ title }}
			either: entries(relatedTo: [{team}, {jobs}], orderBy: "title") {{
				title
			}}
			both: entryCount(relatedTo: ["and", {team}, {jobs}])
			none: entryCount(relatedTo: [])
		}}"#
		),
		false,
	)
	.await;
	assert_eq!(titles(&data["from"]), ["Jazz"]);
	assert_eq!(titles(&data["to"]), ["Team"]);
	assert_eq!(titles(&data["either"]), ["Jazz", "Rock"]);
	assert_eq!(data["both"], 0);
	// an empty list is no filter, like in Craft
	assert_eq!(data["none"], 6);
}

#[tokio::test]
async fn related_to_entries_of_criteria() {
	let (gql, _) = tree();
	// Lerbermatt's categories that have wiki entries; Rock isn't live,
	// so for a frontend Jobs has none
	let query = r#"{
		pages: entries(
			section: "pages"
			relatedToEntries: [{ section: "events" }]
			orderBy: "title"
		) { title }
		top: entries(section: "pages", level: 1) {
			title
			children(relatedToEntries: [{ section: "events" }]) { title }
		}
		each: entryCount(relatedToEntries: [
			{ section: "events" }, { type: "page", slug: "nope" }
		])
	}"#;
	let data = run(&gql, query, true).await;
	assert_eq!(titles(&data["pages"]), ["Team"]);
	assert_eq!(
		data["top"],
		json!([
			{ "title": "About", "children": [{ "title": "Team" }] },
			{ "title": "Jobs", "children": [] },
		])
	);
	assert_eq!(data["each"], 0);

	// with drafts both
	let data = run(&gql, query, false).await;
	assert_eq!(titles(&data["pages"]), ["Jobs", "Team"]);

	// a token only reads its sections, also in the criteria
	let mut v = viewer(true);
	v.scope.sections = Some(vec!["pages".into()]);
	let req = async_graphql::Request::new(
		r#"{ entryCount(
			section: "pages", relatedToEntries: [{ section: "events" }]
		) }"#,
	)
	.data(v);
	let resp = gql.execute(req).await;
	assert!(resp.errors.is_empty(), "{:?}", resp.errors);
	assert_eq!(resp.data.into_json().unwrap()["entryCount"], 0);
}

#[tokio::test]
async fn children_take_arguments() {
	let (gql, _) = tree();
	let data = run(
		&gql,
		r#"{ entries(section: "pages", level: 1) {
			title
			all: children { title }
			first: children(limit: 1) { title }
			byTitle: children(orderBy: "title") { title }
			second: children(orderBy: "title", limit: 1, offset: 1) { title }
			notTeam: children(slug: "board") { title }
		} }"#,
		false,
	)
	.await;
	let about = &data["entries"][0];
	assert_eq!(titles(&about["all"]), ["Team", "Board"]);
	assert_eq!(titles(&about["first"]), ["Team"]);
	assert_eq!(titles(&about["byTitle"]), ["Board", "Team"]);
	assert_eq!(titles(&about["second"]), ["Team"]);
	assert_eq!(titles(&about["notTeam"]), ["Board"]);
	assert_eq!(titles(&data["entries"][1]["all"]), Vec::<&str>::new());
}

#[tokio::test]
async fn order_by_score_is_the_best_match() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	{
		let conn = db.conn();
		let page = |title: &str, intro: &str| {
			let p = input("pages", "page", title, json!({ "intro": intro }));
			save(&conn, &s, None, p, None).unwrap();
		};
		page("Anfahrt", "Mit dem Konzert-Bus");
		page("Konzert", "Am Abend");
	}
	let gql = build(s, db, String::new(), SiteUrls::new()).unwrap();
	let data = run(
		&gql,
		r#"{
			best: entries(search: "konzert", orderBy: "score") { title }
			ignored: entries(section: "pages", orderBy: "score") { title }
		}"#,
		false,
	)
	.await;
	// the title counts most
	assert_eq!(titles(&data["best"]), ["Konzert", "Anfahrt"]);
	// without a search the order of the section
	assert_eq!(titles(&data["ignored"]), ["Anfahrt", "Konzert"]);
}

#[tokio::test]
async fn blocks_have_a_slug_of_their_title_field() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site().block_title("button", "label"));
	{
		let conn = db.conn();
		let body = json!({ "body": [
			{ "id": "a", "type": "button", "fields": { "label": "Über uns!" } },
			{ "id": "b", "type": "button", "fields": { "label": "!!" } },
			{ "id": "c", "type": "text", "fields": { "content": "Hi" } },
		] });
		save(&conn, &s, None, input("pages", "page", "About", body), None)
			.unwrap();
	}
	let gql = build(s, db, String::new(), SiteUrls::new()).unwrap();
	let data = run(
		&gql,
		r#"{ entry(section: "pages") { ... on page_Entry { body {
			... on button_Block { slug }
			... on text_Block { slug }
		} } } }"#,
		false,
	)
	.await;
	assert_eq!(
		data["entry"]["body"],
		json!([{ "slug": "über-uns" }, { "slug": null }, { "slug": null }])
	);
}
