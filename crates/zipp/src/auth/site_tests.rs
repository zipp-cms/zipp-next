use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};

use super::api::*;
use super::throttle::MAX_FAILS;
use super::*;
use crate::SiteUrls;
use crate::fixtures::site_with_roles;
use crate::schema::Role;
use crate::testing::TestServer;
use crate::users::api::{GetReq, LoginAsLinkReq, PasswordLinkReq};

/// The test site with members: they edit events, but not in the studio.
fn site() -> crate::Schema {
	site_with_roles().role(
		Role::new("member", "Member")
			.section("events", Access::Edit)
			.no_studio(),
	)
}

/// A website route of a plugin, who is logged in.
async fn whoami(user: SiteUser) -> Json<SiteUser> {
	Json(user)
}

/// The test server with the website's url and the route above.
fn server() -> TestServer {
	let urls =
		SiteUrls::from([("de".to_string(), "https://x.ch/".to_string())]);
	TestServer::new(site())
		.site_urls(urls)
		.routes(Router::new().route("/whoami", get(whoami)))
}

fn add(ts: &TestServer, email: &str, role: Option<&str>) -> i64 {
	let conn = ts.state.db.conn();
	add_user(&conn, email, "password1", role).unwrap();
	conn.query_row("SELECT id FROM users WHERE email = ?1", [email], |r| {
		r.get(0)
	})
	.unwrap()
}

fn site_login(email: &str, password: &str) -> SiteLoginReq {
	SiteLoginReq {
		email: email.into(),
		password: password.into(),
	}
}

/// The status of `GET /whoami` with the client's token.
async fn whoami_status(client: &crate::testing::Client) -> u16 {
	let req = axum::http::Request::get("/whoami")
		.body(axum::body::Body::empty())
		.unwrap();
	client.send(req).await.status().as_u16()
}

/// The mails of the background, a moment later.
async fn mails(ts: &TestServer) -> Vec<crate::mail::Mail> {
	tokio::time::sleep(std::time::Duration::from_millis(50)).await;
	ts.mails()
}

/// The token of a website link in a text.
fn token_in(text: &str) -> String {
	let prefix = "https://x.ch/reset-password?token=";
	let start = text.find(prefix).unwrap() + prefix.len();
	text[start..].split_whitespace().next().unwrap().to_string()
}

#[tokio::test]
async fn members_stay_out_of_the_studio() {
	let ts = server();
	let id = add(&ts, "m@x.ch", Some("member"));

	// the password is right, the studio says why
	let login = LoginReq {
		email: "m@x.ch".into(),
		password: "password1".into(),
	};
	let err = ts.anonymous().call(&login).await.unwrap_err();
	assert!(matches!(&err, Error::Request(m) if m.contains("website")));

	// a session of the studio from before the role is refused
	let token = new_session(&ts.state.db.conn(), id).unwrap().token;
	let member = ts.with_token(&token);
	let err = member.call(&GetReq { id }).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	let err = member.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);

	// an author still logs in
	add(&ts, "a@x.ch", Some("author"));
	ts.login("a@x.ch", "password1").await;
}

