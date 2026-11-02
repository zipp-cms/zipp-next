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
