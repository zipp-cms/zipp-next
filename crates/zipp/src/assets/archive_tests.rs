use std::io::Cursor;

use axum::http::StatusCode;

use super::api::*;
use super::tests::zip_of;
use super::*;
use crate::fixtures::{png, site_with_roles};
use crate::schema::Role;
use crate::testing::{Client, TestServer};

/// The paths in a zip, sorted, and the content of each.
fn unpacked(data: &[u8]) -> Vec<(String, Vec<u8>)> {
	use std::io::Read;
	let mut zip = zip::ZipArchive::new(Cursor::new(data)).unwrap();
	let mut out = vec![];
	for i in 0..zip.len() {
		let mut f = zip.by_index(i).unwrap();
		let mut content = vec![];
		f.read_to_end(&mut content).unwrap();
		out.push((f.name().unwrap().to_string(), content));
	}
	out.sort();
	out
}

fn paths(files: &[(String, Vec<u8>)]) -> Vec<&str> {
	files.iter().map(|(p, _)| p.as_str()).collect()
}

#[tokio::test]
async fn downloads_a_folder_as_zip() {
	let schema = site_with_roles()
		.role(Role::new("media", "Media").assets(crate::Access::View));
	let ts = TestServer::new(schema);
	let editor = ts.user(Some("editor"));
	let create = |parent_id, name: &str| CreateFolderReq {
		parent_id,
		name: name.into(),
	};
	let photos = editor.call(&create(None, "Photos")).await.unwrap();
	let events = editor.call(&create(Some(photos.id), "Events")).await;
	let events = events.unwrap();
	let into = |id: i64| format!("/assets?folder={id}");
	let up = async |path: &str, name: &str, data: &[u8]| {
		let resp = editor.upload(path, &[(name, data)]).await;
		Client::json::<Vec<Asset>>(resp).await.unwrap().remove(0)
	};
	let a = up(&into(photos.id), "a.png", &png(2, 2)).await;
	up(&into(photos.id), "A.png", &png(3, 3)).await;
	up(&into(events.id), "notes.txt", b"hello").await;
	let gone = up(&into(events.id), "old.txt", b"old").await;
	editor.call(&DeleteReq { id: gone.id }).await.unwrap();
	let top = up("/assets", "top.txt", b"top").await;

	// the tree below it, two of the same name told apart
	let media = ts.user(Some("media"));
	let path = format!("/api/assets/zip?folder={}", photos.id);
	let resp = media.get(&path).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let files = unpacked(&Client::bytes(resp).await);
	assert_eq!(paths(&files), ["A-2.png", "Events/notes.txt", "a.png"]);
	assert_eq!(files[0].1, png(3, 3));
	assert_eq!(files[1].1, b"hello");

	// assets by id, the trashed one left out
	let ids = format!("/api/assets/zip?ids={},{},{}", a.id, top.id, gone.id);
	let resp = media.get(&ids).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let files = unpacked(&Client::bytes(resp).await);
	assert_eq!(paths(&files), ["a.png", "top.txt"]);
	// an id that isn't one is an error, like in a query of entries
	let bad = format!("/api/assets/zip?ids={},x", a.id);
	let resp = media.get(&bad).await;
	assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

	let resp = media.get("/api/assets/zip?folder=999").await;
	assert_eq!(resp.status(), StatusCode::NOT_FOUND);
	let resp = ts.user(Some("nobody")).get(&path).await;
	assert_eq!(resp.status(), StatusCode::FORBIDDEN);
	let resp = ts.anonymous().get(&path).await;
	assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn unzips_into_folders() {
	let ts = TestServer::new(site_with_roles());
	let author = ts.user(Some("author"));
	let create = |parent_id, name: &str| CreateFolderReq {
		parent_id,
		name: name.into(),
	};
	let target = author.call(&create(None, "Target")).await.unwrap();
	let existing = author.call(&create(Some(target.id), "photos")).await;
	let existing = existing.unwrap();

	let zip = zip_of(&[
		("Photos/", b""),
		("Photos/a.png", &png(2, 2)),
		("Photos/Sub/b.txt", b"b"),
		("Empty/", b""),
		("readme.txt", b"read me"),
		("__MACOSX/Photos/._a.png", b"x"),
		(".DS_Store", b"x"),
		("Photos/.hidden/c.txt", b"x"),
	]);
	let path = format!("/assets/unzip?folder={}", target.id);
	let resp = author.upload(&path, &[("photos.zip", &zip)]).await;
	let done: Unzipped = Client::json(resp).await.unwrap();
	assert!(done.skipped.is_empty());
	let mut names: Vec<_> =
		done.assets.iter().map(|a| a.filename.as_str()).collect();
	names.sort();
	assert_eq!(names, ["a.png", "b.txt", "readme.txt"]);

	// Photos went into the photos that was there
	let all = author.call(&FoldersReq {}).await.unwrap().folders;
	let named = |name: &str| all.iter().find(|f| f.name == name).unwrap();
	assert_eq!(all.len(), 4, "{all:?}");
	assert_eq!(named("Sub").parent_id, Some(existing.id));
	assert_eq!(named("Empty").parent_id, Some(target.id));
	let folder_of = |name: &str| {
		done.assets
			.iter()
			.find(|a| a.filename == name)
			.unwrap()
			.folder_id
	};
	assert_eq!(folder_of("a.png"), Some(existing.id));
	assert_eq!(folder_of("b.txt"), Some(named("Sub").id));
	assert_eq!(folder_of("readme.txt"), Some(target.id));
	let a = done.assets.iter().find(|a| a.filename == "a.png").unwrap();
	assert_eq!((a.kind.as_str(), a.width), ("image", Some(2)));

	// a role that only views doesn't
	let nobody = ts.user(Some("nobody"));
	let resp = nobody.upload(&path, &[("photos.zip", &zip)]).await;
	let err = Client::json::<Unzipped>(resp).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
}

#[tokio::test]
async fn refuses_zips_out_of_bounds() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let unzip = async |zip: &[u8]| {
		let resp = editor.upload("/assets/unzip", &[("x.zip", zip)]).await;
		Client::json::<Unzipped>(resp).await
	};
	for evil in ["../evil.txt", "a/../../evil.txt", "/etc/evil.txt"] {
		let zip = zip_of(&[("ok.txt", b"ok"), (evil, b"x")]);
		let err = unzip(&zip).await.unwrap_err();
		assert!(matches!(err, Error::Request(m) if m.contains("out of")));
	}
	let err = unzip(b"no zip").await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));

	let names: Vec<String> = (0..=2000).map(|i| format!("{i}.txt")).collect();
	let many: Vec<(&str, &[u8])> =
		names.iter().map(|n| (n.as_str(), &b""[..])).collect();
	let err = unzip(&zip_of(&many)).await.unwrap_err();
	assert!(matches!(err, Error::Request(m) if m.contains("2000 files")));

	// nothing came of them
	assert!(editor.call(&ListReq::default()).await.unwrap().is_empty());
	let folders = editor.call(&FoldersReq {}).await.unwrap();
	assert!(folders.folders.is_empty());
}
