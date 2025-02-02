use std::{
    sync::Arc,
    thread::{self},
    time::Duration,
};
use thread_map::ThreadMapX;

const NTHREADS: i32 = 20;
const NITER: i32 = 10;
const SLEEP_MICROS: u64 = 10;

#[test]
fn test() {
    main();
}

fn main() {
    // There is no real need to wrap in `Arc` here because references can be used in scoped threads instead
    // of clones, but the `Arc` wrapper would be required if non-scoped threads were used.
    let tm: Arc<ThreadMapX<i32>> = Arc::new(ThreadMapX::default());

    thread::scope(|s| {
        for i in 0..NTHREADS {
            let tm = tm.clone();
            s.spawn(move || {
                for _ in 0..NITER {
                    thread::sleep(Duration::from_micros(SLEEP_MICROS));
                    tm.with_mut(move |i0: &mut i32| *i0 += i);
                }
                let value = tm.get();
                assert_eq!(i * NITER, value);
            });
        }

        // Snapshot before thread-local value in main thread is updated.
        let probed = tm.probe().unwrap();
        println!("probed={probed:?}");

        for _ in 0..NITER {
            tm.with_mut(move |i0: &mut i32| *i0 += NTHREADS)
        }

        // Snapshot before all scoped threads terminate.
        let probed = tm.probe().unwrap();
        println!("\nprobed={probed:?}");
    });

    // Snapshot after all scoped threads terminate.
    let probed = tm.probe().unwrap();
    println!("\nprobed={probed:?}");

    let expected_sum = (0..=NTHREADS).map(|i| i * NITER).sum::<i32>();
    let sum = tm.fold_values(0, |z, v| z + v).unwrap();
    assert_eq!(expected_sum, sum);

    // Extracted values after all scoped threads terminate.
    let dumped = tm.drain().unwrap();
    println!("\ndumped={dumped:?}");
}
