use super::api::*;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::TestServer;

fn page_request(text: &str) -> CreateReq {
	CreateReq {
		page: "/admin/setup/types/page".into(),
		context: "Entry types › Page".into(),
		text: text.into(),
	}
}

#[tokio::test]
async fn anyone_requests_a_change() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let author = ts.user(Some("author"));

	let created = author
		.call(&page_request(" I need a subtitle field "))
		.await
		.unwrap();
	assert_eq!(created.text, "I need a subtitle field");
	assert!(created.user.unwrap().starts_with("author"));
	assert!(!created.done);

	// the admin gets a mail of it
	let mails = ts.mails();
	assert_eq!(mails.len(), 1);
	assert!(mails[0].to.starts_with("admin"));
	assert!(mails[0].subject.contains("Entry types › Page"));
	assert!(mails[0].text.contains("I need a subtitle field"));

	admin.call(&page_request("A video block")).await.unwrap();
	let empty = author.call(&page_request("  ")).await.unwrap_err();
	assert!(matches!(empty, Error::Invalid(_)));
	assert_eq!(
		ts.anonymous().call(&page_request("x")).await.unwrap_err(),
		Error::MissingSessionToken
	);
}

#[tokio::test]
async fn admins_list_and_tick_off_requests() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let editor = ts.user(Some("editor"));
	let first = editor.call(&page_request("A subtitle")).await.unwrap();
	editor.call(&page_request("A video block")).await.unwrap();

	assert_eq!(
		editor.call(&ListReq {}).await.unwrap_err(),
		Error::Forbidden
	);
	let done = DoneReq {
		id: first.id,
		done: true,
	};
	assert_eq!(editor.call(&done).await.unwrap_err(), Error::Forbidden);

	// the open ones first, the newest first
	let list = admin.call(&ListReq {}).await.unwrap();
	assert_eq!(list.len(), 2);
	assert_eq!(list[0].text, "A video block");
	admin.call(&done).await.unwrap();
	admin
		.call(&DoneReq {
			id: list[0].id,
			done: false,
		})
		.await
		.unwrap();
	let list = admin.call(&ListReq {}).await.unwrap();
	assert_eq!(list[1].text, "A subtitle");
	assert!(list[1].done);

	let gone = DoneReq { id: 99, done: true };
	assert_eq!(admin.call(&gone).await.unwrap_err(), Error::NotFound);
}

#[tokio::test]
async fn the_page_is_a_path_of_the_studio() {
	let ts = TestServer::new(site_with_roles());
	ts.admin();
	let author = ts.user(Some("author"));

	// admins open it as a link and get it in a mail after the studio's url
	for page in [
		"javascript:alert(1)",
		"@evil.example/login",
		"https://evil.example/admin/",
		"//evil.example/admin/",
		"/admin",
		"/admin/x y",
		"/admin/x\nAll requests: https://evil.example",
		"/admin/\\evil.example",
	] {
		let req = CreateReq {
			page: page.into(),
			..page_request("A subtitle")
		};
		let err = author.call(&req).await.unwrap_err();
		assert!(matches!(err, Error::Invalid(_)), "{page}");
	}
	assert!(ts.mails().is_empty());

	author.call(&page_request("A subtitle")).await.unwrap();
	let mails = ts.mails();
	assert!(
		mails[0]
			.text
			.contains("(http://zipp.test/admin/setup/types/page)")
	);
}
