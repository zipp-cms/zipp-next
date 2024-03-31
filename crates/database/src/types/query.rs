use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Query {
	pub fields: FieldsSelector,
	pub filter: Filter,
	pub sorting: Sorting,
	pub limit: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct FieldsSelector {
	pub fields: BTreeMap<String, FieldSelector>,
}

#[derive(Debug, Clone)]
pub struct FieldSelector {
	pub children: Option<FieldsSelector>,
}

#[derive(Debug, Clone)]
pub struct Filter {}

#[derive(Debug, Clone)]
pub struct Sorting {}
