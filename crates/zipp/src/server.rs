use super::*;

pub(super) async fn serve(
	schema: Schema,
	db_path: &Path,
	migrations: &Migrations,
	args: ServeArgs,
	(routes, hooks): (Router<AppState>, Vec<Hook>),
) -> Result<(), String> {
	let ServeArgs {
		listen,
		files,
		public_url,
		site_url: site_urls,
		openrouter_key_file: key_file,
		frame_ancestor,
		graphql_token,
		dev,
		noindex,
	} = args;
	if graphql_token.is_none() && !dev {
		tracing::warn!(
			"GraphQL is closed: set ZIPP_GRAPHQL_TOKEN for the frontends"
		);
	}
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
	if !migrate::unmigrated(&migrations, &schema)?.is_empty() {
		return Err("The schema changed since the last migration, run \
			`makemigration` first"
			.into());
	}

	let schema = Arc::new(schema);
	let ai = match key_file {
		Some(path) => {
			let key = fs::read_to_string(&path)
				.map_err(|e| format!("{}: {e}", path.display()))?;
			Some(ai::AiConfig::openrouter(key.trim().to_string()))
		}
		None => None,
	};
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
		frame_ancestors: Arc::new(frame_ancestor),
		graphql_token: graphql_token.map(Arc::new),
		dev,
		indexing: !dev && !noindex,
		ai,
		hooks: Arc::new(hooks),
	};
	let api = Router::new()
		.nest("/auth", auth::routes())
		.merge(studio::api_routes())
		.merge(entries::routes())
		.merge(users::routes())
		.merge(seo::routes::routes())
		.merge(drafts::routes())
		.merge(ai::routes())
		.merge(assets::api_routes())
		.merge(graphql::routes());
	let app = Router::new()
		.nest("/api", api)
		.merge(assets::file_routes())
		.merge(seo::sitemap::routes())
		.merge(routes)
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

	let listener = tokio::net::TcpListener::bind(&listen)
		.await
		.map_err(|e| e.to_string())?;
	info!("listening on {listen}");
	axum::serve(listener, app).await.map_err(|e| e.to_string())
}
