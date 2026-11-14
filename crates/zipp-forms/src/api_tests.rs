use zipp::error::Error;
use zipp::testing::TestServer;
use zipp::{Access, Role};

use crate::FORMS;
use crate::api::{DeleteReq, ListReq};
use crate::tests::{contact, post, schema, valid};

/// A reader of the forms, an editor who deletes, an author of nothing.
fn server() -> TestServer {
	let schema = schema()
		.role(Role::new("reader", "Reader").section(FORMS, Access::View))
		.role(Role::new("editor", "Editor").section(FORMS, Access::Delete))
		.role(Role::new("author", "Author"));
	TestServer::new(schema).trusted_proxy("127.0.0.1")
}

fn all() -> ListReq {
	ListReq {
		form: None,
		offset: 0,
		site: None,
	}
}

#[tokio::test]
async fn who_may_view_the_forms_lists_their_submissions() {
	let ts = server();
	let form = contact(&ts).await;
	let path = format!("/forms/{}", form.id);
	for ip in ["1.1.1.1", "1.1.1.2"] {
		post(&ts, &path, valid(), ip).await;
	}

	let list = ts.user(Some("reader")).call(&all()).await.unwrap();
	assert_eq!(list.total, 2);
	assert_eq!(list.submissions[0].form, "Kontakt");
	assert_eq!(list.submissions[0].answers[0].value, "Anna");
	let other = ListReq {
		form: Some(form.id + 1),
		..all()
	};
	let list = ts.admin().call(&other).await.unwrap();
	assert_eq!(list.total, 0);

	let err = ts.user(Some("author")).call(&all()).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
}

#[tokio::test]
async fn deleting_needs_delete() {
	let ts = server();
	let form = contact(&ts).await;
	post(&ts, &format!("/forms/{}", form.id), valid(), "1.1.1.1").await;
	let id = ts.admin().call(&all()).await.unwrap().submissions[0].id;

	let reader = ts.user(Some("reader"));
	let err = reader.call(&DeleteReq { id }).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);

	let editor = ts.user(Some("editor"));
	editor.call(&DeleteReq { id }).await.unwrap();
	assert_eq!(ts.admin().call(&all()).await.unwrap().total, 0);
	let err = editor.call(&DeleteReq { id }).await.unwrap_err();
	assert_eq!(err, Error::NotFound);
}
