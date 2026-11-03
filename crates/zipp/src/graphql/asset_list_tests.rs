use serde_json::json;

use super::tests::{run, viewer};
use super::*;
use crate::assets::{AssetChange, create_folder, move_assets, store, update};
use crate::entries::save;
use crate::fixtures::{png, site};
use crate::schema::Field;
use crate::testing::input;

/// A library: `Clubs` with `Clubs/Bern`, `Timetables/2025`, and a file on
/// the top.
fn library(db: &Db, s: &Schema, dir: &std::path::Path) -> [i64; 6] {
	let conn = db.conn();
	let clubs = create_folder(&conn, None, "Clubs").unwrap().id;
	let bern = create_folder(&conn, Some(clubs), "Bern").unwrap().id;
	let times = create_folder(&conn, None, "Timetables").unwrap().id;
	let year = create_folder(&conn, Some(times), "2025").unwrap().id;

	let add = |name: &str, data: &[u8], folder: Option<i64>| {
		let a = store(&conn, dir, name, data, None).unwrap();
		move_assets(&conn, &[a.id], folder).unwrap();
		a.id
	};
	let ids = [
		add("zurich.png", &png(4, 2), Some(clubs)),
		add("bern.png", &png(2, 2), Some(bern)),
		add("intro.mp4", b"video", Some(clubs)),
		add("1a.pdf", b"%PDF-1.4", Some(year)),
		add("1b.docx", b"word", Some(year)),
		add("notes.txt", b"hi", None),
	];
	let alt = [("de", "Wappen"), ("en", "Coat of arms")];
	let change = AssetChange {
		title: Some("Bärn".into()),
		alt: alt
			.iter()
			.map(|(k, v)| (k.to_string(), v.to_string()))
			.collect(),
		focal_x: 0.5,
		focal_y: 0.5,
		..Default::default()
	};
	update(&conn, dir, s, ids[1], change, None).unwrap();
	ids
}

fn names(list: &Value) -> Vec<&str> {
	let list = list.as_array().unwrap().iter();
	list.map(|a| a["filename"].as_str().unwrap()).collect()
}