#[tokio::test]
async fn logs_in_on_the_website() {
	let ts = server();
	let id = add(&ts, "m@x.ch", Some("member"));
	let anon = ts.visitor(None);

	let err = anon.call(&site_login("m@x.ch", "wrong")).await.unwrap_err();
	assert_eq!(err, Error::LoginIncorrect);
	let err = anon.call(&site_login("no@x.ch", "x")).await.unwrap_err();
	assert_eq!(err, Error::LoginIncorrect);
	assert_eq!(whoami_status(&anon).await, 401);

	let auth = anon.call(&site_login("m@x.ch", "password1")).await.unwrap();
	assert_eq!(auth.user.id, id);
	assert_eq!(auth.user.role.as_deref(), Some("member"));
	let left = auth.session.expires_at - now();
	assert!(left > 29 * crate::db::DAY_MS, "{left}");

	let member = ts.visitor(Some(&auth.session.token));
	assert_eq!(member.call(&SiteMeReq {}).await.unwrap(), auth);
	assert_eq!(whoami_status(&member).await, 200);

	// a site session renews like one of the studio
	ts.state
		.db
		.conn()
		.execute(
			"UPDATE sessions SET expires_at = ?1",
			[now() + crate::db::DAY_MS],
		)
		.unwrap();
	let me = member.call(&SiteMeReq {}).await.unwrap();
	assert!(me.session.expires_at - now() > 29 * crate::db::DAY_MS);

	member.call(&SiteLogoutReq {}).await.unwrap();
	let err = member.call(&SiteMeReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	assert_eq!(whoami_status(&member).await, 403);

	// admins log in on the website too
	add(&ts, "admin@x.ch", None);
	let auth = anon.call(&site_login("admin@x.ch", "password1")).await;
	assert_eq!(auth.unwrap().user.role, None);
}

#[tokio::test]
async fn site_and_studio_tokens_are_apart() {
	let ts = server();
	let id = add(&ts, "a@x.ch", None);

	// a site token doesn't open the studio
	let site = ts.site_user(None);
	let token = site.token.clone().unwrap();
	let studio = ts.with_token(&token);
	let err = studio.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	let err = studio.call(&GetReq { id }).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	// and the studio's logout doesn't end it
	studio.call(&LogoutReq {}).await.unwrap();
	site.call(&SiteMeReq {}).await.unwrap();

	// a studio token isn't a site token
	let studio = ts.login("a@x.ch", "password1").await;
	let token = studio.token.clone().unwrap();
	let visitor = ts.visitor(Some(&token));
	let err = visitor.call(&SiteMeReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	assert_eq!(whoami_status(&visitor).await, 403);
	visitor.call(&SiteLogoutReq {}).await.unwrap();
	studio.call(&SessionReq {}).await.unwrap();
}

#[tokio::test]
async fn wrong_passwords_on_the_website_wait() {
	let ts = server();
	add(&ts, "m@x.ch", Some("member"));
	let anon = ts.visitor(None);

	for _ in 0..MAX_FAILS {
		let err = anon.call(&site_login("m@x.ch", "wrong")).await;
		assert_eq!(err.unwrap_err(), Error::LoginIncorrect);
	}
	// the right one waits too, and the studio counts the same
	let err = anon.call(&site_login("m@x.ch", "password1")).await;
	assert!(matches!(err.unwrap_err(), Error::Request(m) if m.contains("15")));
	let login = LoginReq {
		email: "m@x.ch".into(),
		password: "password1".into(),
	};
	let err = ts.anonymous().call(&login).await;
	assert!(matches!(err.unwrap_err(), Error::Request(m) if m.contains("15")));
}

#[tokio::test]
async fn a_password_is_reset_on_the_website() {
	let ts = server();
	add(&ts, "m@x.ch", Some("member"));
	let anon = ts.visitor(None);
	let old = anon.call(&site_login("m@x.ch", "password1")).await.unwrap();
	let old = ts.visitor(Some(&old.session.token));
	let reset = |email: &str| SiteResetReq {
		email: email.into(),
		site: None,
	};

	// the same answer for an unknown email, no mail
	anon.call(&reset("no@x.ch")).await.unwrap();
	assert!(mails(&ts).await.is_empty());
	anon.call(&reset("m@x.ch")).await.unwrap();
	let sent = mails(&ts).await;
	assert_eq!(sent.len(), 1);
	assert_eq!(sent[0].to, "m@x.ch");
	let token = token_in(&sent[0].text);

	let set = |password: &str| SitePasswordReq {
		token: token.clone(),
		password: password.into(),
	};
	let err = anon.call(&set("short")).await.unwrap_err();
	assert!(matches!(err, Error::Invalid(_)));
	let auth = anon.call(&set("password2")).await.unwrap();
	assert_eq!(auth.user.email, "m@x.ch");
	ts.visitor(Some(&auth.session.token))
		.call(&SiteMeReq {})
		.await
		.unwrap();

	// once, and the old session is gone
	let err = anon.call(&set("password3")).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
	assert!(old.call(&SiteMeReq {}).await.is_err());
	anon.call(&site_login("m@x.ch", "password2")).await.unwrap();
}

#[tokio::test]
async fn reset_needs_the_url_of_the_website() {
	let ts = TestServer::new(site());
	add(&ts, "m@x.ch", Some("member"));
	let anon = ts.visitor(None);

	// known or not, the same error
	let reset = |email: &str| SiteResetReq {
		email: email.into(),
		site: None,
	};
	let known = anon.call(&reset("m@x.ch")).await.unwrap_err();
	let unknown = anon.call(&reset("no@x.ch")).await.unwrap_err();
	assert!(matches!(&known, Error::Request(m) if m.contains("site-url")));
	assert_eq!(known, unknown);
	assert!(mails(&ts).await.is_empty());
}

#[tokio::test]
async fn members_are_invited_to_the_website() {
	let ts = server();
	let admin = ts.admin();
	let id = {
		let conn = ts.state.db.conn();
		crate::users::create(&conn, "m@x.ch", "", "", Some("member"), "")
			.unwrap()
	};

	let link = admin
		.call(&PasswordLinkReq { id, send: true })
		.await
		.unwrap();
	assert_eq!(link.kind, links::Kind::Invite);
	assert!(link.url.starts_with("https://x.ch/reset-password?token="));
	let sent = ts.mails();
	assert_eq!(sent.len(), 1);
	assert!(sent[0].text.contains(&link.url));
	assert!(!sent[0].text.contains("studio"));
	let token = token_in(&link.url);

	// the studio's page doesn't set it
	let studio = SetPasswordReq {
		token: token.clone(),
		password: "password2".into(),
		name: None,
	};
	let err = ts.anonymous().call(&studio).await.unwrap_err();
	assert!(matches!(&err, Error::Request(m) if m.contains("website")));
	let set = SitePasswordReq {
		token,
		password: "password2".into(),
	};
	let auth = ts.visitor(None).call(&set).await.unwrap();
	assert_eq!(auth.user.id, id);

	// and no logging in as them
	let err = admin.call(&LoginAsLinkReq { id }).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
}

#[tokio::test]
async fn a_reset_link_leads_where_the_user_logs_in() {
	let ts = server();
	add(&ts, "m@x.ch", Some("member"));
	add(&ts, "a@x.ch", Some("author"));
	let anon = ts.visitor(None);

	// a member who asks the studio gets the website's link
	let forgot = ForgotReq {
		email: "m@x.ch".into(),
	};
	ts.anonymous().call(&forgot).await.unwrap();
	let sent = mails(&ts).await;
	assert_eq!(sent.len(), 1);
	assert!(sent[0].text.contains("https://x.ch/reset-password?token="));

	// a user of the studio who asks the website gets the studio's
	let reset = SiteResetReq {
		email: "a@x.ch".into(),
		site: None,
	};
	anon.call(&reset).await.unwrap();
	let sent = mails(&ts).await;
	assert_eq!(sent.len(), 2);
	assert!(sent[1].text.contains("http://zipp.test/admin/password/"));
}

/// `PUT /site-auth/me` with any JSON, its status and answer.
async fn put_me(client: &crate::testing::Client, body: Value) -> (u16, Value) {
	let req = axum::http::Request::put("/site-auth/me")
		.header(axum::http::header::CONTENT_TYPE, "application/json")
		.body(axum::body::Body::from(body.to_string()))
		.unwrap();
	let resp = client.send(req).await;
	let status = resp.status().as_u16();
	let body = crate::testing::Client::bytes(resp).await;
	(status, serde_json::from_slice(&body).unwrap_or_default())
}

#[tokio::test]
async fn site_users_edit_their_profile() {
	let schema = site().user_fields([
		crate::schema::Field::text("bio").multiline(),
		crate::schema::Field::text("city").max(10),
	]);
	let ts = TestServer::new(schema);
	let member = ts.site_user(Some("member"));

	let (status, me) = put_me(
		&member,
		json!({ "name": "Mia", "fields": { "bio": "Cook" } }),
	)
	.await;
	assert_eq!(status, 200, "{me}");
	assert_eq!(me["name"], "Mia");
	assert_eq!(me["fields"], json!({ "bio": "Cook" }));
	let slug = me["slug"].clone();
	assert_eq!(slug, "mia");
	// the fields it sends, the others stay
	let (_, me) =
		put_me(&member, json!({ "fields": { "city": "Bern" } })).await;
	assert_eq!(me["fields"], json!({ "bio": "Cook", "city": "Bern" }));
	assert_eq!(me["name"], "Mia");

	// checked like in the studio
	let too_long = json!({ "fields": { "city": "Ober-Unterdorf" } });
	let (status, err) = put_me(&member, too_long).await;
	assert_eq!(status, 400);
	assert_eq!(err["error"]["detail"][0]["path"], "fields.city", "{err}");
	let (status, _) = put_me(&member, json!({ "name": " " })).await;
	assert_eq!(status, 400);
	// a public profile stays small
	let (status, _) = put_me(&member, json!({ "name": "x".repeat(101) })).await;
	assert_eq!(status, 400);
	let big = json!({ "fields": { "bio": "x".repeat(70_000) } });
	let (status, err) = put_me(&member, big).await;
	assert_eq!(status, 400);
	assert_eq!(err["error"]["detail"][0]["path"], "fields", "{err}");

	// slug, email and role are an admin's
	for body in [
		json!({ "slug": "queen" }),
		json!({ "email": "x@x.ch" }),
		json!({ "role": null }),
	] {
		let (status, _) = put_me(&member, body).await;
		assert_eq!(status, 400);
	}
	let me = member.call(&SiteMeReq {}).await.unwrap().user;
	assert_eq!(
		(me.slug, me.role.as_deref()),
		("mia".into(), Some("member"))
	);

	// only with a site session, never a studio's
	let (status, _) = put_me(&ts.visitor(None), json!({ "name": "X" })).await;
	assert_eq!(status, 401);
	let studio = ts.user(Some("member"));
	let req = axum::http::Request::put("/site-auth/me")
		.header(axum::http::header::CONTENT_TYPE, "application/json")
		.body(axum::body::Body::from(r#"{"name":"X"}"#))
		.unwrap();
	assert_eq!(studio.send(req).await.status().as_u16(), 401);
}
