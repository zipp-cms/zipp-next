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
	assets::index_usage(&db.conn(), &schema).map_err(|e| e.to_string())?;
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
		site_urls: Arc::new(site_urls),
		frame_ancestors: Arc::new(frame_ancestor),
		graphql_token: graphql_token.map(Arc::new),
		dev,
		indexing: !dev && !noindex,
		ai,
		hooks: Arc::new(hooks),
		..AppState::new(db, schema, gql, files)
	};
	tokio::spawn(watch_dates(state.clone()));
	let app = app(state, routes);

	let listener = tokio::net::TcpListener::bind(&listen)
		.await
		.map_err(|e| e.to_string())?;
	info!("listening on {listen}");
	axum::serve(listener, app).await.map_err(|e| e.to_string())
}

/// zipp's routes, the plugins' and the site's, under the state.
pub(crate) fn app(state: AppState, routes: Router<AppState>) -> Router {
	let mut api = Router::new()
		.merge(auth::routes())
		.merge(studio::api_routes())
		.merge(entries::routes())
		.merge(users::routes())
		.merge(drafts::routes())
		.merge(ai::routes())
		.merge(assets::api_routes())
		.merge(graphql::routes());
	let mut app = Router::new()
		.merge(assets::file_routes())
		.merge(routes)
		.merge(studio::routes());
	// the plugins': under /api for the studio, others for the website
	for p in &state.schema.plugins.0 {
		api = api.merge(p.api_routes());
		app = app.merge(p.routes(&state.schema));
	}

	// frontends on other domains read the content, graphql is a POST
	app.nest("/api", api)
		.layer(
			CorsLayer::new()
				.allow_origin(cors::Any)
				.allow_methods([Method::GET, Method::POST])
				.allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
		)
		.layer(TraceLayer::new_for_http())
		.with_state(state)
}

/// Every minute: entries that went live or expired by their dates tell
/// the hooks (`ChangeKind::Dated`), a cache of the website clears like
/// after a save.
async fn watch_dates(state: AppState) {
	let mut last = crate::db::now();
	let mut tick = tokio::time::interval(std::time::Duration::from_secs(60));
	loop {
		tick.tick().await;
		let now = crate::db::now();
		let dated = crate::entries::dated_between(&state.db.conn(), last, now);
		match dated {
			Ok(entries) => {
				for (id, section) in entries {
					state.changed(id, &section, crate::ChangeKind::Dated);
				}
				last = now;
			}
			Err(e) => tracing::warn!("dates of entries: {e}"),
		}
	}
}
