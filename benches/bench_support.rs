//! Provides support for benchmarks.

use bench_diff::{DiffOut, LatencyUnit, bench_diff_with_status, statistics::AltHyp};
use std::{
    hint::black_box,
    thread::{self},
};
use thread_map::{ThreadMap, ThreadMapLockError};

const NTHREADS: i32 = 5;
const NITER: i32 = 2_000;
const FOLD_RATIO: i32 = 100;
const EXEC_COUNT: usize = 1_000;
const NREPEATS: i32 = 100;

fn print_params() {
    println!(
        "Params: NTHREADS={NTHREADS}, NITER={NITER}, FOLD_RATIO={FOLD_RATIO}, EXEC_COUNT={EXEC_COUNT}, NREPEATS={NREPEATS}"
    );
}

fn update_value((i0, v0): &mut (i32, i32), j: i32) {
    *i0 = black_box(j);
    *v0 += black_box(j);
}

pub trait Tm<V: Clone> {
    fn with_mut<W>(&self, f: impl FnOnce(&mut V) -> W) -> W;
    fn get(&self) -> V;
    fn fold_values<W>(&self, z: W, f: impl Fn(W, &V) -> W) -> Result<W, ThreadMapLockError>;

    #[allow(unused)]
    fn type_name(&self) -> &'static str;
}

impl<V: Clone> Tm<V> for ThreadMap<V> {
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
        "ThreadMap"
    }
}

fn tm_bench(tm: impl Tm<(i32, i32)> + Sync) {
    let per_thread = |j: i32| {
        for i in 0..NITER {
            tm.with_mut(move |p: &mut (i32, i32)| update_value(p, j));

            let value = tm.get();
            black_box(value);

            if i % FOLD_RATIO == 0 {
                let sum = tm
                    .fold_values((0, 0), |acc, p| (acc.0 + p.0, acc.1 + p.1))
                    .unwrap();
                black_box(sum);
            }
        }

        // let sum = tm
        //     .fold_values((0, 0), |acc, p| (acc.0 + p.0, acc.1 + p.1))
        //     .unwrap();
        // println!(
        //     "type_name(tm)={}, thread_id={:?}, sum={:?}",
        //     tm.type_name(),
        //     thread::current().id(),
        //     sum
        // );
        // black_box(sum);
    };

    thread::scope(|s| {
        for j in 0..NTHREADS {
            s.spawn(move || per_thread(j));
        }

        // {
        //     per_thread(NTHREADS);
        // }
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

pub fn bench_compare<Tm1, Tm2>(ftm1: fn() -> Tm1, ftm2: fn() -> Tm2)
where
    Tm1: Tm<(i32, i32)> + Sync,
    Tm2: Tm<(i32, i32)> + Sync,
{
    let f1 = || tm_bench(ftm1());
    let f2 = || tm_bench(ftm2());

    let type_name1 = ftm1().type_name();
    let type_name2 = ftm2().type_name();

    print_params();
    println!();

    for i in 1..=NREPEATS {
        eprintln!("*** i={i} ***");
        let out = bench_diff_with_status(LatencyUnit::Nano, f1, f2, EXEC_COUNT, |_, _| {
            println!(">>> bench_diff comparison: {type_name1} vs. {type_name2}; *** i={i} ***")
        });
        print_diff_out(&out);
    }
}
