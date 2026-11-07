use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::{Value, json};

use super::api::*;
use crate::SiteUrls;
use crate::drafts::SiteDoc;
use crate::drafts::api::{PreviewReq, PutReq, SitePreviewReq};
use crate::entries::api::CreateReq;
use crate::error::Error;
use crate::fixtures::site_with_roles;
use crate::schema::{Access, Role};
use crate::testing::input;
use crate::testing::{Client, TestServer};
use crate::users::api::DeleteReq;

const TITLES: &str =
	r#"{"query": "{ entries(section: \"events\") { title } }"}"#;

const ENV: Option<&str> = Some("test-token");

/// The answer of GraphQL, with a token of a frontend or without.
async fn graphql(
	client: &Client,
	path: &str,
	body: &str,
	bearer: Option<&str>,
) -> (StatusCode, Value) {
	let mut req = Request::post(path)
		.header(header::CONTENT_TYPE, "application/json")
		.body(Body::from(body.to_string()))
		.unwrap();
	if let Some(t) = bearer {
		let token = format!("Bearer {t}").parse().unwrap();
		req.headers_mut().insert(header::AUTHORIZATION, token);
	}
	let resp = client.send(req).await;
	let status = resp.status();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	(status, serde_json::from_slice(&body).unwrap())
}

fn titles(data: &Value) -> Vec<&str> {
	data["data"]["entries"]
		.as_array()
		.unwrap()
		.iter()
		.map(|e| e["title"].as_str().unwrap())
		.collect()
}

#[tokio::test]
async fn private_like_crafts() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let mut draft = input("events", "event", "Hidden", json!({}));
	draft.published = false;
	admin.call(&CreateReq(draft)).await.unwrap();
	let live = input("events", "event", "Jazz", json!({}));
	admin.call(&CreateReq(live)).await.unwrap();
	let anon = ts.anonymous();

	let (status, body) = graphql(&anon, "/api/graphql", TITLES, None).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	assert_eq!(body["error"]["type"], "MISSING_ENDPOINT_TOKEN");

	// a frontend sees what is published, an admin everything
	let (status, body) = graphql(&anon, "/api/graphql", TITLES, ENV).await;
	assert_eq!(status, StatusCode::OK);
	assert_eq!(titles(&body), ["Jazz"]);
	let (_, body) = graphql(&admin, "/api/graphql", TITLES, None).await;
	assert_eq!(titles(&body).len(), 2);

	// a role reads it like a frontend
	let author = ts.user(Some("author"));
	let (status, _) = graphql(&author, "/api/graphql", TITLES, None).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	let (_, body) = graphql(&author, "/api/graphql", TITLES, ENV).await;
	assert_eq!(titles(&body), ["Jazz"]);
}

#[tokio::test]
async fn preview_tokens_show_the_draft() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let live = input("events", "event", "Jazz", json!({}));
	let id = admin.call(&CreateReq(live)).await.unwrap().id;
	let put = PutReq {
		id,
		site: "de".into(),
		base: None,
		content: SiteDoc {
			ty: "event".into(),
			dates: Default::default(),
			title: "Jazz am Abend".into(),
			slug: "jazz".into(),
			published: true,
			fields: json!({}),
		},
	};
	admin.call(&put).await.unwrap();
	let token = admin.call(&PreviewReq { id }).await.unwrap().token;

	let anon = ts.anonymous();
	let (_, body) = graphql(&anon, "/api/graphql", TITLES, ENV).await;
	assert_eq!(titles(&body), ["Jazz"]);
	let path = format!("/api/graphql?token={token}");
	let (_, body) = graphql(&anon, &path, TITLES, ENV).await;
	assert_eq!(titles(&body), ["Jazz am Abend"]);

	// an unknown or expired token shows what is live
	let wrong = "/api/graphql?token=wrong";
	let (_, body) = graphql(&anon, wrong, TITLES, ENV).await;
	assert_eq!(titles(&body), ["Jazz"]);

	// only for tokens that may show drafts
	let frontend = new_token(&admin, "live", None).await;
	let (_, body) = graphql(&anon, &path, TITLES, Some(&frontend)).await;
	assert_eq!(titles(&body), ["Jazz"]);
	let frontend = new_token(&admin, "website", None).await;
	let (_, body) = graphql(&anon, &path, TITLES, Some(&frontend)).await;
	assert_eq!(titles(&body), ["Jazz am Abend"]);
}

