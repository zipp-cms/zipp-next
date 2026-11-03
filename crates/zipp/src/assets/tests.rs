use std::io::Cursor;

use axum::http::header;
use image::ImageFormat;

use super::*;
use crate::fixtures::png;

#[test]
fn cleans_filenames() {
	assert_eq!(clean_filename("../../etc/Hä llo.JPG"), "H--llo.JPG");
	assert_eq!(clean_filename(".."), "file");
}

#[test]
fn sizes_after_transforms() {
	let fit = Transform::new("m").width(1500).height(1000);
	assert_eq!(transform_size(3000, 1000, &fit), (1500, 500));
	assert_eq!(transform_size(800, 600, &fit), (800, 600));
	let w = Transform::new("s").width(600);
	assert_eq!(transform_size(1200, 900, &w), (600, 450));

	let crop = Transform::new("sq").width(500).height(500).crop();
	assert_eq!(transform_size(2000, 1000, &crop), (500, 500));
	// smaller than the transform: the biggest square that fits
	assert_eq!(transform_size(400, 300, &crop), (300, 300));
}

#[test]
fn crops_around_the_focal_point() {
	// a square out of a wide image, focal point right
	assert_eq!(
		crop_rect(2000, 1000, 500, 500, (0.9, 0.5)),
		(1000, 0, 1000, 1000)
	);
	// centered
	assert_eq!(
		crop_rect(2000, 1000, 500, 500, (0.5, 0.5)),
		(500, 0, 1000, 1000)
	);
	// a wide one out of a tall image, focal point at the top
	assert_eq!(
		crop_rect(1000, 2000, 1000, 500, (0.5, 0.0)),
		(0, 0, 1000, 500)
	);
}

#[test]
fn stores_and_transforms_an_image() {
	let db = crate::db::Db::memory().unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let png = png(300, 200);

	let a = store(&db.conn(), dir, "test image.png", &png, None).unwrap();
	assert_eq!(
		(a.kind.as_str(), a.width, a.height),
		("image", Some(300), Some(200))
	);
	assert_eq!(
		a.url(Some("small")),
		format!("/assets/{}/small/test-image.png", a.id)
	);
	// another type is another extension, also without one
	assert_eq!(a.format_filename(Format::Webp), "test-image.webp");
	let plain = Asset {
		filename: "scan".into(),
		..a.clone()
	};
	assert_eq!(plain.format_filename(Format::Jpeg), "scan.jpg");

	let t = Transform::new("sq").width(100).height(100).crop();
	let out = dir.join("out.png");
	let original = dir.join(a.id.to_string()).join(&a.filename);
	generate(&a, &original, &t, "image/png", &out).unwrap();
	let img = image::open(&out).unwrap();
	assert_eq!((img.width(), img.height()), (100, 100));

	let f = store(&db.conn(), dir, "notes.txt", b"hello", None).unwrap();
	assert_eq!((f.kind.as_str(), f.mime.as_str()), ("file", "text/plain"));
	let v = store(&db.conn(), dir, "clip.mp4", b"v", None).unwrap();
	assert_eq!((v.kind.as_str(), v.mime.as_str()), ("video", "video/mp4"));
	// html is no type of ours, whatever the upload says
	let f = store(&db.conn(), dir, "x.html", b"<script>", None).unwrap();
	assert_eq!(f.mime, "application/octet-stream");
}

#[test]
fn files_never_run_as_pages() {
	let h: std::collections::HashMap<_, _> =
		file_headers("image/svg+xml").into();
	assert_eq!(h[&header::CONTENT_DISPOSITION], "attachment");
	assert_eq!(h[&header::CONTENT_SECURITY_POLICY], "sandbox");
	assert_eq!(h[&header::X_CONTENT_TYPE_OPTIONS], "nosniff");
	// a file in the trash leaves caches soon
	assert_eq!(h[&header::CACHE_CONTROL], "public, max-age=300");
	let h: std::collections::HashMap<_, _> = file_headers("image/png").into();
	assert_eq!(h[&header::CONTENT_DISPOSITION], "inline");
}

