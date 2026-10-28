//! zipp is a headless cms. The content model is code: a site has a small
//! crate that builds its [`Schema`] and calls [`run`]. The content is
//! edited in the studio and read by any frontend over the api.

pub mod ai;
pub mod assets;
pub mod auth;
mod cli;
pub mod content;
pub mod db;
pub mod drafts;
pub mod entries;
pub mod error;
pub mod graphql;
pub mod migrate;
pub mod schema;
pub mod seo;
mod server;
mod studio;
pub mod typescript;
pub mod users;

pub use content::richtext;
pub use entries::query;
pub use migrate::apply;
pub use schema::field_type;

use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::Router;
use axum::extract::FromRef;
use axum::http::{Method, header};
use clap::{Parser, Subcommand};
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::db::Db;
use crate::migrate::{Migration, Migrations, Rename};
use cli::*;
use server::*;

pub use crate::field_type::{FieldType, GraphqlType, Scalar, StudioModule};
pub use crate::schema::{
	Access, Field, LinkKind, Role, Schema, Section, Seo, SeoDefaults, Transform,
};
pub use axum;

#[doc(hidden)]
pub use rust_embed;

/// The url of each site, by handle, like Craft's base urls.
pub type SiteUrls = std::collections::HashMap<String, String>;

/// The url of a page of a site, `None` if the site has no url.
pub fn page_url(urls: &SiteUrls, site: &str, uri: &str) -> Option<String> {
	let base = urls.get(site)?.trim_end_matches('/');
	Some(format!("{base}/{uri}"))
}

/// The `migrations/` directory of the calling crate. Debug builds read it
/// from disk, release builds contain it.
#[macro_export]
macro_rules! migrations {
	() => {{
		#[derive($crate::rust_embed::Embed)]
		#[folder = "migrations/"]
		#[crate_path = "zipp::rust_embed"]
		#[allow_missing = true]
		struct Files;
		$crate::migrate::Migrations::new::<Files>(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/migrations"
		))
	}};
}

#[derive(Clone, FromRef)]
pub struct AppState {
	pub db: Db,
	pub schema: Arc<Schema>,
	pub gql: async_graphql::dynamic::Schema,
	/// where uploads go
	pub files: PathBuf,
	/// the url of each site, for previews
	pub site_urls: Arc<graphql::SiteUrls>,
	/// origins that may frame the studio
	pub frame_ancestors: Arc<Vec<String>>,
	/// what frontends send to GraphQL, see `serve --graphql-token`
	pub graphql_token: Option<Arc<String>>,
	/// development mode, see `serve --dev`
	pub dev: bool,
	/// whether search engines may index, see `serve --noindex`
	#[from_ref(skip)]
	pub indexing: bool,
	/// the AI chat, if there is an api key
	pub ai: Option<ai::AiConfig>,
	hooks: Arc<Vec<Hook>>,
}

/// What happened to an entry, for `Cms::on_change`.
#[derive(Debug, Clone, PartialEq)]
pub struct Change {
	pub entry_id: i64,
	pub section: String,
	pub kind: ChangeKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChangeKind {
	/// created or saved directly, without a draft
	Saved,
	/// a draft was published
	Published,
	Moved,
	Deleted,
}

type Hook = Arc<dyn Fn(&Change) + Send + Sync>;

impl AppState {
	/// Tells the hooks of the site, after the change is written.
	pub fn changed(&self, entry_id: i64, section: &str, kind: ChangeKind) {
		let change = Change {
			entry_id,
			section: section.into(),
			kind,
		};
		for hook in self.hooks.iter() {
			hook(&change);
		}
	}
}

/// The cms of a site, for what `run` can't do:
///
/// ```no_run
/// # use zipp::{axum::{Router, routing::get}, Cms, Schema};
/// # fn schema() -> Schema { Schema::new().site("de", "Deutsch", "de") }
/// Cms::new(schema(), zipp::migrations!())
///     // routes of the site, they get zipp's AppState
///     .routes(Router::new().route("/hello", get(|| async { "hi" })))
///     // after every save, publish, move and delete, e.g. to clear a cache
///     .on_change(|c| println!("{} changed", c.entry_id))
///     .run();
/// ```
pub struct Cms {
	schema: Schema,
	migrations: Migrations,
	routes: Router<AppState>,
	hooks: Vec<Hook>,
}

impl Cms {
	pub fn new(schema: Schema, migrations: Migrations) -> Self {
		Self {
			schema,
			migrations,
			routes: Router::new(),
			hooks: vec![],
		}
	}

	/// Routes of the site next to zipp's, with `State<AppState>`.
	pub fn routes(mut self, routes: Router<AppState>) -> Self {
		self.routes = self.routes.merge(routes);
		self
	}

	/// Called after an entry changed. It runs in the request, so spawn
	/// anything slow.
	pub fn on_change(
		mut self,
		hook: impl Fn(&Change) + Send + Sync + 'static,
	) -> Self {
		self.hooks.push(Arc::new(hook));
		self
	}

	/// The main of the cms crate: the commands and the server.
	pub fn run(self) {
		tracing_subscriber::fmt()
			.with_env_filter(
				tracing_subscriber::EnvFilter::try_from_default_env()
					.unwrap_or_else(|_| "zipp=info,tower_http=info".into()),
			)
			.init();

		let Self {
			schema,
			migrations,
			routes,
			hooks,
		} = self;
		if let Err(e) = schema.check() {
			eprintln!("The schema has a mistake: {e}");
			std::process::exit(1);
		}

		let opts = Opts::parse();
		let res = match opts.cmd {
			Cmd::Serve(args) => {
				tokio::runtime::Runtime::new().unwrap().block_on(serve(
					schema,
					&opts.db,
					&migrations,
					args,
					(routes, hooks),
				))
			}
			Cmd::Makemigration { rename, name } => {
				makemigration(&schema, &migrations, &rename, name.as_deref())
			}
			Cmd::Check { queries } => check(schema, &migrations, &queries),
			Cmd::Types => {
				print!("{}", typescript::generate(&schema));
				Ok(())
			}
			Cmd::User { email, role } => {
				add_user(&schema, &opts.db, &email, role.as_deref())
			}
		};

		if let Err(e) = res {
			eprintln!("{e}");
			std::process::exit(1);
		}
	}
}

/// The main of a site's cms crate: `zipp::run(schema(), zipp::migrations!())`,
/// `Cms` for routes and hooks of the site.
pub fn run(schema: Schema, migrations: Migrations) {
	Cms::new(schema, migrations).run();
}
