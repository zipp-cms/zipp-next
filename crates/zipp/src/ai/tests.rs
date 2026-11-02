use std::sync::Arc;

use super::*;
use crate::SiteUrls;
use crate::fixtures::site_with_roles;
use crate::testing::TestServer;

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

#[test]
fn the_view_shows_only_entries_the_user_sees() {
	let mut ts = TestServer::new(site_with_roles());
	let urls = [("de".to_string(), "https://x.ch".to_string())];
	ts.state.site_urls = Arc::new(SiteUrls::from(urls));
	let user = |role: &str| Authed {
		user_id: 1,
		role: Some(role.into()),
		token: String::new(),
	};

	// the home page is the editor's, not the author's
	ts.state
		.db
		.conn()
		.execute(
			"UPDATE entry_sites SET uri = 'start'
			WHERE entry_id IN (SELECT id FROM entries WHERE section = 'home')",
			[],
		)
		.unwrap();
	let url = Some("https://x.ch/start");
	let editor = describe_view(&ts.state, &user("editor"), url);
	assert!(editor.contains("of section home"), "{editor}");
	let author = describe_view(&ts.state, &user("author"), url);
	assert_eq!(
		author,
		"The editor sees the page https://x.ch/start in site de."
	);
}

#[test]
fn internal_errors_stay_in_the_log() {
	// what the chat shows and the model reads of a failed tool
	let e = Error::Internal("/var/lib/zipp/files: permission denied".into());
	assert_eq!(e.public().to_string(), "Internal server error: see the log");
	let e = Error::Request("unknown site fr".into());
	assert_eq!(e.public().to_string(), "Request error: unknown site fr");
}

#[test]
fn tools_change_drafts_only() {
	let ts = TestServer::new(site_with_roles());
	let conn = ts.state.db.conn();
	crate::auth::add_user(&conn, "e@x.ch", "password1", Some("editor"))
		.unwrap();
	let user_id = conn.last_insert_rowid();
	// the tools take the connection themselves
	drop(conn);
	let editor = Authed {
		user_id,
		role: Some("editor".into()),
		token: String::new(),
	};
	let run = |name: &str, input: Value| {
		run_tool(&ts.state, &editor, name, &input).unwrap()
	};

	let (out, action) = run(
		"create_entry",
		json!({ "section": "pages", "type": "page", "title": "Neu" }),
	);
	let id = out["id"].as_i64().unwrap();
	assert_eq!(action.unwrap().text, "Created Neu (pages)");
	let input =
		json!({ "id": id, "title": "Neuer", "fields": { "intro": "Hi" } });
	let (_, action) = run("update_entry", input);
	assert_eq!(action.unwrap().text, "Changed Neuer (de)");

	// live it stays off and old, the draft is published in every site
	let conn = ts.state.db.conn();
	let live = get_entry(&conn, id, "de").unwrap();
	assert_eq!((live.title.as_str(), live.published), ("Neu", false));
	let draft = get_draft(&conn, id).unwrap().unwrap();
	assert!(draft.doc.values().all(|sd| sd.published));
	assert_eq!(draft.doc["de"].title, "Neuer");
	assert_eq!(draft.doc["de"].fields["intro"], "Hi");
}
