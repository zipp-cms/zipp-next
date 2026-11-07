use serde_json::json;

use super::sort::Sorted;
use super::*;
use crate::entries::api::{CreateReq, UpdateReq};
use crate::fixtures::png;
use crate::schema::{Field, Schema, Section};
use crate::testing::{TestServer, input};

fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.section(Section::channel("topics", ["topic"]).uri("{slug}"))
		.entry(
			"topic",
			"Topic",
			[
				Field::images("image").folder("Topics/{slug}"),
				Field::assets("files").folder("{section}/Files"),
				Field::images("other"),
				Field::blocks("gallery", ["photos"]),
			],
		)
		.block(
			"photos",
			"Photos",
			[Field::images("images").folder("Topics/{slug}/Gallery")],
		)
}

#[tokio::test]
async fn sorts_assets_into_their_fields_folders() {
	let ts = TestServer::new(schema());
	let admin = ts.admin();
	let conn = || ts.state.db.conn();
	let image = |name: &str| {
		store(&conn(), &ts.state.files, name, &png(10, 10), None)
			.unwrap()
			.id
	};
	let (wool, felt, shared, file, other, photo, lost) = (
		image("wool.png"),
		image("felt.png"),
		image("shared.png"),
		image("file.png"),
		image("other.png"),
		image("photo.png"),
		image("lost.png"),
	);
	let topic = |title: &str, fields| {
		CreateReq(input("topics", "topic", title, fields))
	};
	admin
		.call(&topic(
			"Wool",
			json!({
				"image": [wool, shared],
				"files": [file],
				"other": [other],
				"gallery": [{ "type": "photos", "fields": { "images": [photo] } }],
			}),
		))
		.await
		.unwrap();
	admin
		.call(&topic("Felt", json!({ "image": [felt, shared] })))
		.await
		.unwrap();

	let sorted = sort_assets(&conn(), &ts.state.schema).unwrap();
	assert_eq!(
		sorted,
		Sorted {
			moved: 4,
			shared: 1
		}
	);

	let paths = folder_paths(&conn()).unwrap();
	let path = |id: i64| {
		let a = get_asset(&conn(), id).unwrap();
		a.folder_id.map(|f| paths[&f].clone())
	};
	assert_eq!(path(wool).as_deref(), Some("Topics/wool"));
	assert_eq!(path(felt).as_deref(), Some("Topics/felt"));
	assert_eq!(path(photo).as_deref(), Some("Topics/wool/Gallery"));
	// names match without case: {section} goes into Topics
	assert_eq!(path(file).as_deref(), Some("Topics/Files"));
	// in two entries, in a field without a folder, in none
	assert_eq!(path(shared), None);
	assert_eq!(path(other), None);
	assert_eq!(path(lost), None);

	// once is enough
	let again = sort_assets(&conn(), &ts.state.schema).unwrap();
	assert_eq!(
		again,
		Sorted {
			moved: 0,
			shared: 1
		}
	);
}

#[tokio::test]
async fn translated_slugs_are_one_folder() {
	let ts = TestServer::new(schema().site("en", "English", "en-GB"));
	let admin = ts.admin();
	let conn = || ts.state.db.conn();
	let wool = store(&conn(), &ts.state.files, "wool.png", &png(10, 10), None)
		.unwrap()
		.id;
	let fields = json!({ "image": [wool] });
	let entry = admin
		.call(&CreateReq(input("topics", "topic", "Wool", fields.clone())))
		.await
		.unwrap();
	let mut en = input("topics", "topic", "Laine", fields);
	en.site = Some("en".into());
	admin
		.call(&UpdateReq {
			id: entry.id,
			input: en,
		})
		.await
		.unwrap();

	// the slug of the primary site
	let sorted = sort_assets(&conn(), &ts.state.schema).unwrap();
	assert_eq!(
		sorted,
		Sorted {
			moved: 1,
			shared: 0
		}
	);
	let a = get_asset(&conn(), wool).unwrap();
	let paths = folder_paths(&conn()).unwrap();
	assert_eq!(paths[&a.folder_id.unwrap()], "Topics/wool");
}
