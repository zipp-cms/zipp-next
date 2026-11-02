use super::api::*;
use super::*;
use crate::fixtures::site_with_roles;
use crate::testing::TestServer;
use crate::users::api::{
	CreateReq, GetReq, LoginAsLinkReq, PasswordLinkReq, UpdateReq,
};

/// The token of the link in a mail.
fn token_in(text: &str) -> String {
	let start = text.find("/admin/password/").unwrap() + 16;
	text[start..].split_whitespace().next().unwrap().to_string()
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
	let old = ts.login("a@x.ch", "password1").await;

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
		(links::Kind::Reset, "a@x.ch")
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
	ts.login("a@x.ch", "password2").await;
}

#[tokio::test]
async fn forgot_tells_nothing_without_a_public_url() {
	let ts = TestServer::new(site_with_roles()).public_url("");
	add_user(&ts.state.db.conn(), "a@x.ch", "password1", None).unwrap();
	let anon = ts.anonymous();

	// known or not, the same error
	let forgot = |email: &str| ForgotReq {
		email: email.into(),
	};
	let known = anon.call(&forgot("a@x.ch")).await.unwrap_err();
	let unknown = anon.call(&forgot("no@x.ch")).await.unwrap_err();
	assert!(matches!(&known, Error::Request(m) if m.contains("public-url")));
	assert_eq!(known, unknown);
	assert!(mails(&ts).await.is_empty());
}

#[tokio::test]
async fn links_end_with_a_new_password_or_email() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	add_user(&ts.state.db.conn(), "a@x.ch", "password1", None).unwrap();
	let id = ts
		.state
		.db
		.conn()
		.query_row("SELECT id FROM users WHERE email = 'a@x.ch'", [], |r| {
			r.get(0)
		})
		.unwrap();
	let link = async || {
		let req = PasswordLinkReq { id, send: false };
		token_in(&admin.call(&req).await.unwrap().url)
	};
	let gone = async |token: &str| {
		let req = LinkReq {
			token: token.into(),
		};
		ts.anonymous().call(&req).await.unwrap_err()
	};

	// a new password
	let token = link().await;
	add_user(&ts.state.db.conn(), "a@x.ch", "password2", None).unwrap();
	assert!(matches!(gone(&token).await, Error::Request(_)));

	// a new email
	let token = link().await;
	let change = UpdateReq {
		id,
		email: "b@x.ch".into(),
		name: String::new(),
		role: None,
		language: String::new(),
		password: None,
		current_password: None,
	};
	admin.call(&change).await.unwrap();
	assert!(matches!(gone(&token).await, Error::Request(_)));

	// an invite once the user has a password
	ts.state.db.conn()
		.execute(
			"INSERT INTO user_links (token, user_id, kind, created_at, expires_at)
			VALUES ('inv', ?1, 'invite', 0, ?2)",
			rusqlite::params![id, now() + 100_000],
		)
		.unwrap();
	assert!(matches!(gone("inv").await, Error::Request(_)));
	let err = ts.anonymous().call(&set("inv", "password3", None)).await;
	assert!(matches!(err.unwrap_err(), Error::Request(_)));
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
		language: String::new(),
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
	assert_eq!(link.kind, links::Kind::Invite);
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
	let editor = ts.with_token(&auth.session.token);
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
	let again = ts
		.with_token(&auth.session.token)
		.call(&SessionReq {})
		.await
		.unwrap();
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
