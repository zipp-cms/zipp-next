//! Wrong passwords wait. An attempt counts before the password is
//! checked, so a burst of parallel logins can't slip through. The lock
//! is per address and email: someone else trying an admin's email locks
//! only themself out, not the admin. Each address and each email has a
//! higher limit of its own, against one address trying many emails and
//! many addresses trying one email.

use std::collections::BTreeMap;
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Mutex, PoisonError};

use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::request::Parts;

/// Wrong passwords of an email from one address before it has to wait.
pub(super) const MAX_FAILS: u32 = 10;
/// Of one address, any emails.
const MAX_PER_ADDRESS: u32 = 50;
/// Of one email, any addresses.
const MAX_PER_EMAIL: u32 = 100;
/// How long a count lasts from the first attempt, then it starts anew.
pub(super) const FAIL_WINDOW_MS: i64 = 1000 * 60 * 15;
/// The counts it keeps; when full, attempts that aren't counted yet wait
/// too, the memory stays small.
const MAX_ENTRIES: usize = 10_000;
/// Longer isn't an email of a user, it isn't counted.
pub(super) const MAX_EMAIL_LEN: usize = 254;

/// The attempts per key: count and the time of the first one.
#[derive(Debug, Default)]
pub struct Throttle {
	counts: Mutex<BTreeMap<String, (u32, i64)>>,
}

fn keys(address: &str, email: &str) -> [(String, u32); 3] {
	[
		(format!("{address} {email}"), MAX_FAILS),
		(format!("{address} "), MAX_PER_ADDRESS),
		(format!(" {email}"), MAX_PER_EMAIL),
	]
}

impl Throttle {
	/// Whether an attempt may try, counted if so; `email` in lowercase.
	pub(super) fn attempt(&self, address: &str, email: &str, now: i64) -> bool {
		let mut counts =
			self.counts.lock().unwrap_or_else(PoisonError::into_inner);
		counts.retain(|_, (_, first)| *first + FAIL_WINDOW_MS > now);

		let keys = keys(address, email);
		let full = counts.len() + keys.len() > MAX_ENTRIES;
		for (key, max) in &keys {
			match counts.get(key) {
				Some((n, _)) if n >= max => return false,
				None if full => return false,
				_ => {}
			}
		}
		for (key, _) in keys {
			counts.entry(key).or_insert((0, now)).0 += 1;
		}
		true
	}

	/// The password was right: the attempt doesn't count, the email starts
	/// anew from this address.
	pub(super) fn succeeded(&self, address: &str, email: &str) {
		let mut counts =
			self.counts.lock().unwrap_or_else(PoisonError::into_inner);
		let [(own, _), others @ ..] = keys(address, email);
		counts.remove(&own);
		for (key, _) in others {
			if let Some((n, _)) = counts.get_mut(&key) {
				*n = n.saturating_sub(1);
			}
		}
	}
}

/// The address of who sends a request: the peer, or behind a proxy on
/// the same machine or network (a loopback or private peer) the address
/// that proxy added last to `X-Forwarded-For`. The first ones of that
/// header are whatever the client wrote. `-` when there is none, like in
/// tests.
#[derive(Debug, Clone, PartialEq)]
pub struct ClientAddr(pub String);

impl<S: Send + Sync> FromRequestParts<S> for ClientAddr {
	type Rejection = Infallible;

	async fn from_request_parts(
		parts: &mut Parts,
		_state: &S,
	) -> Result<Self, Infallible> {
		let peer = parts
			.extensions
			.get::<ConnectInfo<SocketAddr>>()
			.map(|c| c.0.ip());
		let forwarded = parts
			.headers
			.get("x-forwarded-for")
			.and_then(|v| v.to_str().ok())
			.and_then(|v| v.rsplit(',').next())
			.and_then(|v| v.trim().parse::<IpAddr>().ok());
		let addr = match (peer, forwarded) {
			(Some(peer), Some(fwd)) if behind_proxy(peer) => Some(fwd),
			(peer, _) => peer,
		};
		Ok(Self(addr.map_or_else(|| "-".into(), |a| a.to_string())))
	}
}

/// Whether a peer is a proxy in front of zipp rather than a visitor.
fn behind_proxy(peer: IpAddr) -> bool {
	match peer {
		IpAddr::V4(ip) => ip.is_loopback() || ip.is_private(),
		IpAddr::V6(ip) => {
			ip.is_loopback()
				|| ip.is_unique_local()
				|| ip
					.to_ipv4_mapped()
					.is_some_and(|v4| v4.is_loopback() || v4.is_private())
		}
	}
}
