# `wasm-testsuite`

> `wasm-testsuite` used to be part of [`tinywasm`](https://github.com/explodingcamera/tinywam) and was extracted into its own crate.
> Currently it only includes the test data and not a test runner.

[![crates.io](https://img.shields.io/crates/v/wasm-testsuite.svg)](https://crates.io/crates/wasm-testsuite)
[![docs.rs](https://docs.rs/wasm-testsuite/badge.svg)](https://docs.rs/wasm-testsuite)

This crate contains the the [WebAssembly Test Suite](https://github.com/WebAssembly/spec/tree/main/test) for all versions of the WebAssembly spec and new proposals that are not yet part of the spec. Tests cases and the wast version can change between minor versions of this crate as new tests are added or existing tests are modified, so be sure to pin the version of this crate in your `Cargo.toml` (e.g. `wasm-testsuite = "=0.4.4"`).

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
