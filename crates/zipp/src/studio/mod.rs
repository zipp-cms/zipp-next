//! The studio, built from `studio/` of the repository into
//! `crates/zipp/studio` (`npm run build`), so a site needs no npm.

pub mod api;
#[cfg(test)]
mod api_tests;
mod routes;

pub use routes::{api_routes, routes};

#[derive(rust_embed::Embed)]
#[folder = "studio/"]
struct Files;
