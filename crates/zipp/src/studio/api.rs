use serde::{Deserialize, Serialize};

use crate::request;
use crate::schema::{Role, Schema};

/// What the studio loads first: the schema and its settings.
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaReq {}

request!(SchemaReq, GET "/schema" => SchemaResp);

/// A plugin of the studio, its module is `/_plugins/<name>.js`.
#[derive(Debug, Serialize)]
pub struct PluginResp {
	pub name: String,
	pub data: serde_json::Value,
}

/// A doc of the code, as html.
#[derive(Debug, Serialize)]
pub struct DocResp {
	pub handle: String,
	pub name: String,
	pub html: String,
}

#[derive(Debug, Serialize)]
pub struct AiModel {
	pub id: String,
	pub name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaResp {
	#[serde(flatten)]
	pub schema: Schema,
	/// for the preview button of the studio
	pub site_urls: crate::graphql::SiteUrls,
	/// the models of the AI chat, none if it is off
	pub ai_models: Vec<AiModel>,
	pub docs: Vec<DocResp>,
	/// the roles, the studio shows what the user's may do
	pub roles: Vec<Role>,
	/// sites not on the website yet
	pub offline: Vec<String>,
	/// the plugins with a studio module and its data
	pub plugins: Vec<PluginResp>,
	/// development mode, see `serve --dev`
	pub dev: bool,
	/// the svgs of the Lucide icons the schema names, see
	/// [`schema_icons`](super::schema_icons)
	pub icons: std::collections::BTreeMap<String, String>,
}
