#![doc = include_str!("lib1.md")]
//!
//! ```rust
#![doc = include_str!("../examples/doc_comparative.rs")]
//! ```
#![doc = include_str!("lib2.md")]
#![deny(clippy::unwrap_used)]
#![allow(clippy::type_complexity, clippy::new_without_default)]

#[cfg(test)]
mod api_check;

mod common;
mod thread_map_u;
mod thread_map_x;

pub use common::*;
pub use thread_map_u::*;
pub use thread_map_x::*;

/// For backward compatibility only and eventually may be deprecated. The library's structs are now available
/// directly at top level.
pub mod thread_map {
    pub use super::common::ThreadMapLockError;
    pub use super::thread_map_u::ThreadMap;
}
