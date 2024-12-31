use crate::{POISONED_OBJECT_READ_LOCK, POISONED_OBJECT_WRITE_LOCK, POISONED_THREAD_LOCK};

use super::ThreadMapLockError;
use std::{
    collections::HashMap,
    mem::take,
    ops::DerefMut,
    sync::{Mutex, RwLock},
    thread::{self, ThreadId},
};

/// Like [`crate::ThreadMap`],
/// this type encapsulates the association of [`ThreadId`]s to values of type `V` and is a simple and easy-to-use alternative
/// to the [`std::thread_local`] macro and the [`thread_local`](https://crates.io/crates/thread_local) crate.
/// It differs from [`crate::ThreadMap`] in that it contains a [`Mutex`] for each value, allowing the methods
/// [`Self::fold`], [`Self::fold_values`], and [`Self::probe`]
/// to run more efficiently when there are concurrent calls to the per-thread methods ([`Self::with`] and [`Self::with_mut`])
/// by using fine-grained per-thread locking instead of acquiring an object-level write lock.
/// On the other hand, the per-thread methods may run a bit slower as they require the acquision of the per-thread lock.
#[derive(Debug)]
pub struct ThreadMapX<V> {
    state: RwLock<HashMap<ThreadId, Mutex<V>>>,
    value_init: fn() -> V,
}

impl<V> ThreadMapX<V> {
    /// Creates a new [`ThreadMapX`] instance, with `value_init` used to create the initial value for each thread.
    pub fn new(value_init: fn() -> V) -> Self {
        Self {
            state: RwLock::new(HashMap::new()),
            value_init,
        }
    }

    /// Invokes `f` mutably on the value associated with the [`ThreadId`] of the current thread and returns the invocation result.
    /// If there is no value associated with the current thread then the `value_init` argument of [`Self::new`] is used
    /// to instantiate an initial associated value before `f` is applied.
    pub fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W {
        let lock = self.state.read().expect(POISONED_OBJECT_READ_LOCK);
        let tid = thread::current().id();
        match lock.get(&tid) {
            Some(c) => {
                let mut v = c.lock();
                let rv = v.as_mut().expect(POISONED_THREAD_LOCK);
                f(rv)
            }
            None => {
                // Drop read lock and acquire write lock.
                drop(lock);
                let mut lock = self.state.write().expect(POISONED_OBJECT_WRITE_LOCK);
                let mut v0 = (self.value_init)();
                let w = f(&mut v0);
                lock.insert(tid, Mutex::new(v0));
                w
            }
        }
    }

    /// Invokes `f` on the value associated with the [`ThreadId`] of the current thread and returns the invocation result.
    /// If there is no value associated with the current thread then the `value_init` argument of [`Self::new`] is used
    /// to instantiate an initial associated value before `f` is applied.
    pub fn with<W>(&self, f: impl FnOnce(&V) -> W) -> W {
        let g = |v: &mut V| f(v);
        self.with_mut(g)
    }

    /// Returns a [`HashMap`] with the values associated with each [`ThreadId`] key and clears `self`'s state.
    ///
    /// # Errors
    /// - [`ThreadMapLockError`] if the internal lock is poisoned.
    pub fn drain(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError> {
        let mut lock = self.state.write()?;
        let rmap = lock.deref_mut();
        let tmap = take(rmap);
        tmap.into_iter()
            .map(|(k, v)| {
                let v = v.into_inner()?;
                Ok((k, v))
            })
            .collect()
    }

    /// Folds every association in `self` into an accumulator (with initial value `z`) by applying an operation `f`,
    /// returning the final result.
    ///
    /// # Errors
    /// - [`ThreadMapLockError`] if the internal lock is poisoned.
    pub fn fold<W>(
        &self,
        z: W,
        mut f: impl FnMut(W, (ThreadId, &V)) -> W,
    ) -> Result<W, ThreadMapLockError> {
        self.state.read()?.iter().try_fold(z, |w, (tid, v)| {
            let tid = *tid;
            let mut mlock = v.lock()?;
            let v = mlock.deref_mut();
            Ok(f(w, (tid, v)))
        })
    }

    /// Folds every value in `self` into an accumulator (with initial value `z`) by applying an operation `f`,
    /// returning the final result.
    ///
    /// # Errors
    /// - [`ThreadMapLockError`] if the internal lock is poisoned.
    pub fn fold_values<W>(
        &self,
        z: W,
        mut f: impl FnMut(W, &V) -> W,
    ) -> Result<W, ThreadMapLockError> {
        self.fold(z, |w, (_, v)| f(w, v))
    }

    /// Returns a [`HashMap`] with clones of the values associated with each [`ThreadId`] key at the time the probe
    /// was executed.
    ///
    /// # Errors
    /// - [`ThreadMapLockError`] if the internal lock is poisoned.
    pub fn probe(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError>
    where
        V: Clone,
    {
        let z = HashMap::<ThreadId, V>::new();
        self.fold(z, |mut w, (tid, v)| {
            w.insert(tid, v.clone());
            w
        })
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        thread::{self},
        time::Duration,
    };

    use super::ThreadMapX;

    const NTHREADS: i32 = 20;
    const NITER: i32 = 10;
    const SLEEP_MICROS: u64 = 10;

    fn value_init() -> (i32, i32) {
        (0, 0)
    }

    fn g((i0, v0): &mut (i32, i32), i: i32) {
        *i0 = i;
        *v0 += i;
    }

    fn read_value(p: &(i32, i32)) -> (i32, i32) {
        (p.0, p.1)
    }

    #[test]
    fn test() {
        let tm = ThreadMapX::new(value_init);

        thread::scope(|s| {
            let tm = &tm;
            for i in 0..NTHREADS {
                let f = move |p: &mut (i32, i32)| g(p, i);
                s.spawn(move || {
                    for _ in 0..NITER {
                        thread::sleep(Duration::from_micros(SLEEP_MICROS));
                        tm.with_mut(f);
                    }
                    let value = tm.with(read_value);
                    assert_eq!((i, i * NITER), value);
                });
            }

            let probed = tm.probe().unwrap().into_values().collect::<HashMap<_, _>>();
            println!("probed={probed:?}");

            let f = move |p: &mut (i32, i32)| g(p, NTHREADS);
            for _ in 0..NITER {
                tm.with_mut(f)
            }

            let probed = tm.probe().unwrap().into_values().collect::<HashMap<_, _>>();
            println!("probed={probed:?}");
        });

        let expected = (0..=NTHREADS)
            .map(|i| (i, i * NITER))
            .collect::<HashMap<_, _>>();
        let expected_sum = expected.values().sum::<i32>();

        let sum = tm.fold_values(0, |z, (_, v)| z + v).unwrap();
        assert_eq!(expected_sum, sum);

        let probed = tm.probe().unwrap().into_values().collect::<HashMap<_, _>>();
        println!("probed={probed:?}");
        assert_eq!(expected, probed);

        let dumped = tm.drain().unwrap().into_values().collect::<HashMap<_, _>>();
        assert_eq!(expected, dumped);
    }
}
