//! Benchmark to compare `thread_map` and `thread_map_x`.

mod bench_support;

use bench_support::{Tm, bench_compare};
use std::{ops::Deref, sync::Mutex};
use thread_local::ThreadLocal;
use thread_map::{ThreadMap, ThreadMapLockError};

type Tl<V> = ThreadLocal<Mutex<V>>;

impl<V: Clone + Send + Default> Tm<V> for Tl<V> {
    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W {
        let cell = self.get_or(|| Mutex::new(V::default()));
        let mut x = cell.lock().unwrap();
        f(&mut x)
    }

    fn get(&self) -> V {
        let cell = self.get_or(|| Mutex::new(V::default()));
        cell.lock().unwrap().clone()
    }

    fn fold_values<W>(&self, z: W, f: impl Fn(W, &V) -> W) -> Result<W, ThreadMapLockError> {
        let iter = self.iter();
        let w = iter.fold(z, |acc, item| f(acc, item.lock().unwrap().deref()));
        Ok(w)
    }

    fn type_name(&self) -> &'static str {
        "ThreadLocal"
    }
}

fn main() {
    let ftm1 = || ThreadMap::default();
    let ftm2 = || Tl::default();
    bench_compare(ftm1, ftm2);
}
