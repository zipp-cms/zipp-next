//! A zipp plugin for development: the AI chat of the studio hands
//! everything to a coding agent (Claude Code), content as drafts through
//! the api, the content model in the cms crate, the frontend.
//!
//! ```ignore
//! Schema::new()
//!     // ...
//!     .plugin(zipp_agent::Agent::new())
//! ```
//!
//! It only works with `serve --dev` and for admins, in production it does
//! nothing. Where it runs is the environment's: `ZIPP_AGENT_COMMAND`
//! (default `claude`), `ZIPP_AGENT_DIR` (the project, default `..`),
//! `ZIPP_AGENT_RESTART` (a shell command that rebuilds and restarts the
//! cms), `ZIPP_AGENT_API` (this cms' api, default
//! `http://127.0.0.1:8090/api`).
//!
//! Each message starts the agent in the project folder; its output goes to
//! a file, not a pipe, so it outlives a restart of the cms after a change
//! of its code (the unit needs `KillMode=process`). The studio reads the
//! file as events and reconnects after a restart. The agent continues its
//! conversation per user (`--resume`).

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::json;
use zipp::axum::Router;
use zipp::error::{Error, Result};
use zipp::{AppState, Plugin, PluginStudio};

pub mod api;
mod harness;
mod routes;

pub use harness::{ChatEvent, ClaudeCode, Harness};

/// The plugin, see the crate's docs.
pub struct Agent {
	harness: Arc<dyn Harness>,
}

impl Agent {
	/// With Claude Code.
	pub fn new() -> Self {
		Self::with(ClaudeCode::new())
	}

	/// With another coding agent.
	pub fn with(harness: impl Harness) -> Self {
		Self {
			harness: Arc::new(harness),
		}
	}
}

impl Default for Agent {
	fn default() -> Self {
		Self::new()
	}
}

impl Plugin for Agent {
	fn name(&self) -> &'static str {
		"agent"
	}

	fn api_routes(&self) -> Router<AppState> {
		routes::routes()
	}

	/// The studio's chat uses the agent, it has no module of its own.
	fn studio(&self) -> Option<PluginStudio> {
		Some(PluginStudio {
			js: "",
			data: json!({ "chat": true }),
		})
	}
}

/// Where the agent runs, from the environment.
#[derive(Clone)]
struct AgentConfig {
	harness: Arc<dyn Harness>,
	/// the project, the agent works there
	pub dir: PathBuf,
	/// where the runs and sessions are written
	pub runs: PathBuf,
	/// a shell command that rebuilds and restarts the cms
	pub restart: Option<String>,
	/// the api of this cms for the agent, `http://127.0.0.1:8090/api`
	pub api: String,
}

/// The agent, in development only; the routes are an admin's.
fn agent(state: &AppState) -> Result<AgentConfig> {
	if !state.dev {
		return Err(Error::NotFound);
	}
	let Some(plugin) = state.schema.plugin_of::<Agent>() else {
		return Err(Error::NotFound);
	};
	let env = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
	let dir = PathBuf::from(env("ZIPP_AGENT_DIR").unwrap_or("..".into()));
	check_dir(&dir).map_err(Error::Internal)?;
	Ok(AgentConfig {
		harness: plugin.harness.clone(),
		dir: fs::canonicalize(&dir)
			.map_err(|e| Error::Internal(e.to_string()))?,
		runs: state.files.join("../agent-runs"),
		restart: env("ZIPP_AGENT_RESTART"),
		api: env("ZIPP_AGENT_API")
			.unwrap_or("http://127.0.0.1:8090/api".into()),
	})
}

/// A run's id is a token: only hex, it names files.
fn run_file(agent: &AgentConfig, id: &str, ext: &str) -> Result<PathBuf> {
	if id.is_empty() || !id.chars().all(|c| c.is_ascii_hexdigit()) {
		return Err(Error::NotFound);
	}
	Ok(agent.runs.join(format!("{id}.{ext}")))
}

fn session_file(agent: &AgentConfig, user: i64) -> PathBuf {
	agent.runs.join(format!("session-{user}"))
}

/// What the agent is told besides the project's own docs.
fn system_prompt(agent: &AgentConfig, view: Option<&str>) -> String {
	let restart = agent
		.restart
		.as_deref()
		.unwrap_or("(none, tell the user to restart the cms)");
	let view = view.unwrap_or("the home page");
	format!(
		"You work in a website project in development through the chat of \
		the zipp studio (zipp is the cms, see AGENT.md and docs/zipp.md). The \
		person writing is an admin of the site, maybe not a developer: answer \
		in their language, short, say what you changed. Next to the chat they \
		see the website: {view}.\n\n\
		Content (texts, images, entries) goes through zipp's api as drafts, \
		never by editing the database: base url $ZIPP_API, header \
		`session-token: $ZIPP_TOKEN`. GET /schema (the content model), GET \
		/entries?section=<handle>&site=<site> (list), GET \
		/entries/<id>?site=<site>, GET /entries/<id>/draft, PUT \
		/entries/<id>/draft with {{site, base, title, slug, published, \
		fields}} (all fields of the site, base is the draft's updatedAt or \
		null), POST /entries with {{section, type, site, title, fields}} \
		creates one. Don't publish, the user publishes in the studio.\n\n\
		Code: the content model is cms/src/main.rs, the frontend svelte/ \
		(its dev server reloads by itself). After changing cms/: \
		`cargo run -- makemigration` when the schema changed, `cargo run -- \
		check --queries ../svelte/src/queries`, then restart the cms with \
		`{restart}`. When it works, commit with git (a short message on the \
		current branch). Never print secrets."
	)
}

/// After a run: its session continues the next one, `.done` ends the
/// events.
fn finish(agent: &AgentConfig, id: &str, user: i64) {
	let Ok(path) = run_file(agent, id, "jsonl") else {
		return;
	};
	let out = fs::read_to_string(&path).unwrap_or_default();
	if let Some(s) = agent.harness.session(&out) {
		let _ = fs::write(session_file(agent, user), s);
	}
	let _ = fs::write(path.with_extension("done"), "");
}

/// Whether the folder looks like a project for the agent.
fn check_dir(dir: &Path) -> std::result::Result<(), String> {
	if dir.join("cms").is_dir() || dir.join("AGENT.md").is_file() {
		Ok(())
	} else {
		Err(format!(
			"--agent-dir {}: no project there (no cms/ or AGENT.md)",
			dir.display()
		))
	}
}
