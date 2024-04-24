use database::{
	id::Id,
	memory::{ReadWrite, Table},
	Connection,
};

use crate::users::KIND;

use super::{
	Error, InsertRawUser, RawUser, UsersPersistent, UsersPersistentBuilder,
};

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

impl UsersPersistentBuilder for Memory {
	fn with_conn(&self, conn: Connection<'_>) -> Box<dyn UsersPersistent> {
		let _conn = conn.into_memory();

		Box::new(Self {
			inner: self.inner.clone(),
		})
	}

	fn clone_box(&self) -> Box<dyn UsersPersistentBuilder> {
		Box::new(Self {
			inner: self.inner.clone(),
		})
	}
}

#[async_trait::async_trait]
impl UsersPersistent for Memory {
	async fn insert(&self, user: InsertRawUser<'_>) -> Result<RawUser, Error> {
		let mut table = self.inner.write();

		// check email does not exist
		if table.any(|u| u.email == user.email) {
			return Err(Error::AlreadyExists {
				email: user.email.to_string(),
			});
		}

		let id = Id::new(KIND);

		let raw_user = RawUser {
			id,
			email: user.email.to_string(),
		};

		table.insert(id, raw_user.clone()).unwrap();

		Ok(raw_user)
	}

	async fn by_email(&self, email: &str) -> Result<Option<RawUser>, Error> {
		let table = self.inner.read();

		Ok(table.find(|u| u.email == email).cloned())
	}

	async fn by_id(&self, id: &Id) -> Result<Option<RawUser>, Error> {
		let table = self.inner.read();

		Ok(table.get(id).cloned())
	}
}
