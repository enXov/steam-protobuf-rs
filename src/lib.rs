//! Auto-updated, type-safe Steam Protobuf definitions for Rust.

/// Steam client build number from Valve's CDN manifest.
pub const STEAM_BUILD: &str = "0.1.11";

/// Timestamp when these definitions were generated.
pub const GENERATED_AT: &str = "2026-09-23T21:46:20Z";

/// Re-export prost for downstream convenience.
pub use prost;

#[allow(clippy::all, unused_imports)]
mod generated;

pub use generated::*;
