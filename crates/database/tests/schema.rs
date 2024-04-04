use database::{
	types::{
		id::Id,
		query::{Query, QueryFields},
		schema::{Field, FieldKind, Schema, SchemaEntries, SchemaEntry},
	},
	Database,
};
use serde_json::Value;

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
			SchemaEntries::builder()
				.entry(SchemaEntry::builder().field("name", "1"))
				.build(),
		)
		.await
		.unwrap();

	let entries = Value::from(entries);

	let id: Id = serde_json::from_value(entries[0]["id"].clone()).unwrap();
	assert_eq!(id.kind(), schema.kind);

	// now query for that entry
	let read_entries = db
		.read_schema_data(
			Query::builder("test")
				.fields(QueryFields::builder().field("id").field("name"))
				.build(),
		)
		.await
		.unwrap();

	let read_entries = Value::from(read_entries);
	assert_eq!(entries, read_entries);

	// now query for that entry
	let less_entries = db
		.read_schema_data(
			Query::builder("test")
				.fields(QueryFields::builder().field("id"))
				.build(),
		)
		.await
		.unwrap();

	let entry = &less_entries.0[0];
	assert_eq!(entry.0.len(), 1);
	let id: Id = serde_json::from_value(entry.0["id"].clone().into()).unwrap();
	assert_eq!(id.kind(), schema.kind);
}
