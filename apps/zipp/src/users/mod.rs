pub mod api;
mod persistent;

use database::{
	id::{Id, Kind},
	Connection, Database, DatabaseKind,
};
use email_address::EmailAddress;
use fire_http::Resource;
use serde::{Deserialize, Serialize};

use crate::users::persistent::memory::Memory;

use self::persistent::{
	postgres::PostgresBuilder, InsertRawUser, RawUser, UsersPersistent,
	UsersPersistentBuilder,
};

pub const KIND: Kind = Kind::new(false, 1);

// contains all migration files
// const MIGRATIONS: &[Migration] = &[];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
	pub id: Id,
	pub email: EmailAddress,
	// pub rights: Rights,
}

impl From<RawUser> for User {
	fn from(user: RawUser) -> Self {
		Self {
			id: user.id,
			email: EmailAddress::new_unchecked(user.email),
		}
	}
}

#[derive(Debug, Clone)]
pub struct CreateUser {
	pub email: EmailAddress,
}

// #[derive(Debug, Clone)]
// pub struct Rights {
// 	inner: BTreeMap<String, bool>,
// }

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("a user with the email {email} already exists!")]
	AlreadyExists { email: String },

	#[error("a connection error occured!")]
	Connection(String),

	#[error("a postgres error occured!")]
	Postgres(#[from] database::Error),
}

#[derive(Debug, Resource)]
pub struct Users {
	inner: Box<dyn UsersPersistentBuilder>,
}

impl Users {
	pub async fn new(conn: &mut Database) -> Result<Self, Error> {
		let persistent: Box<dyn UsersPersistentBuilder> = match conn.kind() {
			DatabaseKind::Memory => Box::new(Memory::new()),
			DatabaseKind::Postgres => {
				Box::new(PostgresBuilder::new(conn).await?)
			}
		};

		Ok(Self { inner: persistent })
	}

	pub fn with_conn<'a>(&'a self, conn: Connection<'a>) -> UsersWithConn<'a> {
		UsersWithConn {
			inner: self.inner.with_conn(conn),
		}
	}
}

#[derive(Debug)]
pub struct UsersWithConn<'a> {
	inner: Box<dyn UsersPersistent + 'a>,
}

impl UsersWithConn<'_> {
	pub async fn create_user(&self, user: CreateUser) -> Result<User, Error> {
		let insert_user = InsertRawUser {
			email: user.email.as_ref(),
		};

		let user = self.inner.insert(insert_user).await?;

		Ok(user.into())
	}

	// pub async fn all_users(&self) -> Vec<User> {
	// 	let users = self.inner.read_all().await;

	// 	users.into_iter().map(Into::into).collect()
	// }

	pub async fn by_email(&self, email: &str) -> Result<Option<User>, Error> {
		let user = self.inner.by_email(email).await?;

		Ok(user.map(Into::into))
	}

	pub async fn by_id(&self, id: &Id) -> Result<Option<User>, Error> {
		let user = self.inner.by_id(id).await?;

		Ok(user.map(Into::into))
	}
}

impl Clone for Users {
	fn clone(&self) -> Self {
		Self {
			inner: self.inner.clone_box(),
		}
	}
}

#[cfg(test)]
mod tests {
	use database::DatabasePool;

	use super::*;

	#[tokio::test]
	async fn test_users() {
		let db = DatabasePool::new_memory();
		let mut db = db.get().await.unwrap();

		let users = Users::new(&mut db).await.unwrap();
		let users = users.with_conn(db.connection());

		let user = users
			.create_user(CreateUser {
				email: "rust@rust.com".parse().unwrap(),
			})
			.await
			.unwrap();

		assert_eq!(user.email.as_ref(), "rust@rust.com");

		let n_user =
			users.by_email(user.email.as_ref()).await.unwrap().unwrap();
		assert_eq!(n_user.id, user.id);

		let n_user = users.by_id(&user.id).await.unwrap().unwrap();
		assert_eq!(n_user.id, user.id);
	}
}
