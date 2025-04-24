//! Provides support for benchmarks.

use bench_diff::{DiffOut, statistics::AltHyp};
use std::{hint::black_box, thread};
use thread_map::{ThreadMap, ThreadMapLockError};

const NTHREADS: i32 = 5;
const NITER: i32 = 1_000;
const FOLD_RATIO: i32 = 100;
pub const EXEC_COUNT: usize = 1_000;

fn update_value((i0, v0): &mut (i32, i32), i: i32) {
    *i0 = black_box(i);
    *v0 += black_box(i);
}

pub trait Tm<V: Clone> {
    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W;
    fn get(&self) -> V;
    fn fold_values<W>(&self, z: W, f: impl FnMut(W, &V) -> W) -> Result<W, ThreadMapLockError>;
}

impl<V: Clone> Tm<V> for ThreadMap<V> {
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

pub fn tm_bench(tm: impl Tm<(i32, i32)> + Sync) {
    let per_thread = |i: i32| {
        for _ in 0..NITER {
            tm.with_mut(move |p: &mut (i32, i32)| update_value(p, i));
        }
        let value = tm.get();
        black_box(value);

        if i % FOLD_RATIO == 0 {
            let sum = tm
                .fold_values((0, 0), |acc, p| (acc.0 + p.0, acc.1 + p.1))
                .unwrap();
            black_box(sum);
        }
    };

    thread::scope(|s| {
        for i in 0..NTHREADS {
            s.spawn(move || per_thread(i));
        }

        {
            per_thread(NTHREADS);
        }
    });
}

pub fn print_diff_out(out: &DiffOut) {
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