#[tokio::test]
async fn site_previews_show_the_drafts_the_user_sees() {
	// a role with the AI chat that sees only events
	let schema = site_with_roles().role(
		Role::new("writer", "Writer")
			.section("events", Access::Edit)
			.ai(),
	);
	let ts = TestServer::new(schema);
	let admin = ts.admin();
	let writer = ts.user(Some("writer"));
	let draft = async |section, ty, title: &str| {
		let live = input(section, ty, title, json!({}));
		let id = admin.call(&CreateReq(live)).await.unwrap().id;
		let put = PutReq {
			id,
			site: "de".into(),
			base: None,
			content: SiteDoc {
				ty: ty.into(),
				dates: Default::default(),
				title: format!("{title} draft"),
				slug: title.to_lowercase(),
				published: true,
				fields: json!({}),
			},
		};
		admin.call(&put).await.unwrap();
	};
	draft("events", "event", "Jazz").await;
	draft("pages", "page", "About").await;

	const PAGES: &str =
		r#"{"query": "{ entries(section: \"pages\") { title } }"}"#;
	let anon = ts.anonymous();
	let token = writer.call(&SitePreviewReq {}).await.unwrap().token;
	let path = format!("/api/graphql?token={token}");
	let (_, body) = graphql(&anon, &path, TITLES, ENV).await;
	assert_eq!(titles(&body), ["Jazz draft"]);
	// the pages it doesn't see stay live
	let (_, body) = graphql(&anon, &path, PAGES, ENV).await;
	assert_eq!(titles(&body), ["About"]);

	// an admin's shows every draft
	let token = admin.call(&SitePreviewReq {}).await.unwrap().token;
	let admin_path = format!("/api/graphql?token={token}");
	let (_, body) = graphql(&anon, &admin_path, PAGES, ENV).await;
	assert_eq!(titles(&body), ["About draft"]);

	// the tokens of a deleted user end with them
	let id = ts
		.state
		.db
		.conn()
		.query_row("SELECT id FROM users WHERE role = 'writer'", [], |r| {
			r.get(0)
		})
		.unwrap();
	admin.call(&DeleteReq { id }).await.unwrap();
	let (_, body) = graphql(&anon, &path, TITLES, ENV).await;
	assert_eq!(titles(&body), ["Jazz"]);
}

/// A token of the studio with a GraphQL schema, made by an admin.
async fn new_token(
	admin: &Client,
	schema: &str,
	expires_at: Option<i64>,
) -> String {
	let req = CreateTokenReq {
		name: "Website".into(),
		schema: schema.into(),
		expires_at,
	};
	admin.call(&req).await.unwrap().token
}

#[tokio::test]
async fn admins_manage_tokens() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let req = CreateTokenReq {
		name: "Website".into(),
		schema: "events".into(),
		expires_at: None,
	};
	let created = admin.call(&req).await.unwrap();
	assert_eq!(created.token.len(), 32);
	assert_eq!(created.info.schema, "events");

	// the list never shows the token again; the schemas of the code
	let list = admin.call(&TokensReq {}).await.unwrap();
	assert!(list.env_token);
	assert_eq!(list.tokens, std::slice::from_ref(&created.info));
	assert_eq!(list.schemas, ts.state.schema.graphql_schemas);
	let json = admin.call_json(&TokensReq {}).await.unwrap();
	assert!(!json.to_string().contains(&created.token));

	// a name and a schema of the code
	let bad = CreateTokenReq {
		name: " ".into(),
		schema: "news".into(),
		expires_at: None,
	};
	let Error::Invalid(fields) = admin.call(&bad).await.unwrap_err() else {
		panic!("not invalid");
	};
	let paths: Vec<_> = fields.iter().map(|f| f.path.as_str()).collect();
	assert_eq!(paths, ["name", "schema"]);

	// only admins
	let editor = ts.user(Some("editor"));
	let err = editor.call(&TokensReq {}).await.unwrap_err();
	assert!(matches!(err, Error::Forbidden));
	let err = editor.call(&req).await.unwrap_err();
	assert!(matches!(err, Error::Forbidden));
	let id = created.info.id;
	let err = editor.call(&DeleteTokenReq { id }).await.unwrap_err();
	assert!(matches!(err, Error::Forbidden));
	let err = ts.anonymous().call(&TokensReq {}).await.unwrap_err();
	assert!(matches!(err, Error::MissingSessionToken));

	// revoked, frontends that send it are out
	let anon = ts.anonymous();
	let token = Some(created.token.as_str());
	let (status, _) = graphql(&anon, "/api/graphql", TITLES, token).await;
	assert_eq!(status, StatusCode::OK);
	admin.call(&DeleteTokenReq { id }).await.unwrap();
	assert!(admin.call(&TokensReq {}).await.unwrap().tokens.is_empty());
	let (status, _) = graphql(&anon, "/api/graphql", TITLES, token).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	let err = admin.call(&DeleteTokenReq { id }).await.unwrap_err();
	assert!(matches!(err, Error::NotFound));
}

