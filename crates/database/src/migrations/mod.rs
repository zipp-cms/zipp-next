//! Migrations
//!
//! How do migrations work
//!
//! A snapshot is a file which contains all previous migrations
//! A migration is an sql script which can be executed on the database

use crate::{
	postgres::{error::Error, Connection},
	Connection as AnyConnection, Database,
};

use std::{borrow::Cow, collections::HashSet, sync::Arc};

use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use tokio::sync::RwLock;

// #[derive(Debug)]
// pub struct Migration {
// 	// this name needs to be unique
// 	pub name: String,
// 	pub sql: Cow<'static, str>,
// }

#[derive(Debug)]
pub struct ExecutedMigration {
	// this name is unique
	pub name: String,
	pub datetime: DateTime<Utc>,
}

#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
	#[error("Migration with the name {0} already exists")]
	AlreadyExists(String),

	#[error("Postgres error: {0}")]
	Postgres(#[from] Error),
}

/// Holds all migrations
///
/// and checks which migrations already ran, and runs the others
#[derive(Debug, Clone)]
pub struct Migrations {
	inner: Arc<RwLock<Inner>>,
}

#[derive(Debug)]
struct Inner {
	executed: IndexMap<String, ExecutedMigration>,
	added_migrations: HashSet<Cow<'static, str>>,
}

impl Migrations {
	/// Create a new Migrations
	pub(super) fn new() -> Self {
		Self {
			inner: Arc::new(RwLock::new(Inner {
				executed: IndexMap::new(),
				added_migrations: HashSet::new(),
			})),
		}
	}

	pub(super) async fn init(
		&self,
		db: &Database,
	) -> Result<(), MigrationError> {
		let conn = db.connection();
		let Some(pg) = conn.try_into_postgres() else {
			return Ok(());
		};

		let mut inner = self.inner.write().await;

		inner.init(&pg).await
	}

	pub async fn add(
		&self,
		conn: &AnyConnection<'_>,
		name: impl Into<Cow<'static, str>>,
		sql: impl Into<Cow<'static, str>>,
	) -> Result<(), MigrationError> {
		let Some(conn) = conn.try_into_postgres() else {
			return Ok(());
		};

		let mut inner = self.inner.write().await;

		inner.add(&conn, name.into(), sql.into()).await
	}
}

const TABLE_EXISTS: &str = "\
SELECT EXISTS (
	SELECT FROM information_schema.tables 
	WHERE table_schema = 'public' 
	AND table_name = 'migrations'
);";

const CREATE_TABLE: &str = "\
CREATE TABLE migrations (
    name text PRIMARY KEY,
    datetime timestamp with time zone
);

CREATE INDEX ON migrations (datetime);";

// now get all migrations in datetime ascending order
const SELECT_ALL_MIGRATIONS: &str = "\
SELECT name, datetime FROM migrations ORDER BY datetime ASC;";

impl Inner {
	async fn init(
		&mut self,
		pg: &Connection<'_>,
	) -> Result<(), MigrationError> {
		// check if the migrations table exists
		let result: bool = pg.query_one(TABLE_EXISTS, &[]).await?.get(0);

		if !result {
			pg.batch_execute(CREATE_TABLE).await?;
		}

		// now get all migrations in datetime ascending order
		let rows = pg.query(SELECT_ALL_MIGRATIONS, &[]).await?;

		for row in rows {
			self.executed.insert(
				row.get(0),
				ExecutedMigration {
					name: row.get(0),
					datetime: row.get(1),
				},
			);
		}

		Ok(())
	}

	async fn add(
		&mut self,
		conn: &Connection<'_>,
		name: Cow<'static, str>,
		sql: Cow<'static, str>,
	) -> Result<(), MigrationError> {
		// check if the migrations already exists
		if !self.added_migrations.insert(name.clone()) {
			return Err(MigrationError::AlreadyExists(name.into_owned()));
		}

		if self.executed.contains_key(name.as_ref()) {
			return Ok(());
		}

		conn.batch_execute(&sql).await?;

		Ok(())
	}
}
