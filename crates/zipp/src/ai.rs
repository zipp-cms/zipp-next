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
use rusqlite::{Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use soe_llms::{Input, Llms, LlmsConfig, Model, Output, Request, Role, Tool};

use crate::AppState;
use crate::auth::Authed;
use crate::drafts::{SiteDoc, get_draft, live_doc, put_draft};
use crate::entries::{EntryInput, get_entry, list, save};
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

fn tools() -> Vec<Tool> {
	let tools = json!([
		{
			"name": "list_entries",
			"description": "Lists entries with id, section, type, title, uri, whether they are \
				published and whether they have a draft, as the editor would see them. \
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
			"description": "Changes an entry in one site as a draft. Give only what changes: \
				title, slug, published and/or fields (top level field handles, each with its \
				whole new value in the shape get_entry returned). Fields that are the same in \
				all sites change everywhere.",
			"input_schema": {
				"type": "object",
				"properties": {
					"id": { "type": "integer" },
					"site": { "type": "string" },
					"title": { "type": "string" },
					"slug": { "type": "string" },
					"published": { "type": "boolean" },
					"fields": { "type": "object" }
				},
				"required": ["id", "site"],
				"additionalProperties": false
			}
		},
		{
			"name": "create_entry",
			"description": "Creates an entry in a section (not a single) in every site. It \
				stays off the website until an editor publishes it; the preview already \
				shows it. Give title and fields for one site, then translate the other \
				sites with update_entry. In a structure, parent_id puts it under a page.",
			"input_schema": {
				"type": "object",
				"properties": {
					"section": { "type": "string" },
					"type": { "type": "string" },
					"site": { "type": "string" },
					"title": { "type": "string" },
					"slug": { "type": "string" },
					"parent_id": { "type": "integer" },
					"fields": { "type": "object" }
				},
				"required": ["section", "type", "title"],
				"additionalProperties": false
			}
		},
		{
			"name": "list_assets",
			"description": "Images and files with id, filename, kind and alt text, for \
				image and asset fields (they hold a list of asset ids).",
			"input_schema": {
				"type": "object",
				"properties": {
					"search": { "type": "string", "description": "part of the filename" }
				},
				"additionalProperties": false
			}
		},
		{
			"name": "get_translations",
			"description": "The texts of the website's code in a site (menu labels, \
				messages), by namespace and key.",
			"input_schema": {
				"type": "object",
				"properties": { "site": { "type": "string" } },
				"required": ["site"],
				"additionalProperties": false
			}
		},
		{
			"name": "set_translation",
			"description": "Sets one text of the website's code in a site. Unlike entries \
				this is live at once, there are no drafts of texts. An empty value goes \
				back to the default.",
			"input_schema": {
				"type": "object",
				"properties": {
					"site": { "type": "string" },
					"namespace": { "type": "string" },
					"key": { "type": "string" },
					"value": { "type": "string" }
				},
				"required": ["site", "namespace", "key", "value"],
				"additionalProperties": false
			}
		},
		{
			"name": "show_page",
			"description": "Opens a page of the website in the preview next to the chat: \
				the page of an entry, or a path of the site like `events`.",
			"input_schema": {
				"type": "object",
				"properties": {
					"entry_id": { "type": "integer" },
					"path": { "type": "string" },
					"site": { "type": "string" }
				},
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

fn system_prompt(schema: &Schema, view: &str) -> String {
	// the content model, so the model knows sections, types and fields
	let model = json!({
		"sites": schema.sites,
		"sections": schema.sections,
		"components": schema.components,
		"translations": schema.translations,
	});
	format!(
		"You help the editors of a website that runs on the headless CMS zipp. You \
		see and change everything an editor can: entries of every section (pages, \
		globals like header and footer), images and the texts of the website's code. \
		Next to the chat the editor sees the website as a preview with every draft.\n\n\
		- Changes of entries are drafts, the website stays as it is until the editor \
		publishes them in the editor. Say what you changed in a sentence or two.\n\
		- Before changing an entry, read it with get_entry and keep the JSON shape of \
		its fields exactly (blocks keep their id and type, rich text is a TipTap \
		document, links are {{\"type\", \"value\", \"label\"}}).\n\
		- Fields marked translatable have a value per site, the others are the same \
		in every site.\n\
		- After a change, look at the page the editor sees (its fields are below). If \
		the change shows there, stay: the preview reloads and keeps its scroll \
		position. It shows there if it is the page itself, a global like the header \
		or footer, or an entry a block of the page lists: a block that lists entries \
		of a section (like upcoming events) also shows new ones. Only if it doesn't \
		show there, open a page where it does with show_page. Also use show_page \
		when the editor wants to see something.\n\n\
		The content model:\n{}\n\n\
		{view}\n\n\
		Today is {}.\n\n\
		Write your answers in the language the editor's last message is written \
		in, not in the language of the content: an English message gets an English \
		answer, even if it names a German page. Content you write into entries is \
		in the language of its site.",
		serde_json::to_string(&model).unwrap(),
		today()
	)
}

fn today() -> String {
	date(crate::db::now())
}

/// The date in UTC of milliseconds since 1970, `2026-10-28`.
fn date(ms: i64) -> String {
	let days = ms.div_euclid(86_400_000);
	// civil from days, Howard Hinnant's algorithm
	let z = days + 719_468;
	let era = z.div_euclid(146_097);
	let doe = z - era * 146_097;
	let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
	let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
	let mp = (5 * doy + 2) / 153;
	let d = doy - (153 * mp + 2) / 5 + 1;
	let m = if mp < 10 { mp + 3 } else { mp - 9 };
	let y = yoe + era * 400 + i64::from(m <= 2);
	format!("{y:04}-{m:02}-{d:02}")
}

/// What the studio shows of a tool call, and what the preview does.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Action {
	pub text: String,
	pub entry_id: Option<i64>,
	pub section: Option<String>,
	/// a page of the website, without a token
	pub url: Option<String>,
	/// whether the preview opens `url`, else it reloads the page it shows
	pub navigate: bool,
}

/// The url of an entry on the website.
fn page_url(state: &AppState, site: &str, uri: Option<&str>) -> Option<String> {
	let base = state.site_urls.get(site)?;
	Some(format!("{}/{}", base.trim_end_matches('/'), uri?))
}

/// The content of an entry in a site as the editor sees it, draft first.
fn current(conn: &Connection, id: i64, site: &str) -> Result<(SiteDoc, bool)> {
	let draft = get_draft(conn, id)?;
	let has_draft = draft.is_some();
	let doc = match draft {
		Some(d) => d.doc,
		None => live_doc(conn, id)?,
	};
	let content = doc
		.get(site)
		.cloned()
		.ok_or_else(|| Error::Request(format!("unknown site {site}")))?;
	Ok((content, has_draft))
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
			let mut out = vec![];
			for e in &entries {
				let (content, draft) = current(&conn, e.id, &site)?;
				out.push(
					json!({ "id": e.id, "section": e.section, "type": e.ty,
					"title": content.title, "uri": e.uri,
					"published": content.published, "draft": draft,
					"parentId": e.parent_id }),
				);
			}
			Ok((json!(out), None))
		}
		"get_entry" => {
			let id = input["id"].as_i64().unwrap_or_default();
			let e = get_entry(&conn, id, &site)?;
			let (content, draft) = current(&conn, id, &site)?;
			Ok((
				json!({ "id": id, "section": e.section, "type": e.ty, "site": site,
					"uri": e.uri, "draft": draft, "content": content }),
				None,
			))
		}
		"update_entry" => {
			let id = input["id"].as_i64().unwrap_or_default();
			let entry = get_entry(&conn, id, &site)?;
			let base = get_draft(&conn, id)?.map(|d| d.updated_at);
			let (mut content, _) = current(&conn, id, &site)?;
			if let Some(t) = input["title"].as_str() {
				content.title = t.into();
			}
			if let Some(t) = input["slug"].as_str() {
				content.slug = t.into();
			}
			if let Some(p) = input["published"].as_bool() {
				content.published = p;
			}
			if let Some(Value::Object(changes)) = input.get("fields") {
				let Some(fields) = content.fields.as_object_mut() else {
					return Err(Error::Request(
						"the entry has no fields".into(),
					));
				};
				for (k, v) in changes {
					fields.insert(k.clone(), v.clone());
				}
			}
			let title = content.title.clone();
			put_draft(&conn, schema, id, &site, content, base, Some(user_id))?;

			let action = Action {
				text: format!("Changed {title} ({site})"),
				entry_id: Some(id),
				section: Some(entry.section.clone()),
				url: page_url(state, &site, entry.uri.as_deref()),
				navigate: false,
			};
			Ok((json!({ "ok": true, "draft": true }), Some(action)))
		}
		"create_entry" => {
			let req =
				|k: &str| input[k].as_str().unwrap_or_default().to_string();
			// live it is off, the draft publishes it in every site
			let entry = save(
				&conn,
				schema,
				None,
				EntryInput {
					section: req("section"),
					ty: req("type"),
					site: Some(site.clone()),
					title: req("title"),
					slug: req("slug"),
					published: false,
					parent_id: input["parent_id"].as_i64(),
					fields: input.get("fields").cloned().unwrap_or(json!({})),
				},
			)?;
			for s in &schema.sites {
				let base = get_draft(&conn, entry.id)?.map(|d| d.updated_at);
				let (mut content, _) = current(&conn, entry.id, &s.handle)?;
				content.published = true;
				put_draft(
					&conn,
					schema,
					entry.id,
					&s.handle,
					content,
					base,
					Some(user_id),
				)?;
			}
			let action = Action {
				text: format!("Created {} ({})", entry.title, entry.section),
				entry_id: Some(entry.id),
				section: Some(entry.section.clone()),
				url: page_url(state, &site, entry.uri.as_deref()),
				navigate: false,
			};
			Ok((
				json!({ "ok": true, "id": entry.id, "uri": entry.uri }),
				Some(action),
			))
		}
		"list_assets" => {
			let search =
				format!("%{}%", input["search"].as_str().unwrap_or_default());
			let mut stmt = conn.prepare(
				"SELECT id, filename, kind, alt FROM assets
				WHERE filename LIKE ?1 ORDER BY id DESC LIMIT 200",
			)?;
			let rows = stmt.query_map([search], |r| {
				Ok(
					json!({ "id": r.get::<_, i64>(0)?, "filename": r.get::<_, String>(1)?,
					"kind": r.get::<_, String>(2)?, "alt": r.get::<_, String>(3)? }),
				)
			})?;
			let out: Vec<Value> = rows.collect::<rusqlite::Result<_>>()?;
			Ok((json!(out), None))
		}
		"get_translations" => {
			let texts = crate::translations::texts(&conn, schema, &site)?;
			Ok((json!(texts), None))
		}
		"set_translation" => {
			let ns = input["namespace"].as_str().unwrap_or_default();
			let key = input["key"].as_str().unwrap_or_default();
			let value = input["value"].as_str().unwrap_or_default();
			crate::translations::set(&conn, schema, &site, ns, key, value)?;
			let action = Action {
				text: format!("Set the text {ns}.{key} ({site}), live at once"),
				entry_id: None,
				section: None,
				url: None,
				navigate: false,
			};
			Ok((json!({ "ok": true }), Some(action)))
		}
		"show_page" => {
			let url = match (input["entry_id"].as_i64(), input["path"].as_str())
			{
				(Some(id), _) => {
					let e = get_entry(&conn, id, &site)?;
					let Some(url) = page_url(state, &site, e.uri.as_deref())
					else {
						return Err(Error::Request(format!(
							"{} has no page, it shows where it is used",
							e.title
						)));
					};
					url
				}
				(None, Some(path)) => {
					page_url(state, &site, Some(path.trim_start_matches('/')))
						.ok_or_else(|| {
						Error::Request(format!("site {site} has no url"))
					})?
				}
				(None, None) => {
					return Err(Error::Request("give entry_id or path".into()));
				}
			};
			let action = Action {
				text: format!("Opened {url}"),
				entry_id: None,
				section: None,
				url: Some(url),
				navigate: true,
			};
			Ok((json!({ "ok": true }), Some(action)))
		}
		_ => Err(Error::Request(format!("unknown tool {name}"))),
	}
}

/// What the editor sees, from the url of the preview: the site and entry
/// of the page, for the system prompt.
fn describe_view(state: &AppState, url: Option<&str>) -> String {
	let Some(url) = url else {
		return "The preview shows no page yet.".into();
	};
	// the longest base first, `/en` is longer than the site without it
	let mut sites: Vec<_> = state.site_urls.iter().collect();
	sites.sort_by_key(|(_, base)| std::cmp::Reverse(base.len()));
	let path = url.split(['?', '#']).next().unwrap_or(url);
	let found = sites.into_iter().find_map(|(site, base)| {
		let base = base.trim_end_matches('/');
		let rest = path.strip_prefix(base)?;
		if !(rest.is_empty() || rest.starts_with('/')) {
			return None;
		}
		Some((site.clone(), rest.trim_matches('/').to_string()))
	});
	// anything else comes from whatever the preview navigated to
	let Some((site, uri)) = found else {
		return "The preview shows a page outside the site.".into();
	};
	let conn = state.db.conn();
	let uri = percent_decode(&uri);
	let entry: Option<(i64, String, String)> = conn
		.query_row(
			"SELECT e.id, e.section, s.title FROM entries e
			JOIN entry_sites s ON s.entry_id = e.id
			WHERE s.site = ?1 AND s.uri = ?2",
			rusqlite::params![site, uri],
			|r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
		)
		.optional()
		.ok()
		.flatten();
	let Some((id, section, title)) = entry else {
		return format!("The editor sees the page {url} in site {site}.");
	};
	// what is on the page, as the preview shows it
	let content = current(&conn, id, &site)
		.ok()
		.and_then(|(c, _)| serde_json::to_string(&c.fields).ok())
		.unwrap_or_default();
	let content: String = content.chars().take(20_000).collect();
	format!(
		"The editor sees the page {url} in site {site}: entry {id} \"{title}\" \
		of section {section}, plus the globals (header, footer). Its fields:\n{content}"
	)
}

/// `%C3%BC` in a path to `ü`.
fn percent_decode(s: &str) -> String {
	let bytes = s.as_bytes();
	let mut out = Vec::with_capacity(bytes.len());
	let mut i = 0;
	while i < bytes.len() {
		if bytes[i] == b'%'
			&& let Some(b) = s
				.get(i + 1..i + 3)
				.and_then(|h| u8::from_str_radix(h, 16).ok())
		{
			out.push(b);
			i += 3;
			continue;
		}
		out.push(bytes[i]);
		i += 1;
	}
	String::from_utf8_lossy(&out).into_owned()
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
	let view = req.view.as_deref().filter(|v| v.len() < 2000);
	let view = describe_view(&state, view);
	let mut messages = req.messages;
	let start = messages.len();
	let mut actions = vec![];
	let mut text = String::new();

	for _ in 0..MAX_TURNS {
		let request = Request {
			input: messages.iter().cloned().map(Into::into).collect(),
			instructions: system_prompt(&state.schema, &view),
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dates() {
		assert_eq!(date(0), "1970-01-01");
		assert_eq!(date(951_782_400_000), "2000-02-29");
		assert_eq!(date(1_793_145_600_000), "2026-10-28");
	}

	#[test]
	fn decodes_paths() {
		assert_eq!(percent_decode("%C3%BCber-uns/team"), "über-uns/team");
	}
}
