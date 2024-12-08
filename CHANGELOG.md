# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
