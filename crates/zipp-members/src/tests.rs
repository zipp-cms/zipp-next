use std::sync::{Arc, Mutex};

use serde_json::{Value, json};
use zipp::entries::Saved;
use zipp::entries::api::CreateReq as StudioCreate;
use zipp::error::Error;
use zipp::testing::{Client, TestServer, input};
use zipp::{Access, Change, ChangeKind, Field, Role, Schema, Section};

use crate::Members;
use crate::api::*;

/// A PNG of 1×1.
const PNG: &[u8] = &[
	137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1,
	0, 0, 0, 1, 8, 4, 0, 0, 0, 181, 28, 12, 2, 0, 0, 0, 11, 73, 68, 65, 84,
	120, 218, 99, 100, 96, 0, 0, 0, 6, 0, 2, 48, 129, 208, 47, 0, 0, 0, 0, 73,
	69, 78, 68, 174, 66, 96, 130,
];

fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.section(Section::channel("posts", ["post"]).uri("posts/{slug}"))
		.section(Section::channel("pages", ["page"]))
		.section(Section::channel("notes", ["note"]))
		.entry(
			"post",
			"Post",
			[
				Field::text("intro").required(),
				Field::text("body"),
				Field::images("image"),
				Field::relation("related", ["page", "post"]),
			],
		)
		.entry("page", "Page", [Field::text("intro")])
		.entry("note", "Note", [Field::text("intro")])
		.role(
			Role::new("member", "Member")
				.own("posts", Access::Delete)
				.section("posts", Access::View)
				.own("notes", Access::Delete)
				.assets(Access::Edit)
				.no_studio(),
		)
		.role(
			Role::new("helper", "Helper")
				.own("posts", Access::Edit)
				.no_studio(),
		)
		.role(
			Role::new("reader", "Reader")
				.own("posts", Access::Delete)
				.no_studio(),
		)
		.plugin(
			Members::new()
				.section("posts")
				.images(1000, "Members/{user}"),
		)
}

fn new_post(title: &str) -> CreateReq {
	CreateReq {
		section: "posts".into(),
		ty: "post".into(),
		site: None,
		title: title.into(),
		fields: json!({ "intro": "Hi" }),
		published: true,
	}
}

fn change(id: i64, title: &str, fields: Value) -> UpdateReq {
	UpdateReq {
		id,
		site: None,
		title: title.into(),
		fields,
		published: None,
	}
}

#[tokio::test]
async fn member_edits_own_entries() {
	let ts = TestServer::new(schema());
	let member = ts.site_user(Some("member"));

	let e = member.call(&new_post("Zopf")).await.unwrap();
	assert_eq!((e.title.as_str(), e.published), ("Zopf", true));
	assert_eq!(e.uri.as_deref(), Some("posts/zopf"));
	assert_eq!(e.fields["intro"], "Hi");
	assert!(!e.has_draft);
	member.call(&new_post("Brot")).await.unwrap();

	// newest first, only theirs
	ts.admin()
		.call(&StudioCreate(input(
			"posts",
			"post",
			"Admin's",
			json!({ "intro": "x" }),
		)))
		.await
		.unwrap();
	let list = ListReq {
		section: "posts".into(),
		site: None,
		offset: 0,
	};
	let found = member.call(&list).await.unwrap();
	let titles: Vec<&str> =
		found.entries.iter().map(|e| e.title.as_str()).collect();
	assert_eq!((titles, found.total), (vec!["Brot", "Zopf"], 2));

	// the fields it sends change, the others stay
	let body = json!({ "body": "Mehl, Wasser" });
	let r = member.call(&change(e.id, "Zopf neu", body)).await.unwrap();
	assert_eq!(r.saved, Saved::Live);
	assert_eq!(r.entry.title, "Zopf neu");
	assert_eq!(r.entry.fields["intro"], "Hi");
	assert_eq!(r.entry.fields["body"], "Mehl, Wasser");
	// the slug stays, links to it keep working
	assert_eq!(r.entry.slug, "zopf");
	let got = member
		.call(&GetReq {
			id: e.id,
			site: None,
		})
		.await
		.unwrap();
	assert_eq!(got, r.entry);

	let mut off = change(e.id, "Zopf neu", json!({}));
	off.published = Some(false);
	let r = member.call(&off).await.unwrap();
	assert!(!r.entry.published);

	member.call(&DeleteReq { id: e.id }).await.unwrap();
	let err = member
		.call(&GetReq {
			id: e.id,
			site: None,
		})
		.await;
	assert_eq!(err.unwrap_err(), Error::NotFound);
}

