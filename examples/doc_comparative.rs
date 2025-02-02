//! This example provides a direct comparison of the usage of [`ThreadMap`] and the
//! `std::thread_local!` macro.
//!
//! Lines that are specific to `ThreadMap` usage are prefixed with
//! ```
//! // **ThreadMap**
//! ```
//! and lines that are specific to `std::thread_local!` usage are prefixed with
//! ```
//! // **std::thread_local**
//! ```

use std::{
    cell::Cell,
    sync::Arc,
    thread::{self},
    time::Duration,
};
use thread_map::ThreadMap;

const NTHREADS: i32 = 20;
const NITER: i32 = 10;
const SLEEP_MICROS: u64 = 10;

// **std::thread_local**
thread_local! {
    static TL: Cell<i32> = Cell::new(0);
}

#[test]
fn test() {
    main();
}

fn main() {
    // **ThreadMap**
    // There is no real need to wrap in `Arc` here because references can be used in scoped threads instead
    // of clones, but the `Arc` wrapper would be required if non-scoped threads were used.
    let tm = Arc::new(ThreadMap::default());

    thread::scope(|s| {
        for i in 0..NTHREADS {
            // **ThreadMap**
            let tm = tm.clone();

            s.spawn(move || {
                for _ in 0..NITER {
                    thread::sleep(Duration::from_micros(SLEEP_MICROS));

                    // **ThreadMap**
                    tm.with_mut(move |i0: &mut i32| *i0 += i);

                    // **std::thread_local**
                    TL.with(move |i0: &Cell<i32>| i0.replace(i0.get() + i));
                }

                // **ThreadMap**
                {
                    let value = tm.get();
                    assert_eq!(i * NITER, value);
                }

                // **std::thread_local**
                {
                    let value = TL.with(Cell::get);
                    assert_eq!(i * NITER, value);
                }
            });
        }

        // **ThreadMap**
        {
            // Snapshot before thread-local value in main thread is updated.
            let probed = tm.probe().unwrap();
            println!("probed={probed:?}");
        }

        // **std::thread_local**
        {
            // Can't do something similar to the above block
        }

        // **ThreadMap**
        for _ in 0..NITER {
            tm.with_mut(|i0: &mut i32| *i0 += NTHREADS)
        }

        // **std::thread_local**
        for _ in 0..NITER {
            TL.with(|i0: &Cell<i32>| i0.replace(i0.get() + NTHREADS));
        }

        // **ThreadMap**
        {
            // Snapshot before all scoped threads terminate.
            let probed = tm.probe().unwrap();
            println!("\nprobed={probed:?}");
        }

        // **std::thread_local**
        {
            // Can't do something similar to the above block
        }
    });

    // **ThreadMap**
    {
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

    // **std::thread_local**
    {
        // Can't do something similar to the above block
    }
}
