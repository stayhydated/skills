# Coding Styles and Idioms

## Borrow Before You Clone

Rust's ownership model makes borrowing the default shape for read-only code. A
function that only observes data should usually accept `&T`, `&str`, or `&[T]`.

```rust
fn render_label(label: &str) -> String {
    format!("label:{label}")
}

let owned_label = String::from("batch-alpha");
let rendered = render_label(&owned_label);
assert_eq!(rendered, "label:batch-alpha");
```

Clone only when you actually need a second owned value. Common valid cases are
reference-counted handles (`Arc`, `Rc`), snapshots, background tasks that need
`'static` ownership, or APIs that intentionally store the value.

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
struct PacketTemplate {
    headers: Vec<String>,
}

fn checkpoint(template: &PacketTemplate) -> PacketTemplate {
    template.clone()
}
```

Avoid accidental clones inside loops. Prefer iterator adapters that show intent.

```rust
let labels = [String::from("fast"), String::from("safe")];
let copied: Vec<String> = labels.iter().cloned().collect();
assert_eq!(copied, labels);
```

## Pass Small `Copy` Values by Value

If a type is small and implements `Copy`, passing by value is often clearer and
at least as efficient as passing by reference.

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
struct GridPoint {
    row: u16,
    col: u16,
}

fn shift_right(point: GridPoint) -> GridPoint {
    GridPoint {
        col: point.col + 1,
        ..point
    }
}

let start = GridPoint { row: 4, col: 7 };
assert_eq!(shift_right(start), GridPoint { row: 4, col: 8 });
assert_eq!(start.col, 7);
```

Consider deriving `Copy` only when all fields are `Copy`, the value is small, and
copying it cannot hide ownership or resource semantics. Do not force `Copy` onto
large arrays, heap-owning data, locks, file handles, sockets, or domain values
whose duplication should be explicit.

## `Option` and `Result`: Choose the Clearest Control Flow

Use `?` when the caller can handle the same error type or a `From` conversion is
available.

```rust
#[derive(Debug, thiserror::Error)]
enum HeaderError {
    #[error("missing separator")]
    MissingSeparator,
    #[error("invalid revision")]
    InvalidRevision(#[from] std::num::ParseIntError),
}

#[derive(Debug, PartialEq, Eq)]
struct Header {
    kind: String,
    revision: u16,
}

fn parse_header(input: &str) -> Result<Header, HeaderError> {
    let Some((kind, revision)) = input.split_once(':') else {
        return Err(HeaderError::MissingSeparator);
    };

    Ok(Header {
        kind: kind.to_owned(),
        revision: revision.parse()?,
    })
}
```

Use `let-else` when a missing value is an expected branch and the fallback can
return, break, continue, or otherwise diverge.

```rust
fn first_non_empty<'a>(lines: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        return Some(trimmed);
    }
    None
}
```

Use `match` when each variant carries meaningful behavior.

```rust
#[derive(Debug, PartialEq, Eq)]
enum Signal {
    Start,
    Data(Vec<u8>),
    Stop,
}

fn signal_cost(signal: &Signal) -> usize {
    match signal {
        Signal::Start => 1,
        Signal::Data(bytes) => bytes.len(),
        Signal::Stop => 0,
    }
}
```

Avoid `unwrap` and `expect` outside tests unless failure is impossible by a local
invariant and the message explains that invariant.

## Avoid Early Allocation

Prefer lazy alternatives when the fallback allocates, logs, performs I/O, or calls
a function with side effects.

```rust
#[derive(Debug, PartialEq, Eq)]
struct ParseNote(String);

fn label_or_default(label: Option<&str>) -> String {
    label
        .map(str::to_owned)
        .unwrap_or_else(|| "unnamed-batch".to_owned())
}

fn note_for_code(code: Result<u16, ParseNote>) -> String {
    code.map_or_else(
        |err| format!("invalid code: {}", err.0),
        |value| format!("code:{value}"),
    )
}
```

Use `ok_or_else`, `unwrap_or_else`, and `map_or_else` when constructing the
fallback is nontrivial. Use `unwrap_or_default` when the default is the obvious
empty/default value.

## Iterators and `for` Loops

Both iterator chains and `for` loops are idiomatic. Pick the one that preserves
intent.

Prefer iterator chains for transformations:

```rust
fn active_codes(records: &[(&str, bool)]) -> Vec<String> {
    records
        .iter()
        .filter(|(_, active)| *active)
        .map(|(code, _)| code.to_ascii_uppercase())
        .collect()
}

let records = [("ab", true), ("cd", false), ("ef", true)];
assert_eq!(active_codes(&records), ["AB", "EF"]);
```

Prefer `for` when early exit, mutation, or side effects are central:

```rust
fn stop_at_marker(values: &mut [u16], marker: u16) {
    for value in values {
        if *value == marker {
            break;
        }
        *value += 1;
    }
}
```

Do not allocate intermediate collections just to iterate again. Accept iterators
when the consumer does not require random access.

```rust
fn checksum(bytes: impl IntoIterator<Item = u8>) -> u8 {
    bytes.into_iter().fold(0, u8::wrapping_add)
}

let checksum = checksum([1, 2, 3].into_iter().map(|byte| byte * 2));
assert_eq!(checksum, 12);
```

## Range APIs on the Rust 1.97 Baseline

The concrete range types stabilized under `core::range` in Rust 1.96 are available
on the Rust 1.97 baseline. They are useful when a range is stored in a small
`Copy` value. For public APIs, prefer accepting `impl RangeBounds<usize>` so
callers can pass both legacy range syntax and concrete range values.

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

For generic public range inputs, translate `RangeBounds` once at the boundary:

```rust
use core::ops::{Bound, RangeBounds};

fn select_window<R>(bytes: &[u8], range: R) -> &[u8]
where
    R: RangeBounds<usize>,
{
    let start = match range.start_bound() {
        Bound::Included(index) => *index,
        Bound::Excluded(index) => *index + 1,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(index) => *index + 1,
        Bound::Excluded(index) => *index,
        Bound::Unbounded => bytes.len(),
    };
    &bytes[start..end]
}

assert_eq!(select_window(b"abcdef", 2..=4), b"cde");
```

## Integer Bit Inspection in Rust 1.97

Use the stable integer methods introduced in Rust 1.97 when the domain is about
bit positions or one-bit masks. They make the zero case and return shape explicit
and avoid hand-written shift arithmetic.

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

Use the corresponding `NonZero` methods when zero is already excluded by the type.
Do not raise a library's MSRV solely to replace clear existing bit operations
unless that change is part of the release policy.

## Prefer `char` Associated Items

Rust 1.97 deprecates constants and functions reached through the `std::char`
module. Use the primitive type's associated items instead:

```rust
let replacement = char::REPLACEMENT_CHARACTER;
let decoded = char::from_u32(0x1F980);

assert_eq!(replacement, '\u{FFFD}');
assert_eq!(decoded, Some('🦀'));
```

Preserve an older spelling only when required by a declared pre-1.97 MSRV.

## Imports

Group imports by origin and keep them boring:

```rust
use std::sync::Arc;
use std::time::Duration;

use tracing::Instrument;

use crate::queue::WorkItem;
use crate::telemetry::SpanName;
```

Use stable `rustfmt` defaults unless the repository already uses nightly rustfmt
for import grouping. Avoid introducing nightly-only formatting configuration just
to reshuffle imports.
