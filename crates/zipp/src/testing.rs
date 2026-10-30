//! A server for tests: the whole app on an in-memory database, requests
//! go through the router without a port. Sites and plugins get it with
//! the feature `testing`:
//!
//! ```ignore
//! let ts = TestServer::new(schema());
//! let author = ts.user(Some("author"));
//! let err = author.call(&PublishReq { id: 1 }).await.unwrap_err();
//! assert_eq!(err, Error::Forbidden);
//! ```

use std::fs;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{self, header};
use axum::response::Response;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tower::ServiceExt;

use crate::api::{Request, has_query, path_of};
use crate::db::Db;
use crate::error::{Error, Result};
use crate::schema::Schema;
use crate::{AppState, graphql, migrate, server};

static SERVERS: AtomicU32 = AtomicU32::new(0);

pub struct TestServer {
	pub state: AppState,
	app: Router,
	/// the site's, see [`TestServer::routes`]
	routes: Router<AppState>,
	users: AtomicU32,
}

impl TestServer {
	/// The schema as the site builds it, the plugins finish it here.
	pub fn new(schema: Schema) -> Self {
		let schema = schema.finish();
		schema.check().expect("the schema has a mistake");
		let db = Db::memory().unwrap();
		migrate::apply::create_singles(&db.conn(), &schema).unwrap();

		let n = SERVERS.fetch_add(1, Ordering::Relaxed);
		let files = std::env::temp_dir()
			.join(format!("zipp-test-{}-{n}", std::process::id()));
		fs::create_dir_all(&files).unwrap();

		let schema = Arc::new(schema);
		let gql = graphql::build(
			schema.clone(),
			db.clone(),
			String::new(),
			Default::default(),
		)
		.unwrap();
		let state = AppState {
			graphql_token: Some(Arc::new("test-token".into())),
			mail: crate::mail::Mailer::memory(),
			..AppState::new(db, schema, gql, files)
		};
		Self {
			app: server::app(state.clone(), Router::new()),
			routes: Router::new(),
			state,
			users: AtomicU32::new(0),
		}
	}

	/// Like [`Cms::on_change`](crate::Cms::on_change).
	pub fn on_change(
		mut self,
		hook: impl Fn(&crate::Change) + Send + Sync + 'static,
	) -> Self {
		let mut hooks = (*self.state.hooks).clone();
		hooks.push(Arc::new(hook));
		self.state.hooks = Arc::new(hooks);
		self.app = server::app(self.state.clone(), self.routes.clone());
		self
	}

	/// Routes of the site, like [`Cms::routes`](crate::Cms::routes).
	pub fn routes(mut self, routes: Router<AppState>) -> Self {
		self.routes = self.routes.clone().merge(routes);
		self.app = server::app(self.state.clone(), self.routes.clone());
		self
	}

	/// Development mode, like `serve --dev`.
	pub fn dev(mut self) -> Self {
		self.state.dev = true;
		self.app = server::app(self.state.clone(), self.routes.clone());
		self
	}

	/// Someone without a session.
	pub fn anonymous(&self) -> Client {
		Client {
			app: self.app.clone(),
			token: None,
		}
	}

	/// A logged in user with a role, `None` is an admin. The password is
	/// `password`.
	pub fn user(&self, role: Option<&str>) -> Client {
		let n = self.users.fetch_add(1, Ordering::Relaxed);
		let email = format!("{}{n}@test.ch", role.unwrap_or("admin"));
		let conn = self.state.db.conn();
		let user_id =
			crate::users::create(&conn, &email, "", password_hash(), role)
				.unwrap();
		let token = crate::auth::new_session(&conn, user_id).unwrap().token;
		Client {
			app: self.app.clone(),
			token: Some(token),
		}
	}

	/// The mails sent so far: a test server keeps them.
	pub fn mails(&self) -> Vec<crate::mail::Mail> {
		self.state.mail.sent()
	}

	pub fn admin(&self) -> Client {
		self.user(None)
	}
}

