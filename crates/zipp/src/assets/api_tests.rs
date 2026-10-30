use std::io::Cursor;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use image::ImageFormat;
use serde_json::json;

use super::api::*;
use super::*;
use crate::entries::save;
use crate::entries::tests::input;
use crate::schema::Role;
use crate::schema::tests::site_with_roles;
use crate::testing::{Client, TestServer};

fn png(w: u32, h: u32) -> Vec<u8> {
	let mut png = vec![];
	image::RgbImage::new(w, h)
		.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
		.unwrap();
	png
}

async fn upload(client: &Client, name: &str, data: &[u8]) -> Result<Asset> {
	let resp = client.upload("/assets", &[(name, data)]).await;
	let mut assets: Vec<Asset> = Client::json(resp).await?;
	Ok(assets.remove(0))
}

async fn get(client: &Client, path: &str) -> (StatusCode, Vec<u8>) {
	let req = Request::get(path).body(Body::empty()).unwrap();
	let resp = client.send(req).await;
	let status = resp.status();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	(status, body.to_vec())
}

#[tokio::test]
async fn uploads_changes_and_deletes() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));

	let a = upload(&editor, "Hero Bild.png", &png(20, 10))
		.await
		.unwrap();
	assert_eq!(a.filename, "Hero-Bild.png");
	assert_eq!(
		(a.kind.as_str(), a.width, a.height),
		("image", Some(20), Some(10))
	);

	// the file and a transform, for anyone
	let (status, body) = get(&ts.anonymous(), &a.url(None)).await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(body, png(20, 10));
	let (status, _) = get(&ts.anonymous(), &a.url(Some("small"))).await;
	assert_eq!(status, StatusCode::OK);
	let wrong = format!("/assets/{}/other.png", a.id);
	assert_eq!(get(&ts.anonymous(), &wrong).await.0, StatusCode::NOT_FOUND);

	let list = editor.call(&ListReq::default()).await.unwrap();
	assert_eq!(list, std::slice::from_ref(&a));
	let search = ListReq {
		search: Some("nothing".into()),
		..Default::default()
	};
	assert!(editor.call(&search).await.unwrap().is_empty());
	let ids = ListReq {
		ids: Some(format!("{},999", a.id)),
		..Default::default()
	};
	assert_eq!(editor.call(&ids).await.unwrap().len(), 1);

	let change = UpdateReq {
		id: a.id,
		change: AssetChange {
			title: Some(" Hero ".into()),
			alt: "A hero".into(),
			focal_x: 2.0,
			focal_y: 0.2,
			filename: Some("hero.png".into()),
			edits: None,
		},
	};
	let changed = editor.call(&change).await.unwrap();
	assert_eq!(changed.title, "Hero");
	assert_eq!(changed.filename, "hero.png");
	assert_eq!((changed.focal_x, changed.focal_y), (1.0, 0.2));
	assert_eq!(get(&editor, &changed.url(None)).await.0, StatusCode::OK);

	// another file, the focal point starts new
	let resp = editor
		.upload(
			&format!("/assets/{}/file", a.id),
			&[("new.png", &png(8, 8))],
		)
		.await;
	let replaced: Asset = Client::json(resp).await.unwrap();
	assert_eq!(replaced.filename, "new.png");
	assert_eq!((replaced.width, replaced.focal_x), (Some(8), 0.5));

	editor.call(&DeleteReq { id: a.id }).await.unwrap();
	let err = editor.call(&DeleteReq { id: a.id }).await.unwrap_err();
	assert_eq!(err, Error::NotFound);
	assert_eq!(
		get(&editor, &replaced.url(None)).await.0,
		StatusCode::NOT_FOUND
	);
}

#[tokio::test]
async fn roles_and_assets() {
	let ts = TestServer::new(site_with_roles());
	let author = ts.user(Some("author"));
	let a = upload(&author, "a.png", &png(4, 4)).await.unwrap();

	// an author edits, but doesn't delete
	let change = UpdateReq {
		id: a.id,
		change: AssetChange {
			alt: "x".into(),
			..Default::default()
		},
	};
	author.call(&change).await.unwrap();
	let err = author.call(&DeleteReq { id: a.id }).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);

	let anon = ts.anonymous();
	let err = anon.call(&ListReq::default()).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
	let err = upload(&anon, "b.png", &png(4, 4)).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn edits_only_images() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let pdf = upload(&admin, "doc.pdf", b"%PDF-1.4").await.unwrap();
	assert_eq!(pdf.kind, "file");
	let edits: Edits = serde_json::from_value(json!({ "rotate": 90 })).unwrap();
	let change = UpdateReq {
		id: pdf.id,
		change: AssetChange {
			edits: Some(edits),
			..Default::default()
		},
	};
	let err = admin.call(&change).await.unwrap_err();
	assert!(matches!(err, Error::Request(m) if m.contains("only images")));
}

#[tokio::test]
async fn usage_shows_what_the_role_sees() {
	let schema = site_with_roles()
		.role(Role::new("media", "Media").assets(crate::Access::View));
	let ts = TestServer::new(schema);
	let admin = ts.admin();
	let a = upload(&admin, "a.png", &png(4, 4)).await.unwrap();
	{
		let conn = ts.state.db.conn();
		let fields = json!({ "image": [a.id] });
		save(
			&conn,
			&ts.state.schema,
			None,
			input("events", "event", "Jazz", fields),
			None,
		)
		.unwrap();
	}

	let used = admin.call(&UsageReq { id: a.id }).await.unwrap();
	assert_eq!(used.len(), 1);
	assert_eq!((used[0].title.as_str(), used[0].sites.len()), ("Jazz", 2));

	// a role without events doesn't see where
	let media = ts.user(Some("media"));
	assert!(media.call(&UsageReq { id: a.id }).await.unwrap().is_empty());
}

