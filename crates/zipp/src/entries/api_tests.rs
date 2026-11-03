use std::sync::{Arc, Mutex};

use serde_json::json;

use super::api::*;
use crate::error::Error;
use crate::fixtures::site_with_roles;
use crate::schema::{Access, Condition, Field, Role};
use crate::testing::TestServer;
use crate::testing::input;
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
async fn refuses_entries_below_the_max_level() {
	let mut schema = site_with_roles();
	for s in schema.sections.iter_mut().filter(|s| s.handle == "pages") {
		s.max_level = Some(1);
	}
	let ts = TestServer::new(schema);
	let admin = ts.admin();
	let about = admin
		.call(&CreateReq(input("pages", "page", "About", json!({}))))
		.await
		.unwrap();
	let team = admin
		.call(&CreateReq(input("pages", "page", "Team", json!({}))))
		.await
		.unwrap();

	let mut child = input("pages", "page", "Child", json!({}));
	child.parent_id = Some(about.id);
	let err = admin.call(&CreateReq(child)).await.unwrap_err();
	let too_deep = Error::Request(
		"the entry goes too deep, pages has entries down to level 1".into(),
	);
	assert_eq!(err, too_deep);
	let below = MoveReq {
		id: team.id,
		parent_id: Some(about.id),
		index: 0,
	};
	assert_eq!(admin.call(&below).await.unwrap_err(), too_deep);
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

	// the price has two decimals
	let err = ts
		.admin()
		.call(&CreateReq(input(
			"events",
			"event",
			"Jazz",
			json!({ "price": 12.555 }),
		)))
		.await
		.unwrap_err();
	let Error::Invalid(fields) = err else {
		panic!("{err:?}")
	};
	assert_eq!(fields[0].path, "price");
	assert_eq!(fields[0].message, "at most 2 decimal places");
}

#[tokio::test]
async fn emails_and_defaults_of_new_entries() {
	let ts = TestServer::new(site_with_roles());
	let author = ts.user(Some("author"));

	let event =
		|title, fields| CreateReq(input("events", "event", title, fields));
	let err = author
		.call(&event("Jazz", json!({ "contact": "info at textrem" })))
		.await
		.unwrap_err();
	let Error::Invalid(fields) = err else {
		panic!("{err:?}")
	};
	assert_eq!(fields[0].path, "contact");
	assert_eq!(fields[0].message, "an email address like name@example.com");

	// a new entry without the field gets its default, one that sends it
	// keeps its value
	let e = author
		.call(&event("Jazz", json!({ "contact": "info@textrem.ch" })))
		.await
		.unwrap();
	assert_eq!(e.fields["listed"], true);
	assert_eq!(e.fields["contact"], "info@textrem.ch");
	let off = author
		.call(&event("Blues", json!({ "listed": false })))
		.await
		.unwrap();
	assert_eq!(off.fields["listed"], false);

	// an update without it doesn't bring it back
	let mut up = input("events", "event", "Jazz", json!({}));
	up.slug = e.slug.clone();
	let e = ts
		.admin()
		.call(&UpdateReq {
			id: e.id,
			input: up,
		})
		.await
		.unwrap();
	assert!(e.fields.get("listed").is_none());
}

#[tokio::test]
async fn notes_take_no_value() {
	let ts = TestServer::new(site_with_roles());
	let err = ts
		.user(Some("author"))
		.call(&CreateReq(input(
			"events",
			"event",
			"Jazz",
			json!({ "howto": "mine" }),
		)))
		.await
		.unwrap_err();
	let Error::Invalid(fields) = err else {
		panic!("{err:?}")
	};
	assert_eq!(fields[0].path, "howto");
	assert_eq!(fields[0].message, "a note holds no value");
}

#[tokio::test]
async fn hidden_fields_aren_t_required() {
	let mut schema = site_with_roles();
	let why = Field::text("why").required().when(Condition::off("listed"));
	schema.get_mut("event").unwrap().fields.push(why);
	let ts = TestServer::new(schema);
	let author = ts.user(Some("author"));
	let event =
		|title, fields| CreateReq(input("events", "event", title, fields));

	// listed by its default, the reason is hidden
	author.call(&event("Jazz", json!({}))).await.unwrap();
	let err = author
		.call(&event("Blues", json!({ "listed": false })))
		.await
		.unwrap_err();
	let Error::Invalid(fields) = err else {
		panic!("{err:?}")
	};
	assert_eq!(fields[0].path, "why");
	assert_eq!(fields[0].message, "is required");
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

#[tokio::test]
async fn lists_tell_nothing_of_hidden_sections() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let author = ts.user(Some("author"));
	let home = admin.call(&params(&[("section", "home")])).await.unwrap();
	let home = &home.entries[0];
	let id = home.id.to_string();

	// the author doesn't see the home page: not by its id, nor in a
	// search, and the total doesn't count it either
	for p in [("ids", id.as_str()), ("search", home.title.as_str())] {
		let found = author.call(&params(&[p])).await.unwrap();
		assert!(found.entries.is_empty(), "{p:?}");
		assert_eq!(found.total, 0, "{p:?}");
		let found = admin.call(&params(&[p])).await.unwrap();
		assert_eq!(found.total, 1, "{p:?}");
	}
}

