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

	let a = store(&db.conn(), &dir, "test image.png", &png).unwrap();
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

	let f = store(&db.conn(), &dir, "notes.txt", b"hello").unwrap();
	assert_eq!((f.kind.as_str(), f.mime.as_str()), ("file", "text/plain"));
	// html is no type of ours, whatever the upload says
	let f = store(&db.conn(), &dir, "x.html", b"<script>").unwrap();
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
