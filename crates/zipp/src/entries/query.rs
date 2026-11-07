//! Listing entries: `?section=events&f.city=Bern&f.price.lt=20&order=-price`.
//!
//! - `site`: whose titles, uris and fields, default the primary site
//! - `section`, `type`: one or several `a,b`; `notSections`,
//!   `notTypes` leave them out
//! - `slug`, `uri`: exact match
//! - `ids=1,2,3`: only these entries
//! - `author=4`: the entries user 4 made, one or several `4,5`
//! - `parent=5`: the children of entry 5 in a structure
//! - `level=1`: the entries at that level of a structure, 1 the top
//! - `search`: part of the title
//! - `text`: full text, all the words in the title, slug or texts (see
//!   `search/`), the best match first unless an order is asked for
//! - `related=5`: entries that link to entry 5, also from inside blocks
//! - `relatedTo=1,2`: entries that link to or are linked from one of
//!   them, like Craft's; each `relatedTo` more must hold too (and)
//! - `relatedToEntries=section%3Dwiki`: the same with the entries of a
//!   query (its parameters url encoded), in the site of the query
//! - `f.<field>[.<op>]=value`: an indexed field, op is one of eq (default),
//!   ne, lt, lte, gt, gte; needs `type`
//! - `order`: `title`, `created`, `updated`, `structure` (the tree of a
//!   structure, depth first; with `parent` the order of the children) or
//!   an indexed field, `-` for descending, default `-updated`; Craft's
//!   `orderBy` (`postDate DESC`, `score` the best match of `text`)
//! - `limit` (default 100), `offset`

use rusqlite::types::Value;

use crate::db::{escape_like, marks};
use crate::entries::LIVE_DATES;
use crate::schema::{IndexKind, Schema, SectionKind};

/// The most entries of one query.
pub(crate) const MAX_LIMIT: i64 = 1000;

/// The most ids of a list in a query, `ids=1,2,3`.
pub(crate) const MAX_IDS: usize = 1000;

/// The ids of a list `1,2,3` in the parameter `key`, at most
/// [`MAX_IDS`]; anything that isn't an id is an error.
pub(crate) fn parse_ids(key: &str, list: &str) -> Result<Vec<i64>, String> {
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
#[non_exhaustive]
pub struct Query {
	pub site: String,
	pub section: Option<String>,
	/// several sections, `section=a,b`
	pub(crate) any_sections: Vec<String>,
	/// sections left out, Craft's `section: ["not", "news"]`
	pub(crate) not_sections: Vec<String>,
	/// only these sections, what the token of a frontend reads
	pub sections: Option<Vec<String>>,
	pub ty: Option<String>,
	/// several types, `type=a,b`; filters and order need a single one
	pub types: Vec<String>,
	/// types left out, Craft's `type: ["not", "wikiList"]`
	pub(crate) not_types: Vec<String>,
	/// entries left out, Craft's `id: ["not", 1]`
	pub not_ids: Vec<i64>,
	pub slug: Option<String>,
	pub uri: Option<String>,
	pub ids: Option<Vec<i64>>,
	/// made by one of these users, `author=`: none
	pub authors: Option<Vec<i64>>,
	/// children of these entries, `parent=5`; GraphQL loads the children
	/// of several entries at once
	pub parents: Vec<i64>,
	/// the level in a structure, 1 the top entries
	pub(crate) level: Option<i64>,
	pub search: Option<String>,
	/// full text, the words of the website's search
	pub text: Option<String>,
	pub related: Option<i64>,
	/// related to some entries, each of the list (Craft's `relatedTo`)
	pub(crate) related_to: Vec<Related>,
	/// by a relation field of the entries, `r.offerCategory=5`
	pub(crate) relations: Vec<RelationFilter>,
	/// what `related` and `relations` look at, `None`: all entries; a
	/// viewer's are the ones it sees, a relation to another one tells
	/// nothing of it
	pub(crate) targets: Option<Box<Query>>,
	pub(crate) filters: Vec<Filter>,
	pub(crate) order: Order,
	pub(crate) desc: bool,
	pub limit: i64,
	pub offset: i64,
	/// set for requests without a session
	pub published_only: bool,
	/// shown even if not published, the entry of a preview token
	pub(crate) preview_id: Option<i64>,
	/// the sections whose drafts are shown, see
	/// [`Preview::All`](crate::drafts::Preview)
	pub(crate) preview_sections: Vec<String>,
	/// only entries with a page
	pub with_uri: bool,
	/// no entries at all, an offline site for a frontend
	pub(crate) nothing: bool,
	/// of some sections only the entries of the user, see
	/// [`Role::own`](crate::schema::Role::own)
	pub(crate) own_only: Option<OwnOnly>,
}

/// Of these sections only the entries `author` made, what a role with
/// own access to them sees in the studio.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct OwnOnly {
	pub author: i64,
	pub sections: Vec<String>,
}

