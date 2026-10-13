# Doctest and example patterns

Use doctests when documentation examples are part of the public contract.

## Good fits

- public API examples in rustdoc comments;
- README examples included by crate docs or validated by the repository's existing workflow;
- small examples that demonstrate user-facing behavior with `assert!`, `assert_eq!`, or `assert_matches!` on the Rust 1.99 baseline when a public enum/error variant is the documented contract;
- public invalid-usage examples where `compile_fail` communicates a type-level contract;
- examples that should compile but should not run in CI, using `no_run`.

## Poor fits

Prefer ordinary unit, integration, or compile-fail/UI tests for:

- private implementation details;
- large workflows that make the documentation hard to read;
- examples requiring uncontrolled network, time, host, credentials, or external services;
- diagnostics where exact compiler output matters more than documentation clarity;
- `ignore` blocks used only because the example was not made testable.

## Rustdoc mechanics

- Keep visible examples focused on what a user should learn. Doctests link
  against public crate items; use unit tests for private logic. Broader public
  documentation style belongs to `rust-best-practices` unless the example is
  being tested.
- Unmarked Rust code fences in public docs are usually treated as Rust doctests.
- Ordinary doctests pass when they compile and run without panicking.
- Use hidden `#` setup lines to keep examples compilable without cluttering rendered documentation, including hidden imports such as `# use std::assert_matches;` unless the import itself teaches the public API.
- Prefer `?` for fallible public-API usage. Supply an explicit `fn main() -> Result<..., ...>` or hidden return scaffolding such as `# Ok::<(), ErrorType>(())` so the example compiles without teaching panic-based handling of routine failures.
- Defining a `#[test]` function in an ordinary Rust fence does not run it. Use `rust,test_harness` for test-function examples, or call the behavior directly in the doctest.
- Prefer `no_run` when an example should compile but not execute in CI, such as
  examples with network, file, or process side effects.
- Use `should_panic` only when panic is the documented behavior.
- Use `ignore` only when neither compilation nor execution is appropriate in the normal doctest environment, and explain why when practical.
- Use `compile_fail` only for stable, meaningful invalid-usage examples; keep
  cases small and focused on the documented public rule.
- Avoid relying on exact rustc wording in doctest `compile_fail`; use a UI-test or `trybuild` harness when exact diagnostics are the contract.
- Account for feature flags that affect public documentation examples. A doctest
  command without the required feature does not prove feature-gated examples.

## Rust 1.99 compatibility

The [Rust 1.99 compatibility notes](https://doc.rust-lang.org/releases.html#version-1990-2026-10-01)
change two doctest assumptions:

- Attributes that do not apply to anything are now errors in documentation code
  blocks. Attach the attribute to its intended item or remove the stray
  attribute. Do not hide the failure with `ignore`.
- `doc(cfg(...))` is no longer considered when filtering doctests. It describes
  documentation availability, not an execution guard. Inspect actual `cfg`
  conditions and run the relevant feature/target combinations.

The new `rustdoc::unused_footnote_definition` lint belongs to documentation
validation. Run the repository's rustdoc lint/build command when footnotes or
warning policy change; passing doctests alone does not prove rendered docs are
warning-free. Follow [assertion guidance](automated-testing.md#prefer-assert_matches-for-variant-checks)
for macro stabilization and lower-MSRV boundaries.

## Validation

Use the repository's documented doctest command when present. Common focused commands include:

- `cargo test --doc -p <crate>` for one crate's doctests;
- `cargo test --doc --workspace` when doctest changes span the workspace;
- `cargo test --doc -p <crate> --features <feature>` for feature-gated doctests;
- `cargo test --doc -p <crate> --target <target>` when the target can compile and execute locally or through a working runner, and target-specific doctests are part of the contract;
- a repository-specific `just`, `make`, or CI recipe when one is already established.

Ordinary cross-target doctests execute target binaries. An installed target or a
successful `cargo check --target <target>` establishes compilation support only;
it does not validate doctests. Without an execution environment, use an existing,
verified workflow that only compiles the snippets and report that limited scope,
or mark cross-target doctests not run. Do not add `ignore` or `no_run` merely to
make runnable examples pass on an unavailable target.

When the repository uses cargo-nextest, do not assume `cargo nextest run` covers doctests. Run or recommend a separate doctest command when doctests or public examples are affected. If `.cargo/config.toml` uses cfg-specific `rustdocflags`, or the repository invokes rustdoc `--emit` or `--remap-path-prefix`, inspect that configuration and preserve the documented command before claiming what a doctest command covers. Review emitted or remapped paths when generated documentation output is part of the contract; plain `cargo test --doc` may not reproduce the custom pipeline.
