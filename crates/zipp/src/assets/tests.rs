use super::*;

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
	let dir = std::env::temp_dir()
		.join(format!("zipp-test-{}", rand::random::<u32>()));
	let mut png = vec![];
	image::RgbImage::new(300, 200)
		.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
		.unwrap();

	let a = store(&db.conn(), &dir, "test image.png", &png, None).unwrap();
	assert_eq!(
		(a.kind.as_str(), a.width, a.height),
		("image", Some(300), Some(200))
	);
	assert_eq!(
		a.url(Some("small")),
		format!("/assets/{}/small/test-image.png", a.id)
	);

	let t = Transform::new("sq").width(100).height(100).crop();
	let out = dir.join("out.png");
	generate(&a, &dir.join(a.id.to_string()).join(&a.filename), &t, &out)
		.unwrap();
	let img = image::open(&out).unwrap();
	assert_eq!((img.width(), img.height()), (100, 100));

	let f = store(&db.conn(), &dir, "notes.txt", b"hello", None).unwrap();
	assert_eq!((f.kind.as_str(), f.mime.as_str()), ("file", "text/plain"));
	// html is no type of ours, whatever the upload says
	let f = store(&db.conn(), &dir, "x.html", b"<script>", None).unwrap();
	assert_eq!(f.mime, "application/octet-stream");
	fs::remove_dir_all(dir).unwrap();
}

#[test]
fn files_never_run_as_pages() {
	let r = file_response("image/svg+xml", vec![]);
	let h = r.headers();
	assert_eq!(h[header::CONTENT_DISPOSITION], "attachment");
	assert_eq!(h[header::CONTENT_SECURITY_POLICY], "sandbox");
	assert_eq!(h[header::X_CONTENT_TYPE_OPTIONS], "nosniff");
	let r = file_response("image/png", vec![]);
	assert_eq!(r.headers()[header::CONTENT_DISPOSITION], "inline");
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
	let dir = std::env::temp_dir()
		.join(format!("zipp-test-{}", rand::random::<u32>()));
	let mut jpg = vec![];
	image::RgbImage::from_pixel(300, 200, image::Rgb([200, 30, 30]))
		.write_to(&mut Cursor::new(&mut jpg), ImageFormat::Jpeg)
		.unwrap();
	let a = store(&db.conn(), &dir, "red.jpg", &jpg, None).unwrap();
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
	generate(&a, &original, &Transform::new("_edited"), &out).unwrap();
	let img = image::load_from_memory(&fs::read(&out).unwrap())
		.unwrap()
		.to_rgba8();
	assert_eq!(img.dimensions(), (400, 400));
	// a corner of the canvas is transparent, the middle the red image
	assert_eq!(img.get_pixel(0, 0)[3], 0);
	assert!(img.get_pixel(200, 200)[0] > 150);
	// the uploaded file is untouched
	assert_eq!(image::open(&original).unwrap().width(), 300);
	fs::remove_dir_all(dir).unwrap();
}

#[test]
fn knows_where_assets_are_used() {
	use crate::entries::save;
	use crate::entries::tests::input;
	let db = crate::db::Db::memory().unwrap();
	let s = crate::schema::tests::site();
	let dir = std::env::temp_dir()
		.join(format!("zipp-test-{}", rand::random::<u32>()));
	let mut png = vec![];
	image::RgbImage::new(20, 10)
		.write_to(&mut Cursor::new(&mut png), ImageFormat::Png)
		.unwrap();
	let conn = db.conn();
	let a = store(&conn, &dir, "hero.png", &png, None).unwrap();
	let fields = serde_json::json!({ "image": [a.id] });
	let e = save(&conn, &s, None, input("events", "event", "Jazz", fields))
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
	fs::remove_dir_all(dir).unwrap();
}
