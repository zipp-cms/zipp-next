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

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::connect_info::MockConnectInfo;
use axum::http::{self, header};
use axum::response::Response;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tempfile::TempDir;
use tower::ServiceExt;

use crate::api::{Request, has_query, path_of};
use crate::db::Db;
use crate::entries::EntryInput;
use crate::error::{Error, Result};
use crate::schema::Schema;
use crate::{AppState, SiteUrls, graphql, migrate, server};

pub struct TestServer {
	pub state: AppState,
	app: Router,
	/// the site's, see [`TestServer::routes`]
	routes: Router<AppState>,
	users: AtomicU32,
	/// the asset files, deleted with the server
	_files: TempDir,
}

impl TestServer {
	/// The schema as the site builds it, the plugins finish it here.
	pub fn new(schema: Schema) -> Self {
		let schema = schema.finish();
		schema.check().expect("the schema has a mistake");
		let db = Db::memory().unwrap();
		migrate::apply::create_singles(&db.conn(), &schema).unwrap();

		let files = tempfile::tempdir().unwrap();

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
			public_url: Arc::new("http://zipp.test".into()),
			..AppState::new(db, schema, gql, files.path().into())
		};
		Self {
			app: app(&state, Router::new()),
			routes: Router::new(),
			state,
			users: AtomicU32::new(0),
			_files: files,
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
		self.app = app(&self.state, self.routes.clone());
		self
	}

	/// Routes of the site, like [`Cms::routes`](crate::Cms::routes).
	pub fn routes(mut self, routes: Router<AppState>) -> Self {
		self.routes = self.routes.clone().merge(routes);
		self.app = app(&self.state, self.routes.clone());
		self
	}

	/// Development mode, like `serve --dev`.
	pub fn dev(mut self) -> Self {
		self.state.dev = true;
		self.app = app(&self.state, self.routes.clone());
		self
	}

	/// The cms' url, like `serve --public-url`; empty is none.
	pub fn public_url(mut self, url: &str) -> Self {
		self.state.public_url = Arc::new(url.into());
		self.app = app(&self.state, self.routes.clone());
		self
	}

	/// The url of each site, like `serve --site-url`: previews, and
	/// GraphQL's urls of pages.
	pub fn site_urls(mut self, urls: SiteUrls) -> Self {
		self.state.gql = graphql::build(
			self.state.schema.clone(),
			self.state.db.clone(),
			String::new(),
			urls.clone(),
		)
		.unwrap();
		self.state.site_urls = Arc::new(urls);
		self.app = app(&self.state, self.routes.clone());
		self
	}

	/// Whether search engines may index, off like `serve --noindex`.
	pub fn indexing(mut self, on: bool) -> Self {
		self.state.indexing = on;
		self.app = app(&self.state, self.routes.clone());
		self
	}

	/// A proxy that may name the client, like `serve --trusted-proxy`.
	/// Requests come from `127.0.0.1`.
	pub fn trusted_proxy(mut self, proxy: &str) -> Self {
		let mut proxies = (*self.state.trusted_proxies).clone();
		proxies.push(proxy.parse().unwrap());
		self.state.trusted_proxies = Arc::new(proxies);
		self.app = app(&self.state, self.routes.clone());
		self
	}

	/// Someone without a session.
	pub fn anonymous(&self) -> Client {
		Client {
			app: self.app.clone(),
			token: None,
			site: false,
		}
	}

	/// Someone with a session token, e.g. of a login.
	pub fn with_token(&self, token: &str) -> Client {
		Client {
			app: self.app.clone(),
			token: Some(token.into()),
			site: false,
		}
	}

	/// Someone logged in with an email and password, like the studio.
	pub async fn login(&self, email: &str, password: &str) -> Client {
		let login = crate::auth::api::LoginReq {
			email: email.into(),
			password: password.into(),
		};
		let auth = self.anonymous().call(&login).await.unwrap();
		self.with_token(&auth.session.token)
	}

	/// A query of a frontend with its token, like crelte sends it; the
	/// whole answer with `data` and `errors`.
	pub async fn graphql(&self, query: &str, variables: Value) -> Value {
		let body =
			serde_json::json!({ "query": query, "variables": variables });
		let req = http::Request::post("/api/graphql")
			.header(header::CONTENT_TYPE, "application/json")
			.header(header::AUTHORIZATION, "Bearer test-token")
			.body(Body::from(body.to_string()))
			.unwrap();
		let resp = self.anonymous().send(req).await;
		let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
		serde_json::from_slice(&body).unwrap()
	}