impl Drop for TestServer {
	fn drop(&mut self) {
		let _ = fs::remove_dir_all(&self.state.files);
	}
}

/// bcrypt of `password`, made once: it takes a while.
fn password_hash() -> &'static str {
	static HASH: std::sync::OnceLock<String> = std::sync::OnceLock::new();
	HASH.get_or_init(|| bcrypt::hash("password", 4).unwrap())
}

/// Sends requests to a [`TestServer`], as one user.
#[derive(Clone)]
pub struct Client {
	app: Router,
	pub token: Option<String>,
}

#[derive(Deserialize)]
struct ErrorResp {
	error: Error,
}

impl Client {
	/// Sends a request, the answer is its response or the error.
	pub async fn call<R: Request>(&self, req: &R) -> Result<R::Response>
	where
		R::Response: DeserializeOwned,
	{
		let body = self.call_bytes(req).await?;
		let body = if body.is_empty() { b"null" } else { &body[..] };
		Ok(serde_json::from_slice(body).expect("the response of R"))
	}

	/// [`Client::call`] for responses that are only `Serialize`.
	pub async fn call_json<R: Request>(&self, req: &R) -> Result<Value> {
		let body = self.call_bytes(req).await?;
		Ok(serde_json::from_slice(&body).unwrap_or_default())
	}

	async fn call_bytes<R: Request>(&self, req: &R) -> Result<Vec<u8>> {
		let mut uri = format!("/api{}", path_of(req));
		let body = if has_query(&R::METHOD) {
			let query = serde_urlencoded::to_string(req).unwrap();
			if !query.is_empty() {
				uri = format!("{uri}?{query}");
			}
			Body::empty()
		} else {
			Body::from(serde_json::to_vec(req).unwrap())
		};
		let req = http::Request::builder()
			.method(R::METHOD)
			.uri(uri)
			.header(header::CONTENT_TYPE, "application/json")
			.body(body)
			.unwrap();

		let resp = self.send(req).await;
		let status = resp.status();
		let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
		if status.is_success() {
			return Ok(body.to_vec());
		}
		let resp: ErrorResp = serde_json::from_slice(&body)
			.unwrap_or_else(|_| panic!("{status}: an error response"));
		Err(resp.error)
	}

	/// A multipart upload of files (name, data) to a path under `/api`.
	pub async fn upload(
		&self,
		path: &str,
		files: &[(&str, &[u8])],
	) -> Response {
		let boundary = "zipp-test-boundary";
		let mut body = vec![];
		for (name, data) in files {
			body.extend(
				format!(
					"--{boundary}\r\nContent-Disposition: form-data; \
				name=\"file\"; filename=\"{name}\"\r\n\
				Content-Type: application/octet-stream\r\n\r\n"
				)
				.bytes(),
			);
			body.extend_from_slice(data);
			body.extend(b"\r\n");
		}
		body.extend(format!("--{boundary}--\r\n").bytes());
		let req = http::Request::builder()
			.method("POST")
			.uri(format!("/api{path}"))
			.header(
				header::CONTENT_TYPE,
				format!("multipart/form-data; boundary={boundary}"),
			)
			.body(Body::from(body))
			.unwrap();
		self.send(req).await
	}

	/// The JSON of a response, or its error.
	pub async fn json<T: DeserializeOwned>(resp: Response) -> Result<T> {
		let status = resp.status();
		let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
		if status.is_success() {
			return Ok(serde_json::from_slice(&body).unwrap());
		}
		let resp: ErrorResp = serde_json::from_slice(&body)
			.unwrap_or_else(|_| panic!("{status}: an error response"));
		Err(resp.error)
	}

	/// Any request, with the session of the client.
	pub async fn send(&self, mut req: http::Request<Body>) -> Response {
		if let Some(token) = &self.token {
			req.headers_mut()
				.insert("session-token", token.parse().unwrap());
		}
		self.app.clone().oneshot(req).await.unwrap()
	}
}
