use super::api::*;
use crate::auth::api::{LoginReq, SessionReq};
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::{Client, TestServer};

fn new_user(email: &str, role: Option<&str>) -> CreateReq {
	CreateReq {
		email: email.into(),
		name: "Neu".into(),
		password: "password1".into(),
		role: role.map(Into::into),
	}
}

/// What the account page sends, unchanged.
fn unchanged(a: &super::Account) -> UpdateReq {
	UpdateReq {
		id: a.user.id,
		email: a.user.email.clone(),
		name: a.user.name.clone(),
		role: a.user.role.clone(),
		password: None,
		current_password: None,
	}
}

/// A client logged in with an email and password.
async fn login(ts: &TestServer, email: &str, password: &str) -> Client {
	let login = LoginReq {
		email: email.into(),
		password: password.into(),
	};
	let mut client = ts.anonymous();
	client.token =
		Some(ts.anonymous().call(&login).await.unwrap().session.token);
	client
}

fn paths(err: Error) -> Vec<String> {
	match err {
		Error::Invalid(fields) => fields.into_iter().map(|f| f.path).collect(),
		e => panic!("not invalid: {e:?}"),
	}
}

#[tokio::test]
async fn admins_manage_users() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();

	let created = admin
		.call(&new_user("new@x.ch", Some("author")))
		.await
		.unwrap();
	assert_eq!(created.user.name, "Neu");
	assert_eq!(created.last_login_at, None);
	let users = admin.call(&ListReq {}).await.unwrap();
	assert!(users.iter().any(|u| u.user.email == "new@x.ch"));

	// the new user logs in: the time and a session show
	let new_user = login(&ts, "new@x.ch", "password1").await;
	let id = created.user.id;
	let account = admin.call(&GetReq { id }).await.unwrap();
	assert!(account.last_login_at.is_some());
	assert_eq!(account.sessions, 1);

	// name, email and role
	let mut change = unchanged(&account);
	change.name = "Anna".into();
	change.email = "anna@x.ch".into();
	change.role = Some("editor".into());
	let changed = admin.call(&change).await.unwrap();
	assert_eq!(changed.user.name, "Anna");
	assert_eq!(changed.user.role.as_deref(), Some("editor"));

	// a new password from an admin logs the user out
	change.password = Some("password2".into());
	admin.call(&change).await.unwrap();
	let err = new_user.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	let again = login(&ts, "anna@x.ch", "password2").await;

	// logged out everywhere, then deleted
	admin.call(&EndSessionsReq { id }).await.unwrap();
	let err = again.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	admin.call(&DeleteReq { id }).await.unwrap();
	let err = admin.call(&DeleteReq { id }).await.unwrap_err();
	assert_eq!(err, Error::NotFound);
}

#[tokio::test]
async fn users_change_their_own_account() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let me = editor.call(&SessionReq {}).await.unwrap().user;
	let account = editor.call(&GetReq { id: me.id }).await.unwrap();
	let other = ts.user(Some("editor"));

	let mut change = unchanged(&account);
	change.name = "Eddie".into();
	let changed = editor.call(&change).await.unwrap();
	assert_eq!(changed.user.name, "Eddie");

	// not the own role
	let mut promote = unchanged(&changed);
	promote.role = None;
	let err = editor.call(&promote).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));

	// a new password needs the current one
	change.password = Some("password2".into());
	let err = editor.call(&change).await.unwrap_err();
	assert_eq!(paths(err), ["currentPassword"]);
	change.current_password = Some("wrong".into());
	let err = editor.call(&change).await.unwrap_err();
	assert_eq!(paths(err), ["currentPassword"]);

	// with it the other sessions end, this one stays
	let elsewhere = login(&ts, &me.email, "password").await;
	change.current_password = Some("password".into());
	editor.call(&change).await.unwrap();
	editor.call(&SessionReq {}).await.unwrap();
	let err = elsewhere.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);

	// logging out everywhere else keeps this session too
	let elsewhere = login(&ts, &me.email, "password2").await;
	editor.call(&EndSessionsReq { id: me.id }).await.unwrap();
	editor.call(&SessionReq {}).await.unwrap();
	assert!(elsewhere.call(&SessionReq {}).await.is_err());

	// nobody else's
	let them = other.call(&SessionReq {}).await.unwrap().user.id;
	let err = editor.call(&GetReq { id: them }).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	let mut theirs = unchanged(&changed);
	theirs.id = them;
	assert_eq!(editor.call(&theirs).await.unwrap_err(), Error::Forbidden);
	let end = EndSessionsReq { id: them };
	assert_eq!(editor.call(&end).await.unwrap_err(), Error::Forbidden);
}

#[tokio::test]
async fn only_admins_manage_users() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let new = new_user("new@x.ch", None);
	assert_eq!(editor.call(&new).await.unwrap_err(), Error::Forbidden);
	assert_eq!(
		editor.call(&ListReq {}).await.unwrap_err(),
		Error::Forbidden
	);
	let delete = DeleteReq { id: 1 };
	assert_eq!(editor.call(&delete).await.unwrap_err(), Error::Forbidden);

	let err = ts.anonymous().call(&ListReq {}).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn checks_what_admins_send() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	admin.call(&new_user("taken@x.ch", None)).await.unwrap();

	let mut short = new_user("a@x.ch", None);
	short.password = "short".into();
	let err = admin.call(&new_user("no-at", None)).await.unwrap_err();
	assert_eq!(paths(err), ["email"]);
	let err = admin.call(&new_user("taken@x.ch", None)).await.unwrap_err();
	assert_eq!(paths(err), ["email"]);
	assert_eq!(paths(admin.call(&short).await.unwrap_err()), ["password"]);
	let boss = new_user("a@x.ch", Some("boss"));
	let err = admin.call(&boss).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));

	// not oneself
	let me = admin.call(&SessionReq {}).await.unwrap().user;
	let err = admin.call(&DeleteReq { id: me.id }).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
	let account = admin.call(&GetReq { id: me.id }).await.unwrap();
	let mut role = unchanged(&account);
	role.role = Some("author".into());
	let err = admin.call(&role).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
}
