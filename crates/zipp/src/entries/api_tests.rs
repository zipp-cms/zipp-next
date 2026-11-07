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
async fn lists_of_several_sections_and_related_entries() {
	let schema = site_with_roles()
		.role(Role::new("reader", "Reader").section("pages", Access::View));
	let ts = TestServer::new(schema);
	let admin = ts.admin();
	let page = admin
		.call(&CreateReq(input("pages", "page", "About", json!({}))))
		.await
		.unwrap();
	let fields = json!({ "page": [page.id] });
	admin
		.call(&CreateReq(input("events", "event", "Jazz", fields)))
		.await
		.unwrap();

	let both = params(&[("section", "pages,events")]);
	assert_eq!(admin.call(&both).await.unwrap().total, 2);
	// each section needs its role
	let author = ts.user(Some("author"));
	let home = params(&[("section", "home,events")]);
	assert_eq!(author.call(&home).await.unwrap_err(), Error::Forbidden);

	// the pages events point to; a role that doesn't see events doesn't
	// learn of them through the pages either
	let related =
		params(&[("section", "pages"), ("relatedToEntries", "section=events")]);
	assert_eq!(admin.call(&related).await.unwrap().total, 1);
	let reader = ts.user(Some("reader"));
	assert_eq!(reader.call(&related).await.unwrap().total, 0);
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
		offset: None,
		search: None,
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
		offset: None,
		search: None,
	};
	assert_eq!(admin.call(&both).await.unwrap().entries.len(), 5);

	// only the sections of a field
	let pages = ChoicesReq {
		types: "page,event".into(),
		sections: Some("pages,events".into()),
		site: None,
		offset: None,
		search: None,
	};
	let got = admin.call(&pages).await.unwrap().entries;
	assert_eq!(got.len(), 4);
	assert!(got.iter().all(|e| e.section != "home"));
	let wrong = ChoicesReq {
		types: "page".into(),
		sections: Some("nope".into()),
		site: None,
		offset: None,
		search: None,
	};
	let err = admin.call(&wrong).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)));

	let unknown = ChoicesReq {
		types: "nope".into(),
		sections: None,
		site: None,
		offset: None,
		search: None,
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

fn choices_req(types: &str, sections: Option<&str>) -> ChoicesReq {
	ChoicesReq {
		types: types.into(),
		sections: sections.map(Into::into),
		site: None,
		offset: None,
		search: None,
	}
}

#[tokio::test]
async fn choices_past_the_limit_come_with_an_offset() {
	// the tests send 3 of a section at a time
	assert_eq!(super::CHOICES_LIMIT, 3);
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let create = async |section, ty, title, parent| {
		let mut i = input(section, ty, title, json!({}));
		i.parent_id = parent;
		admin.call(&CreateReq(i)).await.unwrap().id
	};
	let mut events = vec![];
	for title in ["Jazz 1", "Jazz 2", "Jazz 3", "Jazz 4", "Rock"] {
		events.push(create("events", "event", title, None).await);
	}
	let about = create("pages", "page", "About", None).await;
	create("pages", "page", "Team", Some(about)).await;
	create("pages", "page", "Contact", None).await;
	let jobs = create("pages", "page", "Jobs", None).await;
	let intern = create("pages", "page", "Intern", Some(jobs)).await;

	// the first of each section and how many there are
	let req = choices_req("event,page", Some("events,pages,"));
	let got = admin.call(&req).await.unwrap();
	assert_eq!(got.entries.len(), 6);
	assert_eq!(got.totals["events"].entries, 5);
	assert_eq!(got.totals["pages"].pickable, 5);
	assert!(!got.totals.contains_key("home"));

	// the rest of a channel, newest first
	let more = ChoicesReq {
		offset: Some(3),
		..choices_req("event", Some("events"))
	};
	let rest = admin.call(&more).await.unwrap().entries;
	let ids: Vec<i64> = rest.iter().map(|e| e.id).collect();
	assert_eq!(ids, [events[1], events[0]]);

	// the rest of a tree keeps its children below their parent
	let more = ChoicesReq {
		offset: Some(3),
		..choices_req("page", Some("pages"))
	};
	let rest = admin.call(&more).await.unwrap().entries;
	let ids: Vec<i64> = rest.iter().map(|e| e.id).collect();
	assert_eq!(ids, [jobs, intern]);
	assert_eq!(rest[1].parent_id, Some(jobs));

	// a search looks at all of them, at most the limit
	let search = ChoicesReq {
		search: Some("jazz".into()),
		..choices_req("event,page", None)
	};
	let found = admin.call(&search).await.unwrap();
	assert_eq!(found.entries.len(), 3);
	assert_eq!(found.totals["events"].entries, 4);
	assert_eq!(found.totals["pages"].entries, 0);
	let search = ChoicesReq {
		search: Some("intern".into()),
		..choices_req("page", None)
	};
	let found = admin.call(&search).await.unwrap();
	assert_eq!(found.entries[0].id, intern);
	assert_eq!(found.parents[0].id, jobs);

	// an author doesn't see home, its search doesn't either
	let author = ts.user(Some("author"));
	let found = author.call(&search).await.unwrap();
	assert!(found.totals.keys().all(|s| s == "pages"));
	let err = ts.anonymous().call(&search).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn choices_skip_empty_parts_of_a_list() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	admin
		.call(&CreateReq(input("events", "event", "Jazz", json!({}))))
		.await
		.unwrap();
	for (types, sections) in [("event,", Some("events,")), ("event", Some(""))]
	{
		let req = choices_req(types, sections);
		let got = admin.call(&req).await.unwrap();
		assert_eq!(got.entries.len(), 1, "{types} {sections:?}");
	}
	// no types, nothing
	let none = admin.call(&choices_req("", None)).await.unwrap();
	assert!(none.entries.is_empty());
}

/// [`site_with_roles`] with a member who deletes their own events and
/// sees no others.
fn site_with_members() -> crate::schema::Schema {
	site_with_roles()
		.role(Role::new("member", "Member").own("events", Access::Delete))
}

#[tokio::test]
async fn entries_have_their_author() {
	let ts = TestServer::new(site_with_members());
	let admin = ts.admin();
	let member = ts.user(Some("member"));
	let jazz = input("events", "event", "Jazz", json!({}));
	let jazz = admin.call(&CreateReq(jazz)).await.unwrap();
	let rock = input("events", "event", "Rock", json!({}));
	let rock = member.call(&CreateReq(rock)).await.unwrap();
	let (admin_id, member_id) =
		(jazz.author_id.unwrap(), rock.author_id.unwrap());
	assert_ne!(admin_id, member_id);
	ts.state
		.db
		.conn()
		.execute("UPDATE users SET name = 'Mia' WHERE id = ?1", [member_id])
		.unwrap();

	// the studio shows the name
	let get = GetReq {
		id: rock.id,
		site: None,
	};
	let got = admin.call(&get).await.unwrap();
	assert_eq!(got.author.unwrap().name, "Mia");

	// only an admin gives an entry another author, of the users there are
	let to_member = AuthorReq {
		id: jazz.id,
		author_id: Some(member_id),
	};
	let err = member.call(&to_member).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	let unknown = AuthorReq {
		id: jazz.id,
		author_id: Some(999),
	};
	let err = admin.call(&unknown).await.unwrap_err();
	assert!(matches!(err, Error::Request(_)), "{err:?}");
	admin.call(&to_member).await.unwrap();
	let mine = params(&[("author", &member_id.to_string())]);
	let list = admin.call(&mine).await.unwrap();
	assert_eq!(list.total, 2);

	// a save keeps it, a deleted user leaves none
	let update = UpdateReq {
		id: jazz.id,
		input: input("events", "event", "Jazz!", json!({})),
	};
	let saved = admin.call(&update).await.unwrap();
	assert_eq!(saved.author_id, Some(member_id));
	let delete = crate::users::api::DeleteReq { id: member_id };
	admin.call(&delete).await.unwrap();
	let got = admin.call(&get).await.unwrap();
	assert_eq!((got.entry.author_id, got.author), (None, None));
}

#[tokio::test]
async fn own_access_reaches_only_own_entries() {
	let ts = TestServer::new(site_with_members());
	let admin = ts.admin();
	let member = ts.user(Some("member"));
	let theirs = input("events", "event", "Jazz", json!({}));
	let theirs = admin.call(&CreateReq(theirs)).await.unwrap();

	// own Delete: creates published, the entry is theirs; nothing else
	let mut mine = input("events", "event", "Rock", json!({}));
	mine.published = true;
	let mine = member.call(&CreateReq(mine)).await.unwrap();
	assert!(mine.published);
	let page = input("pages", "page", "About", json!({}));
	let err = member.call(&CreateReq(page)).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);

	// only theirs in lists, also without a section and in pickers
	for p in [vec![("section", "events")], vec![]] {
		let list = member.call(&params(&p)).await.unwrap();
		let ids: Vec<i64> = list.entries.iter().map(|e| e.id).collect();
		assert_eq!((ids, list.total), (vec![mine.id], 1), "{p:?}");
	}
	let err = member
		.call(&params(&[("section", "pages")]))
		.await
		.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	let found = member.call(&choices_req("event", None)).await.unwrap();
	let ids: Vec<i64> = found.entries.iter().map(|e| e.id).collect();
	assert_eq!(ids, [mine.id]);
	assert_eq!(found.totals["events"].entries, 1);

	// theirs is forbidden, mine works
	let get = |id| GetReq { id, site: None };
	let err = member.call(&get(theirs.id)).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	member.call(&get(mine.id)).await.unwrap();
	let update = |id| UpdateReq {
		id,
		input: input("events", "event", "Changed", json!({})),
	};
	let err = member.call(&update(theirs.id)).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	let saved = member.call(&update(mine.id)).await.unwrap();
	assert_eq!(saved.title, "Changed");
	let err = member
		.call(&AuthorReq {
			id: mine.id,
			author_id: None,
		})
		.await
		.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	let err = member.call(&DeleteReq { id: theirs.id }).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	member.call(&DeleteReq { id: mine.id }).await.unwrap();

	// with View on the section the others show too, read only
	let schema = site_with_roles().role(
		Role::new("member", "Member")
			.section("events", Access::View)
			.own("events", Access::Edit),
	);
	let ts = TestServer::new(schema);
	let admin = ts.admin();
	let member = ts.user(Some("member"));
	let theirs = input("events", "event", "Jazz", json!({}));
	let theirs = admin.call(&CreateReq(theirs)).await.unwrap();
	let mine = input("events", "event", "Rock", json!({}));
	let mine = member.call(&CreateReq(mine)).await.unwrap();
	let list = member
		.call(&params(&[("section", "events")]))
		.await
		.unwrap();
	assert_eq!(list.total, 2);
	member.call(&get(theirs.id)).await.unwrap();
	let err = member.call(&update(theirs.id)).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	// own Edit doesn't publish
	let err = member.call(&update(mine.id)).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
}
