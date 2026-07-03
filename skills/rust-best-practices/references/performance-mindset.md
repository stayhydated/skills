# Performance Mindset

The first rule of Rust performance work is still: **do not guess, measure**.

Rust is often fast without manual tuning. Optimize after identifying a bottleneck,
and keep the before/after evidence in the PR description or benchmark output.

## First Checks

* Build with `--release` before comparing Rust with another language.
* Use `cargo test --release` for tests that are performance-sensitive.
* Use `cargo bench`, Criterion, `hyperfine`, `cargo flamegraph`, `samply`, or
  platform profilers to find actual bottlenecks.
* Run Clippy's performance lints with the repository's ordinary lint command.

```sh
cargo build --release --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo bench --locked
```

## Avoid Redundant Allocation

Do not collect just to iterate once more.

```rust
fn total_encoded_len<'a>(parts: impl IntoIterator<Item = &'a str>) -> usize {
    parts.into_iter().map(str::len).sum()
}

let total = total_encoded_len(["aa", "bbb", "c"]);
assert_eq!(total, 6);
```

If the caller needs a collection, return a collection. If the callee only needs to
consume items, accept an iterator.

```rust
fn write_records<'a>(records: impl IntoIterator<Item = &'a str>, out: &mut String) {
    for record in records {
        out.push_str(record);
        out.push('\n');
    }
}
```

## Allocate with Intent

Use capacity hints when the size is known or cheaply estimated.

```rust
fn join_codes(codes: &[&str]) -> String {
    let estimated = codes.iter().map(|code| code.len() + 1).sum();
    let mut output = String::with_capacity(estimated);

    for code in codes {
        output.push_str(code);
        output.push(';');
    }

    output
}
```

For large buffers, prefer heap allocation through `Vec`/`Box<[T]>` instead of
placing large arrays on the stack.

```rust
let buffer: Box<[u8]> = vec![0; 64 * 1024].into_boxed_slice();
assert_eq!(buffer.len(), 65_536);
```

## Clone Late or Not at All

Clone handles intentionally; avoid cloning payloads accidentally.

```rust
use std::sync::Arc;

#[derive(Debug)]
struct LookupTable(Vec<u32>);

fn fan_out(table: Arc<LookupTable>, workers: usize) -> Vec<Arc<LookupTable>> {
    (0..workers).map(|_| Arc::clone(&table)).collect()
}
```

`Arc::clone` communicates "new owner of the same allocation" better than
`table.clone()` in shared ownership code.

## Static Dispatch First

Generic functions are monomorphized, which allows inlining and optimization at the
cost of compile time and possible binary growth.

```rust
trait Score {
    fn score(&self) -> u64;
}

fn best_score<T>(items: &[T]) -> Option<u64>
where
    T: Score,
{
    items.iter().map(Score::score).max()
}
```

Use `dyn Trait` when you need runtime heterogeneity, not as a default
abstraction.

## Use Layout-Aware Types

Avoid huge enum variants. Box large variants when they make every value of the
enum unnecessarily large.

```rust
enum DecodeStep {
    Pending,
    Complete(Box<[u8; 4096]>),
}
```

Keep recursive structures behind indirection.

```rust
enum TreeNode<T> {
    Leaf(T),
    Branch(Box<TreeNode<T>>, Box<TreeNode<T>>),
}
```

## Profile Before `#[inline]`

The compiler already inlines aggressively where profitable. Add `#[inline]`,
`#[inline(always)]`, or `#[cold]` only when profiling or generated-code review
supports the decision.

```rust
#[cold]
fn invalid_checksum_observed(expected: u32, actual: u32) -> String {
    format!("checksum mismatch: expected {expected}, got {actual}")
}
```

## WebAssembly Note for Rust 1.96

Rust 1.96 no longer silently allows undefined linker symbols on WebAssembly
targets. Treat new wasm link failures as useful build feedback. Re-enable the old
behavior only when the import is intentional and documented in the wasm boundary.
