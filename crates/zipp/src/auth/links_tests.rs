use super::api::*;
use super::links::LinkKind;
use super::*;
use crate::schema::tests::site_with_roles;
use crate::testing::{Client, TestServer};
use crate::users::api::{CreateReq, GetReq, LoginAsLinkReq, PasswordLinkReq};

/// The token of the link in a mail.
fn token_in(text: &str) -> String {
	let start = text.find("/admin/password/").unwrap() + 16;
	text[start..].split_whitespace().next().unwrap().to_string()
}

fn with_token(ts: &TestServer, auth: &Authenticated) -> Client {
	let mut c = ts.anonymous();
	c.token = Some(auth.session.token.clone());
	c
}

fn set(token: &str, password: &str, name: Option<&str>) -> SetPasswordReq {
	SetPasswordReq {
		token: token.into(),
		password: password.into(),
		name: name.map(String::from),
	}
}

/// The mails of the background, a moment later.
async fn mails(ts: &TestServer) -> Vec<crate::mail::Mail> {
	tokio::time::sleep(std::time::Duration::from_millis(50)).await;
	ts.mails()
}

#[tokio::test]
async fn a_forgotten_password_comes_by_mail() {
	let ts = TestServer::new(site_with_roles());
	add_user(&ts.state.db.conn(), "a@x.ch", "password1", None).unwrap();
	let anon = ts.anonymous();
	let login = LoginReq {
		email: "a@x.ch".into(),
		password: "password1".into(),
	};
	let old = with_token(&ts, &anon.call(&login).await.unwrap());

	// the same answer for an unknown email, no mail
	anon.call(&ForgotReq {
		email: "no@x.ch".into(),
	})
	.await
	.unwrap();
	assert!(mails(&ts).await.is_empty());
	anon.call(&ForgotReq {
		email: "A@x.ch".into(),
	})
	.await
	.unwrap();
	// again right away: no second mail
	anon.call(&ForgotReq {
		email: "a@x.ch".into(),
	})
	.await
	.unwrap();
	let sent = mails(&ts).await;
	assert_eq!(sent.len(), 1);
	assert!(sent[0].text.contains("http://zipp.test/admin/password/"));
	let token = token_in(&sent[0].text);

	let link = LinkReq {
		token: token.clone(),
	};
	let info = anon.call(&link).await.unwrap();
	assert_eq!(
		(info.kind, info.email.as_str()),
		(LinkKind::Reset, "a@x.ch")
	);

	// too short: the link stays
	let err = anon.call(&set(&token, "short", None)).await.unwrap_err();
	assert!(matches!(err, Error::Invalid(_)));
	let auth = anon.call(&set(&token, "password2", None)).await.unwrap();
	assert_eq!(auth.user.email, "a@x.ch");
	// once, and the old session is gone
	let err = anon
		.call(&set(&token, "password3", None))
		.await
		.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
	assert!(old.call(&SessionReq {}).await.is_err());
	let login = LoginReq {
		email: "a@x.ch".into(),
		password: "password2".into(),
	};
	anon.call(&login).await.unwrap();
}

#[tokio::test]
async fn an_invited_user_sets_a_password() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let new = CreateReq {
		email: "new@x.ch".into(),
		name: String::new(),
		password: String::new(),
		role: Some("editor".into()),
	};
	let account = admin.call(&new).await.unwrap();
	assert!(account.invited);
	// no password, no login
	let login = LoginReq {
		email: "new@x.ch".into(),
		password: String::new(),
	};
	assert!(ts.anonymous().call(&login).await.is_err());

	let req = PasswordLinkReq {
		id: account.user.id,
		send: true,
	};
	let link = admin.call(&req).await.unwrap();
	assert_eq!(link.kind, LinkKind::Invite);
	let sent = ts.mails();
	assert_eq!(sent[0].to, "new@x.ch");
	assert!(sent[0].text.contains(&link.url));

	let token = token_in(&link.url);
	let anon = ts.anonymous();
	let auth = anon
		.call(&set(&token, "password1", Some("Neu Muster")))
		.await
		.unwrap();
	assert_eq!(auth.user.name, "Neu Muster");
	let get = GetReq {
		id: account.user.id,
	};
	assert!(!admin.call(&get).await.unwrap().invited);

	// only admins make links
	let editor = with_token(&ts, &auth);
	assert_eq!(editor.call(&req).await.unwrap_err(), Error::Forbidden);
}

#[tokio::test]
async fn admins_log_in_as_a_user() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let editor = ts.user(Some("editor"));
	let me = editor.call(&SessionReq {}).await.unwrap().user;

	let link = admin.call(&LoginAsLinkReq { id: me.id }).await.unwrap();
	let token = link.url.rsplit('/').next().unwrap().to_string();
	assert!(link.url.contains("/admin/login-as/"));
	let req = LoginAsReq { token };
	let auth = ts.anonymous().call(&req).await.unwrap();
	assert_eq!(auth.user.id, me.id);
	let by = auth.session.impersonated_by.clone().unwrap();
	assert!(by.starts_with("admin"), "{by}");
	// a short session that isn't renewed
	let left = auth.session.expires_at - now();
	assert!(left <= links::LOGIN_AS_MS, "{left}");
	let again = with_token(&ts, &auth).call(&SessionReq {}).await.unwrap();
	assert_eq!(again.session.expires_at, auth.session.expires_at);
	// once
	assert!(ts.anonymous().call(&req).await.is_err());

	// not as an admin, not by an editor
	let other = ts.admin().call(&SessionReq {}).await.unwrap().user;
	let as_admin = LoginAsLinkReq { id: other.id };
	assert!(matches!(
		admin.call(&as_admin).await.unwrap_err(),
		Error::Request(_)
	));
	let req = LoginAsLinkReq { id: me.id };
	assert_eq!(editor.call(&req).await.unwrap_err(), Error::Forbidden);
}
