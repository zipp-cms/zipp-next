use serde::{Deserialize, Serialize};

use crate::request;

/// How the cms sends mails, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusReq {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MailStatus {
	/// whether mails go to a server, else they are only logged
	pub configured: bool,
	/// its host and port
	pub server: Option<String>,
	pub from: String,
}

request!(StatusReq, GET "/mail" => MailStatus);

/// Sends a test mail to the admin who asks.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestReq {}

request!(TestReq, POST "/mail/test" => ());
