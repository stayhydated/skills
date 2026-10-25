# Rust and Cargo Baseline Additions

Use these implementation and tooling details when the repository targets Rust
1.99. Respect a lower declared MSRV and target-specific availability. Consult the
[Rust release notes](https://doc.rust-lang.org/releases.html#version-1990-2026-10-01)
for stabilization dates; carried-forward APIs are not all new in 1.99.

## Contents

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

Rust 1.99 stabilizes `Box::into_non_null`/`from_non_null` and
`Vec::into_parts`/`from_parts`. Prefer these only when a real boundary must carry
ownership as a non-null pointer. Keep the element type, allocator/layout,
initialized length, and capacity consistent with the originating allocation;
reconstruct ownership exactly once and account for cleanup on every path.
`NonNull<T>` alone proves neither validity nor exclusive ownership. See the
[`Box` safety contract](https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_non_null)
and [`Vec::from_parts` safety contract](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.from_parts).

The newly stable `size_of_val_raw`, `align_of_val_raw`, and
`Layout::for_value_raw` avoid creating a reference merely to query layout, but
remain unsafe and have metadata preconditions. Prefer safe reference-based
layout queries when a valid reference already exists; do not fabricate a
reference or trait-object metadata to satisfy an API.

When reviewing existing unsafe abstractions on 1.99, re-check the updated
[`Pin::new_unchecked` safety requirements](https://doc.rust-lang.org/std/pin/struct.Pin.html#method.new_unchecked).
Do not use `Box::leak` followed by an assumed-safe reconstruction as an ownership
round trip; use the explicit ownership-transfer APIs and their safety contracts.

## Compiler and Tooling Changes

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
  extends runtime-symbol checks to POSIX names and makes
  `no_mangle_generic_items` a hard error. Stable C-variadic definitions and
  `core::ffi::VaList` retain target ABI and safety duties. Use only narrow,
  documented exceptions for adjustable lints at real runtime or FFI boundaries.
- Review 1.99 compiler diagnostics separately from Clippy:
  `raw_borrows_via_references`, expanded `unconditional_panic`, and
  `unreachable_cfg_select_predicates`. Fix
  `semicolon_in_expressions_from_non_local_macros` at the macro provider.
  `unused_footnote_definition` belongs to rustdoc validation. Account for unsafe
  attributes in `unsafe_code` policy.
- Check installed Clippy before changing lint configuration. Preserve historical
  stabilization dates for earlier lints and remove deprecated configuration.

## Tooling Compatibility on Rust 1.99

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
