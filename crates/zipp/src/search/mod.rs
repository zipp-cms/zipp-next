//! The studio's search, like Craft's: entries by their title, slug and
//! texts in a site, assets by filename, title and alt text. Every save
//! writes an entry's texts to `entry_texts` (see `index`), a full text
//! index (SQLite's FTS5) over them finds the words. Only what is live is
//! searched, drafts are not. The website's search is the same index
//! through GraphQL, see [`hits`].

use rusqlite::types::Value::Text;
use rusqlite::{Connection, params, params_from_iter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::assets::Asset;
use crate::auth::Authed;
use crate::content::{inner, markdown_text, richtext};
use crate::db::marks;
use crate::entries::query::{OwnOnly, Query};
use crate::entries::{COLUMNS, Entry, from_row};
use crate::error::Result;
use crate::schema::{Access, ColumnKind, Component, FieldKind, Schema};

pub mod api;
#[cfg(test)]
mod api_tests;
mod routes;
#[cfg(test)]
mod tests;

pub use routes::routes;

/// The most entries and assets a search shows, each.
pub const LIMIT: i64 = 20;

/// Around a match in the snippet: the matched words are marked with
/// these before they are taken out again.
const MARK: char = '\u{1}';

/// An entry the search found.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryHit {
	pub id: i64,
	pub section: String,
	#[serde(rename = "type")]
	pub ty: String,
	pub title: String,
	/// in the site
	pub published: bool,
	/// the words around the match in its texts, none when only the
	/// title or the slug matched
	pub snippet: Option<String>,
	/// its editor in the studio
	pub path: String,
}

/// Around the matched words of an excerpt of the website's search.
pub const OPEN: char = '\u{1}';
pub const CLOSE: char = '\u{2}';

/// An entry the website's search found, see [`hits`].
#[derive(Debug, Clone)]
pub struct Hit {
	pub entry: Entry,
	/// the words around the match in its texts, the matched words
	/// between [`OPEN`] and [`CLOSE`]; the start of the texts when only
	/// the title or the slug matched, none without texts
	pub excerpt: Option<String>,
	/// how well it matches, higher is better
	pub score: f64,
}

/// What a search found, the best first.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Found {
	pub entries: Vec<EntryHit>,
	pub assets: Vec<Asset>,
}

/// Rewrites the texts of an entry in a site: its title and slug and the
/// texts of its fields, also in blocks and groups. [`reindex`] calls it
/// after the row of the site is written.
///
/// [`reindex`]: crate::entries::reindex
pub(crate) fn index(
	conn: &Connection,
	schema: &Schema,
	id: i64,
	site: &str,
	comp: &Component,
	fields: &Value,
) -> Result<()> {
	conn.execute(
		"INSERT INTO entry_texts (entry_id, site, title, slug, body)
		SELECT entry_id, site, title, slug, ?3 FROM entry_sites
		WHERE entry_id = ?1 AND site = ?2",
		params![id, site, texts(schema, comp, fields)],
	)?;
	Ok(())
}

/// The texts of fields that editors write, a line each: text, markdown,
/// rich text, tables and texts, also inside blocks and groups.
pub(crate) fn texts(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
) -> String {
	let mut out = vec![];
	collect(schema, comp, fields, &mut out);
	out.retain(|t| !t.trim().is_empty());
	// the marks of excerpts, they mustn't come from the content
	out.join("\n").replace([OPEN, CLOSE], "")
}

fn collect(
	schema: &Schema,
	comp: &Component,
	fields: &Value,
	out: &mut Vec<String>,
) {
	let strings = |v: &Value| {
		v.as_object()
			.into_iter()
			.flat_map(|o| o.values())
			.filter_map(|s| s.as_str().map(String::from))
			.collect::<Vec<_>>()
	};
	for field in &comp.fields {
		let Some(value) = fields.get(&field.handle) else {
			continue;
		};
		match &field.kind {
			FieldKind::Text { .. } => {
				out.extend(value.as_str().map(String::from));
			}
			FieldKind::Markdown => {
				out.extend(value.as_str().map(markdown_text));
			}
			FieldKind::RichText { .. } => {
				out.push(richtext::plain_text(value, true));
			}
			// the cells of text, not numbers, links or options
			FieldKind::Table { columns } => {
				for row in value.as_array().into_iter().flatten() {
					for c in
						columns.iter().filter(|c| c.kind == ColumnKind::Text)
					{
						out.extend(row[&c.handle].as_str().map(String::from));
					}
				}
			}
			FieldKind::Texts { .. } => out.extend(strings(value)),
			_ => {}
		}
		for i in inner(schema, field, value) {
			collect(schema, i.comp, i.fields, out);
		}
	}
}

/// The most words of a search: each one is a prefix query, and anyone
/// may search the website.
const MAX_WORDS: usize = 5;

