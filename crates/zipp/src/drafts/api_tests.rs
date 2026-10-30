use serde_json::json;

use super::SiteDoc;
use super::api::*;
use crate::entries::api::{CreateReq, GetReq as GetEntry};
use crate::entries::tests::input;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::{Client, TestServer};

fn content(ty: &str, title: &str, fields: serde_json::Value) -> SiteDoc {
	SiteDoc {
		ty: ty.into(),
		dates: Default::default(),
		title: title.into(),
		slug: crate::entries::slugify(title),
		published: true,
		fields,
	}
}

fn put(id: i64, ty: &str, title: &str, base: Option<i64>) -> PutReq {
	PutReq {
		id,
		site: "de".into(),
		base,
		content: content(ty, title, json!({})),
	}
}

async fn create(client: &Client, section: &str, ty: &str) -> i64 {
	let entry = input(section, ty, "Start", json!({}));
	client.call(&CreateReq(entry)).await.unwrap().id
}

async fn live_title(client: &Client, id: i64) -> String {
	let got = client.call(&GetEntry { id, site: None }).await.unwrap();
	got.entry.title
}

#[tokio::test]
async fn drafts_until_published() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let id = create(&editor, "pages", "page").await;

	let draft = editor.call(&put(id, "page", "Draft", None)).await.unwrap();
	assert_eq!(draft.doc["de"].title, "Draft");
	assert!(draft.user.unwrap().starts_with("editor"));
	let got = editor.call(&GetReq { id, site: None }).await.unwrap();
	assert_eq!(got.draft.unwrap().doc["de"].title, "Draft");
	assert_eq!(live_title(&editor, id).await, "Start");

	editor.call(&PublishReq { id }).await.unwrap();
	assert_eq!(live_title(&editor, id).await, "Draft");
	assert!(
		editor
			.call(&GetReq { id, site: None })
			.await
			.unwrap()
			.draft
			.is_none()
	);

	// back to the start: a revision becomes the draft
	let revs = editor.call(&RevisionsReq { id }).await.unwrap();
	assert_eq!(revs.len(), 2);
	let first = revs.last().unwrap().id;
	let doc = editor.call(&RevisionReq { id, rev: first }).await.unwrap();
	assert_eq!(doc["de"].title, "Start");
	let restored = editor.call(&RestoreReq { id, rev: first }).await.unwrap();
	assert_eq!(restored.doc["de"].title, "Start");

	editor.call(&DiscardReq { id }).await.unwrap();
	assert!(
		editor
			.call(&GetReq { id, site: None })
			.await
			.unwrap()
			.draft
			.is_none()
	);
	assert_eq!(live_title(&editor, id).await, "Draft");
}

#[tokio::test]
async fn two_editors_conflict() {
	let ts = TestServer::new(site_with_roles());
	let (a, b) = (ts.admin(), ts.admin());
	let id = create(&a, "pages", "page").await;

	let first = a.call(&put(id, "page", "A", None)).await.unwrap();
	let err = b.call(&put(id, "page", "B", None)).await.unwrap_err();
	assert!(
		matches!(err, Error::Conflict(m) if m.ends_with("changed this entry"))
	);
	// after reloading b knows the draft
	b.call(&put(id, "page", "B", Some(first.updated_at)))
		.await
		.unwrap();

	a.call(&PublishReq { id }).await.unwrap();
	let err = b
		.call(&put(id, "page", "B2", Some(first.updated_at)))
		.await
		.unwrap_err();
	assert_eq!(
		err,
		Error::Conflict("the draft was published or discarded".into())
	);
}

#[tokio::test]
async fn publishing_checks_the_content() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let id = create(&admin, "pages", "page").await;
	let body = json!({ "body": [{ "type": "text", "fields": {} }] });
	let req = PutReq {
		id,
		site: "de".into(),
		base: None,
		content: content("page", "Bad", body),
	};
	// a draft may be unfinished, publishing not
	admin.call(&req).await.unwrap();
	let err = admin.call(&PublishReq { id }).await.unwrap_err();
	assert!(matches!(err, Error::Invalid(_)), "{err:?}");

	let unknown = PutReq {
		site: "fr".into(),
		..put(id, "page", "X", None)
	};
	assert!(matches!(
		admin.call(&unknown).await.unwrap_err(),
		Error::Request(_)
	));
}

#[tokio::test]
async fn roles_on_drafts() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let (author, editor) = (ts.user(Some("author")), ts.user(Some("editor")));
	let page = create(&admin, "pages", "page").await;
	let event = create(&author, "events", "event").await;

	// an author drafts events and someone else publishes them
	author
		.call(&put(event, "event", "Jazz", None))
		.await
		.unwrap();
	let err = author.call(&PublishReq { id: event }).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	editor.call(&PublishReq { id: event }).await.unwrap();

	// pages only to look at
	author
		.call(&GetReq {
			id: page,
			site: None,
		})
		.await
		.unwrap();
	author.call(&RevisionsReq { id: page }).await.unwrap();
	author.call(&PreviewReq { id: page }).await.unwrap();
	let forbidden = [
		author
			.call(&put(page, "page", "X", None))
			.await
			.unwrap_err(),
		author.call(&DiscardReq { id: page }).await.unwrap_err(),
		author
			.call(&RestoreReq { id: page, rev: 1 })
			.await
			.unwrap_err(),
		// every draft is the AI chat's
		author.call(&SitePreviewReq {}).await.unwrap_err(),
	];
	assert!(
		forbidden.iter().all(|e| *e == Error::Forbidden),
		"{forbidden:?}"
	);

	let token = editor.call(&SitePreviewReq {}).await.unwrap().token;
	assert!(!token.is_empty());
	let err = ts
		.anonymous()
		.call(&PreviewReq { id: page })
		.await
		.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn a_draft_knows_the_entries_only_it_links_to() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let page = admin
		.call(&CreateReq(input("pages", "page", "About", json!({}))))
		.await
		.unwrap();
	let event = create(&admin, "events", "event").await;
	let req = PutReq {
		id: event,
		site: "de".into(),
		base: None,
		content: content("event", "Jazz", json!({ "page": [page.id] })),
	};
	admin.call(&req).await.unwrap();

	let got = admin
		.call(&GetReq {
			id: event,
			site: None,
		})
		.await
		.unwrap();
	assert_eq!(got.relations[&page.id].title, "About");
}
