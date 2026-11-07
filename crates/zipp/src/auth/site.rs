//! Logins on the website: a frontend (the crelte server) logs a user in
//! for its visitors and sends the token as `Authorization: Bearer`. A
//! site session is a row of `sessions` with `site` set: it never opens
//! the studio's api, and a session of the studio is no site session.
//! Any user may log in on the website, members of a role without the
//! studio (see [`Role::no_studio`](crate::Role::no_studio)) and editors.
//!
//! ```no_run
//! # use zipp::auth::SiteUser;
//! async fn own(user: SiteUser) -> String {
//!     format!("hello {}", user.email)
//! }
//! ```

use axum::extract::{FromRef, FromRequestParts};
use axum::http::HeaderMap;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use super::links::{self, Kind, create_link};
use super::sessions::get_session;
use crate::SiteUrls;
use crate::db::{Db, now};
use crate::error::{Error, Result};
use crate::mail::Mail;
use crate::schema::Schema;

/// The user of a website request, from its bearer site token; without
/// one the route answers 401. For the website routes of plugins.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SiteUser {
	pub id: i64,
	/// empty when it isn't set
	pub name: String,
	pub email: String,
	/// `None` is an admin
	pub role: Option<String>,
}

impl SiteUser {
	/// The user of a site session's token, while it is valid.
	pub(crate) fn of_token(conn: &Connection, token: &str) -> Result<Self> {
		let s = get_session(conn, token, true)?
			.filter(|s| s.expires_at > now())
			.ok_or(Error::InvalidSessionToken)?;
		Self::get(conn, s.user_id)
	}

	pub(crate) fn get(conn: &Connection, id: i64) -> Result<Self> {
		let u = crate::users::user(conn, id)?;
		Ok(Self {
			id: u.id,
			name: u.name,
			email: u.email,
			role: u.role,
		})
	}
}

impl<S> FromRequestParts<S> for SiteUser
where
	S: Send + Sync,
	Db: FromRef<S>,
{
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
		let token =
			bearer(&parts.headers)?.ok_or(Error::MissingSessionToken)?;
		Self::of_token(&Db::from_ref(state).conn(), token)
	}
}

/// The site token of a request, `Authorization: Bearer <token>`.
pub(crate) fn bearer(headers: &HeaderMap) -> Result<Option<&str>> {
	let Some(v) = headers.get(AUTHORIZATION) else {
		return Ok(None);
	};
	v.to_str()
		.ok()
		.and_then(|v| v.strip_prefix("Bearer "))
		.map(Some)
		.ok_or(Error::InvalidSessionToken)
}

/// The url of the website for its links: of the site `site`, else of the
/// first site with a url (`serve --site-url`).
pub(crate) fn site_url(
	schema: &Schema,
	urls: &SiteUrls,
	site: Option<&str>,
) -> Result<String> {
	let url = match site {
		Some(s) => urls.get(s),
		None => schema.sites.iter().find_map(|s| urls.get(&s.handle)),
	};
	url.map(|u| u.trim_end_matches('/').to_string())
		.ok_or_else(|| {
			Error::Request(
				"links of the website need its url: serve --site-url".into(),
			)
		})
}

/// A new invite or reset link of a user to the website's page
/// `/reset-password?token=…`, see [`create_link`]: the url and when it
/// ends.
pub(crate) fn new_site_link(
	conn: &Connection,
	site_url: &str,
	user_id: i64,
	kind: Kind,
	created_by: Option<i64>,
) -> Result<(String, i64)> {
	let (token, expires_at) = create_link(conn, user_id, kind, created_by)?;
	Ok((
		format!("{site_url}/reset-password?token={token}"),
		expires_at,
	))
}

/// The mail of an invite or reset link of the website; `by` the admin
/// who sent it.
pub(crate) fn site_link_mail(
	site_url: &str,
	kind: Kind,
	to: &str,
	url: &str,
	by: Option<&str>,
) -> Mail {
	match kind {
		Kind::Invite => {
			let site = site_url
				.split_once("://")
				.map_or(site_url, |(_, host)| host);
			let who =
				by.map_or(String::new(), |b| format!("{b} invites you. "));
			let text = format!(
				"Hello\n\n{who}You have an account at {site} now. Choose \
				your password here, the link works for a week:\n\n{url}\n"
			);
			Mail::new(to, &format!("Your account at {site}"), &text)
		}
		kind => links::link_mail(site_url, kind, to, url, by),
	}
}
