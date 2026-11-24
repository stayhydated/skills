# Rust 1.100 testing baseline

## Contents

- [Scope boundary](#scope-boundary)
- [Rust 1.100 release delta](#rust-1100-release-delta)
- [Optional affix trimming](#test-optional-affix-trimming)
- [Scoped cleanup guards](#test-scoped-cleanup-on-every-exit-path)
- [Never type and `Infallible`](#test-never-type-and-infallible-contracts)
- [Custom allocators](#test-allocator-generic-code-and-allocator-implementations)
- [Smart-pointer, cell, and I/O contracts](#test-smart-pointer-cell-and-io-contracts)
- [Owned UTF-8 decoding](#test-owned-utf-8-decoding)
- [Deque tail retention](#test-deque-tail-retention)
- [Ownership and filesystem contracts](#test-ownership-and-filesystem-contracts)
- [Assertion idiom](#assertion-idiom)
- [Range provenance](#test-range-provenance-not-value-search)
- [Paired boundary stripping](#test-paired-boundary-stripping)
- [Formatting and numeric contracts](#test-formatting-and-numeric-contracts)
- [Encoding and typed parsing](#test-encoding-and-typed-parsing)
- [Atomic and iterator contracts](#test-atomic-and-iterator-contracts)
- [Cargo and libtest](#cargo-and-libtest-effects)
- [Doctest and rustdoc effects](#doctest-and-rustdoc-effects)
- [Compiler and expectation compatibility](#compiler-and-expectation-file-compatibility)
- [Target-specific validation](#target-specific-validation)

Use this pattern when a Rust test, doctest, compile-fail/UI expectation, target
check, or validation command depends on the Rust 1.100 baseline. Keep the guidance
limited to testing. General implementation idioms belong in
BeautifulRust's `rust-best-practices` core and `rust-best-practices-extra`.

## Scope boundary

Apply Rust 1.100 guidance here only to:

- assertion choice and failure diagnostics;
- doctest imports, doctest attributes, and rustdoc validation commands;
- compile-fail/UI expectations whose acceptance or diagnostics change on 1.100;
- Cargo command semantics, configuration, feature matrices, target checks, and
  lower-MSRV validation;
- tests for public APIs that expose library behavior available on this baseline;
- snapshots, goldens, and generated output affected by escaping, type names,
  symbols, temporary scopes, formatting discovery, or target-specific behavior.

Do not turn a test patch into a general production-code modernization. Ownership,
API design, lint policy, builders, type-state, documentation style, and production
performance changes remain outside this skill unless they are the explicit tested
contract.

## Rust 1.100 release delta

Review the [Rust 1.100 release notes](https://doc.rust-lang.org/releases.html#version-11000-2026-11-12)
for the newly stable never type, `Allocator` trait and allocator-aware `Box`/`Vec`
constructors, `mem::DropGuard`, `trim_prefix`/`trim_suffix`, `Box`/`Rc`/`Arc::map`,
`Box::take`, `Vec::from_fn`, `bool::toggle`, `UnsafeCell` access methods, Cargo
lints, dependency cooldowns, and repeated libtest arguments. Select tests only for
APIs exposed by the changed code.

The sections after [smart-pointer contracts](#test-smart-pointer-cell-and-io-contracts)
carry forward earlier guidance and are not new 1.100 stabilizations. In
particular, `assert_matches!` stabilized in 1.96; `build.warnings` and
`resolver.lockfile-path` stabilized in Cargo 1.97; the range-recovery, buffer,
algebraic-float, UTF-16, and atomic-view APIs stabilized in 1.98; and owned UTF-8
decoding, deque tail retention, and non-null ownership transfer stabilized in
1.99. Respect a declared lower MSRV instead of assuming every baseline API is
allowed.

## Test optional affix trimming

`trim_prefix` and `trim_suffix` remove at most one occurrence and return the input
unchanged when the affix is absent. Cover present, absent, repeated, and empty
input. Do not substitute them for a `strip_prefix` contract that must report
whether the affix was present:

```rust
fn normalize_tag(raw: &str) -> &str {
    raw.trim_prefix("refs/tags/").trim_prefix('v')
}

#[test]
fn tag_normalization_trims_each_optional_prefix_once() {
    for (input, expected) in [
        ("refs/tags/v1.2.0", "1.2.0"),
        ("v1.2.0", "1.2.0"),
        ("1.2.0", "1.2.0"),
        ("vv1", "v1"),
        ("", ""),
    ] {
        assert_eq!(normalize_tag(input), expected, "input: {input:?}");
    }
}
```

`Path::trim_prefix` matches whole components. Include a partial-component case
such as `/srv/app` against `/srv/application` when path prefixes are the contract.

## Test scoped cleanup on every exit path

When code uses `mem::DropGuard` for rollback or cleanup, test the observable state
after the success path, an early error return, and, when the crate unwinds, a
panic. The dismissed guard must leave committed state in place:

```rust
use std::mem::DropGuard;
use std::panic::{self, AssertUnwindSafe};

fn apply_batch(
    log: &mut Vec<u32>,
    batch: &[u32],
    check: impl Fn(u32) -> Result<(), String>,
) -> Result<(), String> {
    let committed = log.len();
    let mut log = DropGuard::new(log, move |log| log.truncate(committed));
    for &value in batch {
        check(value)?;
        log.push(value);
    }
    DropGuard::dismiss(log);
    Ok(())
}

#[test]
fn failed_batches_roll_back_and_successful_batches_commit() {
    let mut log = vec![1];

    let rejected = apply_batch(&mut log, &[2, 0, 3], |value| {
        if value == 0 { Err("zero".to_owned()) } else { Ok(()) }
    });
    assert_eq!(rejected, Err("zero".to_owned()));
    assert_eq!(log, [1]);

    let panicked = panic::catch_unwind(AssertUnwindSafe(|| {
        apply_batch(&mut log, &[2, 3], |value| {
            assert_ne!(value, 3, "checker rejects 3");
            Ok(())
        })
    }));
    assert!(panicked.is_err());
    assert_eq!(log, [1]);

    apply_batch(&mut log, &[2, 3], |_| Ok(())).unwrap();
    assert_eq!(log, [1, 2, 3]);
}
```

Gate or omit the unwinding case when the tested profile uses `panic = "abort"`.
Do not assert that a guard closure ran by recording calls into a spy; assert the
resource or state it restores.

## Test never-type and `Infallible` contracts

`!` is stable and `Infallible` is now an alias for it. For a `Result<T, !>` or
`Result<T, Infallible>` API, bind the success value with an irrefutable pattern.
Do not write an `is_err()` assertion or an unreachable error-handling test:

```rust
fn parse_label(raw: &str) -> Result<String, !> {
    Ok(raw.trim().to_owned())
}

#[test]
fn label_parsing_trims_without_an_error_path() {
    let Ok(label) = parse_label("  ready  ");
    assert_eq!(label, "ready");
}
```

Expectation changes to review:

- `type_name::<Infallible>()` and compiler diagnostics now render `!`. Review
  snapshots, goldens, and `.stderr` files that print type names rather than
  normalizing the difference away.
- Never-type fallback now resolves to `!` on every edition. Edition-2024 crates
  already used this fallback; in crates on earlier editions, test helpers whose
  inference depended on `()` fallback can fail to compile or change type. Add an
  explicit annotation in the helper.
- Compile-fail fixtures that only proved `!` was feature-gated now compile.
  Replace them with valid usage or a fixture for the real contract.

## Test allocator-generic code and allocator implementations

For code generic over `A: Allocator`, exercise the public behavior with at least
`Global` and one other real allocator, such as `System` or the repository's arena.
Assert outputs, not allocation calls:

```rust
use std::alloc::{Allocator, Global, System};

fn encode_frame<A: Allocator>(payload: &[u8], allocator: A) -> Vec<u8, A> {
    let mut frame = Vec::with_capacity_in(payload.len() + 1, allocator);
    frame.push(u8::try_from(payload.len()).expect("payload fits a one-byte length"));
    frame.extend_from_slice(payload);
    frame
}

#[test]
fn framing_is_independent_of_the_allocator() {
    assert_eq!(encode_frame(b"hi", Global), [2, b'h', b'i']);
    assert_eq!(encode_frame(b"hi", System), [2, b'h', b'i']);
    assert_eq!(encode_frame(b"", System), [0]);
}
```

When the change implements `Allocator`, test its documented contract directly:
returned blocks satisfy the requested size and alignment, zero-sized layouts
are handled, `grow` and `shrink` preserve the retained prefix, and
deallocation accepts only blocks from the same allocator or its clones. Pair those
tests with Miri or sanitizers when the repository uses them. Count allocations only
when an allocation bound is the public contract, and measure it through a real
allocator wrapper rather than a mock. Some `Box` implementations are now
allocator-generic; new inference failures in test code need explicit
annotations, not test-only wrapper types.

## Test smart-pointer, cell, and I/O contracts

- For `Box::map`, `Rc::map`, and `Arc::map`, assert the mapped value and, for
  shared pointers, behavior with other live clones. Do not assert pointer identity
  or allocation reuse; neither is guaranteed.
- For `Box::take`, assert the moved-out value and any reuse of the returned
  `Box<MaybeUninit<T>>` through `Box::write`. Avoid reading the uninitialized
  allocation.
- For `Vec::from_fn`, cover length zero and confirm that elements are produced in
  index order when the closure has side effects that the contract exposes.
- For safe wrappers over `UnsafeCell::replace`, `as_ref_unchecked`, or
  `as_mut_unchecked`, test the public invariant and run Miri where configured.
  Never create overlapping references to exercise the unsafe methods.
- On Unix targets where `ENOTSUP` differs from `EOPNOTSUPP`, such as macOS and
  the BSDs, `ENOTSUP` now maps to `io::ErrorKind::Unsupported`. Linux already
  returned `Unsupported` because the codes are equal there. Update affected
  platform tests that asserted another kind or a raw OS code, and keep the
  assertion structural.
- For types with hand-written `Ord`, `PartialOrd`, and `PartialEq`, test their
  consistency directly. `Iterator::min`, `max`, `min_by_key`, and `max_by_key` use
  different `Ord` methods on 1.100, so tests that depended on an inconsistent
  implementation can change outcome. Ties still return the first minimum and the
  last maximum.

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

When the changed code exposes these APIs stabilized in 1.99:

- For owned and borrowed `IntoIterator` on `Box<[T; N]>`, test element order,
  empty arrays, moves of non-`Copy` elements, or mutation through borrowed
  iteration according to the public contract. Do not accidentally change an
  ownership test into a clone test.
- For `Box::into_non_null`/`from_non_null` or `Vec::into_parts`/`from_parts`,
  test the safe wrapper's ownership round trip and exactly-once cleanup with real
  values. Keep allocator, layout, initialized length, capacity, and aliasing
  invariants explicit. A non-null pointer is not proof of validity. For a
  non-global allocator, test the 1.100 allocator-carrying variants, such as
  `Box::into_non_null_with_allocator`/`from_non_null_in`, with the same
  allocator on both sides.
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

Re-audit existing unsafe wrappers against the 1.99 `Pin::new_unchecked`
safety requirements when relevant. Functional tests alone do not prove pinning
soundness, and `Box::leak` followed by assumed-safe reconstruction is not a
substitute for explicit ownership transfer.

## Assertion idiom

Follow [assertion guidance](automated-testing.md#prefer-assert_matches-for-variant-checks)
for the `assert_matches!` rule, imports, example, and alternatives. The macro
stabilized in Rust 1.96; preserve an idiom that compiles on an explicitly declared
lower-MSRV lane.

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

Separately, Rust 1.99 changed implementation details of exhausted
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

Endian-specific UTF-16 decoding is portable; fixtures must state byte order
explicitly. For those constructors, cover:

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

## Cargo and libtest effects

Follow [Cargo's default behavior and configuration boundaries](cargo-test-semantics.md#default-behavior-to-account-for)
for Cargo lints, dependency cooldowns, build-directory layout, inherited
dependency overrides, profile selection, CI incremental defaults, warning policy,
lockfile selection, and lower-Cargo lanes. Inspect
[active configuration sources](cargo-test-semantics.md#working-directories-configuration-and-environment)
before interpreting a command. Test-relevant 1.100 changes:

- `cargo::unused_dependencies` does not check `[dev-dependencies]`. Put
  test-only crates there, and do not treat a clean lint run as proof that
  dev-dependencies are used.
- Cargo's intermediate build-directory layout changed and is not a stable
  interface. E2E tests should locate binaries through `CARGO_BIN_EXE_<name>`, and
  scripts should use Cargo's JSON messages instead of hard-coded
  `target/<profile>/deps` paths.
- A configured `registry.global-min-publish-age` or per-registry
  `min-publish-age` can make fresh resolution select older versions than an
  unconfigured lane. Record it when comparing lockfile-free or minimal-version
  jobs.
- libtest accepts repeated `--test-threads` and `--color` arguments, and the last
  one wins. When runner scripts append harness arguments, report the effective
  value rather than the first one.

`build.warnings` and `resolver.lockfile-path` stabilized in Cargo 1.97, and the
inherited `default-features` override and built-in `debug` profile arrived in
Cargo 1.99. All are carried forward on this baseline.

## Doctest and rustdoc effects

Use [doctest compatibility guidance](doctests-and-examples.md#rust-1100-compatibility)
for the `rustdoc::invalid_markdown_table` lint and the carried-forward
unattached-attribute, `doc(cfg(...))`, and `rustdoc::unused_footnote_definition`
rules. Follow
[rustdoc mechanics](doctests-and-examples.md#rustdoc-mechanics) for hidden setup.
Hidden assertion imports keep the rendered example focused:

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

Follow [doctest validation](doctests-and-examples.md#validation) for custom rustdoc
pipelines, emitted/remapped artifacts, and separate execution alongside nextest.

## Compiler and expectation-file compatibility

Rust 1.100 can change compile-pass/UI outcomes and rendered expectations without
a behavioral source change. Review the relevant new surfaces:

- the never type is stable, `Infallible` is an alias for `!`, and never-type
  fallback is `!` on all editions, which changes pre-2024-edition fixtures;
  feature-gate fixtures for `!` now compile, and
  diagnostics print `!` where they printed `Infallible`;
- `extern "custom"` is stable for naked functions; reconsider fixtures that only
  proved it was feature-gated;
- `aligned_fields_in_packed` is now a deny-by-default lint rather than a hard
  error, so its `.stderr` shape changes;
- new or extended lints cover repeated `#[repr(..)]` attributes and
  `ptr::drop_in_place` calls under `dropping_references`, `dropping_copy_types`,
  and `undropped_manually_drops`;
- `unfulfilled_lint_expectations` no longer fires for `#[expect(dead_code)]`
  inside code already reported as dead, which can remove an expected diagnostic;
- closures capture variables used in `if let` more precisely, which can add
  borrow errors;
- dyn-compatibility is checked in more places, the next trait solver drives
  orphan checking, and `ambiguous_glob_imported_traits` fires in more cases;
- `box` patterns are no longer accepted syntactically, and `let`-`else` respects
  `macro_rules` `expr` metavariable grouping;
- enum layout computations with explicit discriminants can report query-cycle
  errors in more cases;
- allocator-generic `Box` implementations can add type-inference errors;
- `str::escape_debug` always escapes grapheme extenders, so a combining mark
  after a base character is now escaped: `escape_debug` of `e` followed by
  U+0301 yields `e\u{301}` instead of keeping the raw mark. `{:?}` formatting and
  `char::escape_debug` already escaped them and are unchanged;
- Unicode 18.0.0 can change `char` property results, case mapping, and
  derived goldens.

Retain relevant compatibility coverage from Rust 1.99 without relabeling it as
new in 1.100:

- `no_mangle_generic_items` is a hard error, and runtime-symbol lints cover
  additional POSIX names;
- `raw_borrows_via_references` is allow-by-default; `unconditional_panic` also
  checks zero-sized chunks/windows calls; `unreachable_cfg_select_predicates`
  is included in `unused`;
- exported macros with trailing semicolons in expression position produce
  `semicolon_in_expressions_from_non_local_macros` in consumers; use integration
  tests to exercise the separate-crate boundary and fix the provider;
- outlined modules in custom attribute or derive macros and C-variadic
  definitions are stable; fixtures that depended only on the previous feature
  gate need valid usage instead;
- unused `#[path]` on inline modules and invalid doc attributes on macro
  invocations can add diagnostics;
- inferred `let` pattern types, never-type method resolution, and associated
  constant lifetimes can change typechecking or diagnostics;
- legacy integral module deprecations can add warnings in compiler-facing fixtures;
- debug escaping does not escape U+FF9E and U+FF9F, which can change string,
  stderr, or golden output;
- `transmute_copy` accepts `?Sized` input, and its size check is an unsafe
  precondition checked only when debug assertions are enabled. In those builds a
  violation aborts the process, so `catch_unwind` and `#[should_panic]` cannot
  observe it; in release builds the check is absent and the call is undefined
  behavior. Do not test a size-check failure as a contract, even in a subprocess.
  Test that the safe wrapper rejects mismatched sizes before calling
  `transmute_copy`.

Also retain relevant coverage carried forward from Rust 1.98:

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

- Rust 1.100 promotes `wasm32-wasip3` to Tier 2 and makes `i686-pc-windows-gnu`
  Tier 2 without host tools; add a matrix lane only when the repository actually
  claims that target;
- the WASI SDK moved from 33 to 34, and `windows-gnullvm` always links libunwind
  statically; treat linker or runtime differences as target evidence;
- on ARM targets, ABI-altering `-Ctarget-feature` or `-Ctarget-cpu` values are
  hard errors, and ARMv7-A hardfloat targets no longer assume `d32`; enabling
  SSE or AVX on x86 softfloat targets is an error, and Xtensa requires the
  windowed ABI;
- the x86 Rust ABI uses SSE registers when the target always provides them,
  which can change assembly, codegen, or ABI expectations without a source change;
- carried forward from 1.99: `riscv64gc-unknown-linux-musl` is Tier 2 with host
  tools (the 1.99 release notes spell it `riscv64-unknown-linux-musl`), Emscripten uses the WASM exception-handling ABI unconditionally, Solaris
  `File::lock` reports unsupported, and promoted Thumb targets justify matrix
  changes only when the repository claims them;
- atomic view methods wider than a byte can depend on target atomic support and
  primitive alignment;
- symbol/backtrace tools must understand v0 mangling when expectations include
  symbols;
- timestamp and symlink tests need platform/filesystem evidence, not host
  assumptions.

Report compile-only checks as compile evidence, not runtime validation.
