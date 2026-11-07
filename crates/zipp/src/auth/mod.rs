//! Logins, sessions and what a user may do. A handler says who may call
//! it with its extractor: [`Authed`] any user, [`Admin`], [`EntryAccess`]
//! and [`AssetAccess`] a level of a role, [`AiAccess`] the AI chat.
//! Website routes have [`SiteUser`], a login on the website.

use std::marker::PhantomData;
use std::sync::Arc;

use axum::extract::{FromRef, FromRequestParts, RawPathParams};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use rusqlite::Connection;

use crate::AppState;
use crate::db::{Db, now};
use crate::error::{Error, Result};
use crate::schema::{Access, Role, Schema};

pub(crate) mod api;
#[cfg(test)]
mod api_tests;
pub(crate) mod links;
#[cfg(test)]
mod links_tests;
mod routes;
mod sessions;
pub(crate) mod site;
#[cfg(test)]
mod site_tests;
#[cfg(test)]
mod tests;
mod throttle;

pub(crate) use routes::{routes, site_routes};
pub use sessions::{Session, end_session, short_session};
pub(crate) use sessions::{
	add_user, end_sessions, hash_password, login, new_session,
	new_site_session, password_matches, renew_session, set_password_hash,
};
pub use site::SiteUser;
pub(crate) use throttle::Throttle;

/// Extractor that only succeeds with a valid `session-token` header.
#[non_exhaustive]
pub struct Authed {
	pub user_id: i64,
	/// `None` is an admin
	pub role: Option<String>,
	/// the session of the request
	pub token: String,
}

impl Authed {
	pub fn admin(&self) -> bool {
		self.role.is_none()
	}

	/// An admin, or the user `id` themself.
	pub(crate) fn check_self(&self, id: i64) -> Result<()> {
		allowed(self.admin() || self.user_id == id)
	}

	fn role<'a>(&self, schema: &'a Schema) -> Option<&'a Role> {
		schema.get_role(self.role.as_deref()?)
	}

	/// Whether the user may do `need` in a section.
	pub fn can(&self, schema: &Schema, section: &str, need: Access) -> bool {
		self.admin()
			|| self
				.role(schema)
				.and_then(|r| r.sections.get(section))
				.is_some_and(|a| *a >= need)
	}

	pub fn can_assets(&self, schema: &Schema, need: Access) -> bool {
		self.admin()
			|| self
				.role(schema)
				.and_then(|r| r.assets)
				.is_some_and(|a| a >= need)
	}

	pub fn can_ai(&self, schema: &Schema) -> bool {
		self.admin() || self.role(schema).is_some_and(|r| r.ai)
	}

	pub fn can_setup(&self, schema: &Schema) -> bool {
		self.admin() || self.role(schema).is_some_and(|r| r.setup)
	}

	/// [`Authed::can`] as an error, for a section that comes with the
	/// request.
	pub fn check(
		&self,
		schema: &Schema,
		section: &str,
		need: Access,
	) -> Result<()> {
		allowed(self.can(schema, section, need))
	}

	/// [`Authed::can_assets`] as an error, where no extractor checks it.
	pub fn check_assets(&self, schema: &Schema, need: Access) -> Result<()> {
		allowed(self.can_assets(schema, need))
	}
}

fn allowed(ok: bool) -> Result<()> {
	match ok {
		true => Ok(()),
		false => Err(Error::Forbidden),
	}
}

impl<S> FromRequestParts<S> for Authed
where
	S: Send + Sync,
	Db: FromRef<S>,
	Arc<Schema>: FromRef<S>,
{
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
		let db = Db::from_ref(state);
		let conn = db.conn();
		let token =
			token_of(&parts.headers)?.ok_or(Error::MissingSessionToken)?;
		let Some(user_id) = session(&conn, &parts.headers)? else {
			return Err(Error::MissingSessionToken);
		};
		let role = role_of(&conn, user_id)?;
		// a role that lost the studio: its sessions are gone for it
		if !studio_role(&Arc::<Schema>::from_ref(state), role.as_deref()) {
			return Err(Error::InvalidSessionToken);
		}
		Ok(Self {
			user_id,
			role,
			token: token.into(),
		})
	}
}

/// The levels of [`Access`] as types, for the extractors.
pub trait Level: Send + Sync + 'static {
	const ACCESS: Access;
}

