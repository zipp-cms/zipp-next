use super::api::*;
use crate::auth::api::{LoginReq, SessionReq};
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::TestServer;

#[tokio::test]
async fn admins_manage_users() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();

	let new = CreateReq {
		email: "new@x.ch".into(),
		password: "password1".into(),
		role: Some("author".into()),
	};
	admin.call(&new).await.unwrap();
	let users = admin.call(&ListReq {}).await.unwrap();
	let user = users.iter().find(|u| u.email == "new@x.ch").unwrap();
	assert_eq!(user.role.as_deref(), Some("author"));

	// the new user logs in
	let login = LoginReq {
		email: "new@x.ch".into(),
		password: "password1".into(),
	};
	let token = ts.anonymous().call(&login).await.unwrap().session.token;
	let mut new_user = ts.anonymous();
	new_user.token = Some(token);
	new_user.call(&SessionReq {}).await.unwrap();

	let role = RoleReq {
		id: user.id,
		role: Some("editor".into()),
	};
	admin.call(&role).await.unwrap();
	let users = admin.call(&ListReq {}).await.unwrap();
	let changed = users.iter().find(|u| u.id == user.id).unwrap();
	assert_eq!(changed.role.as_deref(), Some("editor"));

	// deleting ends the sessions
	admin.call(&DeleteReq { id: user.id }).await.unwrap();
	let err = new_user.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
	let err = admin.call(&DeleteReq { id: user.id }).await.unwrap_err();
	assert_eq!(err, Error::NotFound);
}

#[tokio::test]
async fn only_admins_manage_users() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let new = CreateReq {
		email: "new@x.ch".into(),
		password: "password1".into(),
		role: None,
	};
	assert_eq!(editor.call(&new).await.unwrap_err(), Error::Forbidden);
	assert_eq!(
		editor.call(&ListReq {}).await.unwrap_err(),
		Error::Forbidden
	);
	let role = RoleReq { id: 1, role: None };
	assert_eq!(editor.call(&role).await.unwrap_err(), Error::Forbidden);
	let delete = DeleteReq { id: 1 };
	assert_eq!(editor.call(&delete).await.unwrap_err(), Error::Forbidden);

	let err = ts.anonymous().call(&ListReq {}).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn checks_what_admins_send() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let create = |email: &str, password: &str, role: Option<&str>| CreateReq {
		email: email.into(),
		password: password.into(),
		role: role.map(Into::into),
	};
	let bad = [
		create("no-at", "password1", None),
		create("a@x.ch", "short", None),
		create("a@x.ch", "password1", Some("boss")),
	];
	for req in bad {
		let err = admin.call(&req).await.unwrap_err();
		assert!(matches!(err, Error::Request(_)), "{req:?}");
	}

	// not oneself
	let users = admin.call(&ListReq {}).await.unwrap();
	let me = users[0].id;
	let err = admin.call(&DeleteReq { id: me }).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
	let role = RoleReq {
		id: me,
		role: Some("author".into()),
	};
	assert!(matches!(
		admin.call(&role).await.unwrap_err(),
		Error::Request(_)
	));
}