#[tokio::test]
async fn tokens_read_their_scope() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	admin
		.call(&CreateReq(input("events", "event", "Jazz", json!({}))))
		.await
		.unwrap();
	admin
		.call(&CreateReq(input("pages", "page", "Team", json!({}))))
		.await
		.unwrap();
	let token = new_token(&admin, "eventsDe", None).await;
	let token = Some(token.as_str());
	let anon = ts.anonymous();

	// its sections, also without a section in the query
	let all = r#"{"query": "{ entries { title } entryCount }"}"#;
	let (_, body) = graphql(&anon, "/api/graphql", all, token).await;
	assert_eq!(titles(&body), ["Jazz"]);
	assert_eq!(body["data"]["entryCount"], 1);
	let (_, body) = graphql(&anon, "/api/graphql", all, ENV).await;
	// and the single home
	assert_eq!(titles(&body).len(), 3);
	let pages = r#"{"query": "{ entries(section: \"pages\") { title } }"}"#;
	let (_, body) = graphql(&anon, "/api/graphql", pages, token).await;
	assert!(titles(&body).is_empty());

	// its sites
	let en = r#"{"query": "{ entries(site: \"en\") { title } }"}"#;
	let (_, body) = graphql(&anon, "/api/graphql", en, token).await;
	assert!(titles(&body).is_empty());
	let sites = r#"{"query": "{ sites { handle id } }"}"#;
	let (_, body) = graphql(&anon, "/api/graphql", sites, token).await;
	assert_eq!(body["data"]["sites"], json!([{ "handle": "de", "id": 1 }]));
}

/// GraphiQL's query with a GraphQL schema picked.
async fn as_schema(
	client: &Client,
	body: &str,
	schema: &str,
	bearer: Option<&str>,
) -> (StatusCode, Value) {
	let mut req = Request::post("/api/graphql")
		.header(header::CONTENT_TYPE, "application/json")
		.header("x-zipp-schema", schema)
		.body(Body::from(body.to_string()))
		.unwrap();
	if let Some(t) = bearer {
		let token = format!("Bearer {t}").parse().unwrap();
		req.headers_mut().insert(header::AUTHORIZATION, token);
	}
	let resp = client.send(req).await;
	let status = resp.status();
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	(status, serde_json::from_slice(&body).unwrap())
}

