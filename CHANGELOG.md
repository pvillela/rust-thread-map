# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.3] - 2025-04-24

### Added

- Benchmark comparing  `ThreadMap` and `ThreadLocal`.

### Changed

- Fixed the benchmark comparing `ThreadMap` and `ThreadMapX` from the previous version, as it was properly constructed.
- Benchmarks section of library documentation.

## [1.0.2] - 2025-04-24

This release is backward compatible with the previous one.

### Added

- Benchmark comparing `ThreadMap` and `ThreadMapX`.

## [1.0.1] - 2025-02-05

This release is backward compatible with the previous one.

### Changed

Tagged development-only module with `#[cfg(test)]` to prevent it from being unnecessarily compiled by clients of the library.

## [1.0.0] - 2025-02-02

This release is backward compatible with the previous one.

### Added

- Expanded API with `get`, `set`, and `default` methods.
- Added examples.
- Added lib level doc comments.

### Changed

- Improved and expanded existing type level doc comments.

## [0.1.0] - 2024-12-31

This release is backward compatible with the previous one.

### Changed

- Added `ThreadMapX` type that contains a `Mutex` for each value, allowing the methods `Self::fold`, `Self::fold_values`, and `Self::probe` to run more efficiently when there are concurrent calls to the per-thread methods (`Self::with` and `Self::with_mut`) by using fine-grained per-thread locking instead of acquiring an object-level write lock. On the other hand, the per-thread methods may run a bit slower as they require the acquision of the per-thread lock.

## [0.0.2] - 2024-12-30

This release is backward compatible with the previous one.

### Changed

- Pulled structs into library top level, retaining a `thread_map` module with `pub use` of the structs for backward compatibility.

## [0.0.1] - 2024-12-29

Initial release.
