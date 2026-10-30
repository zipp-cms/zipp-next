//! The rest api of the studio, under `/api`. Every request is a type that
//! knows its method, path and response ([`Request`]): handlers take it
//! with [`Req`], routers add it with [`ApiRouter::api`] (`.api::<LoginReq, _>(login)`) and tests send it
//! with [`TestServer`](crate::testing::TestServer), so a path is written
//! once.
//!
//! What isn't JSON stays a plain axum route: uploads (multipart), files,
//! GraphQL and the streams of the AI chat.

use axum::body::Bytes;
use axum::extract::{FromRequest, FromRequestParts, RawPathParams};
use axum::handler::Handler;
use axum::http::Method;
use axum::routing::{MethodFilter, on};
use axum::{Json, Router};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::error::{Error, Result};

/// A request of the api. Path params like `{id}` are fields of the
/// request, the other fields are the query of a `GET` or `DELETE` and
/// the JSON body of the others. A request without fields is an empty
/// struct (`struct LogoutReq {}`), so it reads from `{}`.
pub trait Request: Serialize + DeserializeOwned + Send + 'static {
	const METHOD: Method;
	/// The path under `/api`, like axum's: `/entries/{id}/publish`.
	const PATH: &'static str;
	type Response: Serialize;
}

/// Implements [`Request`]: `request!(PublishReq, POST "/entries/{id}/publish" => ());`
#[macro_export]
macro_rules! request {
	($req:ty, $method:ident $path:literal => $resp:ty) => {
		impl $crate::api::Request for $req {
			const METHOD: $crate::axum::http::Method =
				$crate::axum::http::Method::$method;
			const PATH: &'static str = $path;
			type Response = $resp;
		}
	};
}

/// What a handler of `R` returns.
pub type ApiResult<R> = Result<Json<<R as Request>::Response>>;

/// Extracts a [`Request`] from the path params and the query or body.
pub struct Req<R>(pub R);

impl<R: Request, S: Send + Sync> FromRequest<S> for Req<R> {
	type Rejection = Error;

	async fn from_request(
		req: axum::extract::Request,
		state: &S,
	) -> Result<Self> {
		let (mut parts, body) = req.into_parts();
		let params = RawPathParams::from_request_parts(&mut parts, state)
			.await
			.map_err(|e| Error::Request(e.to_string()))?;
		let params: Vec<(String, String)> = params
			.iter()
			.map(|(k, v)| (k.to_string(), v.to_string()))
			.collect();

		if has_query(&R::METHOD) {
			let query = parts.uri.query().unwrap_or_default();
			return from_query(query, params).map(Self);
		}
		let req = axum::extract::Request::from_parts(parts, body);
		let body = Bytes::from_request(req, state)
			.await
			.map_err(|e| Error::Request(e.to_string()))?;
		from_body(&body, params).map(Self)
	}
}

/// Whether the fields of a request are its query, not a body.
pub(crate) fn has_query(method: &Method) -> bool {
	*method == Method::GET || *method == Method::DELETE
}

fn from_query<R: Request>(
	query: &str,
	params: Vec<(String, String)>,
) -> Result<R> {
	let bad = |e: serde_urlencoded::de::Error| Error::Request(e.to_string());
	let mut pairs: Vec<(String, String)> =
		serde_urlencoded::from_str(query).map_err(bad)?;
	// the path wins over the same name in the query
	pairs.retain(|(k, _)| !params.iter().any(|(p, _)| p == k));
	pairs.extend(params);

	let query = serde_urlencoded::to_string(&pairs)
		.map_err(|e| Error::Internal(e.to_string()))?;
	serde_urlencoded::from_str(&query).map_err(bad)
}

fn from_body<R: Request>(
	body: &[u8],
	params: Vec<(String, String)>,
) -> Result<R> {
	let bad = |e: serde_json::Error| Error::Request(e.to_string());
	let mut value = match body.is_empty() {
		true => Value::Object(Map::new()),
		false => serde_json::from_slice(body).map_err(bad)?,
	};
	// path params are ids or names
	if let Value::Object(map) = &mut value {
		for (k, v) in params {
			let v = v.parse::<i64>().map_or(Value::String(v), Value::from);
			map.insert(k, v);
		}
	}
	serde_json::from_value(value).map_err(bad)
}

/// The path of a request, with its path params filled in.
pub fn path_of<R: Request>(req: &R) -> String {
	let value = serde_json::to_value(req).unwrap_or_default();
	let mut path = String::new();
	let mut rest = R::PATH;
	while let Some((before, after)) = rest.split_once('{') {
		let (name, after) = after.split_once('}').unwrap_or((after, ""));
		let param = match value.get(name) {
			Some(Value::String(s)) => s.clone(),
			Some(v) => v.to_string(),
			None => String::new(),
		};
		path.push_str(before);
		path.push_str(&param);
		rest = after;
	}
	path.push_str(rest);
	path
}

/// Adds requests to a router.
pub trait ApiRouter<S> {
	/// The handler of `R` at its method and path.
	fn api<R: Request, T: 'static>(self, handler: impl Handler<T, S>) -> Self;
}

impl<S: Clone + Send + Sync + 'static> ApiRouter<S> for Router<S> {
	fn api<R: Request, T: 'static>(self, handler: impl Handler<T, S>) -> Self {
		let filter = MethodFilter::try_from(R::METHOD)
			.expect("a request has a standard method");
		self.route(R::PATH, on(filter, handler))
	}
}

#[cfg(test)]
mod tests {
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
		let get: Get =
			from_query("site=en&id=9", params(&[("id", "3")])).unwrap();
		assert_eq!(
			get,
			Get {
				id: 3,
				site: Some("en".into())
			}
		);

		let put: Put = from_body(
			br#"{"title": "Hi"}"#,
			params(&[("id", "3"), ("rev", "7")]),
		)
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
}
