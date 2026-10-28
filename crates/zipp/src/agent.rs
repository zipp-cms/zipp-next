//! In development the AI chat of the studio can hand everything to a
//! coding agent (Claude Code): content as drafts through the api, the
//! content model in the cms crate, the frontend. `serve --dev --agent
//! claude --agent-dir ..` turns it on, only admins use it.
//!
//! Each message starts the agent in the project folder; its output goes to
//! a file, not a pipe, so it outlives a restart of the cms after a change
//! of its code (the unit needs `KillMode=process`). The studio reads the
//! file as events and reconnects after a restart. The agent continues its
//! conversation per user (`--resume`).

use std::convert::Infallible;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use axum::extract::{Path as UrlPath, Query, State};
use axum::response::IntoResponse;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::AppState;
use crate::auth::Authed;
use crate::db::{now, random_token};
use crate::error::{Error, Result};

/// The coding agent, see `serve --agent`.
#[derive(Debug, Clone)]
pub struct AgentConfig {
	/// the command, like `claude`
	pub command: String,
	/// the project, the agent works there
	pub dir: PathBuf,
	/// where the runs and sessions are written
	pub runs: PathBuf,
	/// a shell command that rebuilds and restarts the cms
	pub restart: Option<String>,
	/// the api of this cms for the agent, `http://127.0.0.1:8090/api`
	pub api: String,
}

fn agent(state: &AppState, auth: &Authed) -> Result<AgentConfig> {
	let Some(agent) = state.agent.clone() else {
		return Err(Error::NotFound);
	};
	if !state.dev || !auth.admin() {
		return Err(Error::Forbidden);
	}
	Ok(agent)
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

#[derive(Debug, Deserialize)]
struct StartReq {
	message: String,
	/// the url the user sees next to the chat
	view: Option<String>,
}

#[derive(Debug, Serialize)]
struct StartResp {
	id: String,
}

async fn start(
	State(state): State<AppState>,
	auth: Authed,
	Json(req): Json<StartReq>,
) -> Result<Json<StartResp>> {
	let agent = agent(&state, &auth)?;
	if req.message.trim().is_empty() {
		return Err(Error::Request("say what to do".into()));
	}
	let io = |e: std::io::Error| Error::Internal(e.to_string());
	fs::create_dir_all(&agent.runs).map_err(io)?;

	// the agent acts as the user, with a session of its own
	let token = random_token();
	state.db.conn().execute(
		"INSERT INTO sessions (token, user_id, created_at) VALUES (?1, ?2, ?3)",
		params![token, auth.user_id, now()],
	)?;

	let id = random_token();
	let out = fs::File::create(run_file(&agent, &id, "jsonl")?).map_err(io)?;
	let err = fs::File::create(run_file(&agent, &id, "err")?).map_err(io)?;
	let mut cmd = Command::new(&agent.command);
	cmd.args(["-p", &req.message])
		.args(["--output-format", "stream-json", "--verbose"])
		// the project is a sandbox VM in development
		.arg("--dangerously-skip-permissions")
		.args([
			"--append-system-prompt",
			&system_prompt(&agent, req.view.as_deref()),
		]);
	let session = fs::read_to_string(session_file(&agent, auth.user_id));
	if let Ok(session) = session {
		cmd.args(["--resume", session.trim()]);
	}
	cmd.current_dir(&agent.dir)
		.env("ZIPP_API", &agent.api)
		.env("ZIPP_TOKEN", &token)
		.stdin(Stdio::null())
		.stdout(out)
		.stderr(err);
	// its own process group, a restart of the cms doesn't stop it
	std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
	let mut child = cmd
		.spawn()
		.map_err(|e| Error::Request(format!("{}: {e}", agent.command)))?;

	let (db, user, run) = (state.db.clone(), auth.user_id, id.clone());
	tokio::task::spawn_blocking(move || {
		let _ = child.wait();
		finish(&agent, &run, user);
		let _ = db
			.conn()
			.execute("DELETE FROM sessions WHERE token = ?1", [token]);
	});
	Ok(Json(StartResp { id }))
}

/// After a run: its session continues the next one, `.done` ends the
/// events.
fn finish(agent: &AgentConfig, id: &str, user: i64) {
	let Ok(path) = run_file(agent, id, "jsonl") else {
		return;
	};
	let out = fs::read_to_string(&path).unwrap_or_default();
	let session = out
		.lines()
		.filter_map(|l| serde_json::from_str::<Value>(l).ok())
		.filter_map(|v| v["session_id"].as_str().map(String::from))
		.last();
	if let Some(s) = session {
		let _ = fs::write(session_file(agent, user), s);
	}
	let _ = fs::write(path.with_extension("done"), "");
}

#[derive(Debug, Deserialize)]
struct EventsParams {
	/// the lines already seen, after a reconnect
	#[serde(default)]
	from: usize,
}

/// The lines of a run as events (`data` is a JSON line of the agent, `id`
/// its number), then `done`.
async fn events(
	State(state): State<AppState>,
	auth: Authed,
	UrlPath(id): UrlPath<String>,
	Query(p): Query<EventsParams>,
) -> Result<impl IntoResponse> {
	let agent = agent(&state, &auth)?;
	let path = run_file(&agent, &id, "jsonl")?;
	if !path.exists() {
		return Err(Error::NotFound);
	}
	let (tx, rx) = tokio::sync::mpsc::channel::<
		std::result::Result<Event, Infallible>,
	>(64);
	tokio::spawn(async move {
		let mut sent = p.from;
		loop {
			let done = path.with_extension("done").exists();
			let text =
				tokio::fs::read_to_string(&path).await.unwrap_or_default();
			// whole lines only, the last one may still be written
			let complete = match text.rfind('\n') {
				Some(i) => &text[..=i],
				None => "",
			};
			for (i, line) in complete.lines().enumerate().skip(sent) {
				let event = Event::default().id(i.to_string()).data(line);
				if tx.send(Ok(event)).await.is_err() {
					return;
				}
				sent = i + 1;
			}
			if done {
				let err = tokio::fs::read_to_string(path.with_extension("err"))
					.await
					.unwrap_or_default();
				let event = Event::default().event("done").data(tail(&err));
				let _ = tx.send(Ok(event)).await;
				return;
			}
			tokio::time::sleep(Duration::from_millis(300)).await;
		}
	});
	let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
	Ok(Sse::new(stream).keep_alive(KeepAlive::default()))
}

/// The end of the agent's errors, for a run that failed.
fn tail(err: &str) -> String {
	let start = err.len().saturating_sub(2000);
	let start = (start..=err.len())
		.find(|i| err.is_char_boundary(*i))
		.unwrap_or(0);
	err[start..].trim().to_string()
}

/// "New" in the chat: the next message starts a new conversation.
async fn forget(State(state): State<AppState>, auth: Authed) -> Result<()> {
	let agent = agent(&state, &auth)?;
	let _ = fs::remove_file(session_file(&agent, auth.user_id));
	Ok(())
}

pub fn routes() -> Router<AppState> {
	Router::new()
		.route("/agent", post(start))
		.route("/agent/session", delete(forget))
		.route("/agent/{id}", get(events))
}

/// Whether the folder looks like a project for the agent.
pub fn check_dir(dir: &Path) -> std::result::Result<(), String> {
	if dir.join("cms").is_dir() || dir.join("AGENT.md").is_file() {
		Ok(())
	} else {
		Err(format!(
			"--agent-dir {}: no project there (no cms/ or AGENT.md)",
			dir.display()
		))
	}
}
