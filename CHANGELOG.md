# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-02-02

This release is backward compatible with the previous one.

### Added

- Added examples.
- Added lib doc comments.
- Expanded API with `get`, `set`, and `default` methods.

### Changed

- Improved and expanded existing doc comments.

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
