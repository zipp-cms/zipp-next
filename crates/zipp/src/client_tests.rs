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
