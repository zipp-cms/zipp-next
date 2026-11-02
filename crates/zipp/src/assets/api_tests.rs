use std::fs;
use std::io::Cursor;

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use image::ImageFormat;
use serde_json::{Value, json};

use super::api::*;
use super::*;
use crate::entries::save;
use crate::entries::tests::input;
use crate::schema::tests::site_with_roles;
use crate::schema::{Field, Format, Role};
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
	let many: Vec<String> = (1..=1001).map(|i| i.to_string()).collect();
	let too_many = ListReq {
		ids: Some(many.join(",")),
		..Default::default()
	};
	let err = editor.call(&too_many).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)), "{err:?}");

	let change = UpdateReq {
		id: a.id,
		change: AssetChange {
			title: Some(" Hero ".into()),
			alt: [("en".into(), "A hero".into())].into(),
			focal_x: 2.0,
			focal_y: 0.2,
			filename: Some("hero.png".into()),
			edits: None,
			fields: None,
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
async fn a_new_file_gets_new_urls() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let a = upload(&editor, "photo.png", &png(20, 10)).await.unwrap();
	assert_eq!(a.version, "");
	let (_, small) = get(&ts.anonymous(), &a.url(Some("small"))).await;

	// the same name, another file: caches can't keep the old one
	let replace = async |size| {
		let path = format!("/assets/{}/file", a.id);
		let resp = editor.upload(&path, &[("photo.png", &png(size, 8))]).await;
		Client::json::<Asset>(resp).await.unwrap()
	};
	let b = replace(8).await;
	assert_eq!(b.filename, a.filename);
	assert_ne!(b.version, "");
	assert_ne!(b.url(None), a.url(None));
	let (status, new) = get(&ts.anonymous(), &b.url(Some("small"))).await;
	assert_eq!(status, StatusCode::OK);
	assert_ne!(new, small);
	let c = replace(8).await;
	assert_ne!(c.version, b.version);

	// a transform is kept with the version it was made of
	get(&ts.anonymous(), &c.url(Some("small"))).await;
	let dir = asset_dir(&ts.state.files, a.id).join(TRANSFORMS);
	let names: Vec<String> = fs::read_dir(dir)
		.unwrap()
		.map(|e| e.unwrap().file_name().into_string().unwrap())
		.collect();
	assert_eq!(names, [format!("small-{}-photo.png", c.version)]);
}

#[tokio::test]
async fn transforms_in_other_formats() {
	let webp = Transform::new("hero").width(10).format(Format::Webp);
	let ts = TestServer::new(site_with_roles().transform(webp.clone()));
	let editor = ts.user(Some("editor"));
	// half transparent
	let mut png = vec![];
	image::RgbaImage::from_fn(20, 10, |x, _| {
		image::Rgba([0, 0, 0, if x < 10 { 0 } else { 255 }])
	})
	.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
	.unwrap();
	let a = upload(&editor, "logo.png", &png).await.unwrap();

	// the transform's format by its url, the extension says which
	let url = a.format_url(&webp, None);
	assert_eq!(url, format!("/assets/{}/hero/logo.webp", a.id));
	let req = Request::get(&url).body(Body::empty()).unwrap();
	let resp = ts.anonymous().send(req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let h = resp.headers();
	assert_eq!(h[header::CONTENT_TYPE], "image/webp");
	// not long in caches, then they ask again with the date
	assert_eq!(h[header::CACHE_CONTROL], "public, max-age=300");
	let modified = h[header::LAST_MODIFIED].clone();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	let again = Request::get(&url)
		.header(header::IF_MODIFIED_SINCE, modified)
		.body(Body::empty())
		.unwrap();
	let resp = ts.anonymous().send(again).await;
	assert_eq!(resp.status(), StatusCode::NOT_MODIFIED);
	let img = image::load_from_memory_with_format(&body, ImageFormat::WebP)
		.unwrap()
		.to_rgba8();
	assert_eq!(img.dimensions(), (10, 5));
	// transparent stays transparent
	assert_eq!(img.get_pixel(0, 2)[3], 0);

	// its own type with its own name, a jpeg on white
	let (status, body) = get(&editor, &a.url(Some("hero"))).await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(image::guess_format(&body).unwrap(), ImageFormat::Png);
	let jpg = a.format_url(&webp, Some(Format::Jpeg));
	let (status, body) = get(&editor, &jpg).await;
	assert_eq!(status, StatusCode::OK);
	let img = image::load_from_memory(&body).unwrap().to_rgb8();
	assert!(img.get_pixel(0, 2)[0] > 240);
	assert!(img.get_pixel(9, 2)[0] < 15);

	// other names and types are not there
	for name in ["logo.gif", "other.webp", "logo.png.webp"] {
		let url = format!("/assets/{}/hero/{name}", a.id);
		assert_eq!(get(&editor, &url).await.0, StatusCode::NOT_FOUND, "{name}");
	}
}

#[tokio::test]
async fn alt_text_per_site() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let a = upload(&editor, "a.png", &png(4, 4)).await.unwrap();
	assert!(a.alt.is_empty());

	let change = UpdateReq {
		id: a.id,
		change: AssetChange {
			alt: [
				("de".into(), "Ein Bild".into()),
				("en".into(), " ".into()),
				("xx".into(), "Gone".into()),
			]
			.into(),
			..Default::default()
		},
	};
	let saved = editor.call(&change).await.unwrap();
	// empty ones and unknown sites are left out
	assert_eq!(saved.alt, [("de".into(), "Ein Bild".into())].into());
	assert_eq!(saved.alt_in("en", "de"), "Ein Bild");
	assert_eq!(saved.alt_in("de", "de"), "Ein Bild");

	// found by the alt text of any site, not by a site's handle
	let found = |q: &str| {
		let conn = ts.state.db.conn();
		find(&conn, &[q.to_string()], 10).unwrap().len()
	};
	assert_eq!(found("bild"), 1);
	assert_eq!(found("de"), 0);
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
			alt: [("en".into(), "x".into())].into(),
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
	// a path on Windows
	let slash = author.call(&create(None, "a\\b")).await;
	assert!(matches!(slash.unwrap_err(), Error::Request(_)));

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
async fn lists_files_by_kind() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	upload(&editor, "a.png", &png(2, 2)).await.unwrap();
	upload(&editor, "clip.mp4", b"video").await.unwrap();
	upload(&editor, "song.mp3", b"audio").await.unwrap();
	upload(&editor, "form.pdf", b"%PDF-1.4").await.unwrap();

	let of = async |kind: &str| {
		let req = ListReq {
			kind: Some(kind.into()),
			..Default::default()
		};
		let list = editor.call(&req).await?;
		let mut names: Vec<_> = list.into_iter().map(|a| a.filename).collect();
		names.sort();
		Ok::<_, Error>(names)
	};
	assert_eq!(of("image").await.unwrap(), ["a.png"]);
	assert_eq!(of("video").await.unwrap(), ["clip.mp4"]);
	assert_eq!(of("document").await.unwrap(), ["form.pdf"]);
	assert_eq!(of("file").await.unwrap().len(), 3);
	let err = of("movie").await.unwrap_err();
	assert!(matches!(err, Error::Request(m) if m.contains("kind movie")));
}

#[tokio::test]
async fn videos_play_in_ranges() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let a = upload(&editor, "clip.mp4", b"0123456789").await.unwrap();
	assert_eq!(a.mime, "video/mp4");

	let req = Request::get(a.url(None))
		.header(header::RANGE, "bytes=2-5")
		.body(Body::empty())
		.unwrap();
	let resp = ts.anonymous().send(req).await;
	assert_eq!(resp.status(), StatusCode::PARTIAL_CONTENT);
	let headers = resp.headers();
	assert_eq!(headers[header::CONTENT_TYPE], "video/mp4");
	assert_eq!(headers[header::CONTENT_SECURITY_POLICY], "sandbox");
	assert_eq!(headers[header::CONTENT_DISPOSITION], "inline");
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	assert_eq!(&body[..], b"2345");
}

