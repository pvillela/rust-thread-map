## Benchmarks

The benchmarks use the [bench_diff](https://crates.io/crates/bench_diff), which supports reliable latency comparison between closures.

A fairly typical scenario was defined for the comparisons. The scenario defines a closure that spawns 5 threads and executes methods of the struct (see links below).

**ThreadMap vs. ThreadMapX**

- The [tmu_tmx_bench](https://github.com/pvillela/rust-thread-map/tree/main/benches/tmu_tmx_bench.rs) benchmark compares the latency of [`ThreadMap`] and [`ThreadMapX`]. 
- This benchmark was run 100 times, each with a sample size of 1000 executions of each closure. In these 100 repetitions, there was a mix of results. One would conclude that the latency of the `ThreadMap` closure tended to be somewhat lower but was not consistently statistically significantly lower than for `ThreadMapX`. (`bench_diff` substantially mitigates the problem of time-dependent noise in latency comparisons, but it does eliminate the issue.) Latencies were around 3ms.

**ThreadMap vs. ThreadLocal**

- The [tmu_tlc_bench](https://github.com/pvillela/rust-thread-map/tree/main/benches/tmu_tlc_bench.rs) benchmark compares the latency of [`ThreadMap`] and `ThreadLocal` from the [thread_local](https://crates.io/crates/thread_local) crate.
- As discussed in an earlier section, `ThreadLocal` is optimized for speed but its use requires care as its internal thread IDs are reused (unlike Rust's standard `ThreadId`).
- This benchmark was run 100 times, each with a sample size of 1000 executions of each closure. In all of the 100 repetitions, the latency of the `ThreadMap` closure was substantially higher, approximately 5-6 times as high as the latency of the `ThreadLocal` closure. As before, the latencies for the `ThreadMap` runs were around 3ms. For the `ThreadLocal` runs, the latencies were around 500μs.
