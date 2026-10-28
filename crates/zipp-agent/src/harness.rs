//! The coding agents the plugin can run. A harness builds the command for
//! a message and turns the lines of its output into the events of the
//! chat, so the studio doesn't depend on one agent.

use std::path::Path;
use std::process::Command;

use serde::Serialize;
use serde_json::Value;

/// What the chat shows of a run.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "kind", content = "text", rename_all = "camelCase")]
pub enum ChatEvent {
	/// the agent writes to the user
	Text(String),
	/// it did something, said short
	Step(String),
	Error(String),
}

pub trait Harness: Send + Sync + 'static {
	/// The command for a message; `system` is what zipp adds to the
	/// project's own instructions, `session` continues a conversation.
	fn command(
		&self,
		message: &str,
		system: &str,
		session: Option<&str>,
	) -> Command;

	/// A line of its output as events; `dir` is the project, to show
	/// paths in it.
	fn events(&self, line: &str, dir: &Path) -> Vec<ChatEvent>;

	/// The conversation of a finished run, for the next message.
	fn session(&self, output: &str) -> Option<String>;
}

/// Claude Code (`claude`, or `ZIPP_AGENT_COMMAND`) without asking: the
/// project is a sandbox.
pub struct ClaudeCode {
	pub command: String,
}

impl ClaudeCode {
	pub fn new() -> Self {
		let command = std::env::var("ZIPP_AGENT_COMMAND")
			.ok()
			.filter(|c| !c.is_empty())
			.unwrap_or("claude".into());
		Self { command }
	}
}

impl Default for ClaudeCode {
	fn default() -> Self {
		Self::new()
	}
}

impl Harness for ClaudeCode {
	fn command(
		&self,
		message: &str,
		system: &str,
		session: Option<&str>,
	) -> Command {
		let mut cmd = Command::new(&self.command);
		cmd.args(["-p", message])
			.args(["--output-format", "stream-json", "--verbose"])
			.arg("--dangerously-skip-permissions")
			.args(["--append-system-prompt", system]);
		if let Some(s) = session {
			cmd.args(["--resume", s]);
		}
		cmd
	}

	fn events(&self, line: &str, dir: &Path) -> Vec<ChatEvent> {
		let Ok(v) = serde_json::from_str::<Value>(line) else {
			return vec![];
		};
		match v["type"].as_str() {
			Some("assistant") => v["message"]["content"]
				.as_array()
				.into_iter()
				.flatten()
				.filter_map(|c| match c["type"].as_str() {
					Some("text") => {
						let t = c["text"].as_str().unwrap_or_default().trim();
						(!t.is_empty()).then(|| ChatEvent::Text(t.into()))
					}
					Some("tool_use") => {
						let name = c["name"].as_str().unwrap_or_default();
						step(name, &c["input"], dir).map(ChatEvent::Step)
					}
					_ => None,
				})
				.collect(),
			Some("result") if v["is_error"] == true => {
				let r = v["result"].as_str().unwrap_or("The agent stopped");
				vec![ChatEvent::Error(r.into())]
			}
			_ => vec![],
		}
	}

	fn session(&self, output: &str) -> Option<String> {
		output
			.lines()
			.filter_map(|l| serde_json::from_str::<Value>(l).ok())
			.filter_map(|v| v["session_id"].as_str().map(String::from))
			.last()
	}
}

/// A step of Claude Code as the user reads it, none for its notes.
fn step(tool: &str, input: &Value, dir: &Path) -> Option<String> {
	let path = |key: &str| {
		let p = input[key].as_str().unwrap_or_default();
		let dir = format!("{}/", dir.display());
		p.strip_prefix(&dir).unwrap_or(p).to_string()
	};
	Some(match tool {
		"TodoWrite" => return None,
		"Edit" | "MultiEdit" => format!("Edits {}", path("file_path")),
		"Write" => format!("Writes {}", path("file_path")),
		"Read" => format!("Reads {}", path("file_path")),
		"Grep" | "Glob" => "Searches the code".into(),
		"WebFetch" | "WebSearch" => "Looks something up on the web".into(),
		"Bash" => command_step(input),
		other => other.into(),
	})
}

/// What a shell command does: zipp's api, the checks and git said in
/// words, else the agent's description of it.
fn command_step(input: &Value) -> String {
	let cmd = input["command"].as_str().unwrap_or_default();
	let api = cmd.contains("ZIPP_API");
	if api && cmd.contains("PUT") {
		"Saves a draft".into()
	} else if api && cmd.contains("POST") {
		"Creates an entry".into()
	} else if api {
		"Reads the content".into()
	} else if cmd.contains("makemigration") {
		"Writes a migration".into()
	} else if cmd.contains("restart") {
		"Restarts the cms".into()
	} else if cmd.contains(" check") {
		"Runs the checks".into()
	} else if cmd.contains("git commit") {
		"Commits the changes".into()
	} else if let Some(d) = input["description"].as_str() {
		d.into()
	} else {
		let first = cmd.split_whitespace().next().unwrap_or("a command");
		format!("Runs {first}")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn claude_lines_become_events() {
		let c = ClaudeCode {
			command: "claude".into(),
		};
		let dir = Path::new("/home/dev/zandi");
		let line = r#"{"type":"assistant","message":{"content":[
			{"type":"text","text":"Ich schaue nach."},
			{"type":"tool_use","name":"Edit","input":{"file_path":"/home/dev/zandi/cms/src/main.rs"}},
			{"type":"tool_use","name":"Bash","input":{"command":"curl -X PUT \"$ZIPP_API/entries/3/draft\""}},
			{"type":"tool_use","name":"TodoWrite","input":{}}
		]}}"#
			.replace('\n', "");
		assert_eq!(
			c.events(&line, dir),
			[
				ChatEvent::Text("Ich schaue nach.".into()),
				ChatEvent::Step("Edits cms/src/main.rs".into()),
				ChatEvent::Step("Saves a draft".into()),
			]
		);
		let end = r#"{"type":"result","is_error":true,"result":"no","session_id":"s1"}"#;
		assert_eq!(c.events(end, dir), [ChatEvent::Error("no".into())]);
		assert_eq!(c.session(end).as_deref(), Some("s1"));
		assert!(c.events("not json", dir).is_empty());
	}
}
