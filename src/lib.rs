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
/// SteamLanguage enums (EMsg, EResult, EUniverse, etc.)
/// generated from SteamKit's `.steamd` files.
///
/// Enable the `steam-language` feature to use these types:
/// ```toml
/// [dependencies]
/// steam-protobuf-rs = { version = "...", features = ["steam-language"] }
/// ```
///
/// Access enums via `steam_protobuf_rs::steam_language::EResult`, etc.
/// Some enums (e.g. `EMsg`) exist in both protobuf and steam-language with
/// different variant sets — the steam-language version is more complete.
#[cfg(feature = "steam-language")]
#[allow(clippy::all, non_camel_case_types, non_upper_case_globals, dead_code)]
pub mod steam_language;
