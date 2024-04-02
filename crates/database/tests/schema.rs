use database::{
	types::{
		id::{self, Id},
		schema::{
			Field, FieldKind, Schema, SchemaEntries, SchemaEntry,
			SchemaFieldValue,
		},
	},
	Database,
};
use serde_json::{json, Map, Value};

fn json_object(v: Value) -> Map<String, Value> {
	match v {
		Value::Object(map) => map,
		_ => panic!("Expected object"),
	}
}

#[tokio::test]
async fn test_memory() {
	let db = Database::new_memory();

	let schema = Schema::builder("test")
		.field(Field::builder("id", FieldKind::Id))
		.field(Field::builder("name", FieldKind::Text))
		.build();

	db.set_schema(schema).await.unwrap();

	db.create_schema_entries(
		"test".into(),
		SchemaEntries(vec![SchemaEntry(
			[("name", SchemaFieldValue::Value(json!("1")))]
				.into_iter()
				.map(|(k, v)| (k.to_string(), v))
				.collect(),
		)]),
	)
	.await
	.unwrap();
	// .json_object(json!({
	// 	"test": [
	// 		{
	// 			"id": Id::new(id::Kind::new(false, 12)),
	// 			"name": "1"
	// 		},
	// 		{
	// 			"id": Id::new(id::Kind::new(false, 12)),
	// 			"name": "2"
	// 		}
	// 	],
	// 	"test": {
	// 		"id": Id::new(id::Kind::new(false, 12)),
	// 		"name": "3"
	// 	}
	// })),
}
