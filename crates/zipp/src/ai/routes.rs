use std::convert::Infallible;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::sse::{Event, Sse};
use axum::routing::post;
use axum::{Json, Router};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use super::api::{ChatEvent, ChatReq};
use super::{MAX_CONVERSATION, MODELS, run_chat};
use crate::AppState;
use crate::auth::AiAccess;
use crate::error::{Error, Result};

async fn chat(
	State(state): State<AppState>,
	AiAccess(auth): AiAccess,
	Json(req): Json<ChatReq>,
) -> Result<impl IntoResponse> {
	let cfg = state.ai.clone().ok_or_else(|| {
		Error::Request("the AI chat is off, no api key".into())
	})?;
	let model = match req.model.clone() {
		Some(m) if MODELS.iter().any(|(id, _)| *id == m) => m,
		Some(m) => return Err(Error::Request(format!("unknown model {m}"))),
		None => MODELS[0].0.into(),
	};

	// the conversation goes to the model on every turn
	let size: usize = req
		.messages
		.iter()
		.map(|m| serde_json::to_string(m).map_or(0, |s| s.len()))
		.sum();
	if size > MAX_CONVERSATION {
		return Err(Error::Request(
			"The conversation is too long, start a new one".into(),
		));
	}

	let (tx, rx) = tokio::sync::mpsc::channel(64);
	tokio::spawn(async move {
		let done = match run_chat(&state, &auth, &cfg, model, req, &tx).await {
			Ok(messages) => ChatEvent::Done { messages },
			Err(e) => ChatEvent::Error {
				message: e.public().to_string(),
			},
		};
		let _ = tx.send(done).await;
	});
	let events = ReceiverStream::new(rx).map(|ev| {
		let data = serde_json::to_string(&ev).unwrap();
		Ok::<_, Infallible>(Event::default().data(data))
	});
	// a proxy like nginx would wait for the whole answer
	let headers = [("x-accel-buffering", "no")];
	Ok((headers, Sse::new(events)))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/ai/chat", post(chat))
}
