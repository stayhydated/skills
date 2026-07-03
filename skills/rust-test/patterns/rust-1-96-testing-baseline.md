# Rust 1.96 testing baseline

Use this pattern when a Rust test, doctest, compile-fail/UI expectation, target
check, or validation command depends on Rust 1.96 behavior. Keep the guidance
limited to testing. General implementation idioms belong in `rust-best-practices`.

## Scope boundary

Apply Rust 1.96 guidance here only to:

- assertion choice and failure diagnostics;
- doctest imports, doctest attributes, and rustdoc validation commands;
- compile-fail/UI expectations whose compiler output or acceptance changes by toolchain;
- Cargo command semantics, feature matrices, target checks, and MSRV-sensitive validation;
- tests for public APIs that explicitly expose a Rust 1.96 type or behavior.

Do not use this skill to refactor production code toward broad Rust 1.96 idioms
such as ownership/API shape, `let ... else`, `inspect_err`, concrete range-type
storage, builder selection, type-state modeling, lint policy, documentation style,
or performance tuning unless the test contract directly requires that behavior.

## Assertion idiom

Rust 1.96 stabilizes `assert_matches!` and `debug_assert_matches!`.

Use `assert_matches!` when the contract is a pattern shape and the mismatch value
would make failures clearer:

```rust
use std::assert_matches;

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    Empty,
    MissingSeparator,
}

fn parse_marker(input: &str) -> Result<&str, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Empty);
    }

    input
        .split_once(':')
        .map(|(_, marker)| marker)
        .ok_or(ParseError::MissingSeparator)
}

#[test]
fn empty_marker_is_rejected() {
    assert_matches!(parse_marker(""), Err(ParseError::Empty));
}
```

Use `std::assert_matches` for ordinary `std` tests and doctests. Use
`core::assert_matches` in `#![no_std]` test contexts where `std` is unavailable.
Keep repository MSRV policy first: if the crate supports a compiler older than
Rust 1.96, preserve the existing assertion style or label the macro as a
recommended MSRV-gated cleanup rather than applying it in a patch.

Do not rewrite clearer assertions just to use the new macro:

- keep `assert_eq!` when equality is the contract;
- keep `assert!` with a useful message for boolean properties;
- use structural assertions when several fields matter;
- use snapshots or goldens for large deterministic output that needs reviewable diffs;
- use compile-fail/UI tests when the compiler rejection or diagnostic is the contract.

`debug_assert_matches!` is not a normal test assertion. Use it only when testing
or preserving debug-only internal invariant checks that intentionally disappear
from optimized builds unless debug assertions are enabled.

## Doctest and rustdoc effects

When a doctest uses `assert_matches!`, import the macro in hidden setup so the
rendered example stays focused:

````rust
/// # Examples
///
/// ```rust
/// # use std::assert_matches;
/// # use crate_name::{parse_marker, ParseError};
/// assert_matches!(parse_marker(""), Err(ParseError::Empty));
/// ```
# fn _example() {}
````

When a repository uses `cargo-nextest`, remember that doctests still need a
separate `cargo test --doc ...` command. When `.cargo/config.toml` uses
cfg-specific `rustdocflags`, treat those flags as part of the doctest contract
and validate the affected feature or target shape explicitly.

## Rust-version-sensitive contracts

For compile-fail/UI tests, regenerate diagnostics through the repository's
existing harness and review expectation diffs rather than handwriting compiler
output. Rust version updates can affect accepted syntax, type-checking edges,
warning levels, paths in diagnostics, and rendered rustdoc output.

For target support, especially `no_std`, WASM, embedded, or custom target
claims, prefer the repository's documented target check/build recipe. Do not
claim a normal host `cargo test` validates target compilation or linking.

For `core::range` and related Rust 1.96 APIs, add tests only when the production
contract exposes, stores, accepts, or serializes those types, or when an MSRV or
feature matrix explicitly promises compatibility. Do not turn range guidance into
general code-style advice under this skill.
