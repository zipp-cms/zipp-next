//! The submissions of the forms, a table of the plugin: the values with
//! the labels their fields had, so a changed form keeps what came in.
//! A form deleted for good (out of the trash) takes its submissions.

use serde::{Deserialize, Serialize};
use zipp::db::now;
use zipp::error::{Error, Result};
use zipp::rusqlite::{Connection, Row, params};

use crate::form::Answer;

/// The tables of the plugin, see `Plugin::tables`.
pub(crate) const TABLES: &[&str] = &[r#"
	CREATE TABLE forms_submissions (
		id INTEGER PRIMARY KEY,
		form_id INTEGER NOT NULL REFERENCES entries(id) ON DELETE CASCADE,
		site TEXT NOT NULL,
		answers TEXT NOT NULL,
		created_at INTEGER NOT NULL
	);
	CREATE INDEX forms_submissions_form
	ON forms_submissions(form_id, created_at);
"#];

/// A submission, the newest come first.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Submission {
	pub id: i64,
	pub form_id: i64,
	/// the title of the form in the primary site
	pub form: String,
	pub site: String,
	pub answers: Vec<Answer>,
	pub created_at: i64,
}

/// How many a list has at most.
pub const PAGE: i64 = 50;

pub(crate) fn insert(
	conn: &Connection,
	form: i64,
	site: &str,
	answers: &[Answer],
) -> Result<i64> {
	let answers = serde_json::to_string(answers)
		.map_err(|e| Error::Internal(e.to_string()))?;
	conn.execute(
		"INSERT INTO forms_submissions (form_id, site, answers, created_at)
		VALUES (?, ?, ?, ?)",
		params![form, site, answers, now()],
	)?;
	Ok(conn.last_insert_rowid())
}

/// A page of the submissions of a form or all, the newest first, and how
/// many there are; `site` is the one whose titles name the forms.
pub(crate) fn list(
	conn: &Connection,
	site: &str,
	form: Option<i64>,
	offset: i64,
) -> Result<(Vec<Submission>, i64)> {
	let total = conn.query_row(
		"SELECT count(*) FROM forms_submissions
		WHERE ?1 IS NULL OR form_id = ?1",
		[form],
		|r| r.get(0),
	)?;
	let mut stmt = conn.prepare(
		"SELECT f.id, f.form_id, coalesce(s.title, ''), f.site, f.answers,
			f.created_at
		FROM forms_submissions f
		LEFT JOIN entry_sites s ON s.entry_id = f.form_id AND s.site = ?2
		WHERE ?1 IS NULL OR f.form_id = ?1
		ORDER BY f.created_at DESC, f.id DESC
		LIMIT ?3 OFFSET ?4",
	)?;
	let rows = stmt
		.query_map(params![form, site, PAGE, offset], from_row)?
		.collect::<zipp::rusqlite::Result<_>>()?;
	Ok((rows, total))
}

/// Deletes a submission, NOT_FOUND if there is none.
pub(crate) fn delete(conn: &Connection, id: i64) -> Result<()> {
	match conn.execute("DELETE FROM forms_submissions WHERE id = ?", [id])? {
		0 => Err(Error::NotFound),
		_ => Ok(()),
	}
}

fn from_row(r: &Row) -> zipp::rusqlite::Result<Submission> {
	let answers: String = r.get(4)?;
	Ok(Submission {
		id: r.get(0)?,
		form_id: r.get(1)?,
		form: r.get(2)?,
		site: r.get(3)?,
		answers: serde_json::from_str(&answers).unwrap_or_default(),
		created_at: r.get(5)?,
	})
}
