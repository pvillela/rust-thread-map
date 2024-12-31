use std::{
    error::Error,
    fmt::{Debug, Display},
    sync::PoisonError,
};

pub(crate) const POISONED_OBJECT_READ_LOCK: &str = "poisoned object read lock";
pub(crate) const POISONED_OBJECT_WRITE_LOCK: &str = "poisoned object write lock";
pub(crate) const POISONED_THREAD_LOCK: &str = "poisoned thread lock";

/// Error emitted by some [`crate::ThreadMap`] and [`crate::ThreadMapX`] methods when the object-level internal lock is poisoned.
#[derive(Debug)]
pub struct ThreadMapLockError;

impl Display for ThreadMapLockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Debug).fmt(f)
    }
}

impl Error for ThreadMapLockError {}

impl<T> From<PoisonError<T>> for ThreadMapLockError {
    fn from(_: PoisonError<T>) -> Self {
        Self
    }
}
