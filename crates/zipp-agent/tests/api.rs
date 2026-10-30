//! The agent's api in a site, through zipp's test server.

use zipp::error::Error;
use zipp::testing::TestServer;
use zipp::{Access, Role, Schema, Section};
use zipp_agent::Agent;
use zipp_agent::api::{ForgetReq, StartReq};

fn schema() -> Schema {
	Schema::new()
		.site("de", "Deutsch", "de-CH")
		.section(Section::channel("events", ["event"]))
		.entry("event", "Event", [])
		.role(Role::new("author", "Author").section("events", Access::Edit))
		.plugin(Agent::new())
}

#[tokio::test]
async fn only_admins_in_development() {
	let start = StartReq {
		message: "Hi".into(),
		view: None,
	};
	let ts = TestServer::new(schema());
	let err = ts.admin().call(&start).await.unwrap_err();
	assert_eq!(err, Error::NotFound);

	let ts = TestServer::new(schema()).dev();
	let author = ts.user(Some("author"));
	assert_eq!(author.call(&start).await.unwrap_err(), Error::Forbidden);
	assert_eq!(
		author.call(&ForgetReq {}).await.unwrap_err(),
		Error::Forbidden
	);
	let err = ts.anonymous().call(&ForgetReq {}).await.unwrap_err();
	assert_eq!(err, Error::MissingSessionToken);
}
