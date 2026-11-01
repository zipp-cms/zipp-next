use serde::{Deserialize, Serialize};

use super::tokens::{GraphqlToken, Scope};
use crate::request;

/// The tokens of frontends, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokensReq {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokensResp {
	pub tokens: Vec<GraphqlToken>,
	/// whether `serve --graphql-token` set one, it reads everything
	pub env_token: bool,
}

request!(TokensReq, GET "/graphql/tokens" => TokensResp);

/// A new token for a frontend, for admins.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenReq {
	pub name: String,
	#[serde(flatten)]
	pub scope: Scope,
	/// in milliseconds, `None` never
	pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatedToken {
	#[serde(flatten)]
	pub info: GraphqlToken,
	/// the token itself, it shows only now
	pub token: String,
}

request!(CreateTokenReq, POST "/graphql/tokens" => CreatedToken);

/// Revokes a token, for admins.
#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTokenReq {
	pub id: i64,
}

request!(DeleteTokenReq, DELETE "/graphql/tokens/{id}" => ());
