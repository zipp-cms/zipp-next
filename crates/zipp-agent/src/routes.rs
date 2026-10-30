use std::convert::Infallible;
use std::fs;
use std::process::Stdio;
use std::time::Duration;

use serde::Deserialize;
use zipp::AppState;
use zipp::api::{ApiResult, ApiRouter, Req};
use zipp::auth::{Admin, end_session, new_session};
use zipp::axum::extract::{Path, Query, State};
use zipp::axum::response::IntoResponse;
use zipp::axum::response::sse::{Event, KeepAlive, Sse};
use zipp::axum::routing::get;
use zipp::axum::{Json, Router};
use zipp::db::random_token;
use zipp::error::{Error, Result};

use crate::api::*;
use crate::{agent, finish, run_file, session_file, system_prompt};

async fn start(
	State(state): State<AppState>,
	Admin(auth): Admin,
	Req(req): Req<StartReq>,
) -> ApiResult<StartReq> {
	let agent = agent(&state)?;
	if req.message.trim().is_empty() {
		return Err(Error::Request("say what to do".into()));
	}
	let io = |e: std::io::Error| Error::Internal(e.to_string());
	fs::create_dir_all(&agent.runs).map_err(io)?;

	// the agent acts as the user, with a session of its own
	let token = new_session(&state.db.conn(), auth.user_id)?.token;

	let id = random_token();
	let out = fs::File::create(run_file(&agent, &id, "jsonl")?).map_err(io)?;
	let err = fs::File::create(run_file(&agent, &id, "err")?).map_err(io)?;
	let session = fs::read_to_string(session_file(&agent, auth.user_id)).ok();
	let system = system_prompt(&agent, req.view.as_deref());
	let mut cmd = agent.harness.command(
		&req.message,
		&system,
		session.as_deref().map(str::trim),
	);
	cmd.current_dir(&agent.dir)
		.env("ZIPP_API", &agent.api)
		.env("ZIPP_TOKEN", &token)
		.stdin(Stdio::null())
		.stdout(out)
		.stderr(err);
	// its own process group, a restart of the cms doesn't stop it
	std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
	let program = cmd.get_program().to_string_lossy().to_string();
	let mut child = cmd
		.spawn()
		.map_err(|e| Error::Request(format!("{program}: {e}")))?;

	let (db, user, run) = (state.db.clone(), auth.user_id, id.clone());
	tokio::task::spawn_blocking(move || {
		let _ = child.wait();
		finish(&agent, &run, user);
		let _ = end_session(&db.conn(), &token);
	});
	Ok(Json(StartResp { id }))
}

#[derive(Debug, Deserialize)]
struct EventsParams {
	/// the lines already seen, after a reconnect
	#[serde(default)]
	from: usize,
}

/// The lines of a run as events of the chat (`data` is a [`ChatEvent`],
/// `id` the number of its line), then `done`.
async fn events(
	State(state): State<AppState>,
	_admin: Admin,
	Path(id): Path<String>,
	Query(p): Query<EventsParams>,
) -> Result<impl IntoResponse> {
	let agent = agent(&state)?;
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
				for e in agent.harness.events(line, &agent.dir) {
					let data = serde_json::to_string(&e).unwrap_or_default();
					let event = Event::default().id(i.to_string()).data(data);
					if tx.send(Ok(event)).await.is_err() {
						return;
					}
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

async fn forget(
	State(state): State<AppState>,
	Admin(auth): Admin,
	Req(_): Req<ForgetReq>,
) -> ApiResult<ForgetReq> {
	let agent = agent(&state)?;
	let _ = fs::remove_file(session_file(&agent, auth.user_id));
	Ok(Json(()))
}

pub(crate) fn routes() -> Router<AppState> {
	Router::new()
		.api::<StartReq, _>(start)
		.api::<ForgetReq, _>(forget)
		.route("/agent/{id}", get(events))
}
