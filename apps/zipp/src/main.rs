mod components;
mod users;

use std::fs;

use clap::Parser;
use database::{Config as DbConfig, DatabasePool};
use fire_http::get;
use serde::Deserialize;
use tracing::info;
use users::Users;

#[derive(Debug, Parser)]
struct Opts {
	#[clap(subcommand)]
	subcmd: Option<SubCommand>,

	#[clap(long)]
	use_memory_db: bool,

	#[clap(long)]
	config: Option<String>,

	#[clap(long)]
	tracing: Option<String>,
}

const DEFAULT_CONFIG_PATH: &str = "./zipp.toml";

#[derive(Debug, Parser)]
enum SubCommand {}

#[derive(Debug, Default, Deserialize)]
pub struct Config {
	pub db: Option<DbConfig>,
}

#[get("/")]
async fn hello_world() -> String {
	info!("Hello, world!");
	"Hello, world!".into()
}

#[tokio::main]
async fn main() {
	// read args
	let opts = Opts::parse();

	// read config
	let cfg_path = opts
		.config
		.clone()
		.unwrap_or_else(|| DEFAULT_CONFIG_PATH.into());
	let cfg: Config = match fs::read_to_string(&cfg_path) {
		Ok(cfg) => toml::from_str(&cfg).unwrap(),
		Err(_) if cfg!(debug_assertions) => Config::default(),
		Err(_) => panic!("Config file is required"),
	};

	// init logging using env filter
	let env_tracing = opts.tracing.unwrap_or_else(|| "zipp=info,warn".into());
	tracing_subscriber::fmt()
		.with_env_filter(env_tracing)
		.init();

	// create a database connection
	let db_pool = match (cfg!(debug_assertions), opts.use_memory_db, cfg.db) {
		(_, true, _) | (true, _, None) => {
			info!("Using memory database");

			DatabasePool::new_memory()
		}
		(_, _, Some(db)) => DatabasePool::new_postgres(db)
			.await
			.expect("database failed"),
		(false, false, None) => panic!("Database configuration is required"),
	};
	let mut db = db_pool.get().await.unwrap();

	// create instances
	let users = Users::new(&mut db).await.unwrap();

	// since we don't need the database anymore, we can drop it
	// this makes sure we don't keep a connection running
	drop(db);

	// create http server
	let mut fire = fire_http::build("127.0.0.1:3000").await.unwrap();

	// add global data
	fire.add_data(db_pool);
	fire.add_data(users);

	// register routes
	users::api::register(&mut fire);
	fire.add_route(hello_world);

	// todo run plugins before building

	// build server and prepare to run it
	fire.hide_startup_message();
	let fire = fire.build().await.unwrap();

	// todo prepare cron jobs (async tasks)

	// run server
	info!("running server on 127.0.0.1:3000");
	fire.ignite().await.unwrap();
}
