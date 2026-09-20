//! Auto-updated, type-safe Steam Protobuf definitions for Rust.

/// Steam build metadata used to generate these definitions.
pub const STEAM_BUILD: &str = "0.1.10";

/// Timestamp when these definitions were generated.
pub const GENERATED_AT: &str = "2026-09-20";

/// Re-export prost for downstream convenience.
pub use prost;

#[allow(clippy::all, unused_imports)]
mod generated;

pub use generated::*;
