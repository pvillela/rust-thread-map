## Benchmarks

The benchmarks use the [bench_diff](https://crates.io/crates/bench_diff), which supports reliable latency comparison between closures.

A fairly typical scenario was defined for the comparisons. The scenario defines a closure that spawns 5 threads and executes methods of the struct, as follows:
- There were two scenario variants.
- The following parameters were common to the two variants:
  - Number of threads: 5.
  - Number of iterations within each thread: 2000.
  - One write operation and one read operation per iteration.
- The two variants differed as follows:
  - Variant 1 -- The sum of the values in all thread buckets was computed on each thread every 100 iterations.
  - Variant 2 -- The sum of the values in all thread buckets was computed on each thread every 50 iterations.

**ThreadMap vs. ThreadMapX**

- Each of the scenario variants was run 100 times, each with a sample size of 1000 executions of each closure.
- For Variant 1, there was a mix of results. One would conclude that the latency of the `ThreadMap` closure tended to be somewhat lower but was not consistently statistically significantly lower than for `ThreadMapX`. (`bench_diff` substantially mitigates the problem of time-dependent noise in latency comparisons, but it does eliminate the issue.) Latencies were around 3ms.
- For Variant 2, the situation was reversed, with `ThreadMap` tending to be slower (higher latencies) than `ThreadMapX`. Latencies were slightly higher but still around 3ms.
- On can conclude that the frequency of sweep opearations (operations that combine data from each thread) has an impact on the relative efficiency of the two structs. `ThreadMap` tends to be faster than `ThreadMapX` when sweep operations are infrequent, and slower when sweep operations are frequent.

**ThreadMap vs. ThreadLocal**

- As above, each of the scenario variants was run 100 times, each with a sample size of 1000 executions of each closure.
- As discussed in an earlier section, [`ThreadLocal`](https://crates.io/crates/thread_local) is optimized for speed but its use requires care as its internal thread IDs are reused (unlike Rust's standard `ThreadId`).
- In all cases, the latency of the `ThreadMap` closure was substantially higher, approximately 5-6 times as high as the latency of the `ThreadLocal` closure. As before, the latencies for the `ThreadMap` runs were around 3ms. For the `ThreadLocal` runs, the latencies were around 500μs.
- On can conclude that:
  - For performance-sensitive applications, where the data structure is accessed frequently on many threads, `ThreadLocal` would be a good choice, with the caveat (discussed earlier) about the impact of its reuse of internal thread IDs.
  - For applications where the data structure is not as heavily accessed, `ThreadMap` or `ThreadMapX` can provide a convenient, more ergonomic alternative.

For an alternative that takes advantage of the efficiency of `ThreadLocal` while addressing the above-mentioned caveat, consider using crate [thread_local_collect](https://crates.io/crates/thread_local_collect).

