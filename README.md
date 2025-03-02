# `wasm-testsuite`

This repository contains copies of the [WebAssembly Test Suite](https://github.com/WebAssembly/spec/tree/main/test) for all WebAssembly versions and proposals (even those that are now part of the spec).

Older tests have been ported to newer versions of the wast file format, and some tests have been modified to not fail due to relaxed validation rules in later versions.

# Rust Crate

All tests are also available as a Rust crate, `wasm-testsuite`, which provides utilities for iterating over the tests and parsing the wast files.

[![crates.io](https://img.shields.io/crates/v/wasm-testsuite.svg)](https://crates.io/crates/wasm-testsuite)
[![docs.rs](https://docs.rs/wasm-testsuite/badge.svg)](https://docs.rs/wasm-testsuite)

Tests cases and the wast version can change between minor versions of this crate as new tests are added or existing tests are modified, so be sure to pin the version of this crate in your `Cargo.toml` (e.g. `wasm-testsuite = "=0.4.4"`).

## Usage

```rust
use wasm_testsuite::data::{Proposal, SpecVersion, proposal, spec};

fn main() -> eyre::Result<()> {
    for test in spec(&SpecVersion::V2) {
        let name = test.name();
        let raw = test.raw();
        let wast_directives = test.wast()?.directives();
    }

    for p in Proposal::all() {
        for test in proposal(p) {
            let name = test.name();
            let raw = test.raw();
            let wast_directives = test.wast()?.directives();
        }
    }

    Ok(())
}
```

## License

This crate is licensed under the [Apache License, Version 2.0](https://github.com/WebAssembly/spec/blob/main/test/LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `wasm-testsuite` by you, as defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.

**Note:** The tests contained in `data/` are from the [WebAssembly Test Suite](https://github.com/WebAssembly/spec) and are licensed under the Apache License, Version 2.0 as well.