#[tokio::test]
async fn lists_assets_like_craft() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let tmp = tempfile::tempdir().unwrap();
	library(&db, &s, tmp.path());

	let q = |args: &str| {
		format!("{{ assets{args} {{ filename }} assetCount{args} }}")
	};
	let list = async |args: &str| {
		let data = run(&gql, &q(args), true).await;
		let n = data["assetCount"].as_i64().unwrap() as usize;
		let found: Vec<String> =
			names(&data["assets"]).into_iter().map(Into::into).collect();
		(found, n)
	};

	// the newest first
	let (all, n) = list("").await;
	assert_eq!(all[0], "notes.txt");
	assert_eq!(n, 6);

	// a folder with the ones in it, like Craft's volume; any case
	let (found, n) = list(r#"(folder: "clubs", orderBy: "filename")"#).await;
	assert_eq!(found, ["bern.png", "intro.mp4", "zurich.png"]);
	assert_eq!(n, 3);
	let (found, _) = list(r#"(folder: "Clubs/Bern")"#).await;
	assert_eq!(found, ["bern.png"]);
	let (found, n) = list(r#"(folder: "Nowhere")"#).await;
	assert_eq!((found.len(), n), (0, 0));

	// kinds, one or several, Craft's pdf and word too
	let (found, _) = list(r#"(kind: "image", orderBy: "filename")"#).await;
	assert_eq!(found, ["bern.png", "zurich.png"]);
	let (found, _) = list(r#"(kind: "video")"#).await;
	assert_eq!(found, ["intro.mp4"]);
	let (found, _) = list(
		r#"(folder: "Timetables", kind: ["pdf", "word"], orderBy: "filename")"#,
	)
	.await;
	assert_eq!(found, ["1a.pdf", "1b.docx"]);
	let (found, _) = list(r#"(kind: "file", orderBy: "filename")"#).await;
	assert_eq!(found, ["1a.pdf", "1b.docx", "notes.txt"]);

	// words in the filename, title or alt text
	let (found, _) = list(r#"(search: "coat")"#).await;
	assert_eq!(found, ["bern.png"]);
	let (found, _) = list(r#"(search: "bärn")"#).await;
	assert_eq!(found, ["bern.png"]);

	// the title of one without is its filename's: Bärn, Intro, Zurich
	let (found, n) =
		list(r#"(folder: "Clubs", orderBy: "title DESC", limit: 2)"#).await;
	assert_eq!(found, ["zurich.png", "intro.mp4"]);
	assert_eq!(n, 3);
	let (found, _) =
		list(r#"(folder: "Clubs", orderBy: "title", limit: 1, offset: 1)"#)
			.await;
	assert_eq!(found, ["intro.mp4"]);
}

#[tokio::test]
async fn assets_know_their_folder() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let ids = library(&db, &s, tmp.path());
	let year = db.conn().query_row(
		"SELECT id FROM asset_folders WHERE name = '2025'",
		[],
		|r| r.get::<_, i64>(0),
	);
	let year = year.unwrap();

	// Craft's path is the one in its volume, the top folder
	let data = run(
		&gql,
		&format!(
			r#"{{ assets(folderId: {year}) {{ folderId folder path }}
			top: assets(kind: "file", folder: []) {{ folder path }}
			bern: assets(search: "bern", siteId: 2) {{ folder path alt }}
			}}"#
		),
		true,
	)
	.await;
	let pdf = json!({
		"folderId": year,
		"folder": "Timetables/2025",
		"path": "2025/1a.pdf",
	});
	assert_eq!(data["assets"][1], pdf);
	assert_eq!(data["top"], json!([]));
	assert_eq!(
		data["bern"],
		json!([{ "folder": "Clubs/Bern", "path": "Bern/bern.png",
			"alt": "Coat of arms" }])
	);

	// on the top it has none, the alt of the primary site without one
	let data =
		run(&gql, r#"{ assets(search: "notes") { folder path } }"#, true).await;
	assert_eq!(
		data["assets"],
		json!([{ "folder": null, "path": "notes.txt" }])
	);
	let data = run(&gql, r#"{ assets(search: "bern") { alt } }"#, true).await;
	assert_eq!(data["assets"][0]["alt"], "Wappen");

	// trashed ones are gone
	crate::trash::trash_asset(&db.conn(), ids[1], None).unwrap();
	let data = run(&gql, r#"{ assetCount(search: "bern") }"#, true).await;
	assert_eq!(data["assetCount"], 0);
}

#[tokio::test]
async fn wrong_asset_arguments() {
	let db = Db::memory().unwrap();
	let gql =
		build(Arc::new(site()), db, String::new(), SiteUrls::new()).unwrap();
	for q in [
		r#"{ assets(kind: "movie") { id } }"#,
		r#"{ assets(orderBy: "width") { id } }"#,
		r#"{ assets(orderBy: "title UP") { id } }"#,
		r#"{ assets(site: "fr") { id } }"#,
		r#"{ assetCount(siteId: 9) }"#,
		r#"{ assets(folderId: "clubs") { id } }"#,
	] {
		let req = async_graphql::Request::new(q).data(viewer(true));
		assert!(!gql.execute(req).await.errors.is_empty(), "{q}");
	}
}

#[tokio::test]
async fn a_token_lists_assets_of_its_sites() {
	let db = Db::memory().unwrap();
	let s = Arc::new(site());
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let tmp = tempfile::tempdir().unwrap();
	library(&db, &s, tmp.path());

	let only_en = || {
		let mut v = viewer(true);
		v.scope.sites = Some(vec!["en".into()]);
		v
	};
	let q = r#"{ de: assetCount(site: "de") en: assetCount(site: "en") }"#;
	let req = async_graphql::Request::new(q).data(only_en());
	let data = gql.execute(req).await.data.into_json().unwrap();
	assert_eq!(data, json!({ "de": 0, "en": 6 }));
}

#[tokio::test]
async fn asset_fields_filter_by_kind() {
	let db = Db::memory().unwrap();
	let mut s = site();
	let event = s.get_mut("event").unwrap();
	event.fields.push(Field::assets("media"));
	let s = Arc::new(s);
	let gql =
		build(s.clone(), db.clone(), String::new(), SiteUrls::new()).unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let ids = library(&db, &s, tmp.path());
	{
		let conn = db.conn();
		let media = [ids[3], ids[0], ids[2], ids[1]];
		let e = input("events", "event", "Film", json!({ "media": media }));
		save(&conn, &s, None, e, None).unwrap();
	}

	// in the order of the field, like valgistesshots' `assets(kind:)`
	let data = run(
		&gql,
		r#"{ entry(uri: "events/film") { ... on event_Entry {
			all: media { filename }
			images: media(kind: "image") { filename kind }
			moving: media(kind: ["image", "video"]) { filename }
			pdfs: media(kind: "pdf") { filename }
		} } }"#,
		true,
	)
	.await;
	let e = &data["entry"];
	assert_eq!(names(&e["all"]).len(), 4);
	assert_eq!(
		e["images"],
		json!([
			{ "filename": "zurich.png", "kind": "image" },
			{ "filename": "bern.png", "kind": "image" },
		])
	);
	assert_eq!(names(&e["moving"]), ["zurich.png", "intro.mp4", "bern.png"]);
	assert_eq!(names(&e["pdfs"]), ["1a.pdf"]);

	let q = r#"{ entry(uri: "events/film") { ... on event_Entry {
		media(kind: "movie") { id } } } }"#;
	let req = async_graphql::Request::new(q).data(viewer(true));
	assert!(!gql.execute(req).await.errors.is_empty());
}
