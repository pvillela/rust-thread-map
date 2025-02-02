#![doc = include_str!("lib.md")]
//!
//! ```rust
#![doc = include_str!("../examples/doc_comparative.rs")]
//! ```

#![deny(clippy::unwrap_used)]
#![allow(clippy::type_complexity, clippy::new_without_default)]

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
