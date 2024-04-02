use std::collections::BTreeMap;

use serde_json::{Map, Value};

/*
fields:
```json
{
	"schema": [
		"field1",
		"field2",
	],
	"other_schema": {
		"nested": [
			"field1",
			"field2",
		]
	}
}
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
	pub fields: Map<String, Value>,
}

// #[derive(Debug, Clone)]
// pub struct Query {
// 	pub fields: FieldsSelector,
// 	pub filter: Filter,
// 	pub sorting: Sorting,
// 	pub limit: Option<usize>,
// }

/// Structure
/// ```json
/// {
/// 	"schema": {
///
/// 	}
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FieldsSelector {
	pub fields: Vec<FieldSelector>,
}

#[derive(Debug, Clone)]
pub enum FieldSelector {
	Schema {
		name: String,
		filter: Filter,
		fields: BTreeMap<String, FieldSelector>,
	},
	Field {
		name: String,
	},
}

#[derive(Debug, Clone)]
pub struct Filter {}

#[derive(Debug, Clone)]
pub struct Sorting {}
