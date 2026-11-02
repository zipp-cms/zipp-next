//! Listing entries: `?section=events&f.city=Bern&f.price.lt=20&order=-price`.
//!
//! - `site`: whose titles, uris and fields, default the primary site
//! - `section`, `type`, `slug`, `uri`: exact match
//! - `ids=1,2,3`: only these entries
//! - `parent=5`: the children of entry 5 in a structure
//! - `search`: part of the title
//! - `related=5`: entries that link to entry 5, also from inside blocks
//! - `f.<field>[.<op>]=value`: an indexed field, op is one of eq (default),
//!   ne, lt, lte, gt, gte; needs `type`
//! - `order`: `title`, `created`, `updated`, `structure` (the tree of a
//!   structure, depth first; with `parent` the order of the children) or
//!   an indexed field, `-` for descending, default `-updated`
//! - `limit` (default 100), `offset`

use rusqlite::types::Value;

use crate::db::{escape_like, marks};
use crate::entries::LIVE_DATES;
use crate::schema::{IndexKind, Schema, SectionKind};

/// The most entries of one query.
pub const MAX_LIMIT: i64 = 1000;

/// The most ids of a list in a query, `ids=1,2,3`.
pub const MAX_IDS: usize = 1000;

/// The ids of a list `1,2,3` in the parameter `key`, at most
/// [`MAX_IDS`]; anything that isn't an id is an error.
pub fn parse_ids(key: &str, list: &str) -> Result<Vec<i64>, String> {
	let ids: Vec<i64> = list
		.split(',')
		.filter(|s| !s.is_empty())
		.map(|s| {
			s.parse()
				.map_err(|_| format!("{key} must be ids like 1,2,3"))
		})
		.collect::<Result<_, _>>()?;
	if ids.len() > MAX_IDS {
		return Err(format!("{key} has more than {MAX_IDS} ids"));
	}
	Ok(ids)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
	pub site: String,
	pub section: Option<String>,
	/// only these sections, what the token of a frontend reads
	pub sections: Option<Vec<String>>,
	pub ty: Option<String>,
	/// several types, `type=a,b`; filters and order need a single one
	pub types: Vec<String>,
	/// entries left out, Craft's `id: ["not", 1]`
	pub not_ids: Vec<i64>,
	pub slug: Option<String>,
	pub uri: Option<String>,
	pub ids: Option<Vec<i64>>,
	/// children of these entries, `parent=5`; GraphQL loads the children
	/// of several entries at once
	pub parents: Vec<i64>,
	pub search: Option<String>,
	pub related: Option<i64>,
	/// by a relation field of the entries, `r.offerCategory=5`
	pub relations: Vec<RelationFilter>,
	pub filters: Vec<Filter>,
	pub order: Order,
	pub desc: bool,
	pub limit: i64,
	pub offset: i64,
	/// set for requests without a session
	pub published_only: bool,
	/// shown even if not published, the entry of a preview token
	pub preview_id: Option<i64>,
	/// the sections whose drafts are shown, see
	/// [`Preview::All`](crate::drafts::Preview)
	pub preview_sections: Vec<String>,
	/// only entries with a page
	pub with_uri: bool,
	/// no entries at all, an offline site for a frontend
	pub nothing: bool,
}

/// Entries whose relation field points to one of some entries, or to
/// any (`:notempty:`) or none (`:empty:`), like Craft's arguments.
#[derive(Debug, Clone, PartialEq)]
pub struct RelationFilter {
	pub field: String,
	/// empty with `any`: to anything
	pub ids: Vec<i64>,
	pub any: bool,
	/// to nothing, `:empty:`
	pub none: bool,
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
	Structure,
	/// indexed field, true when it is stored in `num`
	Field(String, bool),
}

/// The order of the entries of a structure, depth first, from its top
/// entries; the trash isn't in it. Positions are padded so the path sorts
/// as text.
const TREE: &str = "WITH RECURSIVE tree(id, sort) AS (
	SELECT id, printf('%010d', position) FROM entries
	WHERE parent_id IS NULL AND deleted_at IS NULL AND section = ?
	UNION ALL
	SELECT e.id, t.sort || printf('%010d', e.position)
	FROM entries e JOIN tree t ON e.parent_id = t.id
	WHERE e.deleted_at IS NULL
) ";