#[tokio::test]
async fn only_own_entries_of_the_sections() {
	let ts = TestServer::new(schema());
	let anna = ts.site_user(Some("member"));
	let ben = ts.site_user(Some("member"));
	let e = anna.call(&new_post("Zopf")).await.unwrap();

	// ben sees posts in the studio's sense, but edits only his own
	let get = GetReq {
		id: e.id,
		site: None,
	};
	assert_eq!(ben.call(&get).await.unwrap_err(), Error::Forbidden);
	let put = change(e.id, "Mine", json!({}));
	assert_eq!(ben.call(&put).await.unwrap_err(), Error::Forbidden);
	let del = DeleteReq { id: e.id };
	assert_eq!(ben.call(&del).await.unwrap_err(), Error::Forbidden);
	let missing = GetReq {
		id: 999,
		site: None,
	};
	assert_eq!(ben.call(&missing).await.unwrap_err(), Error::NotFound);

	// a reader with only own access doesn't even see it
	let reader = ts.site_user(Some("reader"));
	assert_eq!(reader.call(&get).await.unwrap_err(), Error::Forbidden);

	// notes: the role may, the plugin doesn't edit them
	let note = CreateReq {
		section: "notes".into(),
		ty: "note".into(),
		..new_post("Note")
	};
	assert_eq!(anna.call(&note).await.unwrap_err(), Error::Forbidden);
	let list = ListReq {
		section: "notes".into(),
		site: None,
		offset: 0,
	};
	assert_eq!(anna.call(&list).await.unwrap_err(), Error::Forbidden);

	// pages: neither
	let page = CreateReq {
		section: "pages".into(),
		ty: "page".into(),
		..new_post("Page")
	};
	assert_eq!(anna.call(&page).await.unwrap_err(), Error::Forbidden);
}

#[tokio::test]
async fn website_login_needed() {
	let ts = TestServer::new(schema());
	let list = ListReq {
		section: "posts".into(),
		site: None,
		offset: 0,
	};
	let err = ts.visitor(None).call(&list).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
	// a session of the studio is none of the website
	let editor = ts.admin();
	let visitor = ts.visitor(editor.token.as_deref());
	let err = visitor.call(&list).await.unwrap_err();
	assert_eq!(err, Error::InvalidSessionToken);
}

#[tokio::test]
async fn invalid_fields() {
	let ts = TestServer::new(schema());
	let member = ts.site_user(Some("member"));
	let mut req = new_post("");
	req.fields = json!({ "intro": "" });
	let Error::Invalid(errors) = member.call(&req).await.unwrap_err() else {
		panic!("invalid");
	};
	let paths: Vec<&str> = errors.iter().map(|e| e.path.as_str()).collect();
	assert_eq!(paths, ["title", "intro"]);

	let e = member.call(&new_post("Zopf")).await.unwrap();
	let err = member
		.call(&change(e.id, "Zopf", json!({ "intro": "" })))
		.await
		.unwrap_err();
	assert!(matches!(err, Error::Invalid(_)));
	// nothing was stored
	let got = member
		.call(&GetReq {
			id: e.id,
			site: None,
		})
		.await
		.unwrap();
	assert_eq!(
		(got.fields["intro"].as_str(), got.has_draft),
		(Some("Hi"), false)
	);
}

/// What the role can't view isn't linked, like in the studio.
#[tokio::test]
async fn links_only_to_what_the_role_sees() {
	let ts = TestServer::new(schema());
	let admin = ts.admin();
	let page = input("pages", "page", "Secret", json!({}));
	let page = admin.call(&StudioCreate(page)).await.unwrap();
	let post = input("posts", "post", "Other", json!({ "intro": "x" }));
	let post = admin.call(&StudioCreate(post)).await.unwrap();

	let member = ts.site_user(Some("member"));
	let mut req = new_post("Zopf");
	req.fields = json!({ "intro": "Hi", "related": [page.id, post.id] });
	let e = member.call(&req).await.unwrap();
	assert_eq!(e.fields["related"], json!([post.id]));
}

