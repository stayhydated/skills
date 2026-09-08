# Rust 1.98 testing baseline

## Contents

- [Scope boundary](#scope-boundary)
- [Assertion idiom](#assertion-idiom)
- [Range provenance](#test-range-provenance-not-value-search)
- [Paired boundary stripping](#test-paired-boundary-stripping)
- [Formatting and numeric contracts](#test-formatting-and-numeric-contracts)
- [Encoding and typed parsing](#test-encoding-and-typed-parsing)
- [Atomic and iterator contracts](#test-atomic-and-iterator-contracts)
- [Cargo configuration](#cargo-198-configuration-effects)
- [Doctest and rustdoc effects](#doctest-and-rustdoc-effects)
- [Compiler and expectation compatibility](#compiler-and-expectation-file-compatibility)
- [Target-specific validation](#target-specific-validation)

Use this pattern when a Rust test, doctest, compile-fail/UI expectation, target
check, or validation command depends on Rust 1.98 behavior. Keep the guidance
limited to testing. General implementation idioms belong in
`rust-best-practices`.

## Scope boundary

Apply Rust 1.98 guidance here only to:

- assertion choice and failure diagnostics;
- doctest imports, doctest attributes, and rustdoc validation commands;
- compile-fail/UI expectations whose acceptance or diagnostics change on 1.98;
- Cargo command semantics, configuration, feature matrices, target checks, and
  lower-MSRV validation;
- tests for public APIs that expose Rust 1.98 library behavior;
- snapshots, goldens, and generated output affected by escaping, symbols,
  temporary scopes, formatting discovery, or target-specific behavior.

Do not turn a test patch into a general production-code modernization. Ownership,
API design, lint policy, builders, type-state, documentation style, and production
performance changes remain outside this skill unless they are the explicit tested
contract.

## Assertion idiom

Use `assert_matches!` when the contract is one structured pattern and seeing the
mismatched value improves the failure:

```rust,test_harness
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

Use `std::assert_matches` for ordinary tests and doctests, and
`core::assert_matches` in `no_std` test contexts. If the repository explicitly
supports a lower MSRV, preserve an assertion idiom that compiles on that lane.
Do not rewrite clearer assertions merely to use the macro:

- keep `assert_eq!` when equality is the contract;
- keep `assert!` with a useful message for boolean properties;
- use structural assertions when several fields matter;
- use snapshots or goldens for large deterministic output;
- use compile-fail/UI tests when compiler rejection or diagnostics are the
  contract.

`debug_assert_matches!` is not an ordinary test assertion. Use it only when the
contract is intentionally debug-assertion-dependent.

## Test range provenance, not value search

`str::substr_range` and `[T]::subslice_range` recover the location of a borrowed
view derived from a source. Test provenance and offsets, including repeated equal
values:

```rust,test_harness
use core::range::Range;

#[test]
fn split_fields_keep_their_source_offsets() {
    let source = "a|bb|a";
    let ranges: Vec<_> = source
        .split('|')
        .map(|field| source.substr_range(field).expect("field derives from source"))
        .collect();

    assert_eq!(
        ranges,
        vec![
            Range { start: 0, end: 1 },
            Range { start: 2, end: 4 },
            Range { start: 5, end: 6 },
        ],
    );
}
```

Do not use an equal, independently allocated value as a search oracle; use
`find`, `position`, or `windows` for value search. For empty substrings or
subslices, assert only source-derived endpoint behavior because unrelated empty
views can produce an endpoint false positive. Add a `#[should_panic]` case for
`subslice_range` on a zero-sized element type only when that panic is part of the
exposed contract.

## Test paired boundary stripping

`strip_circumfix` succeeds only when the prefix and suffix both match without
overlap. Use a compact table that covers the complete contract:

```rust,test_harness
#[test]
fn quoted_payload_requires_both_delimiters() {
    for (input, expected) in [
        ("<data>", Some("data")),
        ("<data", None),
        ("data>", None),
        ("<>", Some("")),
        ("<", None),
    ] {
        assert_eq!(input.strip_circumfix('<', '>'), expected, "input: {input:?}");
    }

    assert_eq!("|".strip_circumfix('|', '|'), None);
}
```

For slices, include empty prefix/suffix cases when the public API accepts them.
Do not replace a one-sided stripping contract with `strip_circumfix`; test the
actual public rule.

## Test formatting and numeric contracts

For integer `format_into`, verify output across zero, signs, and extremes, then
reuse the same buffer. Consume each returned `&str` before the next mutable use:

```rust,test_harness
use core::fmt::NumBuffer;

#[test]
fn decimal_formatting_reuses_the_caller_buffer() {
    let mut buffer = NumBuffer::new();

    assert_eq!(0_u32.format_into(&mut buffer), "0");
    assert_eq!(42_u32.format_into(&mut buffer), "42");
    assert_eq!(u32::MAX.format_into(&mut buffer), u32::MAX.to_string());
}
```

A performance claim requires the repository's benchmark workflow; a unit test
proves formatting behavior, not allocation count or throughput.

For `algebraic_add`, `algebraic_sub`, `algebraic_mul`, `algebraic_div`, and
`algebraic_rem`, define the acceptable numeric contract first. Prefer relative or
absolute error bounds, monotonicity, range, finiteness, and domain invariants.
Cover NaN, infinity, signed zero, overflow, and underflow only where the public
contract specifies them. Do not assert exact bit patterns, a fixed evaluation
order, or equality with an ordinary-operation expression. The same inputs are
allowed to produce optimization-dependent results, so deterministic snapshots
are usually the wrong test form.

## Test encoding and typed parsing

For endian-specific UTF-16 constructors, cover:

- valid little-endian and big-endian byte sequences;
- surrogate pairs;
- unpaired surrogates;
- odd byte lengths;
- strict constructors returning `Err`;
- lossy constructors inserting `char::REPLACEMENT_CHARACTER` at invalid input.

```rust,test_harness
#[test]
fn utf16_byte_order_is_explicit() {
    assert_eq!(String::from_utf16le(&[0x48, 0x00, 0x69, 0x00]).unwrap(), "Hi");
    assert_eq!(String::from_utf16be(&[0x00, 0x48, 0x00, 0x69]).unwrap(), "Hi");
    assert!(String::from_utf16le(&[0x48]).is_err());
}
```

For `NonZero*::from_str_radix`, test valid radices, case handling where relevant,
zero rejection, malformed digits, signs allowed by the type, and out-of-range
values. Do not parse to a primitive first in the test oracle; assert the typed
result and its `.get()` value.

## Test atomic and iterator contracts

For `Atomic*::from_mut`, `from_mut_slice`, and `get_mut_slice`:

- use the repository's supported targets and guard wider atomics with the
  applicable `target_has_atomic` and
  `target_has_atomic_primitive_alignment` cfgs;
- test the synchronization invariant, not merely that a method compiles;
- use scoped threads or explicit joins so no worker outlives borrowed storage;
- choose an ordering justified by the communication contract;
- never introduce unsafe mixed atomic and non-atomic access to exercise the API.

`std::process::CommandArgs` is `Send + Sync` on this baseline, while
`std::env::Vars` and `VarsOs` are not. Use compile-time trait assertions or the
repository's UI harness only when those auto-traits are part of a public generic
contract. For ordinary worker code, collect owned environment pairs before
crossing a thread boundary.

## Cargo 1.98 configuration effects

Inspect `.cargo/config.toml` and applicable parent or user configuration before
interpreting a command:

- `build.warnings = "deny"` turns adjustable lint warnings in local packages into
  command failures; report that as warning-policy evidence, not automatically as
  a behavioral regression;
- `resolver.lockfile-path` selects the `Cargo.lock` used by resolution and
  `--locked`; identify that path instead of assuming the workspace-root lockfile;
- configuration may come from parent directories or Cargo home, so a repository
  file is not always the complete active configuration.

Cargo 1.98 introduces no new stable command/configuration surface that changes
ordinary test selection. Do not confuse nightly-only Cargo changelog items with
stable behavior.

## Doctest and rustdoc effects

When a doctest uses `assert_matches!`, hide the import so the rendered example
stays focused:

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

Preserve repository rustdoc commands that use `--emit`,
`--remap-path-prefix`, or cfg-specific `rustdocflags`. Emitted artifact paths and
remapped paths can be part of generated-output review. A plain
`cargo test --doc` does not necessarily reproduce a custom rustdoc pipeline.
`cargo-nextest` does not replace a separate doctest run.

## Compiler and expectation-file compatibility

Rust 1.98 can change compile-pass/UI outcomes and rendered expectations without a
behavioral source change. Review the relevant surfaces:

- fully elided trait-object lifetime bounds can resolve differently;
- additional ambiguous imports are errors, and some `ambiguous_glob_imports`
  cases are hard errors;
- where-bounds written as `Type = Type` or `Type == Type` are rejected;
- `invalid_runtime_symbol_definitions`,
  `suspicious_runtime_symbol_definitions`, and `c_void_returns` can add or raise
  diagnostics in runtime/FFI fixtures;
- `repr(transparent)` field rules and `transmute` equal-size checks are stricter;
- structural-equality matching rejects additional invalid constant patterns;
- derived `PartialOrd` can expose inconsistency with a manual `Ord`/`PartialOrd`
  combination;
- `assert_eq!` and `assert_ne!` use a temporary scope that can change drop timing;
- more characters are escaped when strings and chars are rendered, which can
  change snapshots, `.stderr`, and golden files;
- `unsafe_code` is emitted consistently for unsafe attributes;
- Windows thread-local destructor behavior can affect platform-only teardown
  tests;
- rustfmt discovers module files declared inside `cfg_select!`, potentially
  expanding formatting diffs.

Regenerate compiler-facing expectations through the repository's existing
harness and review the resulting diff. Do not handwrite rustc output or accept
broad snapshot churn without isolating the cause.

## Target-specific validation

Use the repository's target commands for `no_std`, WASM, embedded, custom target,
and platform contracts. Host tests do not prove target linking or ABI behavior.
In particular:

- Emscripten uses the WASM exception-handling ABI unconditionally;
- Solaris `File::lock` reports unsupported;
- promoted Thumb targets may justify matrix changes only when the repository
  claims support for them;
- atomic view methods wider than a byte can depend on target atomic support and
  primitive alignment;
- symbol/backtrace tools must understand v0 mangling when expectations include
  symbols.

Report compile-only checks as compile evidence, not runtime validation.
