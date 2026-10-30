use super::api::*;
use super::*;
use crate::schema::tests::site;
use crate::testing::TestServer;

#[tokio::test]
async fn logs_in_and_out() {
	let ts = TestServer::new(site());
	add_user(&ts.state.db.conn(), "a@x.ch", "password1", None).unwrap();
	let anon = ts.anonymous();

	let login = |password: &str| LoginReq {
		email: "a@x.ch".into(),
		password: password.into(),
	};
	let err = anon.call(&login("wrong")).await.unwrap_err();
	assert_eq!(err, Error::LoginIncorrect);
	let unknown = LoginReq {
		email: "nobody@x.ch".into(),
		password: "password1".into(),
	};
	assert_eq!(
		anon.call(&unknown).await.unwrap_err(),
		Error::LoginIncorrect
	);

	let resp = anon.call(&login("password1")).await.unwrap();
	let mut user = ts.anonymous();
	user.token = Some(resp.token);
	user.call(&CheckReq {}).await.unwrap();

	user.call(&LogoutReq {}).await.unwrap();
	let err = user.call(&CheckReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	let err = anon.call(&CheckReq {}).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn too_many_wrong_passwords_wait() {
	let ts = TestServer::new(site());
	add_user(&ts.state.db.conn(), "c@x.ch", "password1", None).unwrap();
	let anon = ts.anonymous();
	let login = |password: &str| LoginReq {
		email: "c@x.ch".into(),
		password: password.into(),
	};
	for _ in 0..MAX_FAILS {
		let err = anon.call(&login("wrong")).await.unwrap_err();
		assert_eq!(err, Error::LoginIncorrect);
	}
	// also the right one
	let err = anon.call(&login("password1")).await.unwrap_err();
	assert!(matches!(err, Error::Request(m) if m.contains("15 minutes")));
}

#[tokio::test]
async fn old_sessions_end() {
	let ts = TestServer::new(site());
	let user = ts.admin();
	ts.state
		.db
		.conn()
		.execute(
			"UPDATE sessions SET created_at = ?1",
			[now() - SESSION_DURATION_MS - 1],
		)
		.unwrap();
	let err = user.call(&CheckReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
}