#[tokio::test]
async fn asset_fields_are_checked_and_saved() {
	let schema = site_with_roles().asset_fields([
		Field::text("credit").max(10),
		Field::dropdown("licence", [("cc", "CC BY"), ("all", "All rights")]),
		Field::images("poster").max(1),
	]);
	let ts = TestServer::new(schema);
	let editor = ts.user(Some("editor"));
	let clip = upload(&editor, "clip.mp4", b"video").await.unwrap();
	assert_eq!(clip.fields, json!({}));
	let poster = upload(&editor, "poster.png", &png(4, 4)).await.unwrap();

	let change = |fields: Value| UpdateReq {
		id: clip.id,
		change: AssetChange {
			alt: [("en".into(), "A clip".into())].into(),
			fields: Some(fields),
			..Default::default()
		},
	};
	let fields =
		json!({ "credit": "Anna", "licence": "cc", "poster": [poster.id] });
	let saved = editor.call(&change(fields.clone())).await.unwrap();
	assert_eq!(saved.fields, fields);

	// checked like the fields of a block, all mistakes at once
	let wrong =
		json!({ "credit": "Anna Muster-Meier", "licence": "x", "other": 1 });
	let err = editor.call(&change(wrong)).await.unwrap_err();
	let Error::Invalid(errors) = err else {
		panic!("{err:?}")
	};
	let mut paths: Vec<_> = errors.iter().map(|e| e.path.as_str()).collect();
	paths.sort();
	assert_eq!(paths, ["credit", "licence", "other"]);
	// the poster takes only images
	let err = editor
		.call(&change(json!({ "poster": [clip.id] })))
		.await
		.unwrap_err();
	let Error::Invalid(errors) = err else {
		panic!("{err:?}")
	};
	assert_eq!(errors[0].path, "poster");

	// a rename without fields keeps them
	let rename = UpdateReq {
		id: clip.id,
		change: AssetChange {
			filename: Some("film.mp4".into()),
			..Default::default()
		},
	};
	let renamed = editor.call(&rename).await.unwrap();
	assert_eq!(renamed.fields, fields);
	assert_eq!(
		get_asset(&ts.state.db.conn(), clip.id).unwrap().fields,
		fields
	);
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
	let top = upload(&editor, "top.txt", b"top").await.unwrap();

	// the tree below it, two of the same name told apart
	let media = ts.user(Some("media"));
	let path = format!("/api/assets/zip?folder={}", photos.id);
	let (status, body) = get(&media, &path).await;
	assert_eq!(status, StatusCode::OK);
	let files = unpacked(&body);
	assert_eq!(paths(&files), ["A-2.png", "Events/notes.txt", "a.png"]);
	assert_eq!(files[0].1, png(3, 3));
	assert_eq!(files[1].1, b"hello");

	// assets by id, the trashed one left out
	let ids = format!("/api/assets/zip?ids={},{},{}", a.id, top.id, gone.id);
	let (status, body) = get(&media, &ids).await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(paths(&unpacked(&body)), ["a.png", "top.txt"]);
	// an id that isn't one is an error, like in a query of entries
	let bad = format!("/api/assets/zip?ids={},x", a.id);
	let (status, _) = get(&media, &bad).await;
	assert_eq!(status, StatusCode::BAD_REQUEST);

	let (status, _) = get(&media, "/api/assets/zip?folder=999").await;
	assert_eq!(status, StatusCode::NOT_FOUND);
	let (status, _) = get(&ts.user(Some("nobody")), &path).await;
	assert_eq!(status, StatusCode::FORBIDDEN);
	let (status, _) = get(&ts.anonymous(), &path).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
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

	let zip = super::tests::zip_of(&[
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
		let zip = super::tests::zip_of(&[("ok.txt", b"ok"), (evil, b"x")]);
		let err = unzip(&zip).await.unwrap_err();
		assert!(matches!(err, Error::Request(m) if m.contains("out of")));
	}
	let err = unzip(b"no zip").await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));

	let names: Vec<String> = (0..=2000).map(|i| format!("{i}.txt")).collect();
	let many: Vec<(&str, &[u8])> =
		names.iter().map(|n| (n.as_str(), &b""[..])).collect();
	let err = unzip(&super::tests::zip_of(&many)).await.unwrap_err();
	assert!(matches!(err, Error::Request(m) if m.contains("2000 files")));

	// nothing came of them
	assert!(editor.call(&ListReq::default()).await.unwrap().is_empty());
	let folders = editor.call(&FoldersReq {}).await.unwrap();
	assert!(folders.folders.is_empty());
}