#[test]
fn edits_change_the_image_not_the_file() {
	let edits = Edits {
		rotate: 90,
		crop: Some(Crop {
			x: 0.0,
			y: 0.0,
			w: 1.0,
			h: 0.5,
		}),
		canvas: Some(Canvas {
			width: 400,
			height: 400,
		}),
		// no background: transparent
		..Default::default()
	};
	edits.check().unwrap();
	// 300x200 turned is 200x300, half its height 200x150, on 400x400
	assert_eq!(edits.size(300, 200), (400, 400));
	let resized = Edits {
		resize: Some(Resize {
			width: 150,
			height: None,
		}),
		..Default::default()
	};
	assert_eq!(resized.size(300, 200), (150, 100));
	// past the image: the background around it
	let past = Edits {
		crop: Some(Crop {
			x: -0.5,
			y: 0.0,
			w: 1.5,
			h: 1.0,
		}),
		background: Some("#0000ff".into()),
		..Default::default()
	};
	past.check().unwrap();
	let red = image::RgbImage::from_pixel(100, 50, image::Rgb([255, 0, 0]));
	let img = past.apply(red.into()).to_rgba8();
	assert_eq!(img.dimensions(), (150, 50));
	assert_eq!(img.get_pixel(10, 10).0, [0, 0, 255, 255]);
	assert_eq!(img.get_pixel(100, 10).0, [255, 0, 0, 255]);
	let far = Edits {
		crop: Some(Crop {
			x: -1.5,
			y: 0.0,
			w: 1.0,
			h: 1.0,
		}),
		..Default::default()
	};
	assert!(far.check().is_err());
	let bad = Edits {
		rotate: 45,
		..Default::default()
	};
	assert!(bad.check().is_err());

	let db = crate::db::Db::memory().unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let mut jpg = vec![];
	image::RgbImage::from_pixel(300, 200, image::Rgb([200, 30, 30]))
		.write_to(&mut Cursor::new(&mut jpg), ImageFormat::Jpeg)
		.unwrap();
	let a = store(&db.conn(), dir, "red.jpg", &jpg, None).unwrap();
	let conn = db.conn();
	conn.execute(
		"UPDATE assets SET edits = ?1 WHERE id = ?2",
		params![serde_json::to_string(&edits).unwrap(), a.id],
	)
	.unwrap();
	let a = get_asset(&conn, a.id).unwrap();
	// the size after the edits, the uploaded one stays known
	assert_eq!((a.width, a.height), (Some(400), Some(400)));
	assert_eq!(
		(a.original_width, a.original_height),
		(Some(300), Some(200))
	);
	// transparent needs a png
	assert_eq!(a.output_mime(), "image/png");

	let original = dir.join(a.id.to_string()).join(&a.filename);
	let out = dir.join("edited");
	let edited = Transform::new("_edited");
	generate(&a, &original, &edited, a.output_mime(), &out).unwrap();
	let img = image::load_from_memory(&fs::read(&out).unwrap())
		.unwrap()
		.to_rgba8();
	assert_eq!(img.dimensions(), (400, 400));
	// a corner of the canvas is transparent, the middle the red image
	assert_eq!(img.get_pixel(0, 0)[3], 0);
	assert!(img.get_pixel(200, 200)[0] > 150);
	// the uploaded file is untouched
	assert_eq!(image::open(&original).unwrap().width(), 300);
}

#[test]
fn edits_stay_within_the_pixel_limit() {
	let db = crate::db::Db::memory().unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let mut png = vec![];
	image::GrayImage::new(1, 40_000)
		.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
		.unwrap();
	let conn = db.conn();
	let a = store(&conn, dir, "line.png", &png, None).unwrap();

	// 10000 wide keeps the ratio at 400 million high: capped, but the
	// resize would still go through 10000x40000
	let edits = Edits {
		resize: Some(Resize {
			width: 10_000,
			height: None,
		}),
		canvas: Some(Canvas {
			width: 100,
			height: 100,
		}),
		..Default::default()
	};
	assert_eq!(edits.size(1, 40_000), (100, 100));
	assert_eq!(
		Edits {
			canvas: None,
			..edits.clone()
		}
		.size(1, 40_000),
		(10_000, 10_000)
	);
	let change = AssetChange {
		edits: Some(edits.clone()),
		..Default::default()
	};
	let schema = crate::schema::Schema::new();
	let err = update(&conn, dir, &schema, a.id, change, None).unwrap_err();
	assert!(err.to_string().contains("40 megapixels"), "{err}");

	// edits that got in anyway fail their transform, nothing is decoded
	// into a giant image
	conn.execute(
		"UPDATE assets SET edits = ?1 WHERE id = ?2",
		params![serde_json::to_string(&edits).unwrap(), a.id],
	)
	.unwrap();
	let a = get_asset(&conn, a.id).unwrap();
	let original = dir.join(a.id.to_string()).join(&a.filename);
	let out = dir.join("edited.png");
	let t = Transform::new("_edited");
	let err = generate(&a, &original, &t, "image/png", &out).unwrap_err();
	assert!(matches!(err, Error::Internal(_)), "{err}");
	assert!(!out.exists());
}

