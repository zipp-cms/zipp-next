use serde_json::json;

use super::api::*;
use crate::auth::api::SessionReq;
use crate::error::Error;
use crate::fixtures::site_with_roles;
use crate::schema::Field;
use crate::testing::TestServer;

fn new_user(email: &str, role: Option<&str>) -> CreateReq {
	CreateReq {
		email: email.into(),
		name: "Neu".into(),
		password: "password1".into(),
		role: role.map(Into::into),
		language: String::new(),
	}
}

/// What the account page sends, unchanged.
fn unchanged(a: &super::Account) -> UpdateReq {
	UpdateReq {
		id: a.user.id,
		email: a.user.email.clone(),
		name: a.user.name.clone(),
		role: a.user.role.clone(),
		language: a.user.language.clone(),
		password: None,
		current_password: None,
		slug: None,
		fields: None,
	}
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
	let new_user = ts.login("new@x.ch", "password1").await;
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
	let again = ts.login("anna@x.ch", "password2").await;

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

	// so does a new email, and the old address gets a mail
	let mut email = unchanged(&changed);
	email.email = "eddie@x.ch".into();
	let err = editor.call(&email).await.unwrap_err();
	assert_eq!(paths(err), ["currentPassword"]);
	email.current_password = Some("password".into());
	editor.call(&email).await.unwrap();
	let mails = ts.mails();
	assert_eq!(mails.len(), 1);
	assert_eq!(mails[0].to, me.email);
	assert!(mails[0].text.contains("eddie@x.ch"));
	let me = editor.call(&SessionReq {}).await.unwrap().user;
	change.email = me.email.clone();

	// with it the other sessions end, this one stays
	let elsewhere = ts.login(&me.email, "password").await;
	change.current_password = Some("password".into());
	editor.call(&change).await.unwrap();
	editor.call(&SessionReq {}).await.unwrap();
	let err = elsewhere.call(&SessionReq {}).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);

	// logging out everywhere else keeps this session too
	let elsewhere = ts.login(&me.email, "password2").await;
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
async fn users_pick_the_studio_language() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();

	// empty is the browser's; an admin invites Irene in German
	let mut irene = new_user("irene@x.ch", Some("editor"));
	irene.password = String::new();
	irene.language = "de".into();
	let created = admin.call(&irene).await.unwrap();
	assert_eq!(created.user.language, "de");
	let me = admin.call(&SessionReq {}).await.unwrap().user;
	assert_eq!(me.language, "");

	// the own, also in the session the studio loads at the start
	let editor = ts.user(Some("editor"));
	let id = editor.call(&SessionReq {}).await.unwrap().user.id;
	let mut change = unchanged(&editor.call(&GetReq { id }).await.unwrap());
	change.language = "de".into();
	editor.call(&change).await.unwrap();
	let session = editor.call(&SessionReq {}).await.unwrap();
	assert_eq!(session.user.language, "de");
	change.language = String::new();
	let back = editor.call(&change).await.unwrap();
	assert_eq!(back.user.language, "");

	// only languages the studio has
	change.language = "fr".into();
	let err = editor.call(&change).await.unwrap_err();
	assert_eq!(paths(err), ["language"]);
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
	// a reset link would go to both
	let two = new_user("a@x.ch, b@y.ch", None);
	assert_eq!(paths(admin.call(&two).await.unwrap_err()), ["email"]);
	let err = admin
		.call(&new_user(" taken@x.ch", None))
		.await
		.unwrap_err();
	assert_eq!(paths(err), ["email"]);
	let err = admin.call(&new_user("taken@x.ch", None)).await.unwrap_err();
	assert_eq!(paths(err), ["email"]);
	assert_eq!(paths(admin.call(&short).await.unwrap_err()), ["password"]);
	let boss = new_user("a@x.ch", Some("boss"));
	let err = admin.call(&boss).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));

	// one the cli took stays
	let conn = || ts.state.db.conn();
	let id = super::create(&conn(), "dev@localhost", "", "", None, "").unwrap();
	let mut dev = unchanged(&admin.call(&GetReq { id }).await.unwrap());
	dev.name = "Dev".into();
	admin.call(&dev).await.unwrap();

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

#[tokio::test]
async fn profiles_are_checked_and_saved() {
	let schema = site_with_roles().user_fields([
		Field::text("bio").multiline(),
		Field::number("years"),
		Field::relation("favourite", ["event"]).max(1),
	]);
	let ts = TestServer::new(schema);
	let admin = ts.admin();
	let mut anna = new_user("anna@x.ch", Some("author"));
	anna.name = "Anna Muster".into();
	let created = admin.call(&anna).await.unwrap();
	assert_eq!(created.user.slug, "anna-muster");
	assert_eq!(created.user.fields, json!({}));
	// the same name gets a number
	let mut other = new_user("other@x.ch", None);
	other.name = "Anna Muster".into();
	let other = admin.call(&other).await.unwrap();
	assert_eq!(other.user.slug, "anna-muster-2");

	// the fields are checked like a block's
	let mut change = unchanged(&created);
	change.fields = Some(json!({ "bio": "Cook", "years": "many" }));
	let err = admin.call(&change).await.unwrap_err();
	assert_eq!(paths(err), ["fields.years"]);
	change.fields = Some(json!({ "bio": "Cook", "years": 3 }));
	let saved = admin.call(&change).await.unwrap();
	assert_eq!(saved.user.fields, json!({ "bio": "Cook", "years": 3 }));
	// none keeps them
	change.fields = None;
	let saved = admin.call(&change).await.unwrap();
	assert_eq!(saved.user.fields["bio"], "Cook");

	// admins change the slug, like an entry's; one of another is taken
	change.slug = Some("Anna M".into());
	let saved = admin.call(&change).await.unwrap();
	assert_eq!(saved.user.slug, "anna-m");
	change.slug = Some("anna-muster-2".into());
	let err = admin.call(&change).await.unwrap_err();
	assert_eq!(paths(err), ["slug"]);
	change.slug = Some(" ".into());
	let err = admin.call(&change).await.unwrap_err();
	assert_eq!(paths(err), ["slug"]);

	// the user changes their fields, not their slug
	let user = ts.login("anna@x.ch", "password1").await;
	let own = user
		.call(&GetReq {
			id: created.user.id,
		})
		.await
		.unwrap();
	let mut change = unchanged(&own);
	change.fields = Some(json!({ "bio": "Chef" }));
	let saved = user.call(&change).await.unwrap();
	assert_eq!(saved.user.fields, json!({ "bio": "Chef" }));
	change.slug = Some("anna-m".into());
	user.call(&change).await.unwrap();
	change.slug = Some("queen".into());
	let err = user.call(&change).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	// nor another user's
	let mut theirs = unchanged(&other);
	theirs.fields = Some(json!({ "bio": "Hacked" }));
	let err = user.call(&theirs).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
}

#[tokio::test]
async fn a_placeholder_slug_follows_a_name_from_the_studio() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let mut nameless = new_user("x@x.ch", None);
	nameless.name = String::new();
	let created = admin.call(&nameless).await.unwrap();
	let placeholder = format!("user-{}", created.user.id);
	assert_eq!(created.user.slug, placeholder);
	// the studio sends the slug it shows along
	let mut change = unchanged(&created);
	change.name = "Anna".into();
	change.slug = Some(placeholder);
	let saved = admin.call(&change).await.unwrap();
	assert_eq!(saved.user.slug, "anna");
}
