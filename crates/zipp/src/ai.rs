//! A chat with the content of the site: the editor asks ("fix the
//! translation of the about page"), a model reads and changes entries with
//! tools. It only writes drafts, an editor sees them in the preview and
//! publishes them.
//!
//! The models run through soe-llms on OpenRouter. The studio keeps the
//! conversation and sends it with every message.

use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use soe_llms::{Input, Llms, LlmsConfig, Model, Output, Request, Role, Tool};

use crate::AppState;
use crate::auth::Authed;
use crate::drafts::{SiteDoc, create_token, get_draft, live_doc, put_draft};
use crate::entries::{get_entry, list};
use crate::error::{Error, Result};
use crate::query::Query;
use crate::schema::Schema;

/// The models an editor can choose, OpenRouter ids and names; the first is
/// the default.
pub const MODELS: &[(&str, &str)] = &[
	("anthropic/claude-sonnet-5.5", "Claude Sonnet"),
	("openai/gpt-6.1-sol", "GPT Sol"),
	("google/gemini-4-argon", "Gemini"),
];
/// Tool calls of one message, then the chat answers anyway.
const MAX_TURNS: usize = 12;

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

fn tools() -> Vec<Tool> {
	let tools = json!([
		{
			"name": "list_entries",
			"description": "Lists entries of the site with id, section, type, title and uri. \
				Filter by section, type or a part of the title.",
			"input_schema": {
				"type": "object",
				"properties": {
					"site": { "type": "string", "description": "site handle, default the primary site" },
					"section": { "type": "string" },
					"type": { "type": "string" },
					"search": { "type": "string", "description": "part of the title" }
				},
				"additionalProperties": false
			}
		},
		{
			"name": "get_entry",
			"description": "The content of an entry in a site: title, slug, published and \
				its fields as JSON. If there is a draft, the draft.",
			"input_schema": {
				"type": "object",
				"properties": {
					"id": { "type": "integer" },
					"site": { "type": "string" }
				},
				"required": ["id", "site"],
				"additionalProperties": false
			}
		},
		{
			"name": "update_entry",
			"description": "Changes an entry in one site as a draft, the website stays as it is \
				until an editor publishes. Give only what changes: the title and/or fields \
				(top level field handles, each with its whole new value in the same JSON shape \
				get_entry returned). Fields that are the same in all sites change everywhere.",
			"input_schema": {
				"type": "object",
				"properties": {
					"id": { "type": "integer" },
					"site": { "type": "string" },
					"title": { "type": "string" },
					"fields": { "type": "object" }
				},
				"required": ["id", "site"],
				"additionalProperties": false
			}
		}
	]);
	tools
		.as_array()
		.unwrap()
		.iter()
		.map(|t| Tool {
			name: t["name"].as_str().unwrap().into(),
			description: t["description"].as_str().unwrap().into(),
			parameters: Some(t["input_schema"].clone()),
		})
		.collect()
}

fn system_prompt(schema: &Schema) -> String {
	// the content model, so the model knows sections, types and fields
	let model = json!({
		"sites": schema.sites,
		"sections": schema.sections,
		"components": schema.components,
	});
	format!(
		"You help the editors of a website that runs on the headless CMS zipp. \
		You read and change its content with tools.\n\n\
		- Changes are drafts: the editor sees them in the preview next to this chat \
		and publishes them. Say what you changed in a sentence or two, nothing more.\n\
		- Before changing an entry, read it with get_entry and keep the JSON shape of \
		its fields exactly (blocks keep their id and type, rich text is a TipTap \
		document, links are {{\"type\", \"value\", \"label\"}}).\n\
		- Fields marked translatable have a value per site, the others are the same \
		in every site.\n\
		- Answer in the language the editor writes in.\n\n\
		The content model:\n{}",
		serde_json::to_string(&model).unwrap()
	)
}

/// What the studio shows of a tool call, and where the preview goes.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
	pub text: String,
	pub entry_id: Option<i64>,
	pub section: Option<String>,
	/// the draft on the website, with a preview token
	pub preview: Option<String>,
}

