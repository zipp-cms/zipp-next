use std::sync::{Arc, Mutex};

use serde_json::json;

use super::api::*;
use super::tests::input;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::testing::TestServer;
use crate::{Change, ChangeKind};

fn params(p: &[(&str, &str)]) -> ListReq {
	ListReq(
		p.iter()
			.map(|(k, v)| (k.to_string(), v.to_string()))
			.collect(),
	)
}

#[tokio::test]
async fn creates_reads_moves_and_deletes() {
	let changes = Arc::new(Mutex::new(vec![]));
	let seen = changes.clone();
	let ts = TestServer::new(site_with_roles())
		.on_change(move |c: &Change| seen.lock().unwrap().push(c.kind));
	let admin = ts.admin();

	let about = admin
		.call(&CreateReq(input("pages", "page", "About", json!({}))))
		.await
		.unwrap();
	assert_eq!(about.uri.as_deref(), Some("about"));
	let mut team = input("pages", "page", "Team", json!({ "intro": "Hi" }));
	team.parent_id = Some(about.id);
	let team = admin.call(&CreateReq(team)).await.unwrap();
	assert_eq!(team.uri.as_deref(), Some("about/team"));

	let got = admin
		.call(&GetReq {
			id: team.id,
			site: Some("en".into()),
		})
		.await
		.unwrap();
	assert_eq!(
		(got.entry.site.as_str(), got.entry.title.as_str()),
		("en", "Team")
	);

	let list = admin.call(&params(&[("section", "pages")])).await.unwrap();
	assert_eq!(list.total, 2);
	assert_eq!(list.published[&team.id].len(), 2);
	assert!(list.drafts.is_empty());

	// saved live, then moved to the top
	let mut changed = input("pages", "page", "Our team", json!({}));
	changed.slug = "team".into();
	let saved = admin
		.call(&UpdateReq {
			id: team.id,
			input: changed,
		})
		.await
		.unwrap();
	assert_eq!(saved.title, "Our team");
	let to_top = MoveReq {
		id: team.id,
		parent_id: None,
		index: 0,
	};
	admin.call(&to_top).await.unwrap();
	let got = admin
		.call(&GetReq {
			id: team.id,
			site: None,
		})
		.await
		.unwrap();
	assert_eq!(got.entry.uri.as_deref(), Some("team"));

	admin.call(&DeleteReq { id: team.id }).await.unwrap();
	let err = admin
		.call(&GetReq {
			id: team.id,
			site: None,
		})
		.await
		.unwrap_err();
	assert_eq!(err, Error::NotFound);

	use ChangeKind::*;
	assert_eq!(
		*changes.lock().unwrap(),
		[Saved, Saved, Saved, Moved, Deleted]
	);
}

#[tokio::test]
async fn refuses_invalid_content() {
	let ts = TestServer::new(site_with_roles());
	let body = json!({ "body": [{ "type": "text", "fields": {} }] });
	let err = ts
		.admin()
		.call(&CreateReq(input("pages", "page", "About", body)))
		.await
		.unwrap_err();
	let Error::Invalid(fields) = err else {
		panic!("{err:?}")
	};
	assert_eq!(fields[0].path, "body.0.content");
}

#[tokio::test]
async fn roles_on_entries() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let (author, editor) = (ts.user(Some("author")), ts.user(Some("editor")));
	let page = admin
		.call(&CreateReq(input("pages", "page", "About", json!({}))))
		.await
		.unwrap();

	// an author writes events, but doesn't publish them
	let event = author
		.call(&CreateReq(input("events", "event", "Jazz", json!({}))))
		.await
		.unwrap();
	assert!(!event.published);
	let err = author
		.call(&CreateReq(input("pages", "page", "New", json!({}))))
		.await
		.unwrap_err();
	assert_eq!(err, Error::Forbidden);

	// and sees pages, not the home page
	author
		.call(&GetReq {
			id: page.id,
			site: None,
		})
		.await
		.unwrap();
	let all = author.call(&params(&[])).await.unwrap();
	assert!(all.entries.iter().all(|e| e.section != "home"));
	let err = author
		.call(&params(&[("section", "home")]))
		.await
		.unwrap_err();
	assert_eq!(err, Error::Forbidden);

	let update = |id| UpdateReq {
		id,
		input: input("events", "event", "Live", json!({})),
	};
	assert_eq!(
		author.call(&update(event.id)).await.unwrap_err(),
		Error::Forbidden
	);
	let to_top = MoveReq {
		id: page.id,
		parent_id: None,
		index: 0,
	};
	assert_eq!(author.call(&to_top).await.unwrap_err(), Error::Forbidden);
	let delete = DeleteReq { id: event.id };
	assert_eq!(author.call(&delete).await.unwrap_err(), Error::Forbidden);

	// an editor deletes events, not pages
	editor.call(&update(event.id)).await.unwrap();
	editor.call(&to_top).await.unwrap();
	let err = editor.call(&DeleteReq { id: page.id }).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	editor.call(&delete).await.unwrap();

	let err = ts.anonymous().call(&params(&[])).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
	// an entry that isn't there
	let err = admin
		.call(&GetReq {
			id: 999,
			site: None,
		})
		.await
		.unwrap_err();
	assert_eq!(err, Error::NotFound);
}
