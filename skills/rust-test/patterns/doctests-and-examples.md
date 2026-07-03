# Doctest and example patterns

Use doctests when documentation examples are part of the public contract.

## Good fits

- public API examples in rustdoc comments;
- README examples included by crate docs or validated by the repository's existing workflow;
- small examples that demonstrate user-facing behavior with `assert!`, `assert_eq!`, or Rust 1.96+ `assert_matches!` when a public enum/error variant is the documented contract;
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

- Unmarked Rust code fences in public docs are usually treated as Rust doctests.
- Ordinary doctests pass when they compile and run without panicking.
- Use hidden `#` setup lines to keep examples compilable without cluttering rendered documentation, including hidden imports such as `# use std::assert_matches;` when the example uses Rust 1.96+ assertion macros.
- Examples using `?` often need an explicit `fn main() -> Result<..., ...>` shape or another visible return-value pattern.
- Prefer `no_run` when an example should compile but not execute in CI.
- Use `ignore` only when neither compilation nor execution is appropriate in the normal doctest environment, and explain why when practical.
- Use `compile_fail` only for stable, meaningful invalid-usage examples.
- Avoid relying on exact rustc wording in doctest `compile_fail`; use a UI-test or `trybuild` harness when exact diagnostics are the contract.

## Idioms

- Keep visible examples focused on what a user should learn.
- Use hidden `#` setup lines to make examples compile while keeping docs readable.
- Prefer `no_run` over `ignore` when the example should compile but should not execute in the test environment.
- Use `compile_fail` for documented invalid usage, but keep cases small and avoid relying on unstable incidental compiler wording.
- For exact proc-macro or compiler diagnostics, prefer the repository's UI-test or `trybuild` harness over doctests.
- Doctests link against public crate items; use unit tests for private logic.
- Account for feature flags that affect public documentation examples. A doctest command without the required feature does not prove feature-gated examples.
- Keep doctest guidance about compile/run behavior; broader public documentation style belongs to `rust-best-practices` unless the example is being tested.

## Validation

Use the repository's documented doctest command when present. Common focused commands include:

- `cargo test --doc -p <crate>` for one crate's doctests;
- `cargo test --doc --workspace` when doctest changes span the workspace;
- `cargo test --doc -p <crate> --features <feature>` for feature-gated doctests;
- `cargo test --doc -p <crate> --target <target>` only when that target can be compiled in the environment and target-specific rustdoc configuration is part of the contract;
- a repository-specific `just`, `make`, or CI recipe when one is already established.

When the repository uses cargo-nextest, do not assume `cargo nextest run` covers doctests. Run or recommend a separate doctest command when doctests or public examples are affected. If `.cargo/config.toml` uses cfg-specific `rustdocflags`, inspect that configuration before claiming what a doctest command covers.
