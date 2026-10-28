//! zipp is a headless cms. The content model is code: a site has a small
//! crate that builds its [`Schema`] and calls [`run`]. The content is
//! edited in the studio and read by any frontend over the api.

pub mod apply;
pub mod assets;
pub mod auth;
pub mod content;
pub mod db;
pub mod entries;
pub mod error;
pub mod graphql;
pub mod migrate;
pub mod query;
pub mod schema;
mod studio;
pub mod typescript;

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

pub use crate::schema::{Field, Schema, Section, Transform};

#[doc(hidden)]
pub use rust_embed;

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
}

#[derive(Debug, Parser)]
struct Opts {
	#[arg(long, default_value = "zipp.db")]
	db: PathBuf,
	#[command(subcommand)]
	cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
	/// Apply new migrations and serve the api and the studio.
	Serve {
		#[arg(long, default_value = "127.0.0.1:8090")]
		listen: String,
		/// Where uploaded files go
		#[arg(long, default_value = "files")]
		files: PathBuf,
		/// Put before asset urls in GraphQL, e.g. `https://cms.example.com`
		/// when the frontend runs on another domain
		#[arg(long, default_value = "")]
		public_url: String,
		/// The url of a site for crelte and `url` in GraphQL:
		/// `--site-url de=https://example.ch`, repeat for each site
		#[arg(long)]
		site_url: Vec<String>,
	},
	/// Write a migration from the schema in code to `migrations/`.
	Makemigration {
		/// A rename instead of remove and add: `old=new` for a component,
		/// `component.old=new` for a field
		#[arg(long)]
		rename: Vec<String>,
		#[arg(long)]
		name: Option<String>,
	},
	/// Print the TypeScript types of the content.
	Types,
	/// Add a studio user or set a new password, reads it from stdin.
	User { email: String },
}

/// The main of a site's cms crate: `zipp::run(schema(), zipp::migrations!())`.
pub fn run(schema: Schema, migrations: Migrations) {
	tracing_subscriber::fmt()
		.with_env_filter(
			tracing_subscriber::EnvFilter::try_from_default_env()
				.unwrap_or_else(|_| "zipp=info,tower_http=info".into()),
		)
		.init();

	if let Err(e) = schema.check() {
		eprintln!("The schema has a mistake: {e}");
		std::process::exit(1);
	}

	let opts = Opts::parse();
	let res = match opts.cmd {
		Cmd::Serve {
			listen,
			files,
			public_url,
			site_url,
		} => tokio::runtime::Runtime::new().unwrap().block_on(serve(
			schema,
			&opts.db,
			&migrations,
			&listen,
			files,
			(public_url, site_url),
		)),
		Cmd::Makemigration { rename, name } => {
			makemigration(&schema, &migrations, &rename, name.as_deref())
		}
		Cmd::Types => {
			print!("{}", typescript::generate(&schema));
			Ok(())
		}
		Cmd::User { email } => add_user(&opts.db, &email),
	};

	if let Err(e) = res {
		eprintln!("{e}");
		std::process::exit(1);
	}
}

async fn serve(
	schema: Schema,
	db_path: &Path,
	migrations: &Migrations,
	listen: &str,
	files: PathBuf,
	(public_url, site_urls): (String, Vec<String>),
) -> Result<(), String> {
	let site_urls = site_urls
		.iter()
		.map(|s| match s.split_once('=') {
			Some((site, url)) if schema.get_site(site).is_some() => {
				Ok((site.to_string(), url.to_string()))
			}
			_ => Err(format!("--site-url {s:?} needs <site>=<url>")),
		})
		.collect::<Result<graphql::SiteUrls, _>>()?;
	let files = assets::files_dir(files).map_err(|e| e.to_string())?;
	let db = Db::open(db_path).map_err(|e| e.to_string())?;

	let migrations = migrations.load()?;
	let applied = apply::apply_migrations(&db.conn(), &migrations)
		.map_err(|e| e.to_string())?;
	for name in applied {
		info!("applied migration {name}");
	}
	let last = migrations
		.last()
		.map(|m| m.schema.clone())
		.unwrap_or_default();
	if !migrate::diff(&last, &schema, &[])?.is_empty() {
		return Err("The schema changed since the last migration, run \
			`makemigration` first"
			.into());
	}

	let schema = Arc::new(schema);
	let gql = graphql::build(
		schema.clone(),
		db.clone(),
		public_url,
		site_urls.clone(),
	)?;
	let state = AppState {
		db,
		schema,
		gql,
		files,
		site_urls: Arc::new(site_urls),
	};
	let api = Router::new()
		.nest("/auth", auth::routes())
		.merge(entries::routes())
		.merge(assets::api_routes())
		.merge(graphql::routes());
	let app = Router::new()
		.nest("/api", api)
		.merge(assets::file_routes())
		.merge(studio::routes());

	// frontends on other domains read the content, graphql is a POST
	let app = app
		.layer(
			CorsLayer::new()
				.allow_origin(cors::Any)
				.allow_methods([Method::GET, Method::POST])
				.allow_headers([header::CONTENT_TYPE]),
		)
		.layer(TraceLayer::new_for_http())
		.with_state(state);

	let listener = tokio::net::TcpListener::bind(listen)
		.await
		.map_err(|e| e.to_string())?;
	info!("listening on {listen}");
	axum::serve(listener, app).await.map_err(|e| e.to_string())
}

fn makemigration(
	schema: &Schema,
	migrations: &Migrations,
	renames: &[String],
	name: Option<&str>,
) -> Result<(), String> {
	let renames = renames
		.iter()
		.map(|r| Rename::parse(r))
		.collect::<Result<Vec<_>, _>>()?;
	let dir = &migrations.dir;
	let migrations = migrations.load()?;
	let last = migrations
		.last()
		.map(|m| m.schema.clone())
		.unwrap_or_default();

	let ops = migrate::diff(&last, schema, &renames)?;
	if ops.is_empty() {
		println!("No changes");
		return Ok(());
	}

	let name = migrate::next_name(&migrations, name);
	println!("Migration {name}:");
	for op in &ops {
		println!("{}", op.describe());
	}
	if ops.iter().any(|o| o.describe().starts_with('!')) {
		println!(
			"Content marked with ! is deleted. If something was renamed, \
			pass --rename instead."
		);
	}

	let m = Migration {
		name: name.clone(),
		ops,
		schema: schema.clone(),
	};
	fs::create_dir_all(dir).map_err(|e| e.to_string())?;
	let json = serde_json::to_string_pretty(&m).unwrap() + "\n";
	fs::write(dir.join(format!("{name}.json")), json).map_err(|e| e.to_string())
}

fn add_user(db_path: &Path, email: &str) -> Result<(), String> {
	print!("Password: ");
	io::stdout().flush().ok();
	let mut password = String::new();
	io::stdin()
		.lock()
		.read_line(&mut password)
		.map_err(|e| e.to_string())?;
	let password = password.trim_end_matches(['\n', '\r']);
	if password.len() < 8 {
		return Err("The password needs at least 8 characters".into());
	}

	let db = Db::open(db_path).map_err(|e| e.to_string())?;
	auth::add_user(&db.conn(), email, password).map_err(|e| e.to_string())?;
	println!("Saved {email}");
	Ok(())
}
