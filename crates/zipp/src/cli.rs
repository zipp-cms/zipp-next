use super::*;

#[derive(Debug, Parser)]
pub(super) struct Opts {
	#[arg(long, default_value = "zipp.db")]
	pub(super) db: PathBuf,
	#[command(subcommand)]
	pub(super) cmd: Cmd,
}

#[derive(Debug, clap::Args)]
pub(super) struct ServeArgs {
	#[arg(long, default_value = "127.0.0.1:8090")]
	pub(super) listen: String,
	/// Where uploaded files go
	#[arg(long, default_value = "files")]
	pub(super) files: PathBuf,
	/// Put before asset urls in GraphQL, e.g. `https://cms.example.com`
	/// when the frontend runs on another domain
	#[arg(long, default_value = "")]
	pub(super) public_url: String,
	/// The url of a site for crelte and `url` in GraphQL:
	/// `--site-url de=https://example.ch`, repeat for each site
	#[arg(long)]
	pub(super) site_url: Vec<String>,
	/// A file with an OpenRouter api key, turns on the AI chat
	#[arg(long)]
	pub(super) openrouter_key_file: Option<PathBuf>,
	/// A site that may show the studio in a frame, like a dashboard:
	/// `--frame-ancestor https://dash.example.com`, repeat for more
	#[arg(long)]
	pub(super) frame_ancestor: Vec<String>,
	/// The token frontends send to GraphQL as `Authorization: Bearer`,
	/// like Craft's schema tokens; without it GraphQL is closed
	#[arg(long, env = "ZIPP_GRAPHQL_TOKEN", hide_env_values = true)]
	pub(super) graphql_token: Option<String>,
	/// Development: GraphiQL on /api/graphql, and GraphQL is open when
	/// there is no token
	#[arg(long, env = "ZIPP_DEV")]
	pub(super) dev: bool,
}

#[derive(Debug, Subcommand)]
pub(super) enum Cmd {
	/// Apply new migrations and serve the api and the studio.
	Serve(ServeArgs),
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
	/// Add a studio user or set a new password and role, reads the
	/// password from stdin. Without a role the user is an admin.
	User {
		email: String,
		#[arg(long)]
		role: Option<String>,
	},
}

pub(super) fn makemigration(
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

pub(super) fn add_user(
	schema: &Schema,
	db_path: &Path,
	email: &str,
	role: Option<&str>,
) -> Result<(), String> {
	if let Some(r) = role
		&& schema.get_role(r).is_none()
	{
		return Err(format!("unknown role {r}"));
	}
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
	auth::add_user(&db.conn(), email, password, role)
		.map_err(|e| e.to_string())?;
	println!("Saved {email} as {}", role.unwrap_or("admin"));
	Ok(())
}
