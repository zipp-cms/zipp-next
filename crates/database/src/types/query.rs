use std::collections::BTreeMap;

use serde_json::Value;

/*
fields:
```json
{
	"schema": [
		"field1",
		"field2",
	],
	"other_schema": [
		{
			"name": "schema",
			"fields": [
				"field1",
				"field2",
			]
		}
	]
}




{
	"name": "schema",
	"fields": [
		"field1",
		"field2",
		{
			"name": "other_schema",
			"fields": [
				"field1",
				"field2",
			]
		}
	]
	"filter": {
		"type": "and",
		"filters": [
			{
				"type": "eq",
				"field": "field1",
				"value": "value"
			},
			{
				"type": "in",
				"field": "field2",
				"values": ["value"]
			}
		]
	}
}


// when i do an in query how do i make sure the
```

filter:
```json
{
	"schema": {
		"field1": {
			"eq": "value"
		}
	}
}
*/

#[derive(Debug, Clone)]
pub struct Query {
	pub schema: String,
	pub fields: Vec<FieldQuery>,
	pub filter: Filter,
	pub sorting: Sorting,
}

#[derive(Debug, Clone)]
pub enum FieldQuery {
	Schema {
		name: String,
		fields: BTreeMap<String, FieldQuery>,
	},
	Field(String),
}

#[derive(Debug, Clone)]
pub struct FieldSelector {
	pub schema: String,
	pub field: String,
}

#[derive(Debug, Clone, Default)]
pub enum Filter {
	#[default]
	None,
	And(Vec<Filter>),
	Or(Vec<Filter>),
	Eq {
		field: FieldSelector,
		value: Value,
	},
	In {
		field: FieldSelector,
		values: Vec<Value>,
	},
}

#[derive(Debug, Clone)]
pub enum Sorting {
	None,
	Asc(FieldSelector),
	Desc(FieldSelector),
	And(Vec<Sorting>),
}
