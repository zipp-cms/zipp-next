use std::marker::PhantomData;

use database::{
	id::Id,
	memory::{self, ReadWrite, Table},
	Connection,
};

use crate::users::KIND;

use super::{Error, InsertRawUser, RawUser, UsersPersistent};

#[derive(Debug, Clone)]
pub struct Memory {
	inner: ReadWrite<Table<Id, RawUser>>,
}

impl Memory {
	pub fn new() -> Self {
		Self {
			inner: ReadWrite::new(Table::new()),
		}
	}
}

#[async_trait::async_trait]
impl UsersPersistent for Memory {
	async fn insert(
		&self,
		conn: Connection<'_>,
		user: InsertRawUser,
	) -> Result<RawUser, Error> {
		let _conn = conn.into_memory();
		let mut table = self.inner.write();

		// check email does not exist
		if table.any(|u| u.email == user.email) {
			return Err(Error::AlreadyExists { email: user.email });
		}

		let id = Id::new(KIND);

		let raw_user = RawUser {
			id,
			email: user.email,
		};

		table.insert(id, raw_user.clone()).unwrap();

		Ok(raw_user)
	}

	async fn by_email(
		&self,
		conn: Connection<'_>,
		email: &str,
	) -> Result<Option<RawUser>, Error> {
		let _conn = conn.into_memory();
		let table = self.inner.read();

		Ok(table.find(|u| u.email == email).cloned())
	}

	async fn by_id(
		&self,
		conn: Connection<'_>,
		id: &Id,
	) -> Result<Option<RawUser>, Error> {
		let _conn = conn.into_memory();
		let table = self.inner.read();

		Ok(table.get(id).cloned())
	}

	fn clone_box(&self) -> Box<dyn UsersPersistent> {
		Box::new(Self {
			inner: self.inner.clone(),
		})
	}
}
