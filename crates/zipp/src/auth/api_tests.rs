use super::api::*;
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

	// the session and its user, for 30 days
	let auth = anon.call(&login("a@x.ch", "password1")).await.unwrap();
	assert_eq!(auth.user.email, "a@x.ch");
	assert_eq!(auth.user.role, None);
	let left = auth.session.expires_at - now();
	assert!(left > 29 * DAY && left <= 30 * DAY, "{left}");

	let user = with_token(&ts, &auth);
	assert_eq!(user.call(&SessionReq {}).await.unwrap(), auth);

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
