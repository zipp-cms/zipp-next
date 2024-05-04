mod json;
mod memory;

use std::fmt::Debug;
use std::io;

use crate::fields::{Fields, ParseFieldError};

use super::schema::ComponentSchema;

#[derive(Debug, thiserror::Error)]
pub enum PersistentError {
	#[error("IO error: {error} for file: {file_name}")]
	Io { error: io::Error, file_name: String },

	#[error("JSON error: {error} for file: {file_name}")]
	Json {
		error: serde_json::Error,
		file_name: String,
	},

	#[error("Parse error: {error} for file: {file_name}")]
	Parse {
		error: ParseFieldError,
		file_name: String,
	},
}

#[async_trait::async_trait]
pub trait Persistent: Debug + Send + Sync + 'static {
	async fn load(
		&mut self,
		fields: &Fields,
	) -> Result<Vec<ComponentSchema>, PersistentError>;

	async fn save(
		&mut self,
		contents: &[ComponentSchema],
	) -> Result<(), PersistentError>;
}

pub fn new_memory() -> memory::Memory {
	memory::Memory::new()
}

pub fn new_file(file_name: &str) -> json::JsonStorage {
	json::JsonStorage::new(file_name)
}

impl PersistentError {
	pub fn io(error: io::Error, file_name: impl Into<String>) -> Self {
		Self::Io {
			error,
			file_name: file_name.into(),
		}
	}

	pub fn json(
		error: serde_json::Error,
		file_name: impl Into<String>,
	) -> Self {
		Self::Json {
			error,
			file_name: file_name.into(),
		}
	}

	pub fn parse(error: ParseFieldError, file_name: impl Into<String>) -> Self {
		Self::Parse {
			error,
			file_name: file_name.into(),
		}
	}
}