pub struct View;
pub struct Edit;
pub struct Publish;
pub struct Delete;

impl Level for View {
	const ACCESS: Access = Access::View;
}
impl Level for Edit {
	const ACCESS: Access = Access::Edit;
}
impl Level for Publish {
	const ACCESS: Access = Access::Publish;
}
impl Level for Delete {
	const ACCESS: Access = Access::Delete;
}

/// A user without a role.
pub struct Admin(pub Authed);

impl FromRequestParts<AppState> for Admin {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		allowed(auth.admin())?;
		Ok(Self(auth))
	}
}

/// A user who may do `L` in the section of the entry `{id}` of the path.
pub struct EntryAccess<L> {
	pub auth: Authed,
	pub section: String,
	level: PhantomData<L>,
}

impl<L: Level> FromRequestParts<AppState> for EntryAccess<L> {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		let params = RawPathParams::from_request_parts(parts, state)
			.await
			.map_err(|e| Error::Request(e.to_string()))?;
		let id = params
			.iter()
			.find(|(k, _)| *k == "id")
			.and_then(|(_, v)| v.parse().ok())
			.ok_or(Error::NotFound)?;

		let section = crate::entries::section_of(&state.db.conn(), id)?;
		auth.check(&state.schema, &section, L::ACCESS)?;
		Ok(Self {
			auth,
			section,
			level: PhantomData,
		})
	}
}

/// A user who may do `L` with assets.
pub struct AssetAccess<L> {
	pub auth: Authed,
	level: PhantomData<L>,
}

impl<L: Level> FromRequestParts<AppState> for AssetAccess<L> {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		allowed(auth.can_assets(&state.schema, L::ACCESS))?;
		Ok(Self {
			auth,
			level: PhantomData,
		})
	}
}

/// A user who may use the AI chat; it sees every draft.
pub struct AiAccess(pub Authed);

impl FromRequestParts<AppState> for AiAccess {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		allowed(auth.can_ai(&state.schema))?;
		Ok(Self(auth))
	}
}

/// A user who may see Setup, the model read only.
pub(crate) struct SetupAccess(pub Authed);

impl FromRequestParts<AppState> for SetupAccess {
	type Rejection = Error;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self> {
		let auth = Authed::from_request_parts(parts, state).await?;
		allowed(auth.can_setup(&state.schema))?;
		Ok(Self(auth))
	}
}

/// The user of a request, `None` without a token. Public routes use it to
/// show unpublished entries to editors.
pub(crate) fn session(
	conn: &Connection,
	headers: &HeaderMap,
) -> Result<Option<i64>> {
	let Some(token) = token_of(headers)? else {
		return Ok(None);
	};

	match sessions::get_session(conn, token, false)? {
		Some(s) if s.expires_at > now() => Ok(Some(s.user_id)),
		_ => Err(Error::InvalidSessionToken),
	}
}

/// The token of a request, its `session-token` header.
pub(crate) fn token_of(headers: &HeaderMap) -> Result<Option<&str>> {
	match headers.get("session-token") {
		Some(t) => t.to_str().map(Some).map_err(|_| Error::InvalidSessionToken),
		None => Ok(None),
	}
}

/// The user `user_id` as [`Authed`], for checks inside a save that only
/// get the id; `None` (the system: migrations, the import) may do all.
pub(crate) fn user_of(
	conn: &Connection,
	user_id: Option<i64>,
) -> Result<Option<Authed>> {
	let Some(user_id) = user_id else {
		return Ok(None);
	};
	Ok(Some(Authed {
		user_id,
		role: role_of(conn, user_id)?,
		token: String::new(),
	}))
}

/// The role of a user, `None` is an admin.
pub(crate) fn role_of(
	conn: &Connection,
	user_id: i64,
) -> Result<Option<String>> {
	Ok(conn.query_row(
		"SELECT role FROM users WHERE id = ?1",
		[user_id],
		|r| r.get(0),
	)?)
}

/// Whether users of a role may use the studio, see [`Role::no_studio`];
/// admins always.
pub(crate) fn studio_role(schema: &Schema, role: Option<&str>) -> bool {
	role.and_then(|r| schema.get_role(r))
		.is_none_or(|r| r.studio)
}