/// Runs one tool, returns its result for the model and what the studio
/// shows.
fn run_tool(
	state: &AppState,
	user_id: i64,
	name: &str,
	input: &Value,
) -> Result<(Value, Option<Action>)> {
	let conn = state.db.conn();
	let schema = &state.schema;
	let primary = schema.primary_site().handle.clone();
	let site = input["site"].as_str().unwrap_or(&primary).to_string();

	match name {
		"list_entries" => {
			let mut params = vec![
				("site".to_string(), site.clone()),
				("limit".to_string(), "200".to_string()),
			];
			for key in ["section", "type", "search"] {
				if let Some(v) = input[key].as_str() {
					params.push((key.into(), v.into()));
				}
			}
			let q = Query::parse(schema, &params).map_err(Error::Request)?;
			let (entries, _) = list(&conn, &q)?;
			let out: Vec<Value> = entries
				.iter()
				.map(|e| {
					json!({ "id": e.id, "section": e.section, "type": e.ty,
						"title": e.title, "uri": e.uri, "published": e.published })
				})
				.collect();
			Ok((json!(out), None))
		}
		"get_entry" => {
			let id = input["id"].as_i64().unwrap_or_default();
			let e = get_entry(&conn, id, &site)?;
			let doc = match get_draft(&conn, id)? {
				Some(d) => d.doc,
				None => live_doc(&conn, id)?,
			};
			let content = doc.get(&site).cloned();
			Ok((
				json!({ "id": id, "section": e.section, "type": e.ty, "site": site,
					"uri": e.uri, "content": content }),
				None,
			))
		}
		"update_entry" => {
			let id = input["id"].as_i64().unwrap_or_default();
			let entry = get_entry(&conn, id, &site)?;
			let draft = get_draft(&conn, id)?;
			let base = draft.as_ref().map(|d| d.updated_at);
			let doc = match draft {
				Some(d) => d.doc,
				None => live_doc(&conn, id)?,
			};
			let mut content: SiteDoc =
				doc.get(&site).cloned().ok_or_else(|| {
					Error::Request(format!("unknown site {site}"))
				})?;
			if let Some(t) = input["title"].as_str() {
				content.title = t.into();
			}
			if let Some(Value::Object(changes)) = input.get("fields") {
				let fields = content.fields.as_object_mut();
				let Some(fields) = fields else {
					return Err(Error::Request(
						"the entry has no fields".into(),
					));
				};
				for (k, v) in changes {
					fields.insert(k.clone(), v.clone());
				}
			}
			put_draft(&conn, schema, id, &site, content, base, Some(user_id))?;

			let token = create_token(&conn, id)?;
			let preview = match (&entry.uri, state.site_urls.get(&site)) {
				(Some(uri), Some(base)) => Some(format!(
					"{}/{uri}?token={token}",
					base.trim_end_matches('/')
				)),
				_ => None,
			};
			let action = Action {
				text: format!("Changed the draft of {} ({site})", entry.title),
				entry_id: Some(id),
				section: Some(entry.section.clone()),
				preview,
			};
			Ok((json!({ "ok": true, "draft": true }), Some(action)))
		}
		_ => Err(Error::Request(format!("unknown tool {name}"))),
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
	/// the conversation so far, the new user message last
	messages: Vec<Message>,
}

#[derive(Debug, Serialize)]
struct ChatResp {
	/// the messages to append to the conversation
	messages: Vec<Message>,
	/// the answer for the chat
	text: String,
	actions: Vec<Action>,
}

async fn chat(
	State(state): State<AppState>,
	auth: Authed,
	Json(req): Json<ChatReq>,
) -> Result<Json<ChatResp>> {
	let cfg = state.ai.clone().ok_or_else(|| {
		Error::Request("the AI chat is off, no api key".into())
	})?;
	let model = match req.model {
		Some(m) if MODELS.iter().any(|(id, _)| *id == m) => m,
		Some(m) => return Err(Error::Request(format!("unknown model {m}"))),
		None => MODELS[0].0.into(),
	};

	let mut messages = req.messages;
	let start = messages.len();
	let mut actions = vec![];
	let mut text = String::new();

	for _ in 0..MAX_TURNS {
		let request = Request {
			input: messages.iter().cloned().map(Into::into).collect(),
			instructions: system_prompt(&state.schema),
			model: Model::OpenRouter(Some(model.clone())),
			user_id: auth.user_id.to_string(),
			tools: tools(),
			reasoning_effort: None,
		};
		let resp = cfg
			.llms
			.request(&request)
			.await
			.map_err(|e| Error::Internal(format!("ai: {e}")))?
			.wait()
			.await
			.map_err(|e| Error::Internal(format!("ai: {e}")))?;

		let mut calls = vec![];
		for out in resp.output {
			match out {
				Output::Text { content } => {
					text = content.clone();
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
			let output = match run_tool(&state, auth.user_id, &name, &input) {
				Ok((v, action)) => {
					actions.extend(action);
					v.to_string()
				}
				Err(e) => format!("error: {e}"),
			};
			messages.push(Message::ToolOutput { id, output });
		}
	}

	Ok(Json(ChatResp {
		messages: messages.split_off(start),
		text,
		actions,
	}))
}

pub fn routes() -> Router<AppState> {
	Router::new().route("/ai/chat", post(chat))
}
