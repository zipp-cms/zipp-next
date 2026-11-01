use serde_json::{Value, json};
use zipp::Schema;
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::http::{Request, StatusCode};
use zipp::testing::TestServer;

use crate::{FORM_SETTINGS, Form, FormField, Forms};

fn contact() -> Form {
	Form::new("contact", "Contact")
		.field(FormField::text("name").required())
		.field(FormField::email("email").required().reply_to())
		.field(FormField::text("phone"))
		.field(
			FormField::multiline("message")
				.required()
				.label("Your vision"),
		)
}

fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		.plugin(Forms::new().form(contact()))
}

/// The settings of the contact form, in every site.
fn settings(ts: &TestServer, to: &str, success: &str) {
	let fields = json!({ "contactTo": to, "contactSuccess": success });
	ts.state
		.db
		.conn()
		.execute(
			"UPDATE entry_sites SET fields = ?1 WHERE entry_id =
			(SELECT id FROM entries WHERE section = ?2)",
			[fields.to_string(), FORM_SETTINGS.into()],
		)
		.unwrap();
}

async fn post(
	ts: &TestServer,
	path: &str,
	body: Value,
	ip: &str,
) -> (StatusCode, Value) {
	let req = Request::post(path)
		.header("content-type", "application/json")
		.header("x-forwarded-for", ip)
		.body(Body::from(body.to_string()))
		.unwrap();
	let resp = ts.anonymous().send(req).await;
	let status = resp.status();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	(status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

fn valid() -> Value {
	json!({
		"name": " Anna ",
		"email": "anna@example.ch",
		"phone": "079 123 45 67",
		"message": "Fotos für unsere Hochzeit\nim Juni",
	})
}

#[tokio::test]
async fn a_submission_is_mailed() {
	let ts = TestServer::new(schema());
	settings(&ts, "luisa@example.ch, team@example.ch", "Danke!");

	let (status, body) =
		post(&ts, "/forms/contact?site=de", valid(), "1.1.1.1").await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(body, json!({ "success": true, "message": "Danke!" }));
	let mails = ts.mails();
	assert_eq!(mails.len(), 2);
	assert_eq!(mails[0].to, "luisa@example.ch");
	assert_eq!(mails[0].subject, "New message: Contact");
	assert_eq!(mails[0].reply_to.as_deref(), Some("anna@example.ch"));
	assert_eq!(
		mails[0].text,
		"Name: Anna\nEmail: anna@example.ch\nPhone: 079 123 45 67\n\n\
		Your vision:\nFotos für unsere Hochzeit\nim Juni\n"
	);
}

#[tokio::test]
async fn invalid_ones_and_bots_are_not() {
	let ts = TestServer::new(schema());
	settings(&ts, "luisa@example.ch", "");

	let bad = json!({ "name": "A\nB", "email": "no address", "message": "" });
	let (status, body) = post(&ts, "/forms/contact", bad, "2.2.2.2").await;
	assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
	let paths: Vec<&str> = body["errors"]
		.as_array()
		.unwrap()
		.iter()
		.map(|e| e["path"].as_str().unwrap())
		.collect();
	assert_eq!(paths, ["name", "email", "message"]);

	// the honeypot: it looks sent, nothing goes
	let mut bot = valid();
	bot["website"] = json!("https://spam.example");
	let (status, body) = post(&ts, "/forms/contact", bot, "2.2.2.2").await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(body["message"], "Thank you, your message is sent.");
	assert!(ts.mails().is_empty());

	assert_eq!(
		post(&ts, "/forms/other", valid(), "2.2.2.2").await.0,
		StatusCode::NOT_FOUND
	);
}

#[tokio::test]
async fn an_address_may_send_a_few() {
	let ts = TestServer::new(schema());
	settings(&ts, "luisa@example.ch", "");
	for _ in 0..5 {
		let (status, _) = post(&ts, "/forms/contact", valid(), "3.3.3.3").await;
		assert_eq!(status, StatusCode::OK);
	}
	let (status, _) = post(&ts, "/forms/contact", valid(), "3.3.3.3").await;
	assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
	// another one still may
	let (status, _) = post(&ts, "/forms/contact", valid(), "4.4.4.4").await;
	assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn without_a_recipient_it_says_so() {
	let ts = TestServer::new(schema());
	let (status, body) = post(&ts, "/forms/contact", valid(), "5.5.5.5").await;
	assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
	assert_eq!(body["success"], false);
}

#[test]
fn mistakes_of_the_code() {
	let twice = Forms::new().form(contact()).form(contact());
	let s = Schema::new().site("de", "Deutsch", "de");
	assert!(zipp::Plugin::check(&twice, &s).is_err());
	let reply = Forms::new()
		.form(Form::new("f", "F").field(FormField::text("a").reply_to()));
	assert!(zipp::Plugin::check(&reply, &s).is_err());
	let honey = Forms::new()
		.form(Form::new("f", "F").field(FormField::text("website")));
	assert!(zipp::Plugin::check(&honey, &s).is_err());
}