impl OwnOnly {
	/// The condition on the entries `e` and its params.
	pub(crate) fn sql(&self) -> (String, Vec<Value>) {
		let mut values: Vec<Value> = self
			.sections
			.iter()
			.map(|s| Value::Text(s.clone()))
			.collect();
		values.push(Value::Integer(self.author));
		let sql = format!(
			"(e.section NOT IN ({}) OR e.author_id = ?)",
			marks(self.sections.len())
		);
		(sql, values)
	}
}

/// Entries whose relation field points to one of some entries, or to
/// any (`:notempty:`) or none (`:empty:`), like Craft's arguments.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RelationFilter {
	pub field: String,
	/// empty with `any`: to anything
	pub ids: Vec<i64>,
	pub any: bool,
	/// to nothing, `:empty:`
	pub none: bool,
}

/// Entries that link to or are linked from one of some entries, in
/// either direction like Craft's `relatedTo`.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Related {
	Ids(Vec<i64>),
	/// the entries of a query, Craft's `relatedToEntries`
	Entries(Box<Query>),
}

/// The deepest `level` of a structure, the query nests a select per
/// level.
const MAX_LEVEL: i64 = 20;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Filter {
	pub field: String,
	pub op: &'static str,
	/// text for text fields, a number for number and boolean fields
	pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Order {
	Column(&'static str),
	Structure,
	/// indexed field, true when it is stored in `num`
	Field(String, bool),
	/// how well the entry matches `text`
	Relevance,
}

/// The order of the entries of the structures, depth first, from their
/// top entries, with `one` of a section (`?`); the trash isn't in it.
/// Positions are padded so the path sorts as text.
fn tree(one: bool) -> String {
	let section = if one { " AND section = ?" } else { "" };
	format!(
		"WITH RECURSIVE tree(id, sort) AS (
	SELECT id, printf('%010d', position) FROM entries
	WHERE parent_id IS NULL AND deleted_at IS NULL{section}
	UNION ALL
	SELECT e.id, t.sort || printf('%010d', e.position)
	FROM entries e JOIN tree t ON e.parent_id = t.id
	WHERE e.deleted_at IS NULL
) "
	)
}

impl Query {
	/// All entries of the primary site, the newest first.
	pub fn new(schema: &Schema) -> Self {
		Query {
			site: schema.primary_site().handle.clone(),
			section: None,
			any_sections: vec![],
			not_sections: vec![],
			sections: None,
			ty: None,
			types: vec![],
			not_types: vec![],
			not_ids: vec![],
			slug: None,
			uri: None,
			ids: None,
			authors: None,
			parents: vec![],
			level: None,
			search: None,
			text: None,
			related: None,
			related_to: vec![],
			relations: vec![],
			targets: None,
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
			own_only: None,
		}
	}

	/// A query of what is live, for plugins and the site's own routes.
	pub fn published(
		schema: &Schema,
		params: &[(&str, &str)],
	) -> Result<Self, String> {
		let params: Vec<(String, String)> = params
			.iter()
			.map(|(k, v)| (k.to_string(), v.to_string()))
			.collect();
		let mut q = Self::parse(schema, &params)?;
		q.published_only = true;
		Ok(q)
	}

	pub fn parse(
		schema: &Schema,
		params: &[(String, String)],
	) -> Result<Self, String> {
		let mut q = Self::new(schema);

		// type first, filters need it
		if let Some((_, types)) = params.iter().find(|(k, _)| k == "type") {
			let types = entry_types(schema, types)?;
			match types.as_slice() {
				[one] => q.ty = Some(one.clone()),
				_ => q.types = types,
			}
		}

		let mut ordered = false;
		for (key, value) in params {
			ordered |= q.param(schema, key, value)?;
		}

		// the entries related to are in the same site
		for r in &mut q.related_to {
			if let Related::Entries(of) = r {
				of.each_mut(&mut |of| of.site = q.site.clone());
			}
		}

		if !ordered {
			q.default_order(schema);
		}
		Ok(q)
	}

	/// Sets one parameter of [`parse`](Self::parse), true when it is the
	/// order.
	fn param(
		&mut self,
		schema: &Schema,
		key: &str,
		value: &str,
	) -> Result<bool, String> {
		match key {
			"type" => {}
			"site" => {
				if schema.get_site(value).is_none() {
					return Err(format!("unknown site {value}"));
				}
				self.site = value.into();
			}
			"section" | "notSections" => {
				let sections = sections(schema, value)?;
				match (key, sections.as_slice()) {
					("notSections", _) => self.not_sections = sections,
					(_, [one]) => self.section = Some(one.clone()),
					_ => self.any_sections = sections,
				}
			}
			"notTypes" => self.not_types = entry_types(schema, value)?,
			"level" => {
				let level = parse_int(key, value)?;
				if !(1..=MAX_LEVEL).contains(&level) {
					return Err(format!(
						"level must be between 1 and {MAX_LEVEL}"
					));
				}
				self.level = Some(level);
			}
			// none is no filter, like in Craft
			"relatedTo" => match parse_ids(key, value)? {
				ids if ids.is_empty() => {}
				ids => self.related_to.push(Related::Ids(ids)),
			},
			"relatedToEntries" => {
				let of = related_entries(schema, key, value)?;
				self.related_to.push(of);
			}
			"slug" => self.slug = Some(value.into()),
			"uri" => self.uri = Some(value.trim_matches('/').into()),
			"search" => self.search = Some(value.into()),
			"text" => self.text = Some(value.into()),
			"parent" => self.parents = vec![parse_int(key, value)?],
			"author" => self.authors = Some(parse_ids(key, value)?),
			"related" => self.related = Some(parse_int(key, value)?),
			_ if key.starts_with("r.") => {
				let filter = relation_filter(schema, key, value)?;
				self.relations.push(filter);
			}
			"limit" => self.limit = parse_int(key, value)?.clamp(1, MAX_LIMIT),
			"offset" => self.offset = parse_int(key, value)?.max(0),
			"ids" => self.ids = Some(parse_ids(key, value)?),
			"notIds" => self.not_ids = parse_ids(key, value)?,
			"orderBy" => return self.order_by_craft(schema, value),
			"order" => {
				let (desc, name) = match value.strip_prefix('-') {
					Some(n) => (true, n),
					None => (false, value),
				};
				self.desc = desc;
				self.order = self.order_of(schema, name)?;
				return Ok(true);
			}
			_ => match key.strip_prefix("f.") {
				Some(f) => {
					let filter = self.filter(schema, f, value)?;
					self.filters.push(filter);
				}
				None => return Err(format!("unknown parameter {key}")),
			},
		}
		Ok(false)
	}

	/// Sets Craft's `orderBy`, `postDate DESC`, `title`; true when it
	/// is an order.
	fn order_by_craft(
		&mut self,
		schema: &Schema,
		value: &str,
	) -> Result<bool, String> {
		let mut words = value.split_whitespace();
		let name = words.next().unwrap_or("postDate");
		// Craft's relevance: the default with a search, without one it is
		// ignored like in Craft
		if name == "score" {
			return Ok(false);
		}
		self.desc =
			words.next().is_some_and(|d| d.eq_ignore_ascii_case("desc"));
		self.order = self.order_of(schema, name)?;
		Ok(true)
	}

	/// The order without one asked for: the best match of `text`, else
	/// the section's.
	fn default_order(&mut self, schema: &Schema) {
		// signs alone search nothing, so there is no relevance to order by
		let words = self
			.text
			.as_ref()
			.is_some_and(|t| !crate::search::words(t).is_empty());
		if words {
			self.order = Order::Relevance;
			self.desc = false;
		} else {
			self.section_order(schema);
		}
	}

	/// Calls `f` with the query and the queries of its `relatedToEntries`
	/// and `targets`, so what a viewer may see holds for them too.
	pub(crate) fn each_mut(&mut self, f: &mut impl FnMut(&mut Query)) {
		f(self);
		for r in &mut self.related_to {
			if let Related::Entries(of) = r {
				of.each_mut(f);
			}
		}
		if let Some(of) = &mut self.targets {
			of.each_mut(f);
		}
	}

	/// The order of a name of `order` or Craft's `orderBy`, else of an
	/// indexed field.
	fn order_of(&self, schema: &Schema, name: &str) -> Result<Order, String> {
		Ok(match name {
			"title" => Order::Column("title"),
			"postDate" => Order::Column("post_date"),
			"created" | "dateCreated" => Order::Column("created_at"),
			"updated" | "dateUpdated" => Order::Column("updated_at"),
			"structure" | "lft" => Order::Structure,
			_ => Order::Field(name.into(), self.indexed_kind(schema, name)?),
		})
	}

	/// The order of the section without one asked for, like Craft: a
	/// channel newest first, a structure in its order.
	pub(crate) fn section_order(&mut self, schema: &Schema) {
		let section = self.section.as_ref().and_then(|s| schema.get_section(s));
		match section.map(|s| s.kind) {
			Some(SectionKind::Structure) => {
				self.order = Order::Structure;
				self.desc = false;
			}
			Some(SectionKind::Channel) => {
				self.order = Order::Column("post_date")
			}
			_ => {}
		}
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
			.field(field)
			.filter(|f| f.indexed)
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
	pub(crate) fn with(&self) -> (String, Vec<Value>) {
		match &self.section {
			_ if !self.tree() => (String::new(), vec![]),
			Some(s) => (tree(true), vec![Value::Text(s.clone())]),
			None => (tree(false), vec![]),
		}
	}

	/// `FROM ... WHERE ...` and its params, without order and limit. `e`
	/// are the entries, `s` their row of the site.
	pub(crate) fn sql_from(&self) -> (String, Vec<Value>) {
		let mut sql = Sql {
			joins: String::new(),
			join_params: vec![Value::Text(self.site.clone())],
			conds: vec!["e.deleted_at IS NULL".into()],
			params: vec![],
		};
		self.which_conds(&mut sql);
		self.search_conds(&mut sql);
		self.relation_conds(&mut sql);
		self.live_conds(&mut sql);
		self.filter_conds(&mut sql);
		self.order_join(&mut sql);

		let from = format!(
			"FROM entries e \
			JOIN entry_sites s ON s.entry_id = e.id AND s.site = ?{} \
			WHERE {}",
			sql.joins,
			sql.conds.join(" AND ")
		);
		sql.join_params.extend(sql.params);
		(from, sql.join_params)
	}

	/// Which entries by their sections, types, ids, places in the tree
	/// and authors.
	fn which_conds(&self, sql: &mut Sql) {
		if let Some(section) = &self.section {
			sql.cond("e.section = ?", vec![Value::Text(section.clone())]);
		}
		if !self.any_sections.is_empty() {
			sql.in_list("e.section IN", texts(&self.any_sections));
		}
		if !self.not_sections.is_empty() {
			sql.in_list("e.section NOT IN", texts(&self.not_sections));
		}
		match &self.sections {
			// none of none
			Some(sections) if sections.is_empty() => sql.cond("0", vec![]),
			Some(sections) => sql.in_list("e.section IN", texts(sections)),
			None => {}
		}
		if let Some(ty) = &self.ty {
			sql.cond("e.type = ?", vec![Value::Text(ty.clone())]);
		}
		if !self.types.is_empty() {
			sql.in_list("e.type IN", texts(&self.types));
		}
		if !self.not_types.is_empty() {
			sql.in_list("e.type NOT IN", texts(&self.not_types));
		}
		if !self.not_ids.is_empty() {
			sql.in_list("e.id NOT IN", ints(&self.not_ids));
		}
		if let Some(slug) = &self.slug {
			sql.cond("s.slug = ?", vec![Value::Text(slug.clone())]);
		}
		if let Some(uri) = &self.uri {
			sql.cond("s.uri = ?", vec![Value::Text(uri.clone())]);
		}
		if let Some(ids) = &self.ids {
			sql.in_list("e.id IN", ints(ids));
		}
		if let Some(authors) = &self.authors {
			sql.in_list("e.author_id IN", ints(authors));
		}
		if let Some(own) = &self.own_only {
			let (cond, values) = own.sql();
			sql.cond(cond, values);
		}
		if !self.parents.is_empty() {
			sql.in_list("e.parent_id IN", ints(&self.parents));
		}
		if let Some(level) = self.level {
			// the parents of a level are the entries of the level above
			let mut cond = "e.parent_id IS NULL".to_string();
			for _ in 1..level {
				cond = format!(
					"e.parent_id IN (SELECT id FROM entries e \
					WHERE {cond} AND e.deleted_at IS NULL)"
				);
			}
			sql.cond(cond, vec![]);
		}
	}

	/// Part of the title (`search`) and the full text (`text`).
	fn search_conds(&self, sql: &mut Sql) {
		if let Some(search) = &self.search {
			let like = format!("%{}%", escape_like(search));
			sql.cond("s.title LIKE ? ESCAPE '\\'", vec![like.into()]);
		}
		if let Some(text) = &self.text {
			// signs alone find nothing
			let words = crate::search::words(text);
			match words.is_empty() {
				true => sql.cond("0", vec![]),
				false => {
					sql.joins += " JOIN entry_texts x ON x.entry_id = e.id \
						AND x.site = s.site \
						JOIN entry_search ON entry_search.rowid = x.id";
					let fts = crate::search::fts_query(&words);
					sql.cond("entry_search MATCH ?", vec![fts.into()]);
				}
			}
		}
	}

	/// `related`, `relatedTo` and the relation fields, `related` and
	/// the fields only by their relations to `targets`.
	fn relation_conds(&self, sql: &mut Sql) {
		let (targets, target_values) = match &self.targets {
			Some(q) => {
				let (from, values) = q.sql_from();
				(format!(" AND target_id IN (SELECT e.id {from})"), values)
			}
			None => (String::new(), vec![]),
		};
		if let Some(id) = self.related {
			let cond = format!(
				"e.id IN (SELECT entry_id FROM entry_relations \
				WHERE target_id = ? AND site = s.site{targets})"
			);
			let mut values = vec![Value::Integer(id)];
			values.extend(target_values.clone());
			sql.cond(cond, values);
		}
		for r in &self.related_to {
			let (of, values) = match r {
				Related::Ids(ids) => (marks(ids.len()), ints(ids)),
				Related::Entries(q) => {
					let (from, values) = q.sql_from();
					(format!("SELECT e.id {from}"), values)
				}
			};
			// to them or from them
			let cond = format!(
				"(e.id IN (SELECT entry_id FROM entry_relations \
				WHERE site = s.site AND target_id IN ({of})) \
				OR e.id IN (SELECT target_id FROM entry_relations \
				WHERE site = s.site AND entry_id IN ({of})))"
			);
			let mut both = values.clone();
			both.extend(values);
			sql.cond(cond, both);
		}
		for r in &self.relations {
			let of = format!(
				"SELECT entry_id FROM entry_relations \
				WHERE path = ? AND site = s.site{targets}"
			);
			let mut values = vec![Value::Text(r.field.clone())];
			values.extend(target_values.clone());
			if r.none {
				sql.cond(format!("e.id NOT IN ({of})"), values);
			} else if r.any || r.ids.is_empty() {
				sql.cond(format!("e.id IN ({of})"), values);
			} else {
				let marks = marks(r.ids.len());
				values.extend(ints(&r.ids));
				sql.cond(
					format!("e.id IN ({of} AND target_id IN ({marks}))"),
					values,
				);
			}
		}
	}

	/// What may be seen: an offline site nothing, pages, and for
	/// frontends what is live or previewed.
	fn live_conds(&self, sql: &mut Sql) {
		if self.nothing {
			sql.cond("0", vec![]);
		}
		if self.with_uri {
			sql.cond("s.uri IS NOT NULL", vec![]);
		}
		if self.published_only && !self.preview_sections.is_empty() {
			// published as the draft has it, or live without a draft; the
			// other sections live
			let sections = &self.preview_sections;
			let cond = format!(
				"CASE WHEN e.section IN ({}) THEN \
				COALESCE((SELECT json_extract(d.doc, '$.' || s.site || '.published') \
				FROM drafts d WHERE d.entry_id = e.id), s.published) = 1 \
				ELSE s.published = 1 AND {LIVE_DATES} END",
				marks(sections.len())
			);
			sql.cond(cond, texts(sections));
		} else if self.published_only {
			// live: published, after its post date and before it expires;
			// the entry of a preview anyway
			let preview = Value::Integer(self.preview_id.unwrap_or(0));
			let live =
				format!("((s.published = 1 AND {LIVE_DATES}) OR e.id = ?)");
			sql.cond(live, vec![preview]);
		}
	}

	/// The indexed fields of `f.<field>`, each joined.
	fn filter_conds(&self, sql: &mut Sql) {
		for (i, f) in self.filters.iter().enumerate() {
			sql.joins += &format!(
				" JOIN entry_values f{i} ON f{i}.entry_id = e.id \
				AND f{i}.site = s.site AND f{i}.field = ?"
			);
			sql.join_params.push(Value::Text(f.field.clone()));

			let col = match f.value {
				Value::Text(_) => "text",
				_ => "num",
			};
			let cond = format!("f{i}.{col} {} ?", f.op);
			sql.cond(cond, vec![f.value.clone()]);
		}
	}

	/// The join that [`order_by`](Self::order_by) sorts by.
	fn order_join(&self, sql: &mut Sql) {
		match &self.order {
			Order::Field(field, _) => {
				sql.joins += " LEFT JOIN entry_values o ON o.entry_id = e.id \
					AND o.site = s.site AND o.field = ?";
				sql.join_params.push(Value::Text(field.clone()));
			}
			Order::Structure if self.tree() => {
				sql.joins += " JOIN tree t ON t.id = e.id"
			}
			Order::Structure | Order::Column(_) | Order::Relevance => {}
		}
	}

	pub(crate) fn order_by(&self) -> String {
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
			Order::Relevance => crate::search::RANK.into(),
		};
		let dir = if self.desc { "DESC" } else { "ASC" };
		format!("ORDER BY {col} {dir}, e.id {dir}")
	}
}

/// Whether an entry type has a relation field of this handle, so
/// `entries` can filter by it.
pub(crate) fn relation_field(schema: &Schema, handle: &str) -> bool {
	schema.components.iter().filter(|c| c.entry).any(|c| {
		c.field(handle).is_some_and(|f| {
			matches!(f.kind, crate::schema::FieldKind::Relation { .. })
		})
	})
}

/// What [`Query::sql_from`] collects: the joins and the conditions, each
/// with their params.
struct Sql {
	joins: String,
	join_params: Vec<Value>,
	conds: Vec<String>,
	params: Vec<Value>,
}

impl Sql {
	fn cond(&mut self, sql: impl Into<String>, values: Vec<Value>) {
		self.conds.push(sql.into());
		self.params.extend(values);
	}

	/// `<col> IN (?, ?, ?)` of some values, `col_in` with the `IN` or
	/// `NOT IN`.
	fn in_list(&mut self, col_in: &str, values: Vec<Value>) {
		let sql = format!("{col_in} ({})", marks(values.len()));
		self.cond(sql, values);
	}
}

fn texts(list: &[String]) -> Vec<Value> {
	list.iter().map(|s| Value::Text(s.clone())).collect()
}

fn ints(list: &[i64]) -> Vec<Value> {
	list.iter().map(|i| Value::Integer(*i)).collect()
}

fn parse_int(key: &str, value: &str) -> Result<i64, String> {
	value.parse().map_err(|_| format!("{key} must be a number"))
}

/// The entry types of a list `a,b`; an unknown one is an error.
fn entry_types(schema: &Schema, list: &str) -> Result<Vec<String>, String> {
	let types: Vec<String> = list.split(',').map(Into::into).collect();
	if let Some(t) = types.iter().find(|t| schema.entry_type(t).is_none()) {
		return Err(format!("unknown entry type {t}"));
	}
	Ok(types)
}

/// The sections of a list `a,b`; an unknown one is an error.
fn sections(schema: &Schema, list: &str) -> Result<Vec<String>, String> {
	let sections: Vec<String> = list.split(',').map(Into::into).collect();
	if let Some(s) = sections.iter().find(|s| schema.get_section(s).is_none()) {
		return Err(format!("unknown section {s}"));
	}
	Ok(sections)
}

/// `relatedToEntries`: the entries of a query, its parameters url
/// encoded.
fn related_entries(
	schema: &Schema,
	key: &str,
	value: &str,
) -> Result<Related, String> {
	let params: Vec<(String, String)> = serde_urlencoded::from_str(value)
		.map_err(|_| format!("{key} must be url encoded parameters"))?;
	let mut of = Query::parse(schema, &params)?;
	// each level doubles the sql, so only one
	let nested = of
		.related_to
		.iter()
		.any(|r| matches!(r, Related::Entries(_)));
	if nested {
		return Err(format!("{key} can't have another one"));
	}
	// only which entries, no tree to order by
	of.order = Order::Column("updated_at");
	Ok(Related::Entries(Box::new(of)))
}

/// `r.<field>`: by a relation field, to some ids, `:notempty:` or
/// `:empty:`.
fn relation_filter(
	schema: &Schema,
	key: &str,
	value: &str,
) -> Result<RelationFilter, String> {
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
	match value {
		":notempty:" => f.any = true,
		":empty:" => f.none = true,
		v => f.ids = parse_ids(key, v)?,
	}
	Ok(f)
}

#[cfg(test)]
#[path = "query_tests.rs"]
mod tests;