	/// A logged in user with a role, `None` is an admin. The password is
	/// `password`.
	pub fn user(&self, role: Option<&str>) -> Client {
		let n = self.users.fetch_add(1, Ordering::Relaxed);
		let email = format!("{}{n}@test.ch", role.unwrap_or("admin"));
		let conn = self.state.db.conn();
		let user_id =
			crate::users::create(&conn, &email, "", password_hash(), role, "")
				.unwrap();
		let token = crate::auth::new_session(&conn, user_id).unwrap().token;
		Client {
			app: self.app.clone(),
			token: Some(token),
			site: false,
		}
	}

	/// The frontend of the website for a visitor, with the token of a
	/// site login (`None`: not logged in): requests go to the website's
	/// routes (no `/api`), the token as `Authorization: Bearer`.
	pub fn visitor(&self, token: Option<&str>) -> Client {
		Client {
			app: self.app.clone(),
			token: token.map(Into::into),
			site: true,
		}
	}

	/// A user with a role (`None` is an admin) logged in on the website,
	/// like [`TestServer::user`] in the studio; see [`TestServer::visitor`].
	pub fn site_user(&self, role: Option<&str>) -> Client {
		let n = self.users.fetch_add(1, Ordering::Relaxed);
		let email = format!("{}{n}@test.ch", role.unwrap_or("admin"));
		let conn = self.state.db.conn();
		let user_id =
			crate::users::create(&conn, &email, "", password_hash(), role, "")
				.unwrap();
		let token =
			crate::auth::new_site_session(&conn, user_id).unwrap().token;
		self.visitor(Some(&token))
	}

	/// The mails sent so far: a test server keeps them.
	pub fn mails(&self) -> Vec<crate::mail::Mail> {
		self.state.mail.sent()
	}

	pub fn admin(&self) -> Client {
		self.user(None)
	}

	/// The result of `f` and how many SQL statements it ran, so a test
	/// sees when the queries of a page grow with its content.
	pub async fn statements<F: Future>(&self, f: F) -> (F::Output, usize) {
		use rusqlite::trace::{TraceEvent, TraceEventCodes};
		// one count at a time, the counter is the same for all servers
		static COUNTING: tokio::sync::Mutex<()> =
			tokio::sync::Mutex::const_new(());
		static COUNT: AtomicUsize = AtomicUsize::new(0);
		fn count(_: TraceEvent) {
			COUNT.fetch_add(1, Ordering::Relaxed);
		}

		let _counting = COUNTING.lock().await;
		COUNT.store(0, Ordering::Relaxed);
		let stmt = TraceEventCodes::SQLITE_TRACE_STMT;
		self.state.db.conn().trace_v2(stmt, Some(count));
		let out = f.await;
		self.state.db.conn().trace_v2(stmt, None);
		(out, COUNT.load(Ordering::Relaxed))
	}
}

/// The app of `serve`, its requests come from `127.0.0.1`.
fn app(state: &AppState, routes: Router<AppState>) -> Router {
	let peer = SocketAddr::from(([127, 0, 0, 1], 40000));
	server::app(state.clone(), routes).layer(MockConnectInfo(peer))
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
	/// the website's frontend, see [`TestServer::visitor`]
	site: bool,
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
		let prefix = if self.site { "" } else { "/api" };
		let mut uri = format!("{prefix}{}", path_of(req));
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

	/// A multipart upload of files (name, data) to a path under `/api`,
	/// of the website for a visitor.
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
			.uri(match self.site {
				true => path.to_string(),
				false => format!("/api{path}"),
			})
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

	/// A GET of any path, like a file or a page.
	pub async fn get(&self, path: &str) -> Response {
		let req = http::Request::get(path).body(Body::empty()).unwrap();
		self.send(req).await
	}

	/// The body of a response.
	pub async fn bytes(resp: Response) -> Vec<u8> {
		to_bytes(resp.into_body(), usize::MAX)
			.await
			.unwrap()
			.to_vec()
	}

	/// Any request, with the session of the client.
	pub async fn send(&self, mut req: http::Request<Body>) -> Response {
		match &self.token {
			Some(token) if self.site => {
				let bearer = format!("Bearer {token}");
				req.headers_mut()
					.insert(header::AUTHORIZATION, bearer.parse().unwrap());
			}
			Some(token) => {
				req.headers_mut()
					.insert("session-token", token.parse().unwrap());
			}
			None => {}
		}
		self.app.clone().oneshot(req).await.unwrap()
	}
}

/// A published entry of the first site, for `CreateReq` or `save`.
pub fn input(
	section: &str,
	ty: &str,
	title: &str,
	fields: Value,
) -> EntryInput {
	EntryInput {
		published: true,
		..EntryInput::new(section, ty, title, fields)
	}
}
