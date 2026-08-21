---
name: rust-best-practices
description: >
  Guide for writing, refactoring, reviewing, optimizing, and documenting
  idiomatic Rust code on a Rust 1.98 stable baseline.
---

# Rust Best Practices

Use this skill when the user asks for Rust code, Rust refactors, code review,
performance review, error handling, documentation, API design, or lifecycle
modeling.

Assume **Rust 1.98 stable** and **edition 2024** unless the repository explicitly
declares a lower MSRV or the user gives a different target. Respect existing
`rust-toolchain.toml`, CI, `Cargo.toml`, workspace lints, target support, and
public API stability before introducing an API that exceeds the declared MSRV.

## Rust 1.98 Baseline Guidance

* Prefer stable Rust. Do not suggest nightly-only features unless the repository
  already uses nightly and the reason is explicit.
* Use `let PATTERN = expr else { ... };` for early exits where the fallback does
  not need the failed value.
* Use `?` for straightforward error propagation, `map_err` for typed translation,
  and `inspect_err` for local observability.
* Use `str::substr_range` and `[T]::subslice_range` to recover source offsets from
  subslices already derived from the same source. These APIs use provenance and
  pointer arithmetic; they do not search by value.
* Use `strip_circumfix` when a prefix and suffix must both be present and
  non-overlapping. Prefer it to chained stripping that can accidentally accept a
  one-sided match.
* Use `core::range::{Range, RangeInclusive, RangeFrom}` when a concrete stored
  range benefits from being `Copy`. For public APIs, usually accept
  `impl core::ops::RangeBounds<usize>` unless the concrete type is part of the
  domain model.
* Prefer direct typed operations: integer bit-inspection methods for bit-domain
  logic, `NonZero*::from_str_radix` for non-zero radix parsing, and
  `String::from_utf16le`/`from_utf16be` when byte order is part of the input
  format.
* Use `core::fmt::NumBuffer` with integer `format_into` only in measured,
  allocation-sensitive paths where the caller can reuse the buffer. Use normal
  formatting for ordinary code.
* Use floating-point `algebraic_*` operations only when profiling shows a benefit
  and the API contract tolerates reassociation, unspecified precision, and
  non-deterministic behavior for NaN, infinity, and signed zero.
* Prefer primitive `char` associated items such as `char::from_u32` and
  `char::REPLACEMENT_CHARACTER`.
* Prefer borrowing (`&str`, `&[T]`, `&T`) for read-only APIs. Take ownership only
  when the function stores, transforms, or consumes the value.
* Prefer `impl Trait` for single-use input polymorphism. Use named generics when
  multiple parameters or a return value must share the same type.
* Prefer static dispatch until heterogeneity, plugin boundaries, or ABI-like
  abstraction require `dyn Trait`.
* Keep runtime-symbol and FFI definitions lint-clean. Do not globally suppress
  `invalid_runtime_symbol_definitions`, `suspicious_runtime_symbol_definitions`,
  or `c_void_returns`; use a narrow, documented exception only at a real runtime
  or foreign-function boundary.
* Use `bon` for builders. Do not hand-roll type-state builders for ordinary
  construction.
* Use `statum` for real lifecycle or protocol type-state. Do not use type-state as
  a generic replacement for builders or enums.
* Keep examples generic, self-contained, and domain-neutral unless the user gives
  domain vocabulary.

## References

Read only the chapters relevant to the task. For broad reviews or refactors, read
several relevant chapters together before giving feedback.

* [Coding Styles and Idioms](references/coding-styles-and-idioms.md)
* [Code Generation Templates](references/code-generation-templates.md)
* [Enum String Conversions](references/enum-string-conversions.md)
* [Clippy and Linting Discipline](references/clippy-and-linting-discipline.md)
* [Performance Mindset](references/performance-mindset.md)
* [Error Handling](references/error-handling.md)
* [Generics, Static Dispatch, and Dynamic Dispatch](references/generics-static-and-dynamic-dispatch.md)
* [Builder Pattern](references/builder-pattern.md)
* [Type State Pattern](references/type-state-pattern.md)
* [Comments vs Documentation](references/comments-vs-documentation.md)
* [Pointer Types and Thread Safety](references/pointer-types-and-thread-safety.md)
