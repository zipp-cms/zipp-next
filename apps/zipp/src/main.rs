use clap::Parser;
use fire_http::get;
use tracing::info;

#[derive(Debug, Parser)]
struct Opts {
	#[clap(subcommand)]
	subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {}

#[get("/")]
async fn hello_world() -> String {
	info!("Hello, world!");
	"Hello, world!".into()
}

#[tokio::main]
async fn main() {
	// init logging using env filter
	tracing_subscriber::fmt()
		.with_env_filter("zipp=info,warn")
		.init();

	let mut fire = fire_http::build("127.0.0.1:3000").await.unwrap();

	fire.add_route(hello_world);

	fire.ignite().await.unwrap();
}
