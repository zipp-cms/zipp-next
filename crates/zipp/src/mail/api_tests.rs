use super::api::*;
use super::*;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::TestServer;

#[tokio::test]
async fn admins_send_a_test_mail_to_themselves() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let status = admin.call(&StatusReq {}).await.unwrap();
	assert!(status.configured);

	admin.call(&TestReq {}).await.unwrap();
	let sent = ts.mails();
	assert_eq!(sent.len(), 1);
	assert!(sent[0].to.contains('@') && sent[0].subject.contains("test"));

	let editor = ts.user(Some("editor"));
	assert_eq!(
		editor.call(&TestReq {}).await.unwrap_err(),
		Error::Forbidden
	);
}

#[test]
fn builds_a_mail_with_html() {
	let m = Mailer::default();
	let mail = Mail::new("Anna <anna@example.ch>", "Hi", "Hello")
		.html("<p>Hello</p>")
		.reply_to("b@example.ch");
	let raw = String::from_utf8(m.message(&mail).unwrap().formatted()).unwrap();
	assert!(raw.contains("To: Anna <anna@example.ch>"));
	assert!(raw.contains("Reply-To: b@example.ch"));
	assert!(raw.contains("text/html"));
	let bad = Mail::new("no address", "Hi", "Hello");
	assert!(matches!(m.message(&bad), Err(Error::Request(_))));
}

// the pool of the transport lives in a runtime
#[tokio::test]
async fn smtp_needs_a_sender_and_hides_the_password() {
	let m =
		Mailer::smtp("smtp://u:secret@mail.example.ch:587", "cms@example.ch")
			.unwrap();
	assert_eq!(m.server(), Some("mail.example.ch:587"));
	assert!(Mailer::smtp("smtp://localhost:1025", "not an address").is_err());
}