/// The words of a search; signs alone (`-`) find nothing, so they go.
pub(crate) fn words(q: &str) -> Vec<String> {
	q.split_whitespace()
		.filter(|w| w.chars().any(char::is_alphanumeric))
		.take(MAX_WORDS)
		.map(String::from)
		.collect()
}

/// The words as an FTS5 query: each word the start of a word, all of
/// them somewhere in the title, slug or texts. A single letter is only
/// that word: as a start it would read most of the index (`entry_search`
/// has prefix indexes for two and three letters).
pub(crate) fn fts_query(words: &[String]) -> String {
	let quoted: Vec<String> = words
		.iter()
		.map(|w| {
			let star = if w.chars().count() > 1 { "*" } else { "" };
			format!("\"{}\"{star}", w.replace('"', "\"\""))
		})
		.collect();
	quoted.join(" ")
}

/// The rank of a match, the best first: the title counts most, then the
/// slug.
pub(crate) const RANK: &str = "bm25(entry_search, 10.0, 5.0, 1.0)";

/// Searches the entries of the sections the user may view in a site and
/// the assets, if they may view them.
pub fn search(
	conn: &Connection,
	schema: &Schema,
	auth: &Authed,
	site: &str,
	q: &str,
) -> Result<Found> {
	let words = words(q);
	if words.is_empty() {
		return Ok(Found::default());
	}
	let sections: Vec<&str> = schema
		.sections
		.iter()
		.map(|s| s.handle.as_str())
		.filter(|s| auth.can_list(schema, s))
		.collect();

	let own_only = auth.own_only(schema);
	let entries =
		find_entries(conn, site, &sections, own_only.as_ref(), &words)?;
	let assets = match auth.can_assets(schema, Access::View) {
		true => crate::assets::find(conn, &words, LIMIT)?,
		false => vec![],
	};
	Ok(Found { entries, assets })
}

/// The entries of the sections that have all the words, the title
/// counts most, then the slug; of `own_only` only the user's.
fn find_entries(
	conn: &Connection,
	site: &str,
	sections: &[&str],
	own_only: Option<&OwnOnly>,
	words: &[String],
) -> Result<Vec<EntryHit>> {
	if sections.is_empty() {
		return Ok(vec![]);
	}
	let own = own_only.map_or(("1".into(), vec![]), OwnOnly::sql);
	let sql = format!(
		"SELECT e.id, e.section, e.type, t.title, s.published,
		snippet(entry_search, 2, char(1), char(1), '…', 12)
		FROM entry_search
		JOIN entry_texts t ON t.id = entry_search.rowid
		JOIN entries e ON e.id = t.entry_id
		JOIN entry_sites s ON s.entry_id = t.entry_id AND s.site = t.site
		WHERE entry_search MATCH ? AND t.site = ? AND e.deleted_at IS NULL
		AND e.section IN ({}) AND {}
		ORDER BY {RANK} LIMIT {LIMIT}",
		marks(sections.len()),
		own.0
	);
	let mut values = vec![Text(fts_query(words)), Text(site.into())];
	values.extend(sections.iter().map(|s| Text(s.to_string())));
	values.extend(own.1);

	let mut stmt = conn.prepare(&sql)?;
	let hits = stmt
		.query_map(params_from_iter(values), |r| {
			let section: String = r.get(1)?;
			let id: i64 = r.get(0)?;
			let snippet: String = r.get(5)?;
			Ok(EntryHit {
				id,
				path: format!("/admin/entries/{section}/{id}"),
				section,
				ty: r.get(2)?,
				title: r.get(3)?,
				published: r.get(4)?,
				// without a mark the words were in the title or the slug
				snippet: snippet
					.contains(MARK)
					.then(|| snippet.replace(MARK, "")),
			})
		})?
		.collect::<rusqlite::Result<_>>()?;
	Ok(hits)
}

/// The entries of a query with `text`, the website's search: with an
/// excerpt of their texts and their score.
pub fn hits(conn: &Connection, q: &Query) -> Result<Vec<Hit>> {
	// without words nothing matches, and there is no index to rank
	if words(q.text.as_deref().unwrap_or_default()).is_empty() {
		return Ok(vec![]);
	}
	let (with, mut params) = q.with();
	let (from, more) = q.sql_from();
	params.extend(more);

	let sql = format!(
		"{with}SELECT {COLUMNS},
		snippet(entry_search, 2, char(1), char(2), '…', 24) AS excerpt,
		{RANK} AS score {from} {} LIMIT {} OFFSET {}",
		q.order_by(),
		q.limit,
		q.offset
	);
	let mut stmt = conn.prepare(&sql)?;
	let hits = stmt
		.query_map(params_from_iter(&params), |r| {
			let excerpt: String = r.get("excerpt")?;
			let rank: f64 = r.get("score")?;
			Ok(Hit {
				entry: from_row(r)?,
				excerpt: (!excerpt.trim().is_empty()).then_some(excerpt),
				// bm25 is negative, the best the lowest
				score: -rank,
			})
		})?
		.collect::<rusqlite::Result<_>>()?;
	Ok(hits)
}
