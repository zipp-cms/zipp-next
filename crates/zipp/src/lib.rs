//! zipp is a headless cms. The content model is code: a site has a small
//! crate that builds its [`Schema`] and calls [`run`]. The content is
//! edited in the studio and read by any frontend over the api.

mod ai;
pub mod api;
pub mod assets;
pub mod auth;
pub mod backup;
mod change_requests;
mod cli;
pub mod client;
pub mod content;
#[cfg(feature = "craft")]
mod craft;
pub mod db;
pub mod drafts;
pub mod entries;
pub mod error;
#[cfg(test)]
mod fixtures;
pub mod graphql;
pub mod mail;
pub mod migrate;
pub mod plugin;
pub mod schema;
pub mod search;
mod server;
mod studio;
#[cfg(any(test, feature = "testing"))]
pub mod testing;
pub mod trash;
mod typescript;
mod users;

use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use axum::extract::FromRef;
use clap::Parser;

use crate::db::Db;
use crate::migrate::{DataResult, DataTx, Migrations};
#[cfg(feature = "craft")]
use cli::import_craft;
use cli::{
	Cmd, Opts, backup, check, makemigration, restore, save_user, test_mail,
};
use server::serve;

pub use crate::plugin::{EntryField, GraphqlExt, Plugin, PluginStudio};
pub use crate::schema::field_type::{
	FieldRelation, FieldType, GraphqlType, Scalar, StudioModule, Target,
};
pub use crate::schema::{
	Access, Choice, Column, ColumnKind, Condition, Field, FieldKind, Format,
	Icon, LinkKind, Role, Schema, Section, Style, Transform,
};
pub use async_graphql;
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

/// The origin of a url, `https://example.ch` of `https://example.ch/de`;
/// `None` for a relative one or one that doesn't fit into a header.
pub fn origin(url: &str) -> Option<&str> {
	let (scheme, rest) = url.split_once("://")?;
	let host = rest.split('/').next()?;
	let plain = |c: char| c.is_ascii_alphanumeric() || ".-:*[]".contains(c);
	let fits = matches!(scheme, "http" | "https")
		&& !host.is_empty()
		&& host.chars().all(plain);
	fits.then(|| &url[..scheme.len() + 3 + host.len()])
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
	pub site_urls: Arc<SiteUrls>,
	/// origins that may frame the studio
	pub frame_ancestors: Arc<Vec<String>>,
	/// who may name the client in `X-Forwarded-For`, see [`client`]
	pub trusted_proxies: Arc<Vec<client::Proxy>>,
	/// what frontends send to GraphQL, see `serve --graphql-token`
	pub graphql_token: Option<Arc<String>>,
	/// development mode, see `serve --dev`
	pub dev: bool,
	/// whether search engines may index, see `serve --noindex`
	#[from_ref(skip)]
	pub indexing: bool,
	/// the AI chat, if there is an api key
	pub ai: Option<ai::AiConfig>,
	/// sends mails, see [`mail`]
	pub mail: mail::Mailer,
	/// where the cms is, `serve --public-url`: links in mails
	pub public_url: Arc<String>,
	/// the svgs of the schema's Lucide icons, see `studio::schema_icons`
	#[from_ref(skip)]
	pub icons: Arc<std::collections::BTreeMap<String, String>>,
	/// the wrong passwords of logins, see [`auth::Throttle`]
	#[from_ref(skip)]
	pub logins: Arc<auth::Throttle>,
	hooks: Arc<Vec<Hook>>,
}

/// What happened to an entry, for `Cms::on_change`.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Change {
	pub entry_id: i64,
	pub section: String,
	pub kind: ChangeKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum ChangeKind {
	/// created or saved directly, without a draft
	Saved,
	/// a draft was published
	Published,
	Moved,
	Deleted,
	/// it went live or expired by its post or expiry date, nothing was
	/// saved: see `serve`, a minute later at most
	Dated,
}

type Hook = Arc<dyn Fn(&Change) + Send + Sync>;

impl AppState {
	/// The state of a schema and its database: what follows from them is
	/// made here, the settings of `serve` are set after.
	pub(crate) fn new(
		db: Db,
		schema: Arc<Schema>,
		gql: async_graphql::dynamic::Schema,
		files: PathBuf,
	) -> Self {
		Self {
			icons: Arc::new(studio::schema_icons(&schema)),
			db,
			schema,
			gql,
			files,
			site_urls: Default::default(),
			frame_ancestors: Default::default(),
			trusted_proxies: Default::default(),
			graphql_token: None,
			dev: false,
			indexing: true,
			ai: None,
			mail: Default::default(),
			public_url: Default::default(),
			logins: Default::default(),
			hooks: Default::default(),
		}
	}

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
			schema: schema.finish(),
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

	/// The step of a data migration, for what the ops of a schema
	/// migration can't do. `name` is its file, which
	/// `makemigration --data --name …` writes; the step runs once, after
	/// the migrations before it and in their transaction.
	///
	/// ```no_run
	/// # use zipp::{Cms, Schema};
	/// # fn schema() -> Schema { Schema::new().site("de", "Deutsch", "de") }
	/// use zipp::migrate::{DataResult, DataTx};
	///
	/// fn split_names(tx: &DataTx) -> DataResult {
	///     tx.each("person", |c| {
	///         let name: String = c.get("name").unwrap_or_default();
	///         let (first, last) = name.split_once(' ').unwrap_or((&name, ""));
	///         c.set("firstName", first)?;
	///         c.set("lastName", last)?;
	///         Ok(())
	///     })
	/// }
	///
	/// Cms::new(schema(), zipp::migrations!())
	///     .data_migration("0003_split_names", split_names)
	///     .run();
	/// ```
	pub fn data_migration(
		mut self,
		name: &str,
		step: impl Fn(&DataTx) -> DataResult + Send + Sync + 'static,
	) -> Self {
		self.migrations = self.migrations.data(name, step);
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
			Cmd::Makemigration { rename, name, data } => makemigration(
				&schema,
				&migrations,
				&rename,
				name.as_deref(),
				data,
			),
			Cmd::Check { queries } => {
				check(schema, &opts.db, &migrations, &queries)
			}
			Cmd::Types => {
				print!("{}", typescript::generate(&schema));
				Ok(())
			}
			#[cfg(feature = "craft")]
			Cmd::ImportCraft {
				export,
				files,
				volume,
				site,
			} => import_craft(
				&schema,
				&opts.db,
				&migrations,
				&export,
				files,
				&volume,
				&site,
			),
			Cmd::User { email, role } => {
				save_user(&schema, &opts.db, &email, role.as_deref())
			}
			Cmd::Mail { to, mail } => test_mail(&to, &mail),
			Cmd::Backup { dir, files, keep } => {
				backup(&opts.db, &files, &dir, keep)
			}
			Cmd::Restore { backup, files } => {
				restore(&opts.db, &files, &backup)
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
