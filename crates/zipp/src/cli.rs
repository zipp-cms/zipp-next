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
	/// What CI runs before a deploy: the schema, whether the migrations
	/// are up to date, and the GraphQL queries of frontends against the
	/// schema (`--queries ../frontend/src/queries`, `.graphql` files).
	Check {
		#[arg(long)]
		queries: Vec<PathBuf>,
	},
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

/// See [`Cmd::Check`]; the schema itself is checked before every command.
pub(super) fn check(
	schema: Schema,
	migrations: &Migrations,
	queries: &[PathBuf],
) -> Result<(), String> {
	let mut mistakes = 0;

	let changes = migrate::unmigrated(&migrations.load()?, &schema)?;
	if changes.is_empty() {
		println!("ok   migrations are up to date");
	} else {
		mistakes += 1;
		println!("FAIL the schema changed since the last migration:");
		for op in &changes {
			println!("       {}", op.describe());
		}
	}

	let db = Db::memory().map_err(|e| e.to_string())?;
	let gql = graphql::build(
		Arc::new(schema),
		db,
		String::new(),
		Default::default(),
	)?;
	let runtime = tokio::runtime::Runtime::new().unwrap();
	let mut files = vec![];
	for dir in queries {
		graphql_files(dir, &mut files)
			.map_err(|e| format!("{}: {e}", dir.display()))?;
	}
	for file in files {
		let query = fs::read_to_string(&file).map_err(|e| e.to_string())?;
		let errors = runtime.block_on(graphql::check_query(&gql, &query));
		if errors.is_empty() {
			println!("ok   {}", file.display());
			continue;
		}
		mistakes += 1;
		println!("FAIL {}", file.display());
		for e in errors {
			println!("       {e}");
		}
	}

	match mistakes {
		0 => Ok(()),
		n => Err(format!("{n} mistakes")),
	}
}

fn graphql_files(path: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
	if path.is_file() {
		out.push(path.to_path_buf());
		return Ok(());
	}
	let mut entries: Vec<_> = fs::read_dir(path)?.collect::<io::Result<_>>()?;
	entries.sort_by_key(|e| e.path());
	for e in entries {
		let p = e.path();
		if p.is_dir() {
			graphql_files(&p, out)?;
		} else if p.extension().is_some_and(|x| x == "graphql") {
			out.push(p);
		}
	}
	Ok(())
}
