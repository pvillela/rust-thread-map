## Benchmarks

The [compare_bench](https://github.com/pvillela/rust-thread-map/tree/main/benches/compare_bench.rs) benchmark compares the latency of [`ThreadMap`] and [`ThreadMapX`] in a fairly typical scenario. For each of these two structs, the scenario defines a closure that spawns 5 threads and executes methods of the struct. The benchmark uses the [bench_diff](https://crates.io/crates/bench_diff), which supports reliable latency comparison between closures.

This benchmark was run 200 times, each with a sample size of 1000 executions of each closure. In about 90% of the 200 repetitions, [`ThreadMap`]'s latency was assessed not significantly different from [`ThreadMapX`]'s. One may conclude that the two structs have similar performance for typical usage.