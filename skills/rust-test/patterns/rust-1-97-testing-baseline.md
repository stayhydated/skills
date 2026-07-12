# Rust 1.97 testing baseline

## Contents

- [Scope boundary](#scope-boundary)
- [Assertion idiom](#assertion-idiom)
- [Cargo configuration](#cargo-197-configuration-effects)
- [Doctest and rustdoc effects](#doctest-and-rustdoc-effects)
- [Compiler and expectation compatibility](#compiler-and-expectation-file-compatibility)
- [Version-sensitive contracts](#rust-version-sensitive-contracts)

Use this pattern when a Rust test, doctest, compile-fail/UI expectation, target
check, or validation command depends on Rust 1.97 behavior. Keep the guidance
limited to testing. General implementation idioms belong in
`rust-best-practices`.

## Scope boundary

Apply Rust 1.97 guidance here only to:

- assertion choice and failure diagnostics;
- doctest imports, doctest attributes, and rustdoc validation commands;
- compile-fail/UI expectations whose compiler output or acceptance changes by toolchain;
- Cargo command semantics, configuration, feature matrices, target checks, and MSRV-sensitive validation;
- tests for public APIs that explicitly expose a Rust 1.97 type, method, or behavior.

Do not use this skill to refactor production code toward broad Rust 1.97 idioms
such as ownership/API shape, integer bit helpers, `char` associated items, builder
selection, type-state modeling, lint policy, documentation style, or performance
tuning unless the test contract directly requires that behavior.

## Assertion idiom

The Rust 1.97 baseline includes the stable `assert_matches!` and
`debug_assert_matches!` macros introduced in Rust 1.96.

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

Do not rewrite clearer assertions just to use the macro:

- keep `assert_eq!` when equality is the contract;
- keep `assert!` with a useful message for boolean properties;
- use structural assertions when several fields matter;
- use snapshots or goldens for large deterministic output that needs reviewable diffs;
- use compile-fail/UI tests when the compiler rejection or diagnostic is the contract.

`debug_assert_matches!` is not a normal test assertion. Use it only when testing
or preserving debug-only internal invariant checks that intentionally disappear
from optimized builds unless debug assertions are enabled.

## Cargo 1.97 configuration effects

Inspect `.cargo/config.toml` and applicable parent or user configuration before
interpreting a command:

- `build.warnings = "deny"` makes adjustable lint warnings in local packages
  command failures. A test failure caused by this setting is warning-policy
  evidence, not necessarily a behavioral regression.
- `resolver.lockfile-path` changes which `Cargo.lock` Cargo resolves and honors.
  When `--locked` is used, report the configured lockfile rather than assuming the
  workspace-root lockfile.
- `resolver.lockfile-path` requires Rust 1.97 or newer and the configured path must
  end in `Cargo.lock`; do not recommend it for a lower MSRV toolchain.

Do not add either setting merely to modernize a test patch. Preserve the
repository's existing configuration and validate the actual command environment.

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

Rust 1.97 stabilizes rustdoc's `--emit` and `--remap-path-prefix` flags. When a
repository uses either flag, preserve its documented command and treat emitted
artifact paths or remapped paths as part of generated-output and doctest
interpretation. Do not infer that a plain `cargo test --doc` reproduces a custom
rustdoc pipeline.

When a repository uses `cargo-nextest`, remember that doctests still need a
separate `cargo test --doc ...` command. When `.cargo/config.toml` uses
cfg-specific `rustdocflags`, treat those flags as part of the doctest contract
and validate the affected feature or target shape explicitly.

## Compiler and expectation-file compatibility

A Rust 1.97 toolchain update can change accepted code, warnings, symbols, and
rendered expectations even when application behavior is unchanged:

- v0 symbol mangling is now the default, so backtraces, profiler output, crash
  reports, and symbol-based goldens may change or require newer demangling tools;
- linker output is warned on by default, which can alter stderr capture or make
  strict warning configurations fail;
- `pin!` no longer applies an unsound deref coercion, so compile-pass,
  compile-fail, and inferred-type expectations around `&mut T` may change;
- deprecated `std::char` module constants and functions can add warnings to
  doctests or UI fixtures;
- compiler validation changes can alter diagnostics for malformed link
  attributes, tuple-index pattern shorthands, and generic arguments on module
  path segments.

Regenerate compiler-facing expectations through the repository's existing
harness and review diffs rather than handwriting rustc output.

## Rust-version-sensitive contracts

For compile-fail/UI tests, regenerate diagnostics through the repository's
existing harness and review expectation diffs. Rust version updates can affect
accepted syntax, type-checking edges, warning levels, paths in diagnostics,
symbol spelling, and rendered rustdoc output.

The Rust 1.97 integer methods `bit_width`, `highest_one`, `lowest_one`,
`isolate_highest_one`, and `isolate_lowest_one` should trigger test work only
when the production contract exposes or depends on those operations. Do not turn
their availability into general test cleanup.

For target support, especially `no_std`, WASM, embedded, or custom target claims,
prefer the repository's documented target check/build recipe. The stricter
WebAssembly undefined-symbol behavior introduced in Rust 1.96 remains active on
the Rust 1.97 baseline; a normal host `cargo test` does not validate that link
boundary.
