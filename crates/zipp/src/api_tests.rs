use serde::Deserialize;

use super::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Get {
	id: i64,
	site: Option<String>,
}
request!(Get, GET "/things/{id}" => ());

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Put {
	id: i64,
	rev: i64,
	title: String,
}
request!(Put, PUT "/things/{id}/{rev}" => ());

/// free pairs read from a list, not from a map
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(from = "Vec<(String, String)>")]
struct Pairs(Vec<(String, String)>);

impl From<Vec<(String, String)>> for Pairs {
	fn from(v: Vec<(String, String)>) -> Self {
		Self(v)
	}
}
request!(Pairs, GET "/things" => ());

fn params(p: &[(&str, &str)]) -> Vec<(String, String)> {
	p.iter()
		.map(|(k, v)| (k.to_string(), v.to_string()))
		.collect()
}

#[test]
fn fills_the_path() {
	let put = Put {
		id: 3,
		rev: 7,
		title: "x".into(),
	};
	assert_eq!(path_of(&put), "/things/3/7");
	assert_eq!(path_of(&Pairs(vec![])), "/things");
}

#[test]
fn reads_path_query_and_body() {
	// the query may repeat the path, the path wins
	let get: Get = from_query("site=en&id=9", params(&[("id", "3")])).unwrap();
	assert_eq!(
		get,
		Get {
			id: 3,
			site: Some("en".into())
		}
	);

	let put: Put =
		from_body(br#"{"title": "Hi"}"#, params(&[("id", "3"), ("rev", "7")]))
			.unwrap();
	assert_eq!(put.title, "Hi");
	assert_eq!((put.id, put.rev), (3, 7));

	// free pairs, like the filters of the entries
	let pairs: Pairs = from_query("a=1&a=2&b=x", vec![]).unwrap();
	assert_eq!(pairs.0.len(), 3);
	let query = serde_urlencoded::to_string(&pairs).unwrap();
	assert_eq!(query, "a=1&a=2&b=x");

	assert!(from_body::<Put>(b"{}", params(&[("id", "3")])).is_err());
}
