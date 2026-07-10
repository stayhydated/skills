# Compile-fail and diagnostics patterns

Use compile-fail or UI tests when Rust compiler behavior is part of the contract.

## Good fits

- proc-macro syntax errors and generated diagnostics;
- derive macro trait-bound failures;
- type-level API contracts that should fail to compile;
- confusing inference or lifetime failures that the crate intentionally supports with clear errors;
- public diagnostics documented in READMEs, examples, or API docs;
- doctest `compile_fail` examples for small documented invalid usage.

## Harness choice

Prefer the repository's existing harness. If it already uses `trybuild`, add focused `pass` or `compile_fail` cases in the established fixture layout. If it uses another UI-test harness, follow that instead.

Do not introduce `trybuild` or another harness in a patch unless the user requested it or the repository already standardizes on it. In audits or strategy notes, label the addition as **Recommended**.

Use rustdoc `compile_fail` for documentation examples, not as a replacement for a diagnostic harness when exact stderr wording, proc-macro expansion errors, or many UI fixtures are the real contract.

## Fixture discipline

- Keep failing examples minimal and named after the contract.
- Put only one main failure mode in a compile-fail fixture when practical.
- Avoid line-number-sensitive expectations unless line numbers are part of the harness output and unavoidable.
- Avoid compile-fail tests that merely call every API with arbitrary wrong types; they add churn without protecting a meaningful diagnostic.
- Update `.stderr` or diagnostic snapshots only when the diagnostic change is intentional.
- Prefer generated `.stderr` updates through the established harness workflow, then review the diff rather than handwriting large compiler output.
- Keep public docs, examples, and diagnostics aligned when public macro behavior changes.
- Treat Rust version changes as expectation-file risk: a new stable compiler can alter accepted syntax, warnings, path rendering, symbol spelling, or diagnostic wording even when the public contract is unchanged.

## trybuild workflow

When the repository uses `trybuild`:

- Use `TestCases::new().pass(...)` for fixtures that should compile.
- Use `TestCases::new().compile_fail(...)` for fixtures that should fail to compile.
- Put fixtures in the repository's established location, commonly `tests/ui`, `tests/compile-fail`, or a crate-specific equivalent.
- If a `.stderr` expectation is missing or stale, expect trybuild to write observed output under a `wip/` directory.
- Use `TRYBUILD=overwrite cargo test ...` only when intentionally regenerating expectations.
- Always review `git diff` for changed `.stderr` files and disclose reviewed paths in the handoff.
- Keep fixtures independent of rustc wording that is likely to churn unless the wording itself is the public diagnostic contract.
- Do not add compile-fail fixtures merely to call every public API incorrectly; protect meaningful diagnostics and type-level contracts.

## Rust 1.97-sensitive compiler contracts

Use Rust 1.97-specific compile-fail guidance only for compiler-facing contracts. Examples include a public API that must reject invalid trait, type-state, macro, target, or feature combinations; a proc macro whose diagnostic changed with the compiler; or a target support claim that changed compile/link behavior. Do not add UI fixtures merely to enforce broad style guidance that belongs to `rust-best-practices`.

Rust 1.97 compatibility changes can affect fixtures involving `pin!` coercions,
generic arguments on module path segments, tuple-index shorthands in struct
patterns, malformed `link_name`/`link` attributes, empty `export_name` values, or
invalid Mach-O `link_section` specifiers. Update expectations only when the
changed acceptance or diagnostic is relevant to the repository's contract.

If the repository MSRV is lower than Rust 1.97, keep fixtures and expected diagnostics compatible with the declared MSRV unless the user explicitly asks to raise or audit the MSRV. Remember that `assert_matches!` itself requires Rust 1.96 or newer.

## Doctest compile-fail boundary

Use rustdoc `compile_fail` when the invalid example belongs in public documentation and does not need exact stderr matching. Keep the visible example short and focused on the public rule. For exact diagnostics, proc-macro output, or multiple UI fixtures, prefer the repository's established diagnostic harness.

## Validation

Run the narrowest test command that exercises the UI harness. When diagnostic expectations change, report both the validation command and the reviewed expectation path.

Common focused commands include:

- `cargo test -p <crate> <trybuild_test_name>` for a trybuild driver test;
- `TRYBUILD=overwrite cargo test -p <crate> <trybuild_test_name>` only when intentionally regenerating expectations;
- `cargo test --doc -p <crate>` for doctest `compile_fail` examples;
- repository-specific UI-test commands when already documented in CI or runner files.
