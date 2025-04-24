//! Benchmark to compare `thread_map` and `thread_map_x`.

mod bench_support;

use bench_diff::{LatencyUnit, bench_diff_with_status};
use bench_support::{EXEC_COUNT, Tm, print_diff_out, tm_bench};
use thread_map::{ThreadMap, ThreadMapLockError, ThreadMapX};

impl<V: Clone> Tm<V> for ThreadMapX<V> {
    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W {
        self.with_mut(f)
    }

    fn get(&self) -> V {
        self.get()
    }

    fn fold_values<W>(&self, z: W, f: impl FnMut(W, &V) -> W) -> Result<W, ThreadMapLockError> {
        self.fold_values(z, f)
    }
}

fn main() {
    let f1 = || tm_bench(ThreadMap::default());
    let f2 = || tm_bench(ThreadMapX::default());

    let out = bench_diff_with_status(LatencyUnit::Nano, f1, f2, EXEC_COUNT, |_, _| ());
    print_diff_out(&out);
}
