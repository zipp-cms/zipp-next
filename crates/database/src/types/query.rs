use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Query {
	pub fields: FieldsSelector,
	pub filter: Filter,
	pub sorting: Sorting,
	pub limit: Option<usize>,
}

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
