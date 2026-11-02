//! The chat streams server-sent events, it is no `request!`.

use serde::{Deserialize, Serialize};

use super::{Action, Message};

#[derive(Debug, Deserialize)]
pub struct ChatReq {
	/// one of [`MODELS`](super::MODELS), default the first
	pub model: Option<String>,
	/// the page the preview shows
	pub view: Option<String>,
	/// the conversation so far, the new user message last
	pub messages: Vec<Message>,
}

/// What the chat sends while the model works, as server-sent events.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ChatEvent {
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
