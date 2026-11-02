use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use serde_json::{Value, json};

use super::api::*;
use super::tokens::Scope;
use crate::drafts::SiteDoc;
use crate::drafts::api::{PreviewReq, PutReq, SitePreviewReq};
use crate::entries::api::CreateReq;
use crate::entries::tests::input;
use crate::error::Error;
use crate::schema::tests::site_with_roles;
use crate::schema::{Access, Role};
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
	let no_drafts = Scope {
		previews: false,
		..Scope::ALL
	};
	let frontend = new_token(&admin, no_drafts, None).await;
	let (_, body) = graphql(&anon, &path, TITLES, Some(&frontend)).await;
	assert_eq!(titles(&body), ["Jazz"]);
	let frontend = new_token(&admin, Scope::ALL, None).await;
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

/// A token of the studio with a scope, made by an admin.
async fn new_token(
	admin: &Client,
	scope: Scope,
	expires_at: Option<i64>,
) -> String {
	let req = CreateTokenReq {
		name: "Website".into(),
		scope,
		expires_at,
	};
	admin.call(&req).await.unwrap().token
}

#[tokio::test]
async fn admins_manage_tokens() {
	let ts = TestServer::new(site_with_roles());
	let admin = ts.admin();
	let scope = Scope {
		sections: Some(vec!["events".into()]),
		sites: None,
		previews: false,
	};
	let req = CreateTokenReq {
		name: "Website".into(),
		scope: scope.clone(),
		expires_at: None,
	};
	let created = admin.call(&req).await.unwrap();
	assert_eq!(created.token.len(), 32);
	assert_eq!(created.info.scope, scope);

	// the list never shows the token again
	let list = admin.call(&TokensReq {}).await.unwrap();
	assert!(list.env_token);
	assert_eq!(list.tokens, std::slice::from_ref(&created.info));
	let json = admin.call_json(&TokensReq {}).await.unwrap();
	assert!(!json.to_string().contains(&created.token));

	// a name and what the schema has
	let bad = CreateTokenReq {
		name: " ".into(),
		scope: Scope {
			sections: Some(vec!["news".into()]),
			sites: Some(vec!["fr".into()]),
			previews: false,
		},
		expires_at: None,
	};
	let Error::Invalid(fields) = admin.call(&bad).await.unwrap_err() else {
		panic!("not invalid");
	};
	let paths: Vec<_> = fields.iter().map(|f| f.path.as_str()).collect();
	assert_eq!(paths, ["name", "sections", "sites"]);

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
	let events_de = Scope {
		sections: Some(vec!["events".into()]),
		sites: Some(vec!["de".into()]),
		previews: false,
	};
	let token = new_token(&admin, events_de, None).await;
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
	let expired = new_token(&admin, Scope::ALL, Some(past)).await;
	let (status, _) =
		graphql(&anon, "/api/graphql", TITLES, Some(&expired)).await;
	assert_eq!(status, StatusCode::UNAUTHORIZED);

	// a token in use shows when it was used last
	let later = crate::db::now() + 60_000;
	let valid = new_token(&admin, Scope::ALL, Some(later)).await;
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
	let req = Request::get("/api/graphql").body(Body::empty()).unwrap();
	let resp = ts.anonymous().send(req).await;
	assert_eq!(resp.status(), StatusCode::OK);
	let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
	let html = String::from_utf8(body.to_vec()).unwrap();
	assert!(html.contains("GraphiQL"));
	assert!(html.contains("'session-token': zippToken"));
	assert!(html.contains("location.replace('/admin')"));

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
	let events = Scope {
		sections: Some(vec!["events".into()]),
		..Scope::ALL
	};
	let token = new_token(&admin, events, None).await;
	let (_, body) = graphql(&anon, "/api/graphql", all, Some(&token)).await;
	assert!(titles(&body).is_empty());
}
