# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.8] - 2025-06-13

- Updated to `wast` 234

## [0.5.7] - 2025-05-29

- Updated to `wast` 232

## [0.5.6] - 2025-05-24

- Updated tests
- Updated to `wast` 231

## [0.5.5] - 2025-05-19

- Updated tests

## [0.5.4] - 2025-05-06

- Updated to `wast` 230

## [0.5.3] - 2025-04-23

- Updated to `wast` 229
- Allow owned `Proposal` and `SpecVersion` values in `spec` and `proposal` functions

## [0.5.1] - 2025-04-03

- Updated to `wast` 228
- Updated tests

## [0.5.1] - 2025-02-06

- Updated to `wast` 227
- Updated tests

## [0.5.0] - 2025-03-02

- Fixed the `memory64` proposal (has now been merged into wasm-3.0)
- Added old proposals that have since been stabilized (`multi-value`, `mutable-global`, `nontrapping-floating-point-conversions`, `sign-extension-ops`, `reference-types`, `bulk-memory-operations`)

## [0.4.5] - 2025-02-25

- Added `simd` proposal

## [0.4.4] - 2025-02-25

- Updated to `wast` 226
- Updated to rust 2024 / MSRV 1.85

## [0.4.3] - 2025-01-25

- Updated to `wast` 225
- Updated tests

## [0.4.2] - 2025-01-25

- Updated tests

## [0.4.1] - 2025-01-23

- Updated to `wast` 224
- Updated tests

## [0.4.0] - 2024-12-28

- Updated to `wast` 222
- Updated tests
- Set MSRV to 1.83

## [0.3.3] - 2024-12-08

### Changes

- Updated tests from upstream
- Patch wasm V1 to be compatible with later proposals

## [0.3.2] - 2024-12-06

### Changes

- Allow constructing `TestFile`'s

## [0.3.1] - 2024-12-05

### Fixes

- Fixed the return type of the `raw` method to return a `&str`
- Added `repository` and `authors` fields to the `Cargo.toml` file

## [0.3.0] - 2024-12-05

Initial release of `wasm-testsuite` as a standalone crate.