/// [`TREE`] of the structures of all sections.
const TREES: &str = "WITH RECURSIVE tree(id, sort) AS (
	SELECT id, printf('%010d', position) FROM entries
	WHERE parent_id IS NULL AND deleted_at IS NULL
	UNION ALL
	SELECT e.id, t.sort || printf('%010d', e.position)
	FROM entries e JOIN tree t ON e.parent_id = t.id
	WHERE e.deleted_at IS NULL
) ";

impl Query {
	pub fn parse(
		schema: &Schema,
		params: &[(String, String)],
	) -> Result<Self, String> {
		let mut q = Query {
			site: schema.primary_site().handle.clone(),
			section: None,
			sections: None,
			ty: None,
			types: vec![],
			not_ids: vec![],
			slug: None,
			uri: None,
			ids: None,
			parents: vec![],
			search: None,
			related: None,
			relations: vec![],
			filters: vec![],
			order: Order::Column("updated_at"),
			desc: true,
			limit: 100,
			offset: 0,
			published_only: false,
			preview_id: None,
			preview_sections: vec![],
			with_uri: false,
			nothing: false,
		};
		let int = |k: &str, v: &str| {
			v.parse::<i64>()
				.map_err(|_| format!("{k} must be a number"))
		};

		// type first, filters need it
		if let Some((_, types)) = params.iter().find(|(k, _)| k == "type") {
			let types: Vec<String> = types.split(',').map(Into::into).collect();
			if let Some(t) =
				types.iter().find(|t| schema.entry_type(t).is_none())
			{
				return Err(format!("unknown entry type {t}"));
			}
			match types.as_slice() {
				[one] => q.ty = Some(one.clone()),
				_ => q.types = types,
			}
		}
		// like Craft: a channel newest first, a structure in its order
		let mut ordered = false;

		for (key, value) in params {
			match key.as_str() {
				"type" => {}
				"site" => {
					if schema.get_site(value).is_none() {
						return Err(format!("unknown site {value}"));
					}
					q.site = value.clone();
				}
				"section" => {
					if schema.get_section(value).is_none() {
						return Err(format!("unknown section {value}"));
					}
					q.section = Some(value.clone());
				}
				"slug" => q.slug = Some(value.clone()),
				"uri" => q.uri = Some(value.trim_matches('/').into()),
				"search" => q.search = Some(value.clone()),
				"parent" => q.parents = vec![int(key, value)?],
				"related" => q.related = Some(int(key, value)?),
				_ if key.starts_with("r.") => {
					let field = key[2..].to_string();
					if !relation_field(schema, &field) {
						return Err(format!("{field} is no relation field"));
					}
					let mut f = RelationFilter {
						field,
						ids: vec![],
						any: false,
						none: false,
					};
					match value.as_str() {
						":notempty:" => f.any = true,
						":empty:" => f.none = true,
						v => f.ids = parse_ids(key, v)?,
					}
					q.relations.push(f);
				}
				"limit" => q.limit = int(key, value)?.clamp(1, MAX_LIMIT),
				"offset" => q.offset = int(key, value)?.max(0),
				"ids" | "notIds" => {
					let ids = parse_ids(key, value)?;
					match key.as_str() {
						"ids" => q.ids = Some(ids),
						_ => q.not_ids = ids,
					}
				}
				// Craft's `postDate DESC`, `title`
				"orderBy" => {
					let mut words = value.split_whitespace();
					let name = words.next().unwrap_or("postDate");
					let desc = words
						.next()
						.is_some_and(|d| d.eq_ignore_ascii_case("desc"));
					q.desc = desc;
					q.order = match name {
						"title" => Order::Column("title"),
						"postDate" => Order::Column("post_date"),
						"dateCreated" => Order::Column("created_at"),
						"dateUpdated" => Order::Column("updated_at"),
						"lft" | "structure" => Order::Structure,
						_ => {
							let num = q.indexed_kind(schema, name)?;
							Order::Field(name.into(), num)
						}
					};
					ordered = true;
				}
				"order" => {
					let (desc, name) = match value.strip_prefix('-') {
						Some(n) => (true, n),
						None => (false, value.as_str()),
					};
					ordered = true;
					q.desc = desc;
					q.order = match name {
						"title" => Order::Column("title"),
						"created" => Order::Column("created_at"),
						"updated" => Order::Column("updated_at"),
						"structure" => Order::Structure,
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

		if !ordered {
			let kind = q
				.section
				.as_ref()
				.and_then(|s| schema.get_section(s))
				.map(|s| s.kind);
			match kind {
				Some(SectionKind::Structure) => {
					q.order = Order::Structure;
					q.desc = false;
				}
				// newest first, like Craft's channels
				Some(SectionKind::Channel) => {
					q.order = Order::Column("post_date")
				}
				_ => {}
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
		Ok(schema.index_kind(&f.kind) == Some(IndexKind::Number))
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

	/// Whether the order is the tree of the structure: children are in
	/// the order of their parent, they need no tree.
	fn tree(&self) -> bool {
		self.order == Order::Structure && self.parents.is_empty()
	}

	/// The `WITH` clause that goes before `SELECT`, for the tree, and its
	/// params.
	pub fn with(&self) -> (&'static str, Vec<Value>) {
		match &self.section {
			_ if !self.tree() => ("", vec![]),
			Some(s) => (TREE, vec![Value::Text(s.clone())]),
			None => (TREES, vec![]),
		}
	}

	/// `FROM ... WHERE ...` and its params, without order and limit. `e`
	/// are the entries, `s` their row of the site.
	pub fn from_where(&self) -> (String, Vec<Value>) {
		let mut joins = String::new();
		let mut conds = vec!["e.deleted_at IS NULL".to_string()];
		let mut join_params = vec![Value::Text(self.site.clone())];
		let mut params = vec![];

		let mut cond = |sql: String, values: Vec<Value>| {
			conds.push(sql);
			params.extend(values);
		};
		let text = |s: &Option<String>| Value::Text(s.clone().unwrap());
		if self.section.is_some() {
			cond("e.section = ?".into(), vec![text(&self.section)]);
		}
		if let Some(sections) = &self.sections {
			let values = sections.iter().map(|s| Value::Text(s.clone()));
			// none of none
			let sql = match sections.len() {
				0 => "0".into(),
				n => format!("e.section IN ({})", marks(n)),
			};
			cond(sql, values.collect());
		}
		if self.ty.is_some() {
			cond("e.type = ?".into(), vec![text(&self.ty)]);
		}
		if !self.types.is_empty() {
			let marks = marks(self.types.len());
			let values =
				self.types.iter().map(|t| Value::Text(t.clone())).collect();
			cond(format!("e.type IN ({marks})"), values);
		}
		if !self.not_ids.is_empty() {
			let marks = marks(self.not_ids.len());
			let values =
				self.not_ids.iter().map(|i| Value::Integer(*i)).collect();
			cond(format!("e.id NOT IN ({marks})"), values);
		}
		if self.slug.is_some() {
			cond("s.slug = ?".into(), vec![text(&self.slug)]);
		}
		if self.uri.is_some() {
			cond("s.uri = ?".into(), vec![text(&self.uri)]);
		}
		if let Some(ids) = &self.ids {
			let marks = marks(ids.len());
			let values = ids.iter().map(|i| Value::Integer(*i)).collect();
			cond(format!("e.id IN ({marks})"), values);
		}
		if !self.parents.is_empty() {
			let marks = marks(self.parents.len());
			let values =
				self.parents.iter().map(|i| Value::Integer(*i)).collect();
			cond(format!("e.parent_id IN ({marks})"), values);
		}
		if let Some(search) = &self.search {
			let like = format!("%{}%", escape_like(search));
			cond("s.title LIKE ? ESCAPE '\\'".into(), vec![like.into()]);
		}
		if let Some(id) = self.related {
			let sql = "e.id IN (SELECT entry_id FROM entry_relations \
				WHERE target_id = ? AND site = s.site)";
			cond(sql.into(), vec![Value::Integer(id)]);
		}
		for r in &self.relations {
			let of = "SELECT entry_id FROM entry_relations \
				WHERE path = ? AND site = s.site";
			let field = Value::Text(r.field.clone());
			if r.none {
				cond(format!("e.id NOT IN ({of})"), vec![field]);
			} else if r.any || r.ids.is_empty() {
				cond(format!("e.id IN ({of})"), vec![field]);
			} else {
				let marks = marks(r.ids.len());
				let mut values = vec![field];
				values.extend(r.ids.iter().map(|i| Value::Integer(*i)));
				cond(
					format!("e.id IN ({of} AND target_id IN ({marks}))"),
					values,
				);
			}
		}
		if self.nothing {
			cond("0".into(), vec![]);
		}
		if self.with_uri {
			cond("s.uri IS NOT NULL".into(), vec![]);
		}
		if self.published_only && !self.preview_sections.is_empty() {
			// published as the draft has it, or live without a draft; the
			// other sections live
			let sections = &self.preview_sections;
			let sql = format!(
				"CASE WHEN e.section IN ({}) THEN \
				COALESCE((SELECT json_extract(d.doc, '$.' || s.site || '.published') \
				FROM drafts d WHERE d.entry_id = e.id), s.published) = 1 \
				ELSE s.published = 1 AND {LIVE_DATES} END",
				marks(sections.len())
			);
			let values = sections.iter().map(|s| Value::Text(s.clone()));
			cond(sql, values.collect());
		} else if self.published_only {
			// live: published, after its post date and before it expires;
			// the entry of a preview anyway
			let preview = Value::Integer(self.preview_id.unwrap_or(0));
			let live =
				format!("((s.published = 1 AND {LIVE_DATES}) OR e.id = ?)");
			cond(live, vec![preview]);
		}

		for (i, f) in self.filters.iter().enumerate() {
			joins += &format!(
				" JOIN entry_values f{i} ON f{i}.entry_id = e.id \
				AND f{i}.site = s.site AND f{i}.field = ?"
			);
			join_params.push(Value::Text(f.field.clone()));

			let col = match f.value {
				Value::Text(_) => "text",
				_ => "num",
			};
			cond(format!("f{i}.{col} {} ?", f.op), vec![f.value.clone()]);
		}

		match &self.order {
			Order::Field(field, _) => {
				joins += " LEFT JOIN entry_values o ON o.entry_id = e.id \
					AND o.site = s.site AND o.field = ?";
				join_params.push(Value::Text(field.clone()));
			}
			Order::Structure if self.tree() => {
				joins += " JOIN tree t ON t.id = e.id"
			}
			Order::Structure | Order::Column(_) => {}
		}

		let sql = format!(
			"FROM entries e \
			JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?{joins} \
			WHERE {}",
			conds.join(" AND ")
		);
		join_params.extend(params);
		(sql, join_params)
	}

	pub fn order_by(&self) -> String {
		let col = match &self.order {
			Order::Column("title") => "s.title".into(),
			// not published yet: when it was made
			Order::Column("post_date") => {
				"COALESCE(e.post_date, e.created_at)".into()
			}
			Order::Column(c) => format!("e.{c}"),
			Order::Structure if self.tree() => "t.sort".into(),
			// children by parent, then as they are ordered
			Order::Structure => "e.parent_id, e.position".into(),
			Order::Field(_, true) => "o.num".into(),
			Order::Field(_, false) => "o.text".into(),
		};
		let dir = if self.desc { "DESC" } else { "ASC" };
		format!("ORDER BY {col} {dir}, e.id {dir}")
	}
}

/// Whether an entry type has a relation field of this handle, so
/// `entries` can filter by it.
pub fn relation_field(schema: &Schema, handle: &str) -> bool {
	schema.components.iter().filter(|c| c.entry).any(|c| {
		c.fields.iter().any(|f| {
			f.handle == handle
				&& matches!(f.kind, crate::schema::FieldKind::Relation { .. })
		})
	})
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
		assert_eq!(q.site, "de");
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
		assert!(parse("site=fr").unwrap_err().contains("unknown site"));
		assert!(parse("color=red").is_err());
	}

	#[test]
	fn builds_sql_in_param_order() {
		let q = parse("type=event&f.city=Bern&order=price&site=en").unwrap();
		let (sql, params) = q.from_where();
		assert_eq!(
			params,
			[
				Value::Text("en".into()),
				Value::Text("city".into()),
				Value::Text("price".into()),
				Value::Text("event".into()),
				Value::Text("Bern".into()),
			]
		);
		assert!(sql.contains("f0.text = ?"));
		assert_eq!(q.order_by(), "ORDER BY o.num ASC, e.id ASC");
	}

	#[test]
	fn structure_order_uses_the_tree() {
		let q = parse("section=pages&order=structure&uri=/about/").unwrap();
		let (sql, _) = q.from_where();
		let (with, params) = q.with();
		assert!(with.starts_with("WITH RECURSIVE tree"));
		assert_eq!(params, [Value::Text("pages".into())]);
		assert!(sql.contains("JOIN tree t"));
		assert_eq!(q.uri.as_deref(), Some("about"));
		assert_eq!(q.order_by(), "ORDER BY t.sort ASC, e.id ASC");

		// children need no tree
		let q = parse("section=pages&parent=5").unwrap();
		assert_eq!(q.with().0, "");
		assert!(!q.from_where().0.contains("tree"));
		assert_eq!(
			q.order_by(),
			"ORDER BY e.parent_id, e.position ASC, e.id ASC"
		);
	}
}
