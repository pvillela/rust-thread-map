//! Benchmark to compare `thread_map` and `thread_map_x`.

mod work_fns;

use bench_diff::{DiffOut, LatencyUnit, bench_diff_with_status, statistics::AltHyp};
use std::{
    collections::HashMap,
    hint::black_box,
    thread::{self, ThreadId},
    time::Duration,
};
use thread_map::{ThreadMap, ThreadMapLockError, ThreadMapX};
use work_fns::{busy_work, calibrate_busy_work};

const NTHREADS: i32 = 5;
const NITER: i32 = 10;

fn update_value((i0, v0): &mut (i32, i32), i: i32) {
    *i0 = black_box(i);
    *v0 += black_box(i);
}

trait Tm<V> {
    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W;
    fn get(&self) -> V;
    fn probe(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError>;
}

impl<V: Clone> Tm<V> for ThreadMap<V> {
    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W {
        self.with_mut(f)
    }

    fn get(&self) -> V {
        self.get()
    }

    fn probe(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError> {
        self.probe()
    }
}

impl<V: Clone> Tm<V> for ThreadMapX<V> {
    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W {
        self.with_mut(f)
    }

    fn get(&self) -> V {
        self.get()
    }

    fn probe(&self) -> Result<HashMap<ThreadId, V>, ThreadMapLockError> {
        self.probe()
    }
}

fn tm_bench(tm: impl Tm<(i32, i32)> + Sync, work_fn: impl Fn() + Sync) {
    thread::scope(|s| {
        let tm = &tm;
        for i in 0..NTHREADS {
            let work_fn = &work_fn;
            s.spawn(move || {
                for _ in 0..NITER {
                    work_fn();
                    tm.with_mut(move |p: &mut (i32, i32)| update_value(p, i));
                }
                let value = tm.get();
                black_box(value);
            });
        }

        let probed = tm.probe().unwrap().into_values().collect::<HashMap<_, _>>();
        black_box(probed);

        for _ in 0..NITER {
            tm.with_mut(move |p: &mut (i32, i32)| update_value(p, NTHREADS))
        }

        let probed = tm.probe().unwrap().into_values().collect::<HashMap<_, _>>();
        black_box(probed);
    });
}

fn print_diff_out(out: &DiffOut) {
    const ALPHA: f64 = 0.05;

    println!();
    println!("ratio_medians_f1_f2={}", out.ratio_medians_f1_f2(),);
    println!("student_ratio_ci={:?}", out.student_ratio_ci(ALPHA),);
    println!(
        "student_diff_ln_test_lt:{:?}",
        out.student_diff_ln_test(AltHyp::Lt, ALPHA)
    );
    println!(
        "student_diff_ln_test_gt:{:?}",
        out.student_diff_ln_test(AltHyp::Gt, ALPHA)
    );
    println!();
    println!("summary_f1={:?}", out.summary_f1());
    println!();
    println!("summary_f2={:?}", out.summary_f2());
    println!();
}

fn main() {
    let effort = calibrate_busy_work(Duration::from_micros(100));
    let work_fn = || busy_work(effort);

    let f1 = || tm_bench(ThreadMap::default(), work_fn);
    let f2 = || tm_bench(ThreadMapX::default(), work_fn);

    let out = bench_diff_with_status(LatencyUnit::Nano, f1, f2, 1000, |_, _| ());
    print_diff_out(&out);
}
