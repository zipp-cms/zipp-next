use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Query {
	pub schema: String,
	pub fields: QueryFields,
	pub filter: Filter,
	pub sorting: Sorting,
}

#[derive(Debug, Clone)]
pub struct QueryFields(pub Vec<FieldQuery>);

#[derive(Debug, Clone)]
pub enum FieldQuery {
	Schema { name: String, fields: QueryFields },
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

impl Query {
	pub fn builder(schema: impl Into<String>) -> QueryBuilder {
		QueryBuilder::new(schema)
	}
}

impl QueryFields {
	pub fn builder() -> QueryFieldsBuilder {
		QueryFieldsBuilder::new()
	}
}

impl Filter {
	pub fn is_none(&self) -> bool {
		matches!(self, Filter::None)
	}
}

impl Sorting {
	pub fn is_none(&self) -> bool {
		matches!(self, Sorting::None)
	}
}

pub struct QueryBuilder {
	inner: Query,
}

impl QueryBuilder {
	pub fn new(schema: impl Into<String>) -> Self {
		Self {
			inner: Query {
				schema: schema.into(),
				fields: QueryFields(Vec::new()),
				filter: Filter::None,
				sorting: Sorting::None,
			},
		}
	}

	pub fn fields(mut self, fields: QueryFieldsBuilder) -> Self {
		self.inner.fields = fields.build();
		self
	}

	// pub fn filter(mut self, filter: Filter) -> Self {
	// 	self.inner.filter = filter;
	// 	self
	// }

	// pub fn sorting(mut self, sorting: Sorting) -> Self {
	// 	self.inner.sorting = sorting;
	// 	self
	// }

	pub fn build(self) -> Query {
		self.inner
	}
}

pub struct QueryFieldsBuilder {
	inner: QueryFields,
}

impl QueryFieldsBuilder {
	pub fn new() -> Self {
		Self {
			inner: QueryFields(Vec::new()),
		}
	}

	pub fn schema(
		mut self,
		name: impl Into<String>,
		fields: QueryFieldsBuilder,
	) -> Self {
		self.inner.0.push(FieldQuery::Schema {
			name: name.into(),
			fields: fields.build(),
		});
		self
	}

	pub fn field(mut self, name: impl Into<String>) -> Self {
		self.inner.0.push(FieldQuery::Field(name.into()));
		self
	}

	pub fn build(self) -> QueryFields {
		self.inner
	}
}
