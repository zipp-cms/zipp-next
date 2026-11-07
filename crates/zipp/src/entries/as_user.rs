//! Changes as a user, for plugins: a website route of a member goes
//! through the same checks as the studio (the role's sections and own
//! access, validation, drafts and revisions, the index and the hooks),
//! with the user as [`Authed::of_site_user`].

use super::*;
use crate::drafts::{SiteDoc, get_draft, put_draft};
use crate::trash::{check_tree, trash_entry};

/// Where an update of [`update_as`] went.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum Saved {
	/// live, the user may publish
	Live,
	/// a draft, for someone who may publish it
	Draft,
}

/// An entry with the content of its draft in the entry's site, if there
/// is one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WithDraft {
	pub entry: Entry,
	pub draft: Option<SiteDoc>,
}

/// Only the entries the user may see: the sections of the query and of
/// the others of a search, of a section with own access only theirs.
pub(crate) fn restrict(
	auth: &Authed,
	schema: &Schema,
	q: &mut Query,
) -> Result<()> {
	for section in q.section.iter().chain(&q.any_sections) {
		if !auth.can_list(schema, section) {
			return Err(Error::Forbidden);
		}
	}
	// a search over all sections only finds what the role sees, also in
	// the total, or a search would tell whether a hidden entry matches;
	// the entries of `relatedToEntries` too
	if !auth.admin() {
		let sections: Vec<String> = schema
			.sections
			.iter()
			.filter(|s| auth.can_list(schema, &s.handle))
			.map(|s| s.handle.clone())
			.collect();
		let own_only = auth.own_only(schema);
		q.each_mut(&mut |q| {
			q.sections = Some(sections.clone());
			q.own_only = own_only.clone();
		});
	}
	Ok(())
}

/// The entries of a query the user may see, unpublished ones too, with
/// their drafts, and how many there are.
pub fn list_as(
	state: &AppState,
	auth: &Authed,
	mut q: Query,
) -> Result<(Vec<WithDraft>, i64)> {
	restrict(auth, &state.schema, &mut q)?;
	let conn = state.db.conn();
	let (entries, total) = list(&conn, &q)?;
	let ids: Vec<i64> = entries.iter().map(|e| e.id).collect();
	let mut drafts = crate::drafts::docs_of(&conn, &ids)?;
	let entries = entries
		.into_iter()
		.map(|entry| WithDraft {
			draft: drafts
				.remove(&entry.id)
				.and_then(|mut d| d.remove(&entry.site)),
			entry,
		})
		.collect();
	Ok((entries, total))
}

/// An entry in a site (default the primary one) with its draft, if the
/// user may see it.
pub fn get_as(
	state: &AppState,
	auth: &Authed,
	id: i64,
	site: Option<String>,
) -> Result<WithDraft> {
	let conn = state.db.conn();
	let (section, author) = owner_of(&conn, id)?;
	auth.check_entry(&state.schema, &section, author, Access::View)?;
	let entry = get_entry(&conn, id, &state.schema.site_or_primary(site))?;
	let draft =
		get_draft(&conn, id)?.and_then(|mut d| d.doc.remove(&entry.site));
	Ok(WithDraft { entry, draft })
}

/// A new entry of the user, published only if they may publish; like
/// `POST /api/entries`.
pub fn create_as(
	state: &AppState,
	auth: &Authed,
	mut input: EntryInput,
) -> Result<Entry> {
	// a new entry is the user's, own access is enough
	let (schema, me) = (&state.schema, Some(auth.user_id));
	auth.check_entry(schema, &input.section, me, Access::Edit)?;
	// published only by who may publish
	input.published &=
		auth.can_entry(schema, &input.section, me, Access::Publish);
	let entry = {
		let conn = state.db.conn();
		save(&conn, schema, None, input, me)?
	};
	state.changed(entry.id, &entry.section, ChangeKind::Saved);
	Ok(entry)
}

/// Saves an entry in the site of `input` as the user: into its draft
/// (its other sites stay), which goes live at once if they may publish,
/// like saving and publishing in the studio. The input is checked first
/// either way, a draft is only stored when a publish would take it, and
/// without the links the user may not make. The draft is the user's
/// afterwards, also over someone else's.
pub fn update_as(
	state: &AppState,
	auth: &Authed,
	id: i64,
	input: EntryInput,
) -> Result<Saved> {
	let schema = &state.schema;
	let user = Some(auth.user_id);
	let section = {
		let conn = state.db.conn();
		let (section, author) = owner_of(&conn, id)?;
		auth.check_entry(schema, &section, author, Access::Edit)?;
		// the draft gets the save as checked: what the user may not link
		// to is out, an editor who publishes it may see more
		let c = check(&conn, schema, Some(id), input, user)?;
		let doc = SiteDoc {
			ty: c.ty,
			title: c.title,
			slug: c.slug,
			published: c.published,
			fields: c.fields,
			dates: c.dates,
		};
		let base = get_draft(&conn, id)?.map(|d| d.updated_at);
		let draft = put_draft(&conn, schema, id, &c.site, doc, base, user)?;

		let publish = auth.can_entry(schema, &section, author, Access::Publish);
		match (draft, publish) {
			// nothing changed, the live entry is the save
			(None, _) => return Ok(Saved::Live),
			(Some(_), false) => return Ok(Saved::Draft),
			(Some(_), true) => crate::drafts::publish(&conn, schema, id, user)?,
		}
		section
	};
	state.changed(id, &section, ChangeKind::Published);
	Ok(Saved::Live)
}

/// Moves an entry to the trash as the user, like
/// `DELETE /api/entries/{id}`; in a structure with its children, which
/// have to be theirs too if only their own entries are.
pub fn delete_as(state: &AppState, auth: &Authed, id: i64) -> Result<()> {
	let section = {
		let conn = state.db.conn();
		let (section, author) = owner_of(&conn, id)?;
		let (schema, need) = (&state.schema, Access::Delete);
		auth.check_entry(schema, &section, author, need)?;
		check_tree(&conn, schema, auth, id, &section, need, None)?;
		trash_entry(&conn, schema, id, Some(auth.user_id))?;
		section
	};
	state.changed(id, &section, ChangeKind::Deleted);
	Ok(())
}
