//! Migrations
//!
//! How do migrations work
//!
//! A snapshot is a file which contains all previous migrations
//! A migration is an sql script which can be executed on the database

use std::{
	borrow::Cow,
	sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct Migration {
	pub number: u32,
}

#[derive(Debug)]
pub enum MigrationKind {
	Sql(Cow<'static, str>),
	Snapshot(Cow<'static, str>),
}

/// Holds all migrations
///
/// and checks which migrations already ran, and runs the others
///
/// If no migration for the specific name was found, it runs the latest snapshot
#[derive(Debug, Clone)]
pub struct Migrations {
	inner: Arc<RwLock<Inner>>,
}

#[derive(Debug)]
struct Inner {
	// migrations: Vec<Migration>,
}

impl Migrations {
	/// Create a new Migrations
	pub(super) fn new() -> Self {
		Self {
			inner: Arc::new(RwLock::new(Inner {
				// migrations: Vec::new(),
			})),
		}
	}
}
