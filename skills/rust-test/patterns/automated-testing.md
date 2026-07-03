# Automated testing patterns

Tests are executable documentation. They should show what the unit does, what the
important states are, and what failures look like.

## Test Shape

Unit tests belong near the code they test and can inspect private helpers.
Integration tests belong under `tests/` and should exercise the public API.
Doc tests belong in rustdoc examples and should cover public happy paths and
important edge cases.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    mod parse_frame {
        use super::*;

        #[test]
        fn rejects_missing_separator() {
            let result = parse_frame("header-only");
            assert!(result.is_err());
        }
    }
}
```

Prefer a module named after the unit under test and test functions that read like
behavior statements.

## One Behavior per Test

Avoid broad "happy path" tests with many unrelated assertions.

```rust
#[test]
fn active_marker_is_preserved() {
    let marker = normalize_marker(" Active ");
    assert_eq!(marker, "active");
}

#[test]
fn blank_marker_uses_default() {
    let marker = normalize_marker("   ");
    assert_eq!(marker, "default");
}
```

Use helper functions for setup, not for hiding assertions.

```rust
fn sample_batch() -> Batch {
    Batch {
        id: "batch-1".to_owned(),
        records: vec!["a".to_owned(), "b".to_owned()],
    }
}
```

## Prefer `assert_matches!` for Variant Checks

On a Rust 1.96+ MSRV, prefer stable `assert_matches!` when the test contract is a
single structured pattern: enum variants, typed errors, state-machine phases,
parse outcomes, or other values where the mismatched debug shape should appear in
the failure. Import the macro explicitly from `std` in ordinary tests or from
`core` in `no_std` test contexts.

```rust
use std::assert_matches;

#[derive(Debug, PartialEq, Eq)]
enum FrameError {
    Empty,
    MissingHeader,
}

fn parse_frame(input: &str) -> Result<&str, FrameError> {
    if input.is_empty() {
        return Err(FrameError::Empty);
    }
    input.split_once(':').map(|(_, body)| body).ok_or(FrameError::MissingHeader)
}

#[test]
fn empty_frame_is_rejected() {
    assert_matches!(parse_frame(""), Err(FrameError::Empty));
}
```

Do not use `debug_assert_matches!` as the ordinary test assertion; it is for
invariant checks that intentionally follow debug-assertion behavior. If the
repository MSRV is below Rust 1.96, preserve the existing assertion idiom or
label this as an MSRV-gated recommendation.

Use `assert_eq!` when equality is the behavior. Use `assert!` for boolean
properties with a useful failure message.

```rust
let output = normalize_marker(" ready ");
assert!(
    output.chars().all(char::is_lowercase),
    "marker should be lowercase: {output:?}"
);
```

When checking error behavior, prefer matching structured variants, spans, exit
codes, and machine-readable fields. Assert exact error text only when the
user-facing message itself is part of the public contract.

## Parameterized Tests

Use table-driven loops for tiny pure cases where a single failure message is good
enough. Use `rstest` when named cases help navigation.

```rust
#[test]
fn trims_outer_whitespace() {
    for (input, expected) in [(" alpha", "alpha"), ("beta ", "beta"), (" gamma ", "gamma")] {
        assert_eq!(normalize_marker(input), expected, "input: {input:?}");
    }
}
```

With `rstest`, keep case names descriptive:

```rust
use rstest::rstest;

#[rstest]
#[case::leading(" alpha", "alpha")]
#[case::trailing("beta ", "beta")]
#[case::both(" gamma ", "gamma")]
fn normalizes_whitespace(#[case] input: &str, #[case] expected: &str) {
    assert_eq!(normalize_marker(input), expected);
}
```

## Doc Tests

Rustdoc examples are compiled and run by `cargo test` unless marked otherwise.
Use hidden lines (`#`) to make examples self-contained without distracting the
reader. When a doctest uses a test-only macro such as `assert_matches!`, hide the
macro import unless the import itself teaches the public API.

````rust
/// Normalizes an external marker into the canonical lowercase form.
///
/// # Examples
///
/// ```rust
/// # use crate_name::normalize_marker;
/// let marker = normalize_marker(" Ready ");
/// assert_eq!(marker, "ready");
/// ```
pub fn normalize_marker(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        "default".to_owned()
    } else {
        trimmed.to_ascii_lowercase()
    }
}
````

Use rustdoc block attributes intentionally:

* `no_run`: compiles but does not execute, useful for examples with network, file,
  or process side effects.
* `compile_fail`: proves an invalid usage stays invalid.
* `should_panic`: only when panic is the documented behavior.
* `ignore`: avoid unless the example cannot be compiled in CI.

If using `cargo nextest`, run doc tests separately with `cargo test --doc`.

## Integration Tests

Integration tests live outside the crate and can only use public APIs.

```text
crate-name/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    ├── common/
    │   └── mod.rs
    └── public_flow.rs
```

Keep shared integration helpers under `tests/common/mod.rs` and avoid turning the
integration suite into a second application.

## Snapshot Tests

Use snapshot testing when output is structural, textual, generated, or hard to
read in `assert_eq!`.

```toml
[dev-dependencies]
insta = { version = "1", features = ["yaml"] }
```

```rust
#[test]
fn summary_report_shape_is_stable() {
    let report = SummaryReport {
        processed: 3,
        rejected: 1,
    };

    insta::assert_yaml_snapshot!("reports/summary", report);
}
```

Commit snapshots. Review changes as carefully as source code. Redact timestamps,
random identifiers, host paths, and other unstable fields.

```rust
insta::assert_json_snapshot!(
    "jobs/completed",
    job_payload,
    ".finished_at" => "[timestamp]",
    ".run_id" => "[run-id]"
);
```

Do not snapshot tiny primitive logic. `assert_eq!(count, 3)` is clearer than a
snapshot containing `3`.
