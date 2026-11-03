use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::Router;
use axum::http::{Method, header};
use tower_http::cors::{self, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::cli::{CmdError, CmdResult, ServeArgs};
use crate::db::Db;
use crate::migrate::{Migrations, apply};
use crate::{
	AppState, Hook, Schema, SiteUrls, ai, assets, auth, change_requests,
	drafts, entries, graphql, mail, origin, search, studio, trash, users,
};

pub(super) async fn serve(
	schema: Schema,
	db_path: &Path,
	migrations: &Migrations,
	args: ServeArgs,
	(routes, hooks): (Router<AppState>, Vec<Hook>),
) -> CmdResult {
	let ServeArgs {
		listen,
		files,
		public_url,
		site_url: site_urls,
		openrouter_key_file: key_file,
		frame_ancestor,
		trusted_proxy,
		backup_dir,
		backup_keep,
		graphql_token,
		dev,
		noindex,
		mail,
	} = args;
	let mail = mail.mailer()?;
	if graphql_token.is_none() && !dev {
		info!("GraphQL answers the tokens admins make in the studio");
	}
	let site_urls = site_urls
		.iter()
		.map(|s| match s.split_once('=') {
			Some((site, url)) if schema.get_site(site).is_some() => {
				Ok((site.to_string(), url.to_string()))
			}
			_ => Err(format!("--site-url {s:?} needs <site>=<url>")),
		})
		.collect::<Result<SiteUrls, _>>()?;
	// they go into a header of the studio
	if let Some(a) = frame_ancestor
		.iter()
		.find(|a| origin(a) != Some(a.as_str()))
	{
		return Err(format!(
			"--frame-ancestor {a:?}: an origin like https://dash.example.com"
		)
		.into());
	}
	let files = assets::files_dir(files)?;
	let db = Db::open(db_path)?;

	let migrations = migrations.load()?;
	let applied = apply::up_to_date(&db.conn(), &migrations, &schema)?;
	for name in applied {
		info!("applied migration {name}");
	}
	assets::index_usage(&db.conn(), &schema)?;

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
		public_url.clone(),
		site_urls.clone(),
	)?;
	let state = AppState {
		public_url: Arc::new(public_url.trim_end_matches('/').into()),
		site_urls: Arc::new(site_urls),
		frame_ancestors: Arc::new(frame_ancestor),
		trusted_proxies: Arc::new(trusted_proxy),
		graphql_token: graphql_token.map(Arc::new),
		dev,
		indexing: !dev && !noindex,
		ai,
		mail,
		hooks: Arc::new(hooks),
		..AppState::new(db, schema, gql, files)
	};
	tokio::spawn(watch_dates(state.clone()));
	tokio::spawn(empty_trash(state.clone()));
	if let Some(dir) = backup_dir {
		tokio::spawn(backups(
			db_path.into(),
			state.files.clone(),
			dir,
			backup_keep,
		));
	}
	let app = app(state, routes);

	let listener = tokio::net::TcpListener::bind(&listen).await?;
	info!("listening on {listen}");
	// the peer's address, see `client`
	let app = app.into_make_service_with_connect_info::<SocketAddr>();
	axum::serve(listener, app).await?;
	Ok(())
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
		.merge(trash::routes())
		.merge(search::routes())
		.merge(mail::routes())
		.merge(change_requests::routes())
		.merge(graphql::routes());
	let mut app = Router::new()
		.merge(assets::file_routes())
		.merge(routes)
		.merge(studio::routes());
	// the plugins': under /api for the studio, others for the website
	for p in &state.schema.plugins.0 {
		api = api.merge(p.api_routes(&state.schema));
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

/// Deletes what is in the trash for too long, at the start and hourly.
async fn empty_trash(state: AppState) {
	let mut tick = tokio::time::interval(std::time::Duration::from_secs(3600));
	loop {
		tick.tick().await;
		if let Err(e) = crate::trash::purge_old(&state.db.conn(), &state.files)
		{
			tracing::warn!("emptying the trash: {e}");
		}
	}
}

/// A backup a day: checked hourly, so it runs at the start when the
/// last one is older and then about the same time each day. A failure
/// is logged and tried again the next hour.
async fn backups(db: PathBuf, files: PathBuf, dir: PathBuf, keep: usize) {
	let mut tick = tokio::time::interval(std::time::Duration::from_secs(3600));
	loop {
		tick.tick().await;
		let (db, files, dir) = (db.clone(), files.clone(), dir.clone());
		// a bit less than a day, the hourly tick doesn't move it later
		let res = tokio::task::spawn_blocking(move || {
			let now = crate::db::now();
			let last = crate::backup::list(&dir)
				.ok()
				.and_then(|l| l.last().map(|(t, _)| *t));
			if last.is_some_and(|t| now - t < 23 * crate::db::HOUR_MS) {
				return Ok(None);
			}
			let w = crate::backup::backup(&db, &files, &dir, now)?;
			crate::backup::prune(&dir, keep)?;
			Ok::<_, CmdError>(Some(w))
		})
		.await;
		match res {
			Ok(Ok(Some(w))) => info!(
				"backup {}: {} files copied, {} linked",
				w.dir.display(),
				w.copied,
				w.linked
			),
			Ok(Ok(None)) => {}
			Ok(Err(e)) => tracing::warn!("backup: {e}"),
			Err(e) => tracing::warn!("backup: {e}"),
		}
	}
}
