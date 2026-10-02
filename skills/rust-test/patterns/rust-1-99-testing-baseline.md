# Rust 1.99 testing baseline

## Contents

- [Scope boundary](#scope-boundary)
- [Rust 1.99 release delta](#rust-199-release-delta)
- [Owned UTF-8 decoding](#test-owned-utf-8-decoding)
- [Deque tail retention](#test-deque-tail-retention)
- [Ownership and filesystem contracts](#test-ownership-and-filesystem-contracts)
- [Assertion idiom](#assertion-idiom)
- [Range provenance](#test-range-provenance-not-value-search)
- [Paired boundary stripping](#test-paired-boundary-stripping)
- [Formatting and numeric contracts](#test-formatting-and-numeric-contracts)
- [Encoding and typed parsing](#test-encoding-and-typed-parsing)
- [Atomic and iterator contracts](#test-atomic-and-iterator-contracts)
- [Cargo configuration](#cargo-199-configuration-effects)
- [Doctest and rustdoc effects](#doctest-and-rustdoc-effects)
- [Compiler and expectation compatibility](#compiler-and-expectation-file-compatibility)
- [Target-specific validation](#target-specific-validation)

Use this pattern when a Rust test, doctest, compile-fail/UI expectation, target
check, or validation command depends on the Rust 1.99 baseline. Keep the guidance
limited to testing. General implementation idioms belong in
`rust-best-practices`.

## Scope boundary

Apply Rust 1.99 guidance here only to:

- assertion choice and failure diagnostics;
- doctest imports, doctest attributes, and rustdoc validation commands;
- compile-fail/UI expectations whose acceptance or diagnostics change on 1.99;
- Cargo command semantics, configuration, feature matrices, target checks, and
  lower-MSRV validation;
- tests for public APIs that expose library behavior available on this baseline;
- snapshots, goldens, and generated output affected by escaping, symbols,
  temporary scopes, formatting discovery, or target-specific behavior.

Do not turn a test patch into a general production-code modernization. Ownership,
API design, lint policy, builders, type-state, documentation style, and production
performance changes remain outside this skill unless they are the explicit tested
contract.

## Rust 1.99 release delta

Review the [Rust 1.99 release notes](https://doc.rust-lang.org/releases.html#version-1990-2026-10-01)
for newly stable owned UTF-8 decoding, deque tail retention, boxed-array
iteration, non-null ownership transfer, raw layout queries, C variadics, and
filesystem timestamps. Select tests only for APIs exposed by the changed code.

The assertion, range-provenance, paired-stripping, numeric, UTF-16, and atomic
examples carried forward below are not new 1.99 stabilizations. In particular,
`assert_matches!` stabilized in 1.96; `build.warnings` and
`resolver.lockfile-path` stabilized in Cargo 1.97; the range-recovery, buffer,
algebraic-float, UTF-16, and atomic-view APIs shown here stabilized in 1.98.
Respect a declared lower MSRV instead of assuming every baseline API is allowed.

## Test owned UTF-8 decoding

`String::from_utf8_lossy_owned` consumes a byte vector and replaces invalid UTF-8
with `char::REPLACEMENT_CHARACTER`. `FromUtf8Error::into_utf8_lossy` provides
lossy recovery after a strict conversion has already failed. Test valid, empty,
invalid, and truncated input without changing a strict-decoding contract:

```rust
#[test]
fn owned_lossy_utf8_preserves_valid_text_and_replaces_invalid_bytes() {
    for (bytes, expected) in [
        (Vec::new(), ""),
        ("café".as_bytes().to_vec(), "café"),
        (vec![b'a', 0xff, b'b'], "a\u{fffd}b"),
        (vec![0xe2, 0x82], "\u{fffd}"),
    ] {
        assert_eq!(String::from_utf8_lossy_owned(bytes), expected);
    }
}

#[test]
fn strict_utf8_error_can_be_consumed_for_lossy_recovery() {
    let error = String::from_utf8(vec![b'a', 0xff, b'b']).unwrap_err();

    assert_eq!(error.utf8_error().valid_up_to(), 1);
    assert_eq!(error.into_utf8_lossy(), "a\u{fffd}b");
}
```

Do not assert pointer identity or allocation reuse: the
[owned conversion contract](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy_owned)
does not guarantee reuse of the original vector allocation. Use the existing
benchmark workflow only when allocation or throughput is the claim.

## Test deque tail retention

`VecDeque::retain_back(n)` keeps the last `n` elements in their existing order.
It takes a length, not a predicate. Cover lengths below, equal to, and above the
current length, plus empty input:

```rust
use std::collections::VecDeque;

#[test]
fn retain_back_keeps_the_requested_tail_in_order() {
    let mut values = VecDeque::from([1, 2, 3, 4]);

    values.retain_back(4);
    assert_eq!(values, [1, 2, 3, 4]);
    values.retain_back(6);
    assert_eq!(values, [1, 2, 3, 4]);
    values.retain_back(2);
    assert_eq!(values, [3, 4]);
    values.retain_back(0);
    assert!(values.is_empty());
    values.retain_back(1);
    assert!(values.is_empty());
}
```

For a public ring-buffer wrapper, also exercise wrapped storage through normal
queue operations and verify its observable order. Do not assert internal slice
layout. See the [`retain_back` contract](https://doc.rust-lang.org/std/collections/struct.VecDeque.html#method.retain_back).

## Test ownership and filesystem contracts

When the changed code exposes these 1.99 APIs:

- For owned and borrowed `IntoIterator` on `Box<[T; N]>`, test element order,
  empty arrays, moves of non-`Copy` elements, or mutation through borrowed
  iteration according to the public contract. Do not accidentally change an
  ownership test into a clone test.
- For `Box::into_non_null`/`from_non_null` or `Vec::into_parts`/`from_parts`,
  test the safe wrapper's ownership round trip and exactly-once cleanup with real
  values. Keep allocator, layout, initialized length, capacity, and aliasing
  invariants explicit. A non-null pointer is not proof of validity.
- For `size_of_val_raw`, `align_of_val_raw`, and `Layout::for_value_raw`,
  satisfy the documented metadata preconditions. Never manufacture invalid
  pointers merely to test rejection by an unsafe API. Pair safe-wrapper tests
  with Miri or other invariant checks only when the repository uses them.
- For `std::fs::set_times` and `set_times_nofollow`, use isolated temporary
  files and intentional symlink fixtures where supported. Distinguish following
  a symlink from updating the link itself, and account for filesystem timestamp
  resolution rather than assuming nanosecond round trips on every platform.
- For stable C-variadic definitions and `core::ffi::VaList`, validate the real
  FFI boundary with the target's ABI and promoted argument types. A host-only
  compile check does not prove cross-platform calling conventions.

Re-audit existing unsafe wrappers against the updated `Pin::new_unchecked`
safety requirements when relevant. Functional tests alone do not prove pinning
soundness, and `Box::leak` followed by assumed-safe reconstruction is not a
substitute for explicit ownership transfer.

## Assertion idiom

Use `assert_matches!` when the contract is one structured pattern and seeing the
mismatched value improves the failure:

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

```rust
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

Separately, Rust 1.99 changes implementation details of exhausted
`core::ops::RangeInclusive` iterators. Assert the yielded sequence and exhaustion,
not the later `start()`/`end()` values or slice-index behavior of an exhausted
range. Those details are not stable guarantees; keep original bounds separately
when the public contract needs them.

## Test paired boundary stripping

`strip_circumfix` succeeds only when the prefix and suffix both match without
overlap. Use a compact table that covers the complete contract:

```rust
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

```rust
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

```rust
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

## Cargo 1.99 configuration effects

Inspect `.cargo/config.toml`, manifests, and applicable parent or user
configuration before interpreting a command:

- edition-2024 members can override an inherited dependency's `default-features`,
  including disabling defaults enabled in the workspace declaration; test the
  member in isolation and relevant workspace combinations because other edges
  can still enable those features through unification;
- the new built-in `debug` profile currently inherits `dev`; it is not the
  `debug` debuginfo setting, and its presence does not change ordinary tests
  from the `test` profile to `debug`;
- incremental compilation is disabled by default when Cargo detects CI through
  the `CI` environment variable; inspect explicit settings and
  `CARGO_INCREMENTAL` before comparing build timings or cache behavior;
- `build.warnings = "deny"` turns adjustable lint warnings in local packages into
  command failures; report that as warning-policy evidence, not automatically as
  a behavioral regression;
- `resolver.lockfile-path` selects the `Cargo.lock` used by resolution and
  `--locked`; identify that path instead of assuming the workspace-root lockfile;
- configuration may come from parent directories or Cargo home, so a repository
  file is not always the complete active configuration.

See [workspace dependency inheritance](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace)
and [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html).
Do not assume earlier editions or lower-Cargo lanes interpret the new override
identically. Do not confuse nightly-only Cargo changelog items with stable
behavior.

## Doctest and rustdoc effects

On Rust 1.99, attributes that do not apply to anything in a doctest are errors.
Fix the example or hidden setup; do not add `ignore` to hide an unattached
attribute. `doc(cfg(...))` is no longer considered when filtering doctests:
inspect actual `cfg` gates and run the applicable feature/target combinations
rather than treating documentation annotations as execution guards.

The new `rustdoc::unused_footnote_definition` lint affects documentation checks.
Run the repository's documentation lint/build command when that surface changes;
a successful doctest run alone does not establish warning-free rendered docs.

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

Rust 1.99 can change compile-pass/UI outcomes and rendered expectations without a
behavioral source change. Review the relevant new surfaces:

- `no_mangle_generic_items` is a hard error, and runtime-symbol lints cover
  additional POSIX names;
- `raw_borrows_via_references` is allow-by-default; `unconditional_panic` also
  checks zero-sized chunks/windows calls; `unreachable_cfg_select_predicates`
  is included in `unused`;
- exported macros with trailing semicolons in expression position now produce
  `semicolon_in_expressions_from_non_local_macros` in consumers; use integration
  tests to exercise the separate-crate boundary and fix the provider;
- outlined modules in custom attribute or derive macros and C-variadic
  definitions are now stable; reconsider compile-fail fixtures that depended
  only on the previous feature gate and add valid usage where appropriate;
- unused `#[path]` on inline modules and invalid doc attributes on macro
  invocations can add diagnostics;
- inferred `let` pattern types, never-type method resolution, and associated
  constant lifetimes can change typechecking or diagnostics;
- debug escaping no longer escapes U+FF9E and U+FF9F, which can change string,
  stderr, or golden output;
- `transmute_copy` accepts `?Sized` input and uses a non-unwinding panic for its
  size check; do not expect `catch_unwind` or an ordinary `#[should_panic]` test
  to recover from that failure. Exercise a justified process-failure contract in
  a subprocess rather than aborting the test runner.

Retain relevant compatibility coverage from Rust 1.98 without relabeling it as
new in 1.99:

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
- expanded character escaping can change snapshots, `.stderr`, and golden files;
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

- Rust 1.99 promotes `riscv64-unknown-linux-musl` to Tier 2 with host tools; add a
  matrix lane only when the repository actually claims that target;
- Emscripten uses the WASM exception-handling ABI unconditionally;
- Solaris `File::lock` reports unsupported;
- promoted Thumb targets may justify matrix changes only when the repository
  claims support for them;
- atomic view methods wider than a byte can depend on target atomic support and
  primitive alignment;
- symbol/backtrace tools must understand v0 mangling when expectations include
  symbols;
- timestamp and symlink tests need platform/filesystem evidence, not host
  assumptions.

Report compile-only checks as compile evidence, not runtime validation.
