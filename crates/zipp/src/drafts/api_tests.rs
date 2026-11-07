use serde_json::json;

use super::SiteDoc;
use super::api::*;
use crate::entries::api::{CreateReq, GetReq as GetEntry};
use crate::error::Error;
use crate::fixtures::site_with_roles;
use crate::testing::input;
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

	let draft = editor
		.call(&put(id, "page", "Draft", None))
		.await
		.unwrap()
		.unwrap();
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
	let restore = |base| RestoreReq {
		id,
		rev: first,
		base,
	};
	let restored = editor.call(&restore(None)).await.unwrap();
	assert_eq!(restored.doc["de"].title, "Start");

	// not over someone else's draft they haven't seen
	let other = ts.user(Some("editor"));
	let theirs = put(id, "page", "Theirs", Some(restored.updated_at));
	let theirs = other.call(&theirs).await.unwrap().unwrap();
	let err = editor.call(&restore(None)).await;
	assert!(matches!(err.unwrap_err(), Error::Conflict(_)));
	editor
		.call(&restore(Some(theirs.updated_at)))
		.await
		.unwrap();

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
async fn content_from_before_revisions_stays_restorable() {
	let ts = TestServer::new(site_with_roles());
	let editor = ts.user(Some("editor"));
	let id = create(&editor, "pages", "page").await;
	// imported or from before revisions were kept
	ts.state
		.db
		.conn()
		.execute("DELETE FROM revisions WHERE entry_id = ?1", [id])
		.unwrap();

	editor.call(&put(id, "page", "Edited", None)).await.unwrap();
	editor.call(&PublishReq { id }).await.unwrap();

	// the content before the first edit is the oldest, of no one
	let revs = editor.call(&RevisionsReq { id }).await.unwrap();
	assert_eq!(revs.len(), 2);
	let first = revs.last().unwrap();
	assert_eq!(first.user, None);
	assert!(revs[0].user.as_ref().unwrap().starts_with("editor"));
	let restore = RestoreReq {
		id,
		rev: first.id,
		base: None,
	};
	let restored = editor.call(&restore).await.unwrap();
	assert_eq!(restored.doc["de"].title, "Start");

	// the next publish has its revision already
	editor.call(&PublishReq { id }).await.unwrap();
	let revs = editor.call(&RevisionsReq { id }).await.unwrap();
	assert_eq!(revs.len(), 3);
}

#[tokio::test]
async fn two_editors_conflict() {
	let ts = TestServer::new(site_with_roles());
	let (a, b) = (ts.admin(), ts.admin());
	let id = create(&a, "pages", "page").await;

	let first = a.call(&put(id, "page", "A", None)).await.unwrap().unwrap();
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
			.call(&RestoreReq {
				id: page,
				rev: 1,
				base: None,
			})
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

#[tokio::test]
async fn related_entries_of_hidden_sections_don_t_show() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let author = ts.user(Some("author"));
	let list = crate::entries::api::ListReq(
		[("section".to_string(), "home".to_string())].into(),
	);
	let home = admin.call(&list).await.unwrap().entries[0].id;

	// an event links to the home page the author doesn't see (they
	// couldn't link it themselves)
	let mut live = input("events", "event", "Jazz", json!({ "page": [home] }));
	live.published = false;
	let event = admin.call(&CreateReq(live)).await.unwrap().id;
	let req = PutReq {
		id: event,
		site: "de".into(),
		base: None,
		content: content("event", "Jazz", json!({ "page": [home] })),
	};
	author.call(&req).await.unwrap();

	let get = GetReq {
		id: event,
		site: None,
	};
	assert!(author.call(&get).await.unwrap().relations.is_empty());
	assert!(
		admin
			.call(&get)
			.await
			.unwrap()
			.relations
			.contains_key(&home)
	);
	let entry = GetEntry {
		id: event,
		site: None,
	};
	assert!(author.call(&entry).await.unwrap().relations.is_empty());
	assert!(
		admin
			.call(&entry)
			.await
			.unwrap()
			.relations
			.contains_key(&home)
	);
}

