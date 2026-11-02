//! A chat with the content of the site: the editor asks ("fix the
//! translation of the about page"), a model reads and changes entries with
//! tools. It only writes drafts, an editor sees them in the preview and
//! publishes them.
//!
//! The models run through soe-llms on OpenRouter. The studio keeps the
//! conversation and sends it with every message.

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use soe_llms::{
	Input, Llms, LlmsConfig, Model, Output, Request, ResponseEvent, Role, Tool,
};
use tokio::sync::mpsc::Sender;

use crate::AppState;
use crate::auth::Authed;
use crate::drafts::{Draft, SiteDoc, current, get_draft, put_draft};
use crate::entries::query::Query;
use crate::entries::{Entry, EntryInput, by_uri, get_entry, save, select};
use crate::error::{Error, Result};
use crate::schema::Access;
use crate::schema::Schema;

mod api;
#[cfg(test)]
mod api_tests;
mod prompt;
mod routes;
#[cfg(test)]
pub(crate) mod tests;
mod tools;

use api::*;
use prompt::*;
pub use routes::routes;
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

/// The turns of the model and its tools, streaming text and actions;
/// returns the messages to append to the conversation.
async fn run_chat(
	state: &AppState,
	auth: &Authed,
	cfg: &AiConfig,
	model: String,
	req: ChatReq,
	events: &Sender<ChatEvent>,
) -> Result<Vec<Message>> {
	let view = req.view.as_deref().filter(|v| v.len() < 2000);
	let view = describe_view(state, auth, view);
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
	let send = async |ev: ChatEvent| {
		let gone = |_| Error::Request("the chat was closed".into());
		events.send(ev).await.map_err(gone)
	};

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
			send(ChatEvent::Delta { text: content }).await?;
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
						send(ChatEvent::Delta { text }).await?;
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
						send(ChatEvent::Action { action }).await?;
					}
					v.to_string()
				}
				Err(e) => format!("error: {}", e.public()),
			};
			messages.push(Message::ToolOutput { id, output });
		}
	}

	Ok(messages.split_off(start))
}
