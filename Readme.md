Phi-Lib-Rust
My utilities in Rust.

# Tests
cargo test

# Benchmarks
cargo run --features benchmark


#![no_std]: only core is allowed.
You can use everything in core, such as:
- core::ops, core::cmp, core::iter, core::fmt
- Option, Result, Copy, Clone, PartialEq, etc.

See: https://doc.rust-lang.org/reference/