#[tokio::test]
async fn graphiql_reads_as_a_schema() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let mut draft = input("events", "event", "Hidden", json!({}));
	draft.published = false;
	admin.call(&CreateReq(draft)).await.unwrap();
	admin
		.call(&CreateReq(input("events", "event", "Jazz", json!({}))))
		.await
		.unwrap();
	admin
		.call(&CreateReq(input("pages", "page", "Team", json!({}))))
		.await
		.unwrap();
	let all = r#"{"query": "{ entries { title } }"}"#;

	// what its frontends see: its sections, nothing unpublished; no
	// token needed
	let (_, body) = as_schema(&admin, all, "events", None).await;
	assert_eq!(titles(&body), ["Jazz"]);
	let (_, body) = graphql(&admin, "/api/graphql", all, None).await;
	assert!(titles(&body).contains(&"Hidden"));
	assert!(titles(&body).contains(&"Team"));

	// a schema the code doesn't have
	let (status, _) = as_schema(&admin, all, "news", None).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);

	// only for admins: others read with their own token
	let bearer = new_token(&admin, "pages", None).await;
	let author = ts.user(Some("author"));
	let (status, _) = as_schema(&author, all, "events", None).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	let (_, body) = as_schema(&author, all, "events", Some(&bearer)).await;
	assert_eq!(titles(&body), ["Team"]);
	let (status, _) = as_schema(&ts.anonymous(), all, "events", None).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn unknown_and_expired_tokens_are_out() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let anon = ts.anonymous();
	let (status, body) =
		graphql(&anon, "/api/graphql", TITLES, Some("wrong")).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	assert_eq!(body["error"]["type"], "MISSING_ENDPOINT_TOKEN");

	let past = crate::db::now() - 1000;
	let expired = new_token(&admin, "website", Some(past)).await;
	let (status, _) =
		graphql(&anon, "/api/graphql", TITLES, Some(&expired)).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);

	// a token in use shows when it was used last
	let later = crate::db::now() + 60_000;
	let valid = new_token(&admin, "website", Some(later)).await;
	let (status, _) =
		graphql(&anon, "/api/graphql", TITLES, Some(&valid)).await;
	assert_eq!(status, StatusCode::OK);
	let list = admin.call(&TokensReq {}).await.unwrap().tokens;
	let used: Vec<_> = list.iter().map(|t| t.last_used_at.is_some()).collect();
	assert_eq!(used, [false, true]);
}

#[tokio::test]
async fn graphiql_for_admins() {
	let ts = TestServer::new(site_with_roles());
	// the page is the same for all, its queries send the session
	let resp = ts.anonymous().get("/api/graphql").await;
	assert_eq!(resp.status(), StatusCode::OK);
	// bundled with the studio, under its policy: no scripts of others
	let csp = resp.headers()[header::CONTENT_SECURITY_POLICY].to_str();
	assert!(csp.unwrap().contains("script-src 'self';"));
	let html = String::from_utf8(Client::bytes(resp).await).unwrap();
	assert!(html.contains("GraphiQL"));
	assert!(html.contains("/_studio/graphiql-"));
	assert!(!html.contains("unpkg"));
	assert!(html.contains("<body>"));

	// in development it says so, GraphQL is open
	let ts_dev = TestServer::new(site_with_roles()).dev();
	let resp = ts_dev.anonymous().get("/api/graphql").await;
	let body = Client::bytes(resp).await;
	assert!(String::from_utf8_lossy(&body).contains("<body data-dev>"));

	// its introspection, as deep as GraphiQL's, runs with the session
	let intro = format!(
		r#"{{"query": "{{ __schema {{ types {{ fields {{ args {{ type {{ {} }} }} }} }} }} }}"}}"#,
		"ofType { ".repeat(9) + "kind" + &" }".repeat(9)
	);
	let admin = ts.admin();
	let (status, body) = graphql(&admin, "/api/graphql", &intro, None).await;
	assert_eq!(status, StatusCode::OK);
	assert!(body.get("errors").is_none(), "{body}");
}

#[tokio::test]
async fn development_is_open_without_a_token() {
	let mut ts = TestServer::new(site_with_roles());
	ts.state.graphql_token = None;
	let ts = ts.dev();
	let admin = ts.admin();
	admin
		.call(&CreateReq(input("pages", "page", "Team", json!({}))))
		.await
		.unwrap();
	let anon = ts.anonymous();
	let all = r#"{"query": "{ entries { title } }"}"#;

	// without a token everything that is live
	let (status, body) = graphql(&anon, "/api/graphql", all, None).await;
	assert_eq!(status, StatusCode::OK);
	assert!(titles(&body).contains(&"Team"));

	// a wrong token fails like on the server, a token reads its scope
	let (status, _) = graphql(&anon, "/api/graphql", all, Some("wrong")).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	let token = new_token(&admin, "events", None).await;
	let (_, body) = graphql(&anon, "/api/graphql", all, Some(&token)).await;
	assert!(titles(&body).is_empty());
}

