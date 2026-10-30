//! One-time links of a user, `/admin/password/<token>` and
//! `/admin/login-as/<token>`: an invite (a user without a password sets
//! one), a password reset (asked for on the login page or made by an
//! admin) and logging in as a user, for admins, like Craft's
//! impersonation. A link works once; a new one of the same kind ends the
//! one before.

use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

use super::Session;
use crate::db::{now, random_token};
use crate::error::{Error, Result};
use crate::mail::Mail;

const HOUR_MS: i64 = 1000 * 60 * 60;

/// What a link does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LinkKind {
	/// a new user sets their password, a week long
	Invite,
	/// a new password, an hour long
	Reset,
	/// an admin logs in as the user, ten minutes long
	LoginAs,
}

impl LinkKind {
	fn as_str(self) -> &'static str {
		match self {
			Self::Invite => "invite",
			Self::Reset => "reset",
			Self::LoginAs => "loginAs",
		}
	}

	fn parse(s: &str) -> Option<Self> {
		[Self::Invite, Self::Reset, Self::LoginAs]
			.into_iter()
			.find(|k| k.as_str() == s)
	}

	/// How long it works.
	pub fn duration_ms(self) -> i64 {
		match self {
			Self::Invite => 7 * 24 * HOUR_MS,
			Self::Reset => HOUR_MS,
			Self::LoginAs => HOUR_MS / 6,
		}
	}

	/// The page of the studio that opens it.
	pub fn path(self, token: &str) -> String {
		match self {
			Self::LoginAs => format!("/admin/login-as/{token}"),
			_ => format!("/admin/password/{token}"),
		}
	}
}

/// A valid link.
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
	pub user_id: i64,
	pub kind: LinkKind,
	pub created_by: Option<i64>,
	pub created_at: i64,
	pub expires_at: i64,
}

/// A new link of a user, the one of the same kind before ends; its token.
pub fn create_link(
	conn: &Connection,
	user_id: i64,
	kind: LinkKind,
	created_by: Option<i64>,
) -> Result<(String, i64)> {
	conn.execute("DELETE FROM user_links WHERE expires_at < ?1", [now()])?;
	conn.execute(
		"DELETE FROM user_links WHERE user_id = ?1 AND kind = ?2",
		params![user_id, kind.as_str()],
	)?;
	let token = random_token();
	let expires_at = now() + kind.duration_ms();
	conn.execute(
		"INSERT INTO user_links
		(token, user_id, kind, created_by, created_at, expires_at)
		VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
		params![token, user_id, kind.as_str(), created_by, now(), expires_at],
	)?;
	Ok((token, expires_at))
}

/// The link of a token while it works.
pub fn get_link(conn: &Connection, token: &str) -> Result<Link> {
	let row: Option<(i64, String, Option<i64>, i64, i64)> = conn
		.query_row(
			"SELECT user_id, kind, created_by, created_at, expires_at
			FROM user_links WHERE token = ?1 AND expires_at > ?2",
			params![token, now()],
			|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
		)
		.optional()?;
	let gone = || Error::Request("This link is used or expired".into());
	let (user_id, kind, created_by, created_at, expires_at) =
		row.ok_or_else(gone)?;
	Ok(Link {
		user_id,
		kind: LinkKind::parse(&kind).ok_or_else(gone)?,
		created_by,
		created_at,
		expires_at,
	})
}

/// The link of a token if it is of one of the kinds, used up.
pub fn take_link(
	conn: &Connection,
	token: &str,
	kinds: &[LinkKind],
) -> Result<Link> {
	let link = get_link(conn, token)?;
	if !kinds.contains(&link.kind) {
		return Err(Error::Request("This link is used or expired".into()));
	}
	conn.execute("DELETE FROM user_links WHERE token = ?1", [token])?;
	Ok(link)
}

/// Whether a user still has to set a password, invited by an admin.
pub fn invited(conn: &Connection, user_id: i64) -> Result<bool> {
	Ok(conn.query_row(
		"SELECT password_hash = '' FROM users WHERE id = ?1",
		[user_id],
		|r| r.get(0),
	)?)
}

/// How long a session of logging in as someone lasts; it isn't renewed.
pub const LOGIN_AS_MS: i64 = 2 * HOUR_MS;

/// A session of `user_id` for the admin `by`, see [`LinkKind::LoginAs`];
/// it isn't the user's login.
pub fn login_as_session(
	conn: &Connection,
	user_id: i64,
	by: i64,
) -> Result<Session> {
	let session = Session {
		token: random_token(),
		user_id,
		expires_at: now() + LOGIN_AS_MS,
		impersonated_by: Some(by),
	};
	conn.execute(
		"INSERT INTO sessions
		(token, user_id, created_at, expires_at, impersonated_by)
		VALUES (?1, ?2, ?3, ?4, ?5)",
		params![session.token, user_id, now(), session.expires_at, by],
	)?;
	Ok(session)
}

/// The full url of a link, after `serve --public-url`.
pub fn link_url(
	public_url: &str,
	kind: LinkKind,
	token: &str,
) -> Result<String> {
	if public_url.is_empty() {
		return Err(Error::Request(
			"links need the url of the cms: serve --public-url".into(),
		));
	}
	Ok(format!("{public_url}{}", kind.path(token)))
}

/// The mail of an invite or reset link; `by` the admin who sent it.
pub fn link_mail(
	public_url: &str,
	kind: LinkKind,
	to: &str,
	url: &str,
	by: Option<&str>,
) -> Mail {
	let site = public_url
		.split_once("://")
		.map_or(public_url, |(_, host)| host);
	match kind {
		LinkKind::Invite => {
			let who =
				by.map_or(String::new(), |b| format!("{b} invites you. "));
			let text = format!(
				"Hello\n\n{who}You have an account in the studio of {site} \
				now. Choose your password here, the link works for a \
				week:\n\n{url}\n"
			);
			Mail::new(to, &format!("Your account at {site}"), &text)
		}
		_ => {
			let text = format!(
				"Hello\n\nSomeone asked for a new password of your account \
				at {site}. Choose one here, the link works for an \
				hour:\n\n{url}\n\nIf it wasn't you, ignore this mail: \
				your password stays.\n"
			);
			Mail::new(to, &format!("A new password for {site}"), &text)
		}
	}
}
