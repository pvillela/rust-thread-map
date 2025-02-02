//! This private module defines the common API for [`ThreadMap`] and [`ThreadMapX`] and ensures both implement the API.

use crate::{ThreadMap, ThreadMapLockError, ThreadMapX};
use std::{collections::HashMap, thread::ThreadId};

#[allow(unused)]
trait ApiCheck<V> {
    fn new(value_init: fn() -> V) -> Self;

    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W;

    fn with<W>(&self, f: impl FnOnce(&V) -> W) -> W;

    fn get(&self) -> V
    where
        V: Clone;

    fn set(&self, v: V);

    fn drain(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError>;

    fn fold<W>(&self, z: W, f: impl FnMut(W, (ThreadId, &V)) -> W)
        -> Result<W, ThreadMapLockError>;

    fn fold_values<W>(&self, z: W, f: impl FnMut(W, &V) -> W) -> Result<W, ThreadMapLockError>;

    fn probe(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError>
    where
        V: Clone;
}

impl<V> ApiCheck<V> for ThreadMap<V> {
    fn new(value_init: fn() -> V) -> Self {
        Self::new(value_init)
    }

    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W {
        self.with_mut(f)
    }

    fn with<W>(&self, f: impl FnOnce(&V) -> W) -> W {
        self.with(f)
    }

    fn get(&self) -> V
    where
        V: Clone,
    {
        self.get()
    }

    fn set(&self, v: V) {
        self.set(v);
    }

    fn drain(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError> {
        self.drain()
    }

    fn fold<W>(
        &self,
        z: W,
        f: impl FnMut(W, (ThreadId, &V)) -> W,
    ) -> Result<W, ThreadMapLockError> {
        self.fold(z, f)
    }

    fn fold_values<W>(&self, z: W, f: impl FnMut(W, &V) -> W) -> Result<W, ThreadMapLockError> {
        self.fold_values(z, f)
    }

    fn probe(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError>
    where
        V: Clone,
    {
        self.probe()
    }
}

impl<V> ApiCheck<V> for ThreadMapX<V> {
    fn new(value_init: fn() -> V) -> Self {
        Self::new(value_init)
    }

    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W {
        self.with_mut(f)
    }

    fn with<W>(&self, f: impl FnOnce(&V) -> W) -> W {
        self.with(f)
    }

    fn get(&self) -> V
    where
        V: Clone,
    {
        self.get()
    }

    fn set(&self, v: V) {
        self.set(v);
    }

    fn drain(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError> {
        self.drain()
    }

    fn fold<W>(
        &self,
        z: W,
        f: impl FnMut(W, (ThreadId, &V)) -> W,
    ) -> Result<W, ThreadMapLockError> {
        self.fold(z, f)
    }

    fn fold_values<W>(&self, z: W, f: impl FnMut(W, &V) -> W) -> Result<W, ThreadMapLockError> {
        self.fold_values(z, f)
    }

    fn probe(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError>
    where
        V: Clone,
    {
        self.probe()
    }
}
