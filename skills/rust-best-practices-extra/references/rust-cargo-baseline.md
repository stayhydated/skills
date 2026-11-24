# Rust and Cargo Baseline Additions

Use these implementation and tooling details when the repository targets Rust
1.100. Respect a lower declared MSRV and target-specific availability. Consult the
[Rust release notes](https://doc.rust-lang.org/releases.html#version-11000-2026-11-12)
for stabilization dates; carried-forward APIs are not all new in 1.100.

## Contents

- [Rust 1.100 release delta](#rust-1100-release-delta)
- [Optional affix trimming](#trim-optional-affixes-without-an-option)
- [Scoped cleanup guards](#scoped-cleanup-with-dropguard)
- [Never type and `Infallible`](#never-type-and-infallible)
- [Custom allocators](#custom-allocators-at-explicit-boundaries)
- [Smart-pointer and vector construction](#smart-pointer-mapping-and-indexed-construction)
- [Cargo lints and dependency cooldowns](#cargo-lints-and-dependency-cooldowns)
- [Stored ranges and range inputs](#range-apis)
- [Derived substring and subslice offsets](#recover-ranges-from-derived-substrings-and-subslices)
- [Paired boundaries](#strip-paired-boundaries-atomically)
- [Typed parsing and text encoding](#use-typed-parsing-and-explicit-text-encoding)
- [Bit inspection](#integer-bit-inspection)
- [Buffered integer formatting](#reuse-a-numbuffer-in-allocation-sensitive-code)
- [Algebraic floating-point operations](#use-algebraic-floating-point-operations-only-with-an-explicit-contract)
- [Atomic views](#atomic-views-over-exclusive-storage)
- [Environment iterators](#process-and-environment-iterators)
- [Non-null ownership](#non-null-ownership-and-raw-layouts)
- [Compiler and tooling changes](#compiler-and-tooling-changes)

## Rust 1.100 Release Delta

Rust 1.100 stabilizes the never type `!`, the `Allocator` trait with
allocator-aware `Box` and `Vec` constructors, `core::mem::DropGuard`,
`trim_prefix`/`trim_suffix` on `str`, slices, and `Path` (`Path` gains only
`trim_prefix`), `Box::map`, `Rc::map`,
`Arc::map`, `Box::take`, `Vec::from_fn`, `bool::toggle`, `impl Default for Once`,
and `UnsafeCell::replace`/`as_ref_unchecked`/`as_mut_unchecked`. Cargo gains a
lint system with `cargo::unused_dependencies` and minimum-publish-age
dependency cooldowns.

Adopt these APIs where they make the changed code clearer or safer. Do not sweep
unrelated code or raise a declared lower MSRV solely for a stylistic cleanup.
Sections after [Cargo lints](#cargo-lints-and-dependency-cooldowns) carry forward
guidance for APIs stabilized from Rust 1.97 through 1.99; their stabilization
dates are unchanged.

## Trim Optional Affixes Without an `Option`

Use `trim_prefix` and `trim_suffix` when an affix is optional and its absence is
not an error. They remove at most one occurrence and return the input unchanged
otherwise, so they chain without `unwrap_or`:

```rust
use std::path::Path;

fn normalize_tag(raw: &str) -> &str {
    raw.trim_prefix("refs/tags/").trim_prefix('v')
}

assert_eq!(normalize_tag("refs/tags/v1.2.0"), "1.2.0");
assert_eq!(normalize_tag("1.2.0"), "1.2.0");
assert_eq!("vv1".trim_prefix('v'), "v1");

assert_eq!(b"\xef\xbb\xbfdata".trim_prefix(b"\xef\xbb\xbf"), b"data");
assert_eq!(
    Path::new("/srv/app/config.toml").trim_prefix("/srv/app"),
    Path::new("config.toml"),
);
```

Keep `strip_prefix`/`strip_suffix` when the caller must know whether the affix
was present, such as protocol framing or validation. Keep
`trim_start_matches`/`trim_end_matches` when repeated occurrences must all be
removed. `Path::trim_prefix` matches whole components, not string prefixes.

## Scoped Cleanup with `DropGuard`

Use `core::mem::DropGuard` for local cleanup or rollback that must run on every
exit path, including `?`, early `return`, and unwinding. Call
`DropGuard::dismiss` on the success path to keep the value and skip the closure.
Prefer this to `mem::forget` or a hand-written single-use guard type:

```rust
use std::mem::DropGuard;

fn append_all(log: &mut Vec<String>, steps: &[&str]) -> Result<(), String> {
    let committed = log.len();
    let mut log = DropGuard::new(log, move |log| log.truncate(committed));

    for step in steps {
        if step.is_empty() {
            return Err("empty step".to_owned());
        }
        log.push((*step).to_owned());
    }

    DropGuard::dismiss(log);
    Ok(())
}

let mut log = vec!["init".to_owned()];
assert!(append_all(&mut log, &["a", "", "b"]).is_err());
assert_eq!(log, ["init"]);
append_all(&mut log, &["a", "b"]).unwrap();
assert_eq!(log, ["init", "a", "b"]);
```

Keep a named type with a `Drop` implementation when cleanup is part of a public
type's invariant or needs a documented name. Keep guard closures infallible and
non-panicking: they run during unwinding, and a second panic aborts the process.

## Never Type and `Infallible`

`!` is a stable type in every position, and `core::convert::Infallible` is now a
type alias for `!`. Write `!` for new uninhabited error or output parameters, such
as `Result<T, !>` or `type Error = !;`. Keep `Infallible` where an existing public
API already uses it: both spellings name the same type, so switching is churn.
Use an irrefutable pattern instead of `unwrap` when the error type is uninhabited:

```rust
fn parse_label(raw: &str) -> Result<String, !> {
    Ok(raw.trim().to_owned())
}

let Ok(label) = parse_label("  ready  ");
assert_eq!(label, "ready");
```

Two 1.100 compatibility changes need review in existing code:

- Never-type fallback now resolves to `!` on all editions. Edition-2024 crates
  already used this fallback, so the change affects crates on earlier editions.
  There, code whose inference picked `()` for a diverging expression, such as a
  generic call whose only constraint is `return`, `panic!`, or `todo!`, can fail
  to compile or change behavior. Annotate the intended type instead of relying
  on fallback.
- Separate trait implementations for `Infallible` and `!` now overlap. Remove the
  duplicate; do not add a wrapper type solely to keep both.

Diagnostics and `type_name` now render `Infallible` as `!`. Update expected
output only where that text is part of a reviewed contract.

## Custom Allocators at Explicit Boundaries

The stable `std::alloc::Allocator` trait and the `Box::new_in` and
`Vec::new_in`/`with_capacity_in` families let owned values use a chosen allocator.
`Rc` and `Arc` allocator constructors remain unstable. Accept a generic allocator only when callers
genuinely supply one, such as an arena, a pool, or a measured allocation strategy.
Do not thread an allocator parameter through ordinary APIs speculatively; it
becomes part of every affected type's public signature.

```rust
use std::alloc::{Allocator, Global, System};

fn encode_frame<A: Allocator>(payload: &[u8], allocator: A) -> Vec<u8, A> {
    let mut frame = Vec::with_capacity_in(payload.len() + 1, allocator);
    frame.push(u8::try_from(payload.len()).expect("payload fits a one-byte length"));
    frame.extend_from_slice(payload);
    frame
}

assert_eq!(encode_frame(b"hi", Global), [2, b'h', b'i']);
assert_eq!(encode_frame(b"hi", System), [2, b'h', b'i']);
```

`Allocator` is an `unsafe` trait. An implementation must uphold the documented
contracts for alignment, zero-sized requests, `grow`/`shrink` content
preservation, and memory validity across clones and moves. Document those
obligations in `// SAFETY:` comments and validate with the repository's Miri or
sanitizer workflow when it has one. Prefer an existing, audited allocator crate
over a new implementation unless allocator behavior is the requested work.
Some `Box` implementations are now generic over the allocator; fix new inference
errors with explicit type annotations rather than allocator-specific wrappers.

## Smart-Pointer Mapping and Indexed Construction

- Use `Box::map`, `Rc::map`, and `Arc::map` to transform an owned or shared value
  and reuse the allocation when possible. They are associated functions:
  `Rc::map(shared, |value| ...)`. `Rc::map` and `Arc::map` pass a reference to the
  closure. Do not promise allocation reuse; it depends on uniqueness and layout.
- Use `Box::take` when a boxed value must be moved out while keeping the
  allocation as `Box<MaybeUninit<T>>` for reuse. Otherwise move out with `*boxed`.
- Use `Vec::from_fn(len, |index| ...)` when every element is computed from its
  index. Keep `iter().map(...).collect()` when the source is already an iterator.
- Use `bool::toggle` for an in-place flag flip. Use `Once::default()` where a
  `Default` bound requires it; `Once::new()` remains appropriate in `const`
  contexts.
- `UnsafeCell::replace`, `as_ref_unchecked`, and `as_mut_unchecked` are unsafe and
  require the same aliasing proof as raw-pointer access. Write a `// SAFETY:`
  comment that names the exclusive-access invariant. `mem::conjure_zst` is
  unsafe and valid only for inhabited zero-sized types whose construction has no
  safety invariant.
- `Vec`, slices, and mutable slices compare directly with `Cow<'_, [U]>`; remove
  `.as_ref()` or `&*` workarounds in comparisons when touching that code.

## Cargo Lints and Dependency Cooldowns

Cargo 1.100 reports some manifest problems as lints configurable under
`[lints.cargo]` or `[workspace.lints.cargo]`. The warn-by-default
`cargo::unused_dependencies` lint reports normal and build dependencies that no
selected target uses. Fix the manifest rather than allowing the lint broadly:

- Move dependencies used only by tests, examples, or benchmarks to
  `[dev-dependencies]`. That table is not checked.
- Remove a dependency that nothing uses, along with any feature that only enabled
  it.
- A direct dependency that only pins or enables features on a transitive
  dependency can be a false positive. Move it to a
  `[target."cfg(false)".dependencies]` table, as the
  [lint documentation](https://doc.rust-lang.org/cargo/reference/lints.html#unused_dependencies)
  recommends, or allow the lint narrowly with a comment that names the reason.

`cargo::missing_lints_inheritance` warns when `[workspace.lints]` exists but a
member has no `[lints]` table. Add `[lints] workspace = true` to members that
should inherit, or an empty `[lints]` table when opting out is intentional. Do not
change Cargo lint levels beyond the requested scope.

Dependency cooldowns make the resolver skip registry versions published more
recently than `registry.global-min-publish-age` or a per-registry
`registries.<name>.min-publish-age`, unless the version is already locked.
`resolver.incompatible-publish-age` defaults to `"deny"`, so setting a minimum age
is enough. Treat a cooldown as repository supply-chain policy: honor an existing
setting when choosing dependency versions, and propose one rather than adding it
silently. See the
[publish-age resolver docs](https://doc.rust-lang.org/cargo/reference/resolver.html#publish-age).

`cargo install` now uses the lockfile packaged in the `.crate` by default, and
`--locked` is a no-op there. Cargo also uses a new intermediate build-directory
layout. Locate artifacts with
[Cargo's JSON messages](https://doc.rust-lang.org/cargo/reference/external-tools.html#json-messages)
or `CARGO_BIN_EXE_<name>` rather than hard-coded `target/<profile>/deps` paths.

## Range APIs

Use the concrete range types under `core::range` when a range is stored in a
small `Copy` value. For public APIs, prefer accepting `impl RangeBounds<usize>`
so callers can pass ordinary range syntax or a concrete stored range.

```rust
use core::range::Range;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ByteSpan {
    range: Range<usize>,
}

impl ByteSpan {
    fn read_from(self, bytes: &[u8]) -> &[u8] {
        &bytes[self.range]
    }
}

let span = ByteSpan {
    range: Range { start: 1, end: 4 },
};
assert_eq!(span.read_from(b"frames"), b"ram");
```

For generic public range inputs, normalize bounds once and use checked arithmetic
at the boundary:

```rust
use core::ops::{Bound, RangeBounds};

fn select_window(bytes: &[u8], range: impl RangeBounds<usize>) -> Option<&[u8]> {
    let start = match range.start_bound() {
        Bound::Included(&index) => index,
        Bound::Excluded(&index) => index.checked_add(1)?,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(&index) => index.checked_add(1)?,
        Bound::Excluded(&index) => index,
        Bound::Unbounded => bytes.len(),
    };

    bytes.get(start..end)
}

assert_eq!(select_window(b"abcdef", 2..=4), Some(&b"cde"[..]));
assert_eq!(select_window(b"abcdef", ..), Some(&b"abcdef"[..]));
assert_eq!(select_window(b"abcdef", 7..), None);
assert_eq!(select_window(b"abcdef", ..=usize::MAX), None);
assert_eq!(
    select_window(b"abcdef", (Bound::Excluded(usize::MAX), Bound::Unbounded)),
    None,
);
```

## Recover Ranges from Derived Substrings and Subslices

Use `str::substr_range` and `[T]::subslice_range` when an iterator or parser has
already returned a borrow into a source and the caller needs its original range.
These methods do not search for equal contents; they identify where the borrowed
view came from.

```rust
use core::range::Range;

let source = "left|middle|right";
let middle = source.split('|').nth(1).expect("middle field");
assert_eq!(
    source.substr_range(middle),
    Some(Range { start: 5, end: 11 }),
);

let values = [10, 20, 30, 40];
let window = &values[1..3];
assert_eq!(
    values.subslice_range(window),
    Some(Range { start: 1, end: 3 }),
);
```

Do not use these APIs as a value search; use `find`, `position`, or `windows`
when provenance is irrelevant. Treat zero-length views carefully because an empty
view at an allocation boundary can produce a false-positive endpoint range.
`subslice_range` also panics for zero-sized element types.

## Strip Paired Boundaries Atomically

Use `strip_circumfix` when both boundaries must match. It removes each boundary
once and returns `None` when either boundary is absent or the two matches overlap.

```rust
assert_eq!("(payload)".strip_circumfix('(', ')'), Some("payload"));
assert_eq!("(payload".strip_circumfix('(', ')'), None);

let frame = b"<data>";
assert_eq!(frame.strip_circumfix(b"<", b">"), Some(&b"data"[..]));
```

This is clearer and safer than accepting a successful `strip_prefix` before
checking whether the corresponding suffix exists.

## Use Typed Parsing and Explicit Text Encoding

Parse non-zero integers directly into a non-zero type instead of parsing a
primitive and validating it in a second step:

```rust
use core::num::NonZeroU16;

let port = NonZeroU16::from_str_radix("1f90", 16).expect("valid non-zero port");
assert_eq!(port.get(), 8080);
```

When UTF-16 arrives as bytes, select the byte order explicitly rather than
converting to native-endian `u16` values first:

```rust
let little_endian = [0x48, 0x00, 0x69, 0x00];
let big_endian = [0x00, 0x48, 0x00, 0x69];

assert_eq!(String::from_utf16le(&little_endian).unwrap(), "Hi");
assert_eq!(String::from_utf16be(&big_endian).unwrap(), "Hi");
```

Use the corresponding lossy constructor only when replacement characters are an
accepted part of the decoding contract.

## Integer Bit Inspection

Use integer bit-inspection methods when the domain is about bit positions or
one-bit masks. They make the zero case and return shape explicit and avoid
hand-written shift arithmetic.

```rust
let flags = 0b0010_1100_u8;

assert_eq!(flags.bit_width(), 6);
assert_eq!(flags.highest_one(), Some(5));
assert_eq!(flags.lowest_one(), Some(2));
assert_eq!(flags.isolate_highest_one(), 0b0010_0000);
assert_eq!(flags.isolate_lowest_one(), 0b0000_0100);

assert_eq!(0_u8.highest_one(), None);
assert_eq!(0_u8.isolate_lowest_one(), 0);
```

Use the corresponding `NonZero` methods when zero is already excluded by the
type. Respect an explicitly declared lower MSRV instead of changing public
compatibility solely for stylistic cleanup.

## Reuse a `NumBuffer` in Allocation-Sensitive Code

For decimal integer formatting in a measured hot path, let the caller reuse a
`core::fmt::NumBuffer` and consume the returned `&str` before formatting the next
value into that buffer.

```rust
use core::fmt::NumBuffer;

fn write_ids(ids: impl IntoIterator<Item = u64>, output: &mut String) {
    let mut buffer = NumBuffer::new();

    for id in ids {
        output.push_str(id.format_into(&mut buffer));
        output.push('\n');
    }
}

let mut output = String::new();
write_ids([7, 42, 9001], &mut output);
assert_eq!(output, "7\n42\n9001\n");
```

Use ordinary `format!`, `write!`, or `Display` implementations outside such
paths. `NumBuffer` is a specialized optimization, not a replacement for the
formatting ecosystem, and the borrowed text is overwritten by the next mutable
use of the buffer.

## Use Algebraic Floating-Point Operations Only with an Explicit Contract

The `algebraic_add`, `algebraic_sub`, `algebraic_mul`, `algebraic_div`, and
`algebraic_rem` methods permit optimizations that ordinary IEEE-style source
operations do not, including reassociation and reciprocal transformations. They
can unlock vectorization, but their precision and treatment of NaN, infinity, and
signed zero are intentionally less constrained.

```rust
fn relaxed_dot(lhs: &[f32], rhs: &[f32]) -> f32 {
    lhs.iter()
        .zip(rhs)
        .fold(0.0_f32, |sum, (&left, &right)| {
            sum.algebraic_add(left.algebraic_mul(right))
        })
}
```

Use them only when all of the following are true: profiling shows the ordinary
operations are a bottleneck; the public contract accepts platform- and
optimization-dependent results; tests use error bounds or invariants rather than
exact bit patterns; and no unsafe-code soundness argument depends on the result.
Do not use them for reproducible serialization, exact threshold decisions,
financial calculations, or code that assigns semantic meaning to NaN payloads or
signed zero.

## Atomic Views Over Exclusive Storage

Since Rust 1.98, primitive storage can be borrowed as atomic storage, or an
exclusively owned atomic slice can be borrowed as its primitive representation.
This avoids allocation and unsafe pointer casts when a phase of an algorithm
changes its access mode.

```rust
use std::sync::atomic::{AtomicU8, Ordering};

let mut bytes = [0_u8; 4];
let atomics = AtomicU8::from_mut_slice(&mut bytes);

std::thread::scope(|scope| {
    for (index, cell) in atomics.iter().enumerate() {
        scope.spawn(move || cell.store(index as u8, Ordering::Relaxed));
    }
});

assert_eq!(bytes, [0, 1, 2, 3]);
```

Use an ordering justified by the synchronization contract. `Relaxed` provides
atomic access without ordering other memory; the scoped-thread joins above
establish completion before the non-atomic read. The exclusive borrow prevents
safe non-atomic access while the atomic view exists. Do not weaken an ordering
without verifying how other memory is published and observed.
For integer and pointer atomics wider than a byte, these view methods are
available only on targets where the primitive and atomic alignments match. Use
the applicable `target_has_atomic` and
`target_has_atomic_primitive_alignment` cfgs, together with the repository's
target matrix, where portability matters.

`Atomic*::get_mut_slice` performs the reverse operation under an exclusive
borrow. It is appropriate for single-threaded initialization or teardown, not for
bypassing synchronization while other threads can observe the atomics.

## Process and Environment Iterators

`std::process::CommandArgs` can be sent or shared when its borrow remains valid,
but `std::env::Vars` and `VarsOs` are not cross-thread iterators. Collect owned
environment entries before moving them to another thread:

```rust
use std::ffi::OsString;

let environment: Vec<(OsString, OsString)> = std::env::vars_os().collect();
let expected_count = environment.len();
let count = std::thread::spawn(move || environment.len())
    .join()
    .expect("environment worker panicked");

assert_eq!(count, expected_count);
```

Prefer collecting only the entries the worker needs rather than copying the
entire process environment by default.

## Non-Null Ownership and Raw Layouts

Rust 1.99 stabilized `Box::into_non_null`/`from_non_null` and
`Vec::into_parts`/`from_parts`. Prefer these only when a real boundary must carry
ownership as a non-null pointer. Keep the element type, allocator/layout,
initialized length, and capacity consistent with the originating allocation;
reconstruct ownership exactly once and account for cleanup on every path.
`NonNull<T>` alone proves neither validity nor exclusive ownership. For a
non-global allocator, use the allocator-carrying variants stabilized in 1.100,
such as `Box::into_non_null_with_allocator`/`from_non_null_in` and
`Vec::into_parts_with_allocator`/`from_parts_in`, and reconstruct with the same
allocator. See the
[`Box` safety contract](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_non_null)
and [`Vec::from_parts` safety contract](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_parts).

The `size_of_val_raw`, `align_of_val_raw`, and
`Layout::for_value_raw` avoid creating a reference merely to query layout, but
remain unsafe and have metadata preconditions. Prefer safe reference-based
layout queries when a valid reference already exists; do not fabricate a
reference or trait-object metadata to satisfy an API.

When reviewing existing unsafe abstractions, re-check the 1.99 update to the
[`Pin::new_unchecked` safety requirements](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked).
Do not use `Box::leak` followed by an assumed-safe reconstruction as an ownership
round trip; use the explicit ownership-transfer APIs and their safety contracts.

## Compiler and Tooling Changes

- Review Rust 1.100 compiler diagnostics separately from Clippy. Remove repeated
  `#[repr(..)]` attributes. Fix `aligned_fields_in_packed`, now a
  deny-by-default lint instead of a hard error, by changing the layout; do not
  allow it. `dropping_references`, `dropping_copy_types`, and
  `undropped_manually_drops` now also check `ptr::drop_in_place`.
  `unfulfilled_lint_expectations` no longer fires for `#[expect(dead_code)]`
  inside code already reported as dead, so stale expectations can stop being
  reported; remove them when the code is touched.
- Closures capture variables used in `if let` more precisely, which can surface
  new borrow errors; restructure the borrow rather than forcing a `move`
  closure or clone. Dyn-compatibility is checked in more places, `box` patterns
  are no longer accepted syntactically, and `let`-`else` respects `macro_rules`
  `expr` metavariable grouping.
- `Iterator::min`, `max`, `min_by_key`, and `max_by_key` call different `Ord`
  methods. A type whose `Ord`, `PartialOrd`, and `PartialEq` disagree can return
  different elements; fix the inconsistent implementation instead of depending on
  the old choice.
- On Unix targets, `ENOTSUP` now maps to `io::ErrorKind::Unsupported` alongside
  `EOPNOTSUPP`. Linux was unaffected because the two codes are equal there; on
  targets where they differ, such as macOS and the BSDs, `ENOTSUP` previously fell
  through to a less specific kind. Match `Unsupported` instead of `Other` or raw
  OS codes in error handling.
- `extern "custom"` is stable for naked functions with a nonstandard calling
  convention. Such functions cannot be called from Rust; keep them behind an
  assembly boundary and document the convention.
- Use `String::from_utf8_lossy_owned` for owned bytes only when lossy decoding is
  intended. Use `FromUtf8Error::into_utf8_lossy` after strict decoding fails.
  Neither choice warrants a promise of allocation reuse.
- Use `VecDeque::retain_back(n)` to retain the last `n` elements. It does not take
  a filtering predicate. Boxed arrays support owned and borrowed `IntoIterator`;
  preserve the caller's intended move or borrow.
- Prefer primitive associated items such as `char::from_u32`,
  `char::REPLACEMENT_CHARACTER`, and `i32::MAX` over the deprecated primitive
  modules.
- Keep `invalid_runtime_symbol_definitions`,
  `suspicious_runtime_symbol_definitions`, and `c_void_returns` active. Rust 1.99
  extended runtime-symbol checks to POSIX names and made
  `no_mangle_generic_items` a hard error. Stable C-variadic definitions and
  `core::ffi::VaList` retain target ABI and safety duties. Use only narrow,
  documented exceptions for adjustable lints at real runtime or FFI boundaries.
- Continue reviewing 1.99 compiler diagnostics separately from Clippy:
  `raw_borrows_via_references`, expanded `unconditional_panic`, and
  `unreachable_cfg_select_predicates`. Fix
  `semicolon_in_expressions_from_non_local_macros` at the macro provider.
  `unused_footnote_definition` belongs to rustdoc validation. Account for unsafe
  attributes in `unsafe_code` policy.
- Check installed Clippy before changing lint configuration. Preserve historical
  stabilization dates for earlier lints and remove deprecated configuration.

## Tooling Compatibility

Rust 1.100 stabilizes `-Cprofile-sample-use` for sample-based profile-guided
optimization. Adopt it only with a representative, reproducible profile and a
measured comparison. On x86, the Rust ABI uses SSE registers when the target
always provides them. On ARM targets, `-Ctarget-feature` or `-Ctarget-cpu` values
that alter the ABI are now hard errors, and ARMv7-A hardfloat targets no longer
assume `d32`. Pass `-Ctarget-feature=+d32` only when the hardware guarantees it.
Treat these as target configuration changes, not source regressions.

Rust has used v0 symbol mangling by default since 1.97. If a profiler, debugger,
crash reporter, or symbol post-processor cannot demangle Rust frames, update that
tool before restoring legacy mangling. Treat changed backtrace spelling as tooling
compatibility evidence rather than an application performance regression.

Rust 1.99 updates LLVM and optimizes inclusive-range iteration. Re-measure hot
loops rather than assuming a universal gain. Do not use the endpoints or
slice-index behavior of an exhausted `core::ops::RangeInclusive` as a stable
contract; preserve original bounds separately when they are needed later.

For build-time comparisons, record the selected Cargo profile and incremental
settings. The new built-in `debug` profile currently inherits `dev`, while CI
turns incremental compilation off by default unless overridden. Compare like
configurations rather than attributing a local/CI difference to source changes.
See the [Rust 1.99 release notes](https://doc.rust-lang.org/releases.html#version-1990-2026-10-01).

On WebAssembly targets, undefined linker symbols are not silently accepted. Treat
link failures as boundary feedback. Override that behavior only when the import
is intentional, documented, and validated by the target-specific build.