#[test]
fn knows_where_assets_are_used() {
	use crate::entries::save;
	use crate::testing::input;
	let db = crate::db::Db::memory().unwrap();
	let s = crate::fixtures::site();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let png = png(20, 10);
	let conn = db.conn();
	let a = store(&conn, dir, "hero.png", &png, None).unwrap();
	let fields = serde_json::json!({ "image": [a.id] });
	let e = save(
		&conn,
		&s,
		None,
		input("events", "event", "Jazz", fields),
		None,
	)
	.unwrap();
	let used = |conn: &Connection| -> Vec<(i64, String)> {
		let mut stmt = conn
			.prepare(
				"SELECT entry_id, path FROM asset_relations WHERE asset_id = ?1",
			)
			.unwrap();
		stmt.query_map([a.id], |r| Ok((r.get(0)?, r.get(1)?)))
			.unwrap()
			.collect::<rusqlite::Result<_>>()
			.unwrap()
	};
	// in every site, the image field is the same in all
	assert!(
		used(&conn)
			.iter()
			.all(|u| *u == (e.id, "image".to_string()))
	);
	assert!(!used(&conn).is_empty());

	// content from before the index is found once on start
	conn.execute("DELETE FROM asset_relations", []).unwrap();
	index_usage(&conn, &s).unwrap();
	assert!(!used(&conn).is_empty());
}

/// A zip of these files, a name ending in `/` is a folder.
pub(crate) fn zip_of(files: &[(&str, &[u8])]) -> Vec<u8> {
	use std::io::Write;
	let mut zip = zip::ZipWriter::new(Cursor::new(vec![]));
	let options = zip::write::SimpleFileOptions::default();
	for (name, data) in files {
		if name.ends_with('/') {
			zip.add_directory(*name, options).unwrap();
		} else {
			zip.start_file(*name, options).unwrap();
			zip.write_all(data).unwrap();
		}
	}
	zip.finish().unwrap().into_inner()
}

/// Reads a zip and unpacks it on the top.
fn unzip_top(
	conn: &Connection,
	files: &FsPath,
	zip: &[u8],
	limits: &Limits,
) -> Result<Unzipped> {
	let zip = read_zip(files, Cursor::new(zip), limits)?;
	unzip(conn, files, zip, None, None)
}

#[test]
fn unzips_within_limits() {
	let db = crate::db::Db::memory().unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let conn = db.conn();
	let zip = zip_of(&[("a.txt", b"hello"), ("b.txt", b"world!")]);
	let limits = Limits {
		files: 2,
		folders: 2,
		depth: 2,
		size: 11,
		file: 6,
	};
	let refused = |zip: &[u8], limits: Limits| {
		let err = unzip_top(&conn, dir, zip, &limits).unwrap_err();
		assert!(matches!(err, Error::Request(_)), "{err:?}");
	};
	refused(&zip, Limits { files: 1, ..limits });
	refused(&zip, Limits { size: 10, ..limits });
	refused(&zip, Limits { file: 5, ..limits });

	// folders count too, also the ones only in the path of a file, and
	// how deep they go
	let folders = zip_of(&[("x/", b""), ("y/", b""), ("z/a.txt", b"a")]);
	refused(&folders, limits);
	let deep = zip_of(&[("x/y/z/a.txt", b"a")]);
	refused(&deep, limits);
	let fine = zip_of(&[("x/", b""), ("X/y/a.txt", b"a")]);
	let done = unzip_top(&conn, dir, &fine, &limits).unwrap();
	assert_eq!(done.assets.len(), 1);
	assert_eq!(folders_of(&conn), ["x", "y"]);

	let done = unzip_top(&conn, dir, &zip, &limits).unwrap();
	assert_eq!(done.assets.len(), 2);
}

fn folders_of(conn: &Connection) -> Vec<String> {
	folders(conn).unwrap().into_iter().map(|f| f.name).collect()
}

