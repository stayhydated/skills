---
name: rust-best-practices
description: >
  Guide for writing, refactoring, reviewing, optimizing, and documenting
  idiomatic Rust code on a Rust 1.96 stable baseline.
---

# Rust Best Practices

Use this skill when the user asks for Rust code, Rust refactors, code review,
performance review, error handling, documentation, API design, or lifecycle
modeling.

Assume **Rust 1.96 stable** and **edition 2024** unless the repository declares a
stricter MSRV or the user gives a different target. Respect existing
`rust-toolchain.toml`, CI, `Cargo.toml`, workspace lints, and public API stability
before introducing a Rust feature that would raise MSRV.

## Rust 1.96 Baseline Guidance

* Prefer stable Rust. Do not suggest nightly-only features unless the user already
  uses nightly and the reason is explicit.
* Use `let PATTERN = expr else { ... };` for early returns where the fallback does
  not need the failed value.
* Use `?` for straightforward error propagation, `map_err` for typed translation,
  and `inspect_err` for local observability.
* Use `core::range::{Range, RangeInclusive, RangeFrom}` when a concrete stored
  range needs to be `Copy`. For public APIs, prefer accepting
  `impl core::ops::RangeBounds<usize>` unless the concrete range type is part of
  the domain model.
* Prefer borrowing (`&str`, `&[T]`, `&T`) for read-only APIs. Take ownership only
  when the function stores, transforms, or consumes the value.
* Prefer `impl Trait` for single-use input polymorphism. Use named generics when
  two parameters or a return value must share the same type.
* Prefer static dispatch until heterogeneity, plugin boundaries, or ABI-like
  abstraction require `dyn Trait`.
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
