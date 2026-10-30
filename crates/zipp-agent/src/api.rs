//! The api of the agent, for the chat of the studio. The events of a run
//! are a stream: `GET /agent/{id}?from=<line>`, server-sent events.

use serde::{Deserialize, Serialize};
use zipp::request;

/// A message to the agent, it starts a run.
#[derive(Debug, Serialize, Deserialize)]
pub struct StartReq {
	pub message: String,
	/// the url the user sees next to the chat
	pub view: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartResp {
	/// the run, for its events
	pub id: String,
}

request!(StartReq, POST "/agent" => StartResp);

/// "New" in the chat: the next message starts a new conversation.
#[derive(Debug, Serialize, Deserialize)]
pub struct ForgetReq {}

request!(ForgetReq, DELETE "/agent/session" => ());
