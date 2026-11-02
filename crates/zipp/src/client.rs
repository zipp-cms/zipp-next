//! Who sent a request, for the limits of logins and forms: the address
//! of the connection, or the one a trusted proxy names in
//! `X-Forwarded-For` (`serve --trusted-proxy`).
//!
//! ```no_run
//! # use zipp::client::ClientAddr;
//! async fn submit(ClientAddr(addr): ClientAddr) -> String {
//!     format!("from {addr}")
//! }
//! ```

use std::net::{IpAddr, SocketAddr};

use axum::extract::{ConnectInfo, FromRequestParts};
use axum::http::HeaderMap;
use axum::http::request::Parts;

use crate::AppState;

/// An address or a network of proxies: `127.0.0.1`, `172.16.0.0/12`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Proxy {
	addr: IpAddr,
	prefix: u8,
}

impl std::str::FromStr for Proxy {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, String> {
		let err = || format!("--trusted-proxy {s:?}: an address or a network");
		let (addr, prefix) = match s.split_once('/') {
			Some((a, p)) => (a, Some(p.parse::<u8>().map_err(|_| err())?)),
			None => (s, None),
		};
		let addr: IpAddr = addr.parse().map_err(|_| err())?;
		let bits = if addr.is_ipv4() { 32 } else { 128 };
		let prefix = prefix.unwrap_or(bits);
		if prefix > bits {
			return Err(err());
		}
		Ok(Self { addr, prefix })
	}
}

impl Proxy {
	fn contains(&self, ip: IpAddr) -> bool {
		// a proxy on IPv6 sockets may give IPv4 addresses mapped
		let ip = ip.to_canonical();
		let mask = |bits: u32| match self.prefix {
			0 => 0,
			p => u128::MAX << (bits - p as u32),
		};
		match (self.addr, ip) {
			(IpAddr::V4(a), IpAddr::V4(b)) => {
				let m = mask(32) as u32;
				u32::from(a) & m == u32::from(b) & m
			}
			(IpAddr::V6(a), IpAddr::V6(b)) => {
				let m = mask(128);
				u128::from(a) & m == u128::from(b) & m
			}
			_ => false,
		}
	}
}

/// The client: going back from the connection, the first address that
/// isn't a trusted proxy. Only a trusted one may name who it forwards
/// for, anyone else could write any address into the header.
pub(crate) fn client_addr(
	proxies: &[Proxy],
	peer: IpAddr,
	headers: &HeaderMap,
) -> IpAddr {
	let trusted = |ip: IpAddr| proxies.iter().any(|p| p.contains(ip));
	let mut client = peer.to_canonical();
	// each proxy adds the one it got the request from at the end
	let forwarded = headers
		.get_all("x-forwarded-for")
		.iter()
		.filter_map(|v| v.to_str().ok())
		.flat_map(|v| v.split(','))
		.collect::<Vec<_>>();
	for hop in forwarded.iter().rev() {
		if !trusted(client) {
			break;
		}
		match hop.trim().parse::<IpAddr>() {
			Ok(ip) => client = ip.to_canonical(),
			Err(_) => break,
		}
	}
	client
}

/// The address of who sent the request, see [`client_addr`]. Without a
/// connection (a router called directly) it is `0.0.0.0`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClientAddr(pub IpAddr);

impl ClientAddr {
	/// What a limit counts: an IPv6 address by its /64 network, a visitor
	/// usually gets a whole one.
	pub fn network(&self) -> IpAddr {
		match self.0 {
			IpAddr::V6(v6) => {
				let net = u128::from(v6) & (u128::MAX << 64);
				IpAddr::V6(net.into())
			}
			v4 => v4,
		}
	}
}

impl FromRequestParts<AppState> for ClientAddr {
	type Rejection = std::convert::Infallible;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &AppState,
	) -> Result<Self, Self::Rejection> {
		// also the `MockConnectInfo` of the tests
		let peer = ConnectInfo::<SocketAddr>::from_request_parts(parts, state)
			.await
			.map_or(IpAddr::from([0, 0, 0, 0]), |c| c.0.ip());
		Ok(Self(client_addr(
			&state.trusted_proxies,
			peer,
			&parts.headers,
		)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn client(proxies: &[&str], peer: &str, forwarded: &[&str]) -> String {
		let proxies: Vec<Proxy> =
			proxies.iter().map(|p| p.parse().unwrap()).collect();
		let mut headers = HeaderMap::new();
		for f in forwarded {
			headers.append("x-forwarded-for", f.parse().unwrap());
		}
		client_addr(&proxies, peer.parse().unwrap(), &headers).to_string()
	}

	#[test]
	fn only_trusted_proxies_name_the_client() {
		// no proxy: the header is the visitor's, it doesn't count
		assert_eq!(client(&[], "9.9.9.9", &["1.1.1.1"]), "9.9.9.9");
		assert_eq!(client(&["127.0.0.1"], "9.9.9.9", &["1.1.1.1"]), "9.9.9.9");

		// the proxy's last hop, not what the visitor wrote before it
		let proxy = ["127.0.0.1"];
		assert_eq!(client(&proxy, "127.0.0.1", &["1.1.1.1"]), "1.1.1.1");
		assert_eq!(
			client(&proxy, "127.0.0.1", &["6.6.6.6, 2.2.2.2"]),
			"2.2.2.2"
		);
		assert_eq!(
			client(&proxy, "127.0.0.1", &["6.6.6.6", "2.2.2.2"]),
			"2.2.2.2"
		);

		// a chain of proxies, a network
		let proxies = ["127.0.0.1", "172.16.0.0/12"];
		assert_eq!(
			client(&proxies, "127.0.0.1", &["6.6.6.6, 3.3.3.3, 172.18.0.5"]),
			"3.3.3.3"
		);

		// nothing usable: the last proxy
		assert_eq!(client(&proxy, "127.0.0.1", &[]), "127.0.0.1");
		assert_eq!(client(&proxy, "127.0.0.1", &["junk"]), "127.0.0.1");
		assert_eq!(client(&["::1"], "::1", &["2001:db8::1"]), "2001:db8::1");
		assert_eq!(client(&proxy, "::ffff:127.0.0.1", &["1.1.1.1"]), "1.1.1.1");
	}

	#[test]
	fn ipv6_counts_by_its_network() {
		let a = ClientAddr("2001:db8:1:2:3::9".parse().unwrap());
		assert_eq!(a.network().to_string(), "2001:db8:1:2::");
		let b = ClientAddr("1.2.3.4".parse().unwrap());
		assert_eq!(b.network().to_string(), "1.2.3.4");
	}

	#[test]
	fn proxies_are_addresses_or_networks() {
		assert!("127.0.0.1".parse::<Proxy>().is_ok());
		assert!("10.0.0.0/8".parse::<Proxy>().is_ok());
		assert!("fd00::/8".parse::<Proxy>().is_ok());
		assert!("10.0.0.0/33".parse::<Proxy>().is_err());
		assert!("localhost".parse::<Proxy>().is_err());
		let net: Proxy = "10.0.0.0/8".parse().unwrap();
		assert!(net.contains("10.200.1.1".parse().unwrap()));
		assert!(!net.contains("11.0.0.1".parse().unwrap()));
		let all: Proxy = "0.0.0.0/0".parse().unwrap();
		assert!(all.contains("8.8.8.8".parse().unwrap()));
	}
}
