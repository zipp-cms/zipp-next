use database::DatabasePool;
use fire_http::FireBuilder;
use fire_http_api::api;
use tracing::info;

use super::{Error, Login, LoginReq};
use crate::users::Users;

#[api(LoginReq)]
pub async fn login(
	req: LoginReq,
	users: &Users,
	db: &DatabasePool,
) -> Result<Login, Error> {
	let db = db.get().await.map_err(Error::string_internal)?;
	let users = users.with_conn(db.connection());

	let user = users
		.by_email(req.email.as_ref())
		.await
		.map_err(Error::string_internal)?;

	info!("login user: {:?}", user);

	todo!()
}

pub fn register(fire: &mut FireBuilder) {
	fire.add_route(login);
}
