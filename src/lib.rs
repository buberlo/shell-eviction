//! Shell Eviction library.
//!
//! This crate exposes the core components used by the `shell-eviction`
//! binary:
//!
//! - shell history import and normalization,
//! - directory dwell-time and failure metrics,
//! - eviction notice generation,
//! - weekly tenant report construction,
//! - dry-run cleanup suggestion generation,
//! - human-readable and JSON output formatting.

pub mod cli;
pub mod error;
pub mod history;
pub mod metrics;
pub mod model;
pub mod notice;
pub mod output;
pub mod report;
pub mod suggestions;

pub use error::Error;

/// Crate-wide result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Crate version as declared in `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the crate version.
pub fn version() -> &'static str {
    VERSION
}