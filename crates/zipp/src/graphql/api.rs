use serde::{Deserialize, Serialize};

use super::tokens::GraphqlToken;
use crate::request;
use crate::schema::GraphqlSchema;

/// The tokens of frontends, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TokensReq {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct TokensResp {
	pub tokens: Vec<GraphqlToken>,
	/// the GraphQL schemas of the code, a token gets one
	pub schemas: Vec<GraphqlSchema>,
	/// whether `serve --graphql-token` set one, it reads everything
	pub env_token: bool,
}

request!(TokensReq, GET "/graphql/tokens" => TokensResp);

/// A new token for a frontend, for admins.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateTokenReq {
	pub name: String,
	/// the handle of a [`GraphqlSchema`]
	pub schema: String,
	/// in milliseconds, `None` never
	pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct CreatedToken {
	#[serde(flatten)]
	pub info: GraphqlToken,
	/// the token itself, it shows only now
	pub token: String,
}

request!(CreateTokenReq, POST "/graphql/tokens" => CreatedToken);

/// Revokes a token, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DeleteTokenReq {
	pub id: i64,
}

request!(DeleteTokenReq, DELETE "/graphql/tokens/{id}" => ());