#[tokio::test]
async fn a_text_of_only_signs_finds_nothing() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	admin
		.call(&CreateReq(input("pages", "page", "About", json!({}))))
		.await
		.unwrap();
	for text in ["-", "- * \"", ""] {
		let found = admin.call(&params(&[("text", text)])).await.unwrap();
		assert_eq!(found.total, 0, "{text:?}");
		let p = [("text", text), ("section", "pages")];
		let found = admin.call(&params(&p)).await.unwrap();
		assert!(found.entries.is_empty(), "{text:?}");
	}
}

#[tokio::test]
async fn choices_for_a_picker() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let create = async |section, ty, title, parent| {
		let mut i = input(section, ty, title, json!({}));
		i.parent_id = parent;
		admin.call(&CreateReq(i)).await
	};
	// a single has its entry from the start
	let home = admin.call(&params(&[("section", "home")])).await.unwrap();
	let home = &home.entries[0];
	let about = create("pages", "page", "About", None).await.unwrap();
	let team = create("pages", "page", "Team", Some(about.id))
		.await
		.unwrap();
	let contact = create("pages", "page", "Contact", None).await.unwrap();
	create("events", "event", "Jazz", None).await.unwrap();

	// the sections with pages, a structure in its tree order
	let req = ChoicesReq {
		types: "page".into(),
		sections: None,
		site: Some("en".into()),
	};
	let got = admin.call(&req).await.unwrap().entries;
	let ids: Vec<i64> = got.iter().map(|e| e.id).collect();
	assert_eq!(ids, [home.id, about.id, team.id, contact.id]);
	assert_eq!(got[2].parent_id, Some(about.id));
	assert_eq!(got[2].ty, "page");

	// an author sees pages, not the home page
	let author = ts.user(Some("author"));
	let got = author.call(&req).await.unwrap().entries;
	assert!(got.iter().all(|e| e.section == "pages"));
	assert_eq!(got.len(), 3);

	// several types
	let both = ChoicesReq {
		types: "page,event".into(),
		sections: None,
		site: None,
	};
	assert_eq!(admin.call(&both).await.unwrap().entries.len(), 5);

	// only the sections of a field
	let pages = ChoicesReq {
		types: "page,event".into(),
		sections: Some("pages,events".into()),
		site: None,
	};
	let got = admin.call(&pages).await.unwrap().entries;
	assert_eq!(got.len(), 4);
	assert!(got.iter().all(|e| e.section != "home"));
	let wrong = ChoicesReq {
		types: "page".into(),
		sections: Some("nope".into()),
		site: None,
	};
	let err = admin.call(&wrong).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));

	let unknown = ChoicesReq {
		types: "nope".into(),
		sections: None,
		site: None,
	};
	let err = admin.call(&unknown).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));
	let err = ts.anonymous().call(&req).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn saving_tells_nothing_of_hidden_entries_and_assets() {
	let guest = Role::new("guest", "Guest").section("events", Access::Publish);
	let ts = TestServer::new(site_with_roles().role(guest));
	let admin = ts.admin();
	let guest = ts.user(Some("guest"));
	let page = admin
		.call(&CreateReq(input("pages", "page", "Hidden", json!({}))))
		.await
		.unwrap();
	let png = crate::fixtures::png(4, 4);
	let image = {
		let conn = ts.state.db.conn();
		crate::assets::store(&conn, &ts.state.files, "a.png", &png, None)
			.unwrap()
	};

	// a page of a section the guest can't view and an asset without asset
	// access save like ids that don't exist: left out, no error
	let hidden = json!({ "page": [page.id], "image": [image.id] });
	let missing = json!({ "page": [999], "image": [999] });
	let save =
		|title, fields| CreateReq(input("events", "event", title, fields));
	let a = guest.call(&save("A", hidden.clone())).await.unwrap();
	let b = guest.call(&save("B", missing)).await.unwrap();
	assert_eq!(a.fields, b.fields);
	assert!(a.fields.get("page").is_none(), "{}", a.fields);

	// an admin sees them
	let e = admin.call(&save("C", hidden.clone())).await.unwrap();
	assert_eq!(e.fields["page"], json!([page.id]));
	assert_eq!(e.fields["image"], json!([image.id]));

	// what the entry links to already stays when the guest saves it
	let mut again = input("events", "event", "C", hidden);
	again.slug = e.slug.clone();
	let update = UpdateReq {
		id: e.id,
		input: again,
	};
	let e = guest.call(&update).await.unwrap();
	assert_eq!(e.fields["page"], json!([page.id]));
	assert_eq!(e.fields["image"], json!([image.id]));
}
