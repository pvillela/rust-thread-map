#![deny(clippy::unwrap_used)]
#![allow(clippy::type_complexity, clippy::new_without_default)]

mod thread_map_u;

pub use thread_map_u::*;

/// For backward compatibility only and eventually may be deprecated. The library's structs are now available
/// directly at top level.
pub mod thread_map {
    pub use super::thread_map_u::*;
}
