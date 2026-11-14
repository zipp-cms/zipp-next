use std::net::SocketAddr;

use serde_json::{Value, json};
use zipp::Schema;
use zipp::axum::body::{Body, to_bytes};
use zipp::axum::extract::ConnectInfo;
use zipp::axum::http::{Request, StatusCode};
use zipp::entries::Entry;
use zipp::entries::api::{CreateReq, UpdateReq};
use zipp::testing::{TestServer, input};

use crate::form::{FieldKind, Form, handle_of};
use crate::{FORM, FORMS, Forms, submissions};

pub(crate) fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.site("en", "English", "en")
		.plugin(Forms::new())
}

pub(crate) fn block(ty: &str, label: &str, more: Value) -> Value {
	let mut fields = json!({ "label": label });
	fields
		.as_object_mut()
		.unwrap()
		.extend(more.as_object().unwrap().clone());
	json!({ "type": ty, "fields": fields })
}

/// The contact form in German, live in `de`.
pub(crate) fn contact_fields() -> Value {
	json!({
		"fields": [
			block("formText", "Name", json!({ "required": true })),
			block(
				"formEmail",
				"E-Mail",
				json!({ "required": true, "handle": "email" }),
			),
			block("formPhone", "Telefon", json!({})),
			block(
				"formTextarea",
				"Nachricht",
				json!({ "required": true, "placeholder": "Ihre Nachricht" }),
			),
		],
		"successMessage": "Danke!",
	})
}

pub(crate) async fn contact(ts: &TestServer) -> Entry {
	let input = input(FORMS, FORM, "Kontakt", contact_fields());
	ts.admin().call(&CreateReq(input)).await.unwrap()
}