#[tokio::test]
async fn sites_have_the_urls_of_serve() {
	let urls = SiteUrls::from([("de".to_string(), "https://x.ch".into())]);
	let ts = TestServer::new(site_with_roles()).site_urls(urls);
	let body = ts.graphql("{ sites { handle baseUrl } }", json!({})).await;
	assert_eq!(body["data"]["sites"][0]["handle"], "de");
	assert_eq!(body["data"]["sites"][0]["baseUrl"], "https://x.ch");
	assert_eq!(ts.state.site_urls["de"], "https://x.ch");
}

#[tokio::test]
async fn frontends_list_the_assets() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let png = crate::fixtures::png(2, 2);
	let resp = admin.upload("/assets", &[("logo.png", &png)]).await;
	assert!(resp.status().is_success());
	let anon = ts.anonymous();
	let query = r#"{ assets(kind: "image") { filename }
		de: assetCount(site: "de") }"#;
	let body = &json!({ "query": query }).to_string();

	// like entries: with a token, nothing of the sites it doesn't read
	let (status, _) = graphql(&anon, "/api/graphql", body, None).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);
	let (_, data) = graphql(&anon, "/api/graphql", body, ENV).await;
	assert_eq!(
		data["data"],
		json!({ "assets": [{ "filename": "logo.png" }], "de": 1 })
	);
	let token = new_token(&admin, "english", None).await;
	let (_, data) = graphql(&anon, "/api/graphql", body, Some(&token)).await;
	assert_eq!(data["data"], json!({ "assets": [], "de": 0 }));

	// only a schema with assets lists them
	let token = new_token(&admin, "events", None).await;
	let (_, data) = graphql(&anon, "/api/graphql", body, Some(&token)).await;
	assert_eq!(data["data"], json!({ "assets": [], "de": 0 }));
}

#[tokio::test]
async fn entries_have_their_author() {
	let schema = site_with_roles()
		.role(Role::new("member", "Member").own("events", Access::Delete))
		.section(crate::schema::Section::channel("notes", ["note"]))
		.entry("note", "Note", [crate::schema::Field::text("author")]);
	let ts = TestServer::new(schema);
	let member = ts.user(Some("member"));
	let mut jazz = input("events", "event", "Jazz", json!({}));
	jazz.published = true;
	ts.admin().call(&CreateReq(jazz)).await.unwrap();
	let mut rock = input("events", "event", "Rock", json!({}));
	rock.published = true;
	let id = member
		.call(&CreateReq(rock))
		.await
		.unwrap()
		.author_id
		.unwrap();
	let name = "UPDATE users SET name = 'Mia' WHERE id = ?1";
	ts.state.db.conn().execute(name, [id]).unwrap();

	let query = "query($a: [QueryArgument]) {
		entries(section: \"events\", authorId: $a) {
			title authorId ... on event_Entry { author { id name } } } }";
	let body = ts.graphql(query, json!({ "a": [id] })).await;
	let mia = json!({ "id": id.to_string(), "name": "Mia" });
	assert_eq!(
		body["data"]["entries"],
		json!([{ "title": "Rock", "authorId": id.to_string(), "author": mia }]),
		"{body}"
	);
	// no users, no entries
	let body = ts.graphql(query, json!({ "a": [] })).await;
	assert_eq!(body["data"]["entries"], json!([]), "{body}");
	// the public part only
	let query = "{ entries { ... on event_Entry { author { email } } } }";
	let body = ts.graphql(query, json!({})).await;
	assert!(body["errors"][0]["message"].as_str().is_some(), "{body}");

	// a field of the name stays the type's, the other types have the
	// author but `Entry` doesn't
	let mut note = input("notes", "note", "A", json!({ "author": "Sue" }));
	note.published = true;
	ts.admin().call(&CreateReq(note)).await.unwrap();
	let query =
		"{ entries(section: \"notes\") { ... on note_Entry { author } } }";
	let body = ts.graphql(query, json!({})).await;
	assert_eq!(body["data"]["entries"], json!([{ "author": "Sue" }]));
}
