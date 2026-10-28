//! A chat with the content of the site: the editor asks ("fix the
//! translation of the about page"), a model reads and changes entries with
//! tools. It only writes drafts, an editor sees them in the preview and
//! publishes them.
//!
//! The models run through soe-llms on OpenRouter. The studio keeps the
//! conversation and sends it with every message.

use std::convert::Infallible;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::sse::{Event, Sse};
use axum::routing::post;
use axum::{Json, Router};
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use soe_llms::{
	Input, Llms, LlmsConfig, Model, Output, Request, ResponseEvent, Role, Tool,
};
use tokio_stream::wrappers::ReceiverStream;

use crate::AppState;
use crate::auth::Authed;
use crate::drafts::{SiteDoc, get_draft, put_draft};
use crate::entries::{EntryInput, get_entry, list, save};
use crate::error::{Error, Result};
use crate::query::Query;
use crate::schema::Access;
use crate::schema::Schema;

mod prompt;
#[cfg(test)]
pub(crate) mod tests;
mod tools;

use prompt::*;
pub use tools::Action;
use tools::*;

/// The models an editor can choose, OpenRouter ids and names; the first is
/// the default.
pub const MODELS: &[(&str, &str)] = &[
	("anthropic/claude-sonnet-5.5", "Claude Sonnet"),
	("openai/gpt-6.1-sol", "GPT Sol"),
	("google/gemini-4-argon", "Gemini"),
];

/// Tool calls of one message, then the chat answers anyway.
const MAX_TURNS: usize = 12;

/// Bytes of a conversation, it is sent to the model on every turn.
const MAX_CONVERSATION: usize = 400_000;

/// The llms, set up with `serve --openrouter-key-file`.
#[derive(Clone)]
pub struct AiConfig {
	llms: Llms,
}

impl AiConfig {
	pub fn openrouter(key: String) -> Self {
		Self {
			llms: Llms::new(LlmsConfig::new().openrouter(Some(key))),
		}
	}
}

/// A message of the conversation as the studio keeps it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Message {
	User {
		text: String,
	},
	Assistant {
		text: String,
	},
	ToolCall {
		id: String,
		name: String,
		input: Value,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		context: Option<String>,
	},
	ToolOutput {
		id: String,
		output: String,
	},
}

impl From<Message> for Input {
	fn from(m: Message) -> Self {
		match m {
			Message::User { text } => Input::Text {
				role: Role::User,
				content: text,
			},
			Message::Assistant { text } => Input::Text {
				role: Role::Assistant,
				content: text,
			},
			Message::ToolCall {
				id,
				name,
				input,
				context,
			} => Input::ToolCall {
				id,
				name,
				input,
				context,
			},
			Message::ToolOutput { id, output } => {
				Input::ToolCallOutput { id, output }
			}
		}
	}
}

#[derive(Debug, Deserialize)]
struct ChatReq {
	/// one of [`MODELS`], default the first
	model: Option<String>,
	/// the page the preview shows
	view: Option<String>,
	/// the conversation so far, the new user message last
	messages: Vec<Message>,
}

/// What the chat sends while the model works, as server-sent events.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
enum ChatEvent {
	/// more text of the answer
	Delta {
		text: String,
	},
	/// a tool did something the studio shows
	Action {
		action: Action,
	},
	/// the messages to append to the conversation
	Done {
		messages: Vec<Message>,
	},
	Error {
		message: String,
	},
}

async fn chat(
	State(state): State<AppState>,
	auth: Authed,
	Json(req): Json<ChatReq>,
) -> Result<impl IntoResponse> {
	if !auth.can_ai(&state.schema) {
		return Err(Error::Forbidden);
	}
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
		let send = |ev: ChatEvent| {
			let data = serde_json::to_string(&ev).unwrap();
			tx.send(Ok(Event::default().data(data)))
		};
		let done = match run_chat(&state, &auth, &cfg, model, req, &send).await
		{
			Ok(messages) => ChatEvent::Done { messages },
			Err(e) => ChatEvent::Error {
				message: e.to_string(),
			},
		};
		let _ = send(done).await;
	});
	// a proxy like nginx would wait for the whole answer
	let headers = [("x-accel-buffering", "no")];
	Ok((headers, Sse::new(ReceiverStream::new(rx))))
}

/// The turns of the model and its tools, streaming text and actions;
/// returns the messages to append to the conversation.
async fn run_chat<F, Fut>(
	state: &AppState,
	auth: &Authed,
	cfg: &AiConfig,
	model: String,
	req: ChatReq,
	send: &F,
) -> Result<Vec<Message>>
where
	F: Fn(ChatEvent) -> Fut,
	Fut: std::future::Future<
			Output = std::result::Result<
				(),
				tokio::sync::mpsc::error::SendError<
					std::result::Result<Event, Infallible>,
				>,
			>,
		>,
{
	let view = req.view.as_deref().filter(|v| v.len() < 2000);
	let view = describe_view(state, view);
	let mut messages = req.messages;
	let start = messages.len();
	let asked = messages
		.iter()
		.rev()
		.find_map(|m| match m {
			Message::User { text } => Some(text.clone()),
			_ => None,
		})
		.unwrap_or_default();
	// the studio went away
	let gone = || Error::Request("the chat was closed".into());

	for _ in 0..MAX_TURNS {
		let request = Request {
			input: messages.iter().cloned().map(Into::into).collect(),
			instructions: system_prompt(&state.schema, &view, &asked),
			model: Model::OpenRouter(Some(model.clone())),
			user_id: auth.user_id.to_string(),
			tools: tools(),
			reasoning_effort: None,
		};
		let ai = |e: soe_llms::LlmsError| Error::Internal(format!("ai: {e}"));
		let mut stream = cfg.llms.request(&request).await.map_err(ai)?;
		let mut streamed = String::new();
		while let Some(ev) = stream.next().await {
			let ResponseEvent::TextDelta { content } = ev.map_err(ai)?;
			streamed += &content;
			send(ChatEvent::Delta { text: content })
				.await
				.map_err(|_| gone())?;
		}
		let resp = stream.into_response().ok_or_else(|| {
			Error::Internal("ai: the answer ended early".into())
		})?;

		let mut calls = vec![];
		for out in resp.output {
			match out {
				Output::Text { content } => {
					// some providers send the end only with the answer
					if let Some(rest) = content.strip_prefix(streamed.as_str())
						&& !rest.is_empty()
					{
						let text = rest.to_string();
						send(ChatEvent::Delta { text })
							.await
							.map_err(|_| gone())?;
					}
					messages.push(Message::Assistant { text: content });
				}
				Output::ToolCall {
					id,
					name,
					input,
					context,
				} => {
					calls.push((id.clone(), name.clone(), input.clone()));
					messages.push(Message::ToolCall {
						id,
						name,
						input,
						context,
					});
				}
			}
		}
		if calls.is_empty() {
			break;
		}

		for (id, name, input) in calls {
			let output = match run_tool(state, auth, &name, &input) {
				Ok((v, action)) => {
					if let Some(action) = action {
						send(ChatEvent::Action { action })
							.await
							.map_err(|_| gone())?;
					}
					v.to_string()
				}
				Err(e) => format!("error: {e}"),
			};
			messages.push(Message::ToolOutput { id, output });
		}
	}

	Ok(messages.split_off(start))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/ai/chat", post(chat))
}