#[tokio::test]
async fn publishing_leaves_out_hidden_entries() {
	use crate::schema::{Access, Role};
	let guest = Role::new("guest", "Guest").section("events", Access::Publish);
	let ts = TestServer::new(site_with_roles().role(guest));
	let admin = ts.admin();
	let guest = ts.user(Some("guest"));
	let page = create(&admin, "pages", "page").await;
	let event = create(&guest, "events", "event").await;

	// a page the guest can't view is left out like a deleted one
	let req = |id| PutReq {
		id: event,
		site: "de".into(),
		base: None,
		content: content("event", "E", json!({ "page": [id] })),
	};
	guest.call(&req(page)).await.unwrap();
	guest.call(&PublishReq { id: event }).await.unwrap();
	let got = guest
		.call(&GetEntry {
			id: event,
			site: None,
		})
		.await;
	assert!(got.unwrap().entry.fields.get("page").is_none());

	admin.call(&req(page)).await.unwrap();
	admin.call(&PublishReq { id: event }).await.unwrap();
	let got = admin
		.call(&GetEntry {
			id: event,
			site: None,
		})
		.await;
	assert_eq!(got.unwrap().entry.fields["page"], json!([page]));
}

#[tokio::test]
async fn a_change_undone_leaves_no_draft() {
	let ts = TestServer::new(site_with_roles());
	let (a, b) = (ts.admin(), ts.admin());
	let block = json!({ "id": "b1", "type": "text", "fields": {
		"content": "Hallo"
	} });
	let fields = json!({ "intro": "Hi", "body": [block] });
	let entry = input("pages", "page", "Start", fields);
	let id = a.call(&CreateReq(entry)).await.unwrap().id;
	let draft = |site: &str, title: &str, fields, base| PutReq {
		id,
		site: site.into(),
		base,
		content: content("page", title, fields),
	};

	// changed and back: the studio sends what it shows, also empty
	// values, the block in another key order
	let changed = json!({ "intro": "Hi there", "body": [block] });
	let d = a.call(&draft("de", "Start", changed, None)).await.unwrap();
	let base = Some(d.unwrap().updated_at);
	let back = json!({
		"story": { "type": "doc", "content": [{ "type": "paragraph" }] },
		"seo": { "title": "", "description": null },
		"body": [{ "fields": { "content": "Hallo" }, "type": "text",
			"id": "b1" }],
		"intro": "Hi",
	});
	let d = a.call(&draft("de", " Start ", back.clone(), base)).await;
	assert_eq!(d.unwrap(), None);
	let got = a.call(&GetReq { id, site: None }).await.unwrap();
	assert!(got.draft.is_none());

	// the next change starts a new draft, in another site too
	let en = draft("en", "Begin", back.clone(), None);
	let d = a.call(&en).await.unwrap().unwrap();
	assert_eq!(d.doc["de"].title, "Start");
	let undo = draft("en", "Start", back.clone(), Some(d.updated_at));
	assert_eq!(a.call(&undo).await.unwrap(), None);

	// a real change stays: another block, a seo title, the switch
	let other = json!({ "intro": "Hi", "body": [{ "id": "b2", "type": "text",
		"fields": { "content": "Hallo" } }] });
	let seo = json!({ "intro": "Hi", "body": [block], "seo": {
		"title": "T"
	} });
	for fields in [other, seo] {
		let d = a.call(&draft("de", "Start", fields, None)).await.unwrap();
		a.call(&DiscardReq { id }).await.unwrap();
		assert!(d.is_some());
	}
	let mut off = draft("de", "Start", back.clone(), None);
	off.content.published = false;
	let d = a.call(&off).await.unwrap().unwrap();

	// whoever still has the deleted draft open sees the conflict
	let undo = draft("de", "Start", back, Some(d.updated_at));
	assert_eq!(a.call(&undo).await.unwrap(), None);
	let late = draft("de", "Late", json!({}), Some(d.updated_at));
	assert_eq!(
		b.call(&late).await.unwrap_err(),
		Error::Conflict("the draft was published or discarded".into())
	);
}