fn in_folder(folder: &str) -> ListReq {
	ListReq {
		folder: Some(folder.into()),
		..Default::default()
	}
}

#[tokio::test]
async fn folders_hold_assets() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let author = ts.user(Some("author"));

	// nested, with a name of their own in their place
	let create = |parent_id, name: &str| CreateFolderReq {
		parent_id,
		name: name.into(),
	};
	let photos = author.call(&create(None, "Photos")).await.unwrap();
	let events = author.call(&create(Some(photos.id), " Events ")).await;
	let events = events.unwrap();
	assert_eq!(events.name, "Events");
	assert_eq!(events.parent_id, Some(photos.id));
	let twice = author.call(&create(Some(photos.id), "events")).await;
	assert!(matches!(twice.unwrap_err(), Error::Request(_)));
	let missing = author.call(&create(Some(999), "x")).await;
	assert_eq!(missing.unwrap_err(), Error::NotFound);

	// renamed and moved, not into itself
	let rename = UpdateFolderReq {
		id: photos.id,
		parent_id: None,
		name: "Pictures".into(),
	};
	assert_eq!(author.call(&rename).await.unwrap().name, "Pictures");
	let into_own = UpdateFolderReq {
		id: photos.id,
		parent_id: Some(events.id),
		name: "Pictures".into(),
	};
	let err = author.call(&into_own).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));

	// an upload into a folder, one on the top, and a move
	let resp = author
		.upload(
			&format!("/assets?folder={}", events.id),
			&[("a.png", &png(2, 2))],
		)
		.await;
	let a = Client::json::<Vec<Asset>>(resp).await.unwrap().remove(0);
	assert_eq!(a.folder_id, Some(events.id));
	let top = upload(&author, "b.png", &png(2, 2)).await.unwrap();
	assert_eq!(top.folder_id, None);
	let ids = |list: Vec<Asset>| list.iter().map(|a| a.id).collect::<Vec<_>>();
	let here = in_folder(&events.id.to_string());
	assert_eq!(ids(author.call(&here).await.unwrap()), [a.id]);
	assert_eq!(ids(author.call(&in_folder("top")).await.unwrap()), [top.id]);
	assert_eq!(author.call(&ListReq::default()).await.unwrap().len(), 2);

	let moved = MoveReq {
		ids: vec![top.id],
		folder_id: Some(photos.id),
	};
	author.call(&moved).await.unwrap();
	let list = author.call(&FoldersReq {}).await.unwrap();
	assert_eq!(list.top, 0);
	let all = list.folders;
	let count = |id| all.iter().find(|f| f.id == id).unwrap().assets;
	assert_eq!((count(photos.id), count(events.id)), (1, 1));

	// only empty folders go, and not for who may only edit
	let delete = DeleteFolderReq { id: events.id };
	assert_eq!(author.call(&delete).await.unwrap_err(), Error::Forbidden);
	let err = editor.call(&delete).await.unwrap_err();
	assert!(matches!(err, Error::Conflict(_)));
	let out = MoveReq {
		ids: vec![a.id],
		folder_id: None,
	};
	editor.call(&out).await.unwrap();
	editor.call(&delete).await.unwrap();
	let list = editor.call(&FoldersReq {}).await.unwrap();
	assert_eq!((list.folders.len(), list.top), (1, 1));
}

#[tokio::test]
async fn only_roles_with_assets_see_folders() {
	let ts = TestServer::new(site_with_roles());
	let none = ts.user(Some("nobody"));
	let err = none.call(&FoldersReq {}).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	let create = CreateFolderReq {
		parent_id: None,
		name: "x".into(),
	};
	assert_eq!(none.call(&create).await.unwrap_err(), Error::Forbidden);
	let err = ts.anonymous().call(&FoldersReq {}).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn lists_files_by_extension() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	upload(&editor, "a.png", &png(2, 2)).await.unwrap();
	upload(&editor, "Form.PDF", b"%PDF-1.4").await.unwrap();
	upload(&editor, "notes.txt", b"hi").await.unwrap();

	let names = |list: Vec<Asset>| {
		let mut n: Vec<_> = list.into_iter().map(|a| a.filename).collect();
		n.sort();
		n
	};
	let by = |ext: &str| ListReq {
		extensions: Some(ext.into()),
		..Default::default()
	};
	let pdf = editor.call(&by("pdf")).await.unwrap();
	assert_eq!(names(pdf), ["Form.PDF"]);
	let two = editor.call(&by(".txt, png")).await.unwrap();
	assert_eq!(names(two), ["a.png", "notes.txt"]);
}

#[tokio::test]
async fn a_file_named_like_the_transforms() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	// a file called t, and one that tries the folder of the transforms
	for name in ["t", ".t"] {
		let a = upload(&editor, name, &png(4, 4)).await.unwrap();
		assert_ne!(a.filename, TRANSFORMS);
		let (status, _) = get(&ts.anonymous(), &a.url(Some("small"))).await;
		assert_eq!(status, StatusCode::OK, "{}", a.filename);
		let (status, body) = get(&ts.anonymous(), &a.url(None)).await;
		assert_eq!((status, body), (StatusCode::OK, png(4, 4)));
	}
}

#[tokio::test]
async fn search_takes_percent_and_underscore_as_they_are() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	upload(&editor, "a_b.png", &png(2, 2)).await.unwrap();
	upload(&editor, "axb.png", &png(2, 2)).await.unwrap();
	let find = |s: &str| ListReq {
		search: Some(s.into()),
		..Default::default()
	};
	let found = editor.call(&find("a_b")).await.unwrap();
	assert_eq!(found.len(), 1, "{found:?}");
	assert!(editor.call(&find("%")).await.unwrap().is_empty());
}
