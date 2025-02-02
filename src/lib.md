This library provides simple and easy-to-use alternatives to the [`std::thread_local`] macro and the [`thread_local`](https://crates.io/crates/thread_local) crate.

Two main types are provided, [`ThreadMap`] and [`ThreadMapX`], that have identical APIs but slightly different implementations that may be more or less efficient depending on the use case (see type [`ThreadMapX`] docs).

## Typical Usage Workflow

These are the steps typically followed when using this library:

1. Instantiate either [`ThreadMap`] or [`ThreadMapX`], wrap the instance in `Arc`, and name it `tm` for example.
2. Spawn threads that enclose a clone of `tm`. If scoped threads are used, `Arc` is not required in the above step and instead a regular reference `&tm` can be used in the thread.
3. Within each thread, read and/or modify the thread-local value by calling API methods on the `tm` clone or reference.
4. Optionally, from the main thread, before the spawned threads terminate, inspect the thread-local values using the API.
5. Optionally, from the main thread, once the spawned threads have terminated, inspect or extract all the thread-local values using the API.

## How it Differs from `std::thread_local!` and `thread_local::ThreadLocal`

While `std::thread_local!` and `thread_local::ThreadLocal` are optimized for efficiency, their usage can be more cumbersome in many cases. See below an example comparing the usage of `std::thread_local!` and [`ThreadMap`]. In particular, steps 4 and 5 above are not straightforward to do with these other thread-local approaches (but see [`thread_local_collect::tlm`](https://docs.rs/thread_local_collect/latest/thread_local_collect/tlm/index.html) and [`thread_local_collect::tlcr`](https://docs.rs/thread_local_collect/latest/thread_local_collect/tlcr/index.html) for ways to do it).

## Usage Examples

See [`ThreadMap`] and [`ThreadMapX`].

## Example Comparison with `std::thread_local!`

