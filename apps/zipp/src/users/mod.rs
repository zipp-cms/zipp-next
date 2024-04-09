pub mod api;
mod persistent;

use database::{
	id::{Id, Kind},
	migrations::MigrationError,
	postgres::error::{Error as PgError, GetError},
	Connection, Database, DatabaseKind,
};
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

use crate::users::persistent::memory::Memory;

use self::persistent::{
	postgres::Postgres, InsertRawUser, RawUser, UsersPersistent,
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
	Postgres(#[from] PgError),
}

#[derive(Debug)]
pub struct Users {
	inner: Box<dyn UsersPersistent>,
}

impl Users {
	pub async fn new(conn: &Database) -> Result<Self, MigrationError> {
		let persistent: Box<dyn UsersPersistent> = match conn.kind() {
			DatabaseKind::Memory => Box::new(Memory::new()),
			DatabaseKind::Postgres => Box::new(Postgres::new(conn).await?),
		};

		Ok(Self { inner: persistent })
	}

	pub async fn create_user(
		&self,
		conn: Connection<'_>,
		user: CreateUser,
	) -> Result<User, Error> {
		let insert_user = InsertRawUser {
			email: user.email.to_string(),
		};

		let user = self.inner.insert(conn, insert_user).await?;

		Ok(user.into())
	}

	// pub async fn all_users(&self) -> Vec<User> {
	// 	let users = self.inner.read_all().await;

	// 	users.into_iter().map(Into::into).collect()
	// }

	pub async fn by_email(
		&self,
		conn: Connection<'_>,
		email: &str,
	) -> Result<Option<User>, Error> {
		let user = self.inner.by_email(conn, email).await?;

		Ok(user.map(Into::into))
	}

	pub async fn by_id(
		&self,
		conn: Connection<'_>,
		id: &Id,
	) -> Result<Option<User>, Error> {
		let user = self.inner.by_id(conn, id).await?;

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
		let db = db.get().await.unwrap();
		let conn = db.connection();

		let users = Users::new(&db).await.unwrap();

		let user = users
			.create_user(
				conn,
				CreateUser {
					email: "rust@rust.com".parse().unwrap(),
				},
			)
			.await
			.unwrap();

		assert_eq!(user.email.as_ref(), "rust@rust.com");

		let n_user = users
			.by_email(conn, user.email.as_ref())
			.await
			.unwrap()
			.unwrap();
		assert_eq!(n_user.id, user.id);

		let n_user = users.by_id(conn, &user.id).await.unwrap().unwrap();
		assert_eq!(n_user.id, user.id);
	}
}
