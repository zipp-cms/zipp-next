//! Listing entries: `?type=event&f.city=Bern&f.price.lt=20&order=-price`.
//!
//! - `type`, `slug`: exact match
//! - `ids=1,2,3`: only these entries
//! - `search`: part of the title
//! - `related=5`: entries that link to entry 5, also from inside blocks
//! - `f.<field>[.<op>]=value`: an indexed field, op is one of eq (default),
//!   ne, lt, lte, gt, gte; needs `type`
//! - `order`: `title`, `created`, `updated` or an indexed field, `-` for
//!   descending, default `-updated`
//! - `limit` (default 100), `offset`

use rusqlite::types::Value;

use crate::schema::{FieldKind, Schema};

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
	pub ty: Option<String>,
	pub slug: Option<String>,
	pub ids: Option<Vec<i64>>,
	pub search: Option<String>,
	pub related: Option<i64>,
	pub filters: Vec<Filter>,
	pub order: Order,
	pub desc: bool,
	pub limit: i64,
	pub offset: i64,
	/// set for requests without a session
	pub published_only: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
	pub field: String,
	pub op: &'static str,
	/// text for text fields, a number for number and boolean fields
	pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Order {
	Column(&'static str),
	/// indexed field, true when it is stored in `num`
	Field(String, bool),
}

impl Query {
	pub fn parse(
		schema: &Schema,
		params: &[(String, String)],
	) -> Result<Self, String> {
		let mut q = Query {
			ty: None,
			slug: None,
			ids: None,
			search: None,
			related: None,
			filters: vec![],
			order: Order::Column("updated_at"),
			desc: true,
			limit: 100,
			offset: 0,
			published_only: false,
		};
		let int = |k: &str, v: &str| {
			v.parse::<i64>()
				.map_err(|_| format!("{k} must be a number"))
		};

		// type first, filters need it
		if let Some((_, ty)) = params.iter().find(|(k, _)| k == "type") {
			if schema.entry_type(ty).is_none() {
				return Err(format!("unknown entry type {ty}"));
			}
			q.ty = Some(ty.clone());
		}

		for (key, value) in params {
			match key.as_str() {
				"type" => {}
				"slug" => q.slug = Some(value.clone()),
				"search" => q.search = Some(value.clone()),
				"related" => q.related = Some(int(key, value)?),
				"limit" => q.limit = int(key, value)?.clamp(1, 1000),
				"offset" => q.offset = int(key, value)?.max(0),
				"ids" => {
					let ids = value
						.split(',')
						.filter(|s| !s.is_empty())
						.map(|s| int(key, s))
						.collect::<Result<_, _>>()?;
					q.ids = Some(ids);
				}
				"order" => {
					let (desc, name) = match value.strip_prefix('-') {
						Some(n) => (true, n),
						None => (false, value.as_str()),
					};
					q.desc = desc;
					q.order = match name {
						"title" => Order::Column("title"),
						"created" => Order::Column("created_at"),
						"updated" => Order::Column("updated_at"),
						_ => {
							let num = q.indexed_kind(schema, name)?;
							Order::Field(name.into(), num)
						}
					};
				}
				_ => match key.strip_prefix("f.") {
					Some(f) => {
						let filter = q.filter(schema, f, value)?;
						q.filters.push(filter);
					}
					None => return Err(format!("unknown parameter {key}")),
				},
			}
		}

		Ok(q)
	}

	/// Whether an indexed field is stored as number.
	fn indexed_kind(
		&self,
		schema: &Schema,
		field: &str,
	) -> Result<bool, String> {
		let ty = self.ty.as_ref().ok_or("filters and order need a type")?;
		let comp = schema.entry_type(ty).unwrap();
		let f = comp
			.fields
			.iter()
			.find(|f| f.handle == field && f.indexed)
			.ok_or_else(|| format!("{ty}.{field} is not indexed"))?;
		Ok(!matches!(f.kind, FieldKind::Text { .. }))
	}

	fn filter(
		&self,
		schema: &Schema,
		key: &str,
		value: &str,
	) -> Result<Filter, String> {
		let (field, op) = key.split_once('.').unwrap_or((key, "eq"));
		let op = match op {
			"eq" => "=",
			"ne" => "!=",
			"lt" => "<",
			"lte" => "<=",
			"gt" => ">",
			"gte" => ">=",
			_ => return Err(format!("unknown filter {op}")),
		};

		let value = match self.indexed_kind(schema, field)? {
			false => Value::Text(value.into()),
			true => match value {
				"true" => Value::Real(1.0),
				"false" => Value::Real(0.0),
				v => Value::Real(
					v.parse().map_err(|_| format!("{field} needs a number"))?,
				),
			},
		};

		Ok(Filter {
			field: field.into(),
			op,
			value,
		})
	}

	/// `FROM ... WHERE ...` and its params, without order and limit.
	pub fn from_where(&self) -> (String, Vec<Value>) {
		let mut joins = String::new();
		let mut conds = vec!["1 = 1".to_string()];
		let mut join_params = vec![];
		let mut params = vec![];

		let mut cond = |sql: String, values: Vec<Value>| {
			conds.push(sql);
			params.extend(values);
		};
		if let Some(ty) = &self.ty {
			cond("e.type = ?".into(), vec![ty.clone().into()]);
		}
		if let Some(slug) = &self.slug {
			cond("e.slug = ?".into(), vec![slug.clone().into()]);
		}
		if let Some(ids) = &self.ids {
			let marks = vec!["?"; ids.len()].join(", ");
			let values = ids.iter().map(|i| Value::Integer(*i)).collect();
			cond(format!("e.id IN ({marks})"), values);
		}
		if let Some(search) = &self.search {
			cond("e.title LIKE ?".into(), vec![format!("%{search}%").into()]);
		}
		if let Some(id) = self.related {
			let sql = "e.id IN (SELECT entry_id FROM entry_relations \
				WHERE target_id = ?)";
			cond(sql.into(), vec![Value::Integer(id)]);
		}
		if self.published_only {
			cond("e.published = 1".into(), vec![]);
		}

		for (i, f) in self.filters.iter().enumerate() {
			joins += &format!(
				" JOIN entry_values f{i} ON f{i}.entry_id = e.id \
				AND f{i}.field = ?"
			);
			join_params.push(Value::Text(f.field.clone()));

			let col = match f.value {
				Value::Text(_) => "text",
				_ => "num",
			};
			cond(format!("f{i}.{col} {} ?", f.op), vec![f.value.clone()]);
		}

		if let Order::Field(field, _) = &self.order {
			joins += " LEFT JOIN entry_values o ON o.entry_id = e.id \
				AND o.field = ?";
			join_params.push(Value::Text(field.clone()));
		}

		let sql =
			format!("FROM entries e{joins} WHERE {}", conds.join(" AND "));
		join_params.extend(params);
		(sql, join_params)
	}

	pub fn order_by(&self) -> String {
		let col = match &self.order {
			Order::Column(c) => format!("e.{c}"),
			Order::Field(_, true) => "o.num".into(),
			Order::Field(_, false) => "o.text".into(),
		};
		let dir = if self.desc { "DESC" } else { "ASC" };
		format!("ORDER BY {col} {dir}, e.id {dir}")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::schema::tests::site;

	fn parse(q: &str) -> Result<Query, String> {
		let params: Vec<_> = q
			.split('&')
			.filter_map(|p| p.split_once('='))
			.map(|(k, v)| (k.to_string(), v.to_string()))
			.collect();
		Query::parse(&site(), &params)
	}

	#[test]
	fn parses_filters() {
		let q =
			parse("f.price.lt=20&type=event&f.city=Bern&order=-price").unwrap();
		assert_eq!(q.filters[0].op, "<");
		assert_eq!(q.filters[0].value, Value::Real(20.0));
		assert_eq!(q.filters[1].value, Value::Text("Bern".into()));
		assert_eq!(q.order, Order::Field("price".into(), true));
		assert!(q.desc);
	}

	#[test]
	fn rejects_bad_queries() {
		assert!(parse("f.city=Bern").unwrap_err().contains("need a type"));
		assert!(
			parse("type=event&f.page=1")
				.unwrap_err()
				.contains("not indexed")
		);
		assert!(parse("type=event&f.price=cheap").is_err());
		assert!(
			parse("type=text")
				.unwrap_err()
				.contains("unknown entry type")
		);
		assert!(parse("color=red").is_err());
	}

	#[test]
	fn builds_sql_in_param_order() {
		let q = parse("type=event&f.city=Bern&order=price").unwrap();
		let (sql, params) = q.from_where();
		assert_eq!(
			params,
			[
				Value::Text("city".into()),
				Value::Text("price".into()),
				Value::Text("event".into()),
				Value::Text("Bern".into()),
			]
		);
		assert!(sql.contains("f0.text = ?"));
		assert_eq!(q.order_by(), "ORDER BY o.num ASC, e.id ASC");
	}
}
