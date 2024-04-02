use database::{
	types::{
		id::Id,
		schema::{
			Field, FieldKind, Schema, SchemaEntries, SchemaEntry,
			SchemaFieldValue,
		},
	},
	Database,
};
use serde_json::{json, Value};

#[tokio::test]
async fn test_memory() {
	let db = Database::new_memory();

	let schema = Schema::builder("test")
		.field(Field::builder("id", FieldKind::Id).primary())
		.field(Field::builder("name", FieldKind::Text))
		.build();

	let schema = db.create_schema(schema).await.unwrap();

	let entries = db
		.create_schema_entries(
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

	let entries = Value::from(entries);

	let id: Id = serde_json::from_value(entries[0]["id"].clone()).unwrap();
	assert_eq!(id.kind(), schema.kind);
}