pub(crate) async fn post(
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

async fn get(ts: &TestServer, path: &str) -> (StatusCode, Value) {
	let resp = ts.anonymous().get(path).await;
	let status = resp.status();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	(status, serde_json::from_slice(&body).unwrap_or(Value::Null))
}

pub(crate) fn valid() -> Value {
	json!({
		"name": " Anna ",
		"email": "anna@example.ch",
		"telefon": "079 123 45 67",
		"nachricht": "Fotos für unsere Hochzeit\nim Juni",
	})
}

#[tokio::test]
async fn the_website_gets_a_live_form_of_its_site() {
	let ts = TestServer::new(schema());
	let form = contact(&ts).await;

	let (status, body) = get(&ts, &format!("/forms/{}?site=de", form.id)).await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(
		body,
		json!({
			"id": form.id,
			"title": "Kontakt",
			"fields": [
				{
					"type": "text", "handle": "name", "label": "Name",
					"required": true, "placeholder": "", "instructions": "",
				},
				{
					"type": "email", "handle": "email", "label": "E-Mail",
					"required": true, "placeholder": "", "instructions": "",
				},
				{
					"type": "phone", "handle": "telefon", "label": "Telefon",
					"required": false, "placeholder": "", "instructions": "",
				},
				{
					"type": "textarea", "handle": "nachricht",
					"label": "Nachricht", "required": true,
					"placeholder": "Ihre Nachricht", "instructions": "",
				},
			],
			"submitLabel": "Send",
			"successMessage": "Danke!",
			"honeypot": "website",
		})
	);

	assert_eq!(get(&ts, "/forms/999").await.0, StatusCode::NOT_FOUND);

	// its own fields in English
	let en = json!({
		"fields": [block("formText", "Your name", json!({ "handle": "name" }))],
		"submitLabel": "Go",
	});
	let mut en = input(FORMS, FORM, "Contact", en);
	en.site = Some("en".into());
	let req = UpdateReq {
		id: form.id,
		input: en,
	};
	ts.admin().call(&req).await.unwrap();
	let path = format!("/forms/{}?site=en", form.id);
	let (status, body) = get(&ts, &path).await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(body["title"], "Contact");
	assert_eq!(body["fields"][0]["label"], "Your name");
	assert_eq!(body["submitLabel"], "Go");
	assert_eq!(body["successMessage"], "Thank you, your message is sent.");

	// offline in a site: not there
	let mut off = input(FORMS, FORM, "Contact", json!({}));
	off.site = Some("en".into());
	off.published = false;
	let req = UpdateReq {
		id: form.id,
		input: off,
	};
	ts.admin().call(&req).await.unwrap();
	assert_eq!(get(&ts, &path).await.0, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn a_submission_is_stored() {
	let ts = TestServer::new(schema());
	let form = contact(&ts).await;

	let path = format!("/forms/{}?site=de", form.id);
	let (status, body) = post(&ts, &path, valid(), "1.1.1.1").await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(body, json!({ "success": true, "message": "Danke!" }));

	let conn = ts.state.db.conn();
	let (list, total) = submissions::list(&conn, "de", None, 0).unwrap();
	assert_eq!(total, 1);
	assert_eq!(list[0].form_id, form.id);
	assert_eq!(list[0].form, "Kontakt");
	assert_eq!(list[0].site, "de");
	let answers: Vec<(&str, &str, &str)> = list[0]
		.answers
		.iter()
		.map(|a| (a.handle.as_str(), a.label.as_str(), a.value.as_str()))
		.collect();
	assert_eq!(
		answers,
		[
			("name", "Name", "Anna"),
			("email", "E-Mail", "anna@example.ch"),
			("telefon", "Telefon", "079 123 45 67"),
			(
				"nachricht",
				"Nachricht",
				"Fotos für unsere Hochzeit\nim Juni"
			),
		]
	);
}

#[tokio::test]
async fn invalid_ones_and_bots_are_not() {
	let ts = TestServer::new(schema());
	let form = contact(&ts).await;
	let path = format!("/forms/{}", form.id);

	let bad = json!({
		"name": "A\nB",
		"email": "no address",
		"telefon": "call me",
		"nachricht": "",
	});
	let (status, body) = post(&ts, &path, bad, "2.2.2.2").await;
	assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
	let errors: Vec<(&str, &str)> = body["errors"]
		.as_array()
		.unwrap()
		.iter()
		.map(|e| (e["path"].as_str().unwrap(), e["code"].as_str().unwrap()))
		.collect();
	assert_eq!(
		errors,
		[
			("name", "oneLine"),
			("email", "email"),
			("telefon", "phone"),
			("nachricht", "required"),
		]
	);

	// the honeypot: it looks sent, nothing is stored
	let mut bot = valid();
	bot["website"] = json!("https://spam.example");
	let (status, body) = post(&ts, &path, bot, "2.2.2.2").await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(body["message"], "Danke!");
	let stored = submissions::list(&ts.state.db.conn(), "de", None, 0);
	assert_eq!(stored.unwrap().1, 0);

	assert_eq!(
		post(&ts, "/forms/999", valid(), "2.2.2.2").await.0,
		StatusCode::NOT_FOUND
	);
}

#[tokio::test]
async fn an_address_may_send_a_few() {
	let ts = TestServer::new(schema()).trusted_proxy("127.0.0.1");
	let path = format!("/forms/{}", contact(&ts).await.id);
	for _ in 0..5 {
		let (status, _) = post(&ts, &path, valid(), "3.3.3.3").await;
		assert_eq!(status, StatusCode::OK);
	}
	let (status, _) = post(&ts, &path, valid(), "3.3.3.3").await;
	assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
	// what the visitor wrote before the proxy's hop doesn't count
	let spoofed = "8.8.8.8, 3.3.3.3";
	let (status, _) = post(&ts, &path, valid(), spoofed).await;
	assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
	// another one still may
	let (status, _) = post(&ts, &path, valid(), "4.4.4.4").await;
	assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn a_visitor_s_header_doesn_t_count() {
	let ts = TestServer::new(schema());
	let path = format!("/forms/{}", contact(&ts).await.id);
	// straight from the internet, not through a proxy
	let send = async |forwarded: &str| {
		let mut req = Request::post(&path)
			.header("content-type", "application/json")
			.header("x-forwarded-for", forwarded)
			.body(Body::from(valid().to_string()))
			.unwrap();
		let peer: SocketAddr = "9.9.9.9:1".parse().unwrap();
		req.extensions_mut().insert(ConnectInfo(peer));
		ts.anonymous().send(req).await.status()
	};
	for i in 0..5 {
		assert_eq!(send(&format!("6.6.6.{i}")).await, StatusCode::OK);
	}
	assert_eq!(send("6.6.6.9").await, StatusCode::TOO_MANY_REQUESTS);
}

/// An entry of `forms` with these fields, not saved.
fn entry(fields: Value) -> Entry {
	serde_json::from_value(json!({
		"id": 1, "section": FORMS, "type": FORM, "site": "de",
		"title": "Form", "slug": "form", "uri": null, "published": true,
		"parentId": null, "fields": fields, "createdAt": 0, "updatedAt": 0,
		"authorId": null,
	}))
	.unwrap()
}

#[test]
fn handles_are_unique_and_off_blocks_left_out() {
	let mut off = block("formText", "Hidden", json!({}));
	off["enabled"] = json!(false);
	let form = Form::of_entry(
		&entry(json!({ "fields": [
			block("formText", "Website", json!({})),
			block("formText", "Vorname", json!({ "handle": "name" })),
			block("formText", "Name", json!({})),
			off,
			block("formText", "!!", json!({})),
		]})),
		"website",
	);
	let handles: Vec<&str> =
		form.fields.iter().map(|f| f.handle.as_str()).collect();
	assert_eq!(handles, ["website", "name", "name2", "2"]);
	assert!(form.fields.iter().all(|f| f.kind == FieldKind::Text));
	// the honeypot gets out of the way
	assert_eq!(form.honeypot, "website_");
}

#[test]
fn handles_of_labels() {
	assert_eq!(handle_of("E-Mail Adresse"), "eMailAdresse");
	assert_eq!(handle_of("Ihre Straße"), "ihreStraße");
	assert_eq!(handle_of("  "), "");
}
