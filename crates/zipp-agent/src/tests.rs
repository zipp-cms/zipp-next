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
	let end =
		r#"{"type":"result","is_error":true,"result":"no","session_id":"s1"}"#;
	assert_eq!(c.events(end, dir), [ChatEvent::Error("no".into())]);
	assert_eq!(c.session(end).as_deref(), Some("s1"));
	assert!(c.events("not json", dir).is_empty());
}

#[test]
fn the_message_comes_after_the_options() {
	let c = ClaudeCode {
		command: "claude".into(),
	};
	let cmd = c.command("--resume x", "system", Some("s1"));
	let args: Vec<_> = cmd.get_args().map(|a| a.to_str().unwrap()).collect();
	assert_eq!(args[args.len() - 2..], ["--", "--resume x"]);
	assert!(args.contains(&"s1"));
}
