//! Members of the website edit their own entries in the frontend, like
//! Craft users without access to the control panel: a member logs in on
//! the website (`/site-auth/login`), the frontend forwards their
//! requests to `/members/...` with the site token.
//!
//! ```ignore
//! Schema::new()
//!     // ...
//!     .role(
//!         Role::new("member", "Member")
//!             .own("recipes", Access::Delete)
//!             .assets(Access::Edit)
//!             .no_studio(),
//!     )
//!     .plugin(
//!         Members::new()
//!             .section("recipes")
//!             .images(10 * 1024 * 1024, "Members/{user}"),
//!     )
//! ```
//!
//! Only the sections of [`Members::section`] are edited here, of them
//! only the user's own entries, and only as far as their role allows:
//! every change goes through zipp's own checks, as in the studio
//! (`zipp::entries::create_as` and the others). See [`api`] for the
//! routes.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use zipp::{Plugin, Schema};

pub mod api;
mod routes;
#[cfg(test)]
mod tests;

/// What members edit on the website, see the crate.
#[derive(Debug, Clone, Default)]
pub struct Members {
	/// the sections members edit their entries of
	pub sections: Vec<String>,
	pub images: Option<Images>,
	/// the times of the changes per user, see [`routes`]
	writes: Arc<Mutex<HashMap<i64, Vec<i64>>>>,
}

/// The images members upload.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Images {
	pub max_bytes: u64,
	/// the asset folder, `{user}` the user's id: `Members/{user}`
	pub folder: String,
}

impl Members {
	pub fn new() -> Self {
		Self::default()
	}

	/// A section whose entries members edit, as far as their role
	/// allows; others are refused, also with access to them.
	pub fn section(mut self, handle: &str) -> Self {
		self.sections.push(handle.into());
		self
	}

	/// Members upload images up to `max_bytes` into the folder (`{user}`
	/// is the user's id), with the role's asset access.
	pub fn images(mut self, max_bytes: u64, folder: &str) -> Self {
		self.images = Some(Images {
			max_bytes,
			folder: folder.into(),
		});
		self
	}
}

impl Plugin for Members {
	fn name(&self) -> &'static str {
		"members"
	}

	fn check(&self, schema: &Schema) -> Result<(), String> {
		if self.sections.is_empty() {
			return Err("members: no section, add one with .section".into());
		}
		for s in &self.sections {
			if schema.get_section(s).is_none() {
				return Err(format!("members: there is no section {s}"));
			}
		}
		Ok(())
	}

	fn routes(&self, _schema: &Schema) -> zipp::axum::Router<zipp::AppState> {
		routes::routes(Arc::new(self.clone()))
	}
}
