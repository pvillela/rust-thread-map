//! Benchmark to compare `thread_map` and `thread_map_x`.

mod bench_support;

use bench_support::{Tm, bench_compare};
use thread_map::{ThreadMap, ThreadMapLockError, ThreadMapX};

impl<V: Clone> Tm<V> for ThreadMapX<V> {
    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W {
        self.with_mut(f)
    }

    fn get(&self) -> V {
        self.get()
    }

    fn fold_values<W>(&self, z: W, f: impl Fn(W, &V) -> W) -> Result<W, ThreadMapLockError> {
        self.fold_values(z, f)
    }

    fn type_name(&self) -> &'static str {
        "ThreadMapX"
    }
}

fn main() {
    let ftm1 = || ThreadMap::default();
    let ftm2 = || ThreadMapX::default();
    bench_compare(ftm1, ftm2);
}
