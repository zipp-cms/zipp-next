use std::net::SocketAddr;

use axum::body::{Body, to_bytes};
use axum::extract::ConnectInfo;
use axum::http::{Request, header};

use super::api::*;
use super::throttle::MAX_FAILS;
use super::*;
use crate::schema::tests::site;
use crate::testing::{Client, TestServer};

const DAY: i64 = 1000 * 60 * 60 * 24;

fn login(email: &str, password: &str) -> LoginReq {
	LoginReq {
		email: email.into(),
		password: password.into(),
	}
}

/// A client with the token of a login.
fn with_token(ts: &TestServer, auth: &Authenticated) -> Client {
	let mut c = ts.anonymous();
	c.token = Some(auth.session.token.clone());
	c
}

#[tokio::test]
async fn logs_in_and_out() {
	let ts = TestServer::new(site());
	add_user(&ts.state.db.conn(), "a@x.ch", "password1", None).unwrap();
	let anon = ts.anonymous();

	let err = anon.call(&login("a@x.ch", "wrong")).await.unwrap_err();
	assert_eq!(err, Error::LoginIncorrect);
	let err = anon
		.call(&login("nobody@x.ch", "password1"))
		.await
		.unwrap_err();
	assert_eq!(err, Error::LoginIncorrect);

	// the email in any case
	anon.call(&login("A@X.ch", "password1")).await.unwrap();

	// the session and its user, for 30 days
	let auth = anon.call(&login("a@x.ch", "password1")).await.unwrap();
	assert_eq!(auth.user.email, "a@x.ch");
	assert_eq!(auth.user.role, None);
	let left = auth.session.expires_at - now();
	assert!(left > 29 * DAY && left <= 30 * DAY, "{left}");

	let user = with_token(&ts, &auth);
	assert_eq!(user.call(&SessionReq {}).await.unwrap(), auth);

	// the database keeps only the hash of the token
	let stored: Vec<String> = {
		let conn = ts.state.db.conn();
		let mut stmt = conn.prepare("SELECT hash FROM sessions").unwrap();
		stmt.query_map([], |r| r.get(0))
			.unwrap()
			.collect::<rusqlite::Result<_>>()
			.unwrap()
	};
	assert!(stored.contains(&token_hash(&auth.session.token)));
	assert!(!stored.contains(&auth.session.token));

	user.call(&LogoutReq {}).await.unwrap();
	let err = user.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	let err = anon.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn using_the_studio_renews_the_session() {
	let ts = TestServer::new(site());
	add_user(&ts.state.db.conn(), "r@x.ch", "password1", None).unwrap();
	let auth = ts
		.anonymous()
		.call(&login("r@x.ch", "password1"))
		.await
		.unwrap();
	let user = with_token(&ts, &auth);
	let set_left = |left: i64| {
		ts.state
			.db
			.conn()
			.execute("UPDATE sessions SET expires_at = ?1", [now() + left])
			.unwrap();
	};

	// with 20 days left nothing changes
	set_left(20 * DAY);
	let s = user.call(&SessionReq {}).await.unwrap().session;
	assert!(s.expires_at - now() < 21 * DAY);

	// with 5 days left it gets 30 again
	set_left(5 * DAY);
	let s = user.call(&SessionReq {}).await.unwrap().session;
	assert!(s.expires_at - now() > 29 * DAY);
	assert_eq!(s.token, auth.session.token);

	// an ended one is gone
	set_left(-1);
	let err = user.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	assert_eq!(
		ts.admin().call(&SessionReq {}).await.unwrap().user.role,
		None
	);
}

#[tokio::test]
async fn short_sessions_aren_t_renewed() {
	let ts = TestServer::new(site());
	add_user(&ts.state.db.conn(), "s@x.ch", "password1", None).unwrap();
	let id = crate::users::list(&ts.state.db.conn()).unwrap()[0].user.id;
	let short = short_session(&ts.state.db.conn(), id, DAY).unwrap();
	let mut user = ts.anonymous();
	user.token = Some(short.token.clone());

	// it works, but the studio opening doesn't push it back
	let s = user.call(&SessionReq {}).await.unwrap().session;
	assert_eq!(s.expires_at, short.expires_at);
	end_session(&ts.state.db.conn(), &short.token).unwrap();
	let err = user.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
}

#[tokio::test]
async fn a_new_password_ends_every_session() {
	let ts = TestServer::new(site());
	add_user(&ts.state.db.conn(), "p@x.ch", "password1", None).unwrap();
	let anon = ts.anonymous();
	let first = anon.call(&login("p@x.ch", "password1")).await.unwrap();
	let second = anon.call(&login("p@x.ch", "password1")).await.unwrap();
	assert_ne!(first.session.token, second.session.token);

	add_user(&ts.state.db.conn(), "p@x.ch", "password2", None).unwrap();
	for auth in [first, second] {
		let err = with_token(&ts, &auth).call(&SessionReq {}).await;
		assert_eq!(err.unwrap_err(), Error::InvalidSessionToken);
	}
}

#[tokio::test]
async fn too_many_wrong_passwords_wait() {
	let ts = TestServer::new(site());
	add_user(&ts.state.db.conn(), "c@x.ch", "password1", None).unwrap();
	let anon = ts.anonymous();
	for _ in 0..MAX_FAILS {
		let err = anon.call(&login("c@x.ch", "wrong")).await.unwrap_err();
		assert_eq!(err, Error::LoginIncorrect);
	}
	// also the right one
	let err = anon.call(&login("c@x.ch", "password1")).await.unwrap_err();
	assert!(matches!(err, Error::Request(m) if m.contains("15 minutes")));
}

/// A login from a peer address, with `X-Forwarded-For` or not; the error
/// type or `OK`.
async fn login_from(
	ts: &TestServer,
	peer: &str,
	forwarded: Option<&str>,
	password: &str,
) -> String {
	let body = serde_json::to_vec(&login("d@x.ch", password)).unwrap();
	let mut req = Request::post("/api/auth/login")
		.header(header::CONTENT_TYPE, "application/json")
		.body(Body::from(body))
		.unwrap();
	let peer: SocketAddr = peer.parse().unwrap();
	req.extensions_mut().insert(ConnectInfo(peer));
	if let Some(f) = forwarded {
		req.headers_mut()
			.insert("x-forwarded-for", f.parse().unwrap());
	}
	let resp = ts.anonymous().send(req).await;
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
	v["error"]["type"].as_str().unwrap_or("OK").to_string()
}

#[tokio::test]
async fn others_can_t_lock_a_user_out() {
	let ts = TestServer::new(site()).trusted_proxy("127.0.0.1");
	add_user(&ts.state.db.conn(), "d@x.ch", "password1", None).unwrap();

	// someone guesses from their address until it waits
	for _ in 0..MAX_FAILS {
		let err = login_from(&ts, "9.9.9.9:1", None, "wrong").await;
		assert_eq!(err, "LOGIN_INCORRECT");
	}
	let err = login_from(&ts, "9.9.9.9:2", None, "password1").await;
	assert_eq!(err, "REQUEST");
	// a header they write doesn't make them someone else
	let spoofed = Some("1.2.3.4");
	let err = login_from(&ts, "9.9.9.9:3", spoofed, "password1").await;
	assert_eq!(err, "REQUEST");

	// the user logs in from elsewhere, also behind a proxy, which adds
	// the address last
	let ok = login_from(&ts, "8.8.8.8:1", None, "password1").await;
	assert_eq!(ok, "OK");
	let proxied = Some("9.9.9.9, 7.7.7.7");
	let ok = login_from(&ts, "127.0.0.1:1", proxied, "password1").await;
	assert_eq!(ok, "OK");
}

#[tokio::test]
async fn long_emails_aren_t_counted() {
	let ts = TestServer::new(site());
	let long = format!("{}@x.ch", "a".repeat(1000));
	let err = ts.anonymous().call(&login(&long, "x")).await.unwrap_err();
	assert_eq!(err, Error::LoginIncorrect);
}
