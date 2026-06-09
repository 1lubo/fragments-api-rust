//! Library crate root.
//!
//! Java/Spring parallel: think of this as the place that declares which
//! packages exist. Declaring the modules here makes them visible to the binary
//! (`main.rs`) AND to the integration tests in `tests/` — tests can only see
//! items exposed through the library crate.

pub mod api;
pub mod error;
pub mod model;
pub mod repository;
pub mod state;