#[tokio::test]
async fn without_publish_a_draft() {
	let ts = TestServer::new(schema());
	let helper = ts.site_user(Some("helper"));

	// new ones stay unpublished
	let e = helper.call(&new_post("Zopf")).await.unwrap();
	assert!(!e.published);

	// a link it can't see stays out of the draft too, the editor who
	// publishes it would see it
	let page = input("pages", "page", "Secret", json!({}));
	let page = ts.admin().call(&StudioCreate(page)).await.unwrap();
	let fields = json!({ "body": "Mehl", "related": [page.id] });
	let mut req = change(e.id, "Zopf neu", fields);
	req.published = Some(true);
	let r = helper.call(&req).await.unwrap();
	assert_eq!(r.saved, Saved::Draft);
	assert_eq!(r.entry.fields["body"], "Mehl");
	assert_eq!(r.entry.fields.get("related"), None);
	assert!(r.entry.has_draft && !r.entry.published);
	// the member sees what they saved, the website what is live
	assert_eq!(r.entry.title, "Zopf neu");
	let live =
		ts.graphql("{ entries(section: \"posts\") { title } }", json!({}));
	assert_eq!(live.await["data"]["entries"], json!([]));

	// an editor publishes it
	let conn = ts.state.db.conn();
	let live: String = conn
		.query_row(
			"SELECT title FROM entry_sites WHERE entry_id = ?1",
			[e.id],
			|r| r.get(0),
		)
		.unwrap();
	assert_eq!(live, "Zopf");
}

#[tokio::test]
async fn saves_tell_the_hooks_and_go_live() {
	let changes: Arc<Mutex<Vec<Change>>> = Default::default();
	let seen = changes.clone();
	let ts = TestServer::new(schema())
		.on_change(move |c| seen.lock().unwrap().push(c.clone()));
	let member = ts.site_user(Some("member"));

	let e = member.call(&new_post("Zopf")).await.unwrap();
	member
		.call(&change(e.id, "Zopf neu", json!({})))
		.await
		.unwrap();
	let q = "{ entries(section: \"posts\") { title } }";
	let found = ts.graphql(q, json!({})).await;
	assert_eq!(found["data"]["entries"], json!([{ "title": "Zopf neu" }]));
	member.call(&DeleteReq { id: e.id }).await.unwrap();

	let kinds: Vec<ChangeKind> =
		changes.lock().unwrap().iter().map(|c| c.kind).collect();
	assert_eq!(
		kinds,
		[
			ChangeKind::Saved,
			ChangeKind::Published,
			ChangeKind::Deleted
		]
	);
}

async fn upload(
	client: &Client,
	name: &str,
	data: &[u8],
) -> Result<Uploaded, Error> {
	let resp = client.upload("/members/assets", &[(name, data)]).await;
	Client::json(resp).await
}

#[tokio::test]
async fn uploads_images() {
	let ts = TestServer::new(schema());
	let member = ts.site_user(Some("member"));

	let img = upload(&member, "zopf.png", PNG).await.unwrap();
	assert_eq!((img.width, img.height), (Some(1), Some(1)));
	assert!(img.url.ends_with("/zopf.png"), "{}", img.url);
	// into the member's folder
	let folder: String = ts
		.state
		.db
		.conn()
		.query_row(
			"SELECT p.name || '/' || f.name FROM assets a
			JOIN asset_folders f ON f.id = a.folder_id
			JOIN asset_folders p ON p.id = f.parent_id WHERE a.id = ?1",
			[img.id],
			|r| r.get(0),
		)
		.unwrap();
	assert!(folder.starts_with("Members/"), "{folder}");

	// it goes into an image field
	let mut req = new_post("Zopf");
	req.fields = json!({ "intro": "Hi", "image": [img.id] });
	let e = member.call(&req).await.unwrap();
	assert_eq!(e.fields["image"], json!([img.id]));

	let err = upload(&member, "notes.txt", b"hello").await.unwrap_err();
	assert_eq!(err, Error::Request("only images".into()));
	let big = [PNG, &[0; 1000]].concat();
	let err = upload(&member, "big.png", &big).await.unwrap_err();
	assert!(matches!(err, Error::Request(m) if m.contains("too big")));

	// a role without asset access
	let helper = ts.site_user(Some("helper"));
	let err = upload(&helper, "zopf.png", PNG).await.unwrap_err();
	assert_eq!(err, Error::Forbidden);
	let err = upload(&ts.visitor(None), "a.png", PNG).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}

#[tokio::test]
async fn changes_are_limited() {
	let ts = TestServer::new(schema());
	let member = ts.site_user(Some("member"));
	let e = member.call(&new_post("Zopf")).await.unwrap();
	for i in 1..crate::routes::MAX_WRITES {
		let req = change(e.id, &format!("Zopf {i}"), json!({}));
		member.call(&req).await.unwrap();
	}
	let err = member.call(&change(e.id, "Zopf", json!({}))).await;
	assert!(
		matches!(err.unwrap_err(), Error::Request(m) if m.contains("Too many"))
	);
	// others still may
	let other = ts.site_user(Some("member"));
	other.call(&new_post("Brot")).await.unwrap();
}
