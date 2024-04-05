//! Database layer
//!
//! This crate provides an abstraction around tokio postgres and some
//! helper functions to implement a memory database.

pub mod error;
pub mod id;
pub mod types;

use error::Error;
use serde_json::Value;
use types::guards::Valid;