#[test]
fn folder_names_are_no_paths() {
	let db = crate::db::Db::memory().unwrap();
	let conn = db.conn();
	for name in ["a/b", "a\\b", "..", " "] {
		let err = create_folder(&conn, None, name).unwrap_err();
		assert!(matches!(err, Error::Request(_)), "{name}");
	}
	let f = create_folder(&conn, None, "a.b").unwrap();
	assert_eq!(folder(&conn, f.id).unwrap().name, "a.b");
	assert!(matches!(folder(&conn, f.id + 1), Err(Error::NotFound)));
}

#[test]
fn a_zip_that_holds_more_than_it_says_stores_nothing() {
	let db = crate::db::Db::memory().unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let mut zip = zip_of(&[("a.txt", b"fine"), ("b.txt", b"0123456789")]);
	// b.txt says it is 2 bytes, in its header and the directory
	for (sig, at) in [([0x50, 0x4b, 3, 4], 22), ([0x50, 0x4b, 1, 2], 24)] {
		let starts: Vec<usize> = zip
			.windows(4)
			.enumerate()
			.filter(|(_, w)| *w == sig)
			.map(|(i, _)| i)
			.collect();
		let i = starts[1] + at;
		zip[i..i + 4].copy_from_slice(&2u32.to_le_bytes());
	}

	let conn = db.conn();
	let err = unzip_top(&conn, dir, &zip, &Limits::default());
	let larger = matches!(&err, Err(Error::Request(m)) if m.contains("larger"));
	assert!(larger, "{err:?}");
	let n: i64 = conn
		.query_row("SELECT COUNT(*) FROM assets", [], |r| r.get(0))
		.unwrap();
	assert_eq!(n, 0);
	let left = fs::read_dir(dir).map(|d| d.count()).unwrap_or(0);
	assert_eq!(left, 0);
}

#[test]
fn kinds_match_in_sql_like_in_rust() {
	let db = crate::db::Db::memory().unwrap();
	let tmp = tempfile::tempdir().unwrap();
	let dir = tmp.path();
	let conn = db.conn();
	for (name, data) in [
		("a.png", png(2, 2)),
		("clip.MOV", b"v".to_vec()),
		("song.mp3", b"a".to_vec()),
		("Plan.PDF", b"%PDF".to_vec()),
		("old.doc", b"w".to_vec()),
		("notes.txt", b"t".to_vec()),
	] {
		store(&conn, dir, name, &data, None).unwrap();
	}
	let all = list(&conn, &AssetList::default()).unwrap();
	for (kind, want) in [
		("image", vec!["a.png"]),
		("video", vec!["clip.MOV"]),
		("file", vec!["notes.txt", "old.doc", "Plan.PDF", "song.mp3"]),
		("document", vec!["notes.txt", "old.doc", "Plan.PDF"]),
		("pdf", vec!["Plan.PDF"]),
		("word", vec!["old.doc"]),
	] {
		let k = Kind::parse(kind).unwrap();
		let q = AssetList {
			kinds: vec![k],
			..Default::default()
		};
		let names = |l: Vec<&Asset>| -> Vec<String> {
			l.into_iter().map(|a| a.filename.clone()).collect()
		};
		let in_sql = list(&conn, &q).unwrap();
		assert_eq!(names(in_sql.iter().collect()), want, "{kind}");
		let in_rust = all.iter().filter(|a| k.of(a)).collect();
		assert_eq!(names(in_rust), want, "{kind}");
		assert_eq!(count(&conn, &q).unwrap(), want.len() as i64);
	}
	assert!(Kind::parse("movie").is_err());
}

#[test]
fn folders_have_paths() {
	let db = crate::db::Db::memory().unwrap();
	let conn = db.conn();
	let clubs = create_folder(&conn, None, "Clubs").unwrap().id;
	let bern = create_folder(&conn, Some(clubs), "Bern").unwrap().id;
	let other = create_folder(&conn, None, "Clubs old").unwrap().id;
	let paths = folder_paths(&conn).unwrap();
	assert_eq!(paths[&bern], "Clubs/Bern");
	assert_eq!(paths[&other], "Clubs old");

	// with the ones in it, not the ones that only start alike
	assert_eq!(folders_under(&paths, "clubs/"), [clubs, bern]);
	assert_eq!(folders_under(&paths, "Clubs/bern"), [bern]);
	assert!(folders_under(&paths, "Bern").is_empty());
}
