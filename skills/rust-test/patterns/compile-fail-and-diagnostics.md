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

Use rustdoc `compile_fail` when a short, focused invalid example belongs in public documentation and does not need exact stderr matching. For exact diagnostics, proc-macro expansion errors, or multiple UI fixtures, prefer the established diagnostic harness.

## Fixture discipline

- Keep failing examples minimal and named after the contract.
- Put only one main failure mode in a compile-fail fixture when practical.
- Pair meaningful rejection cases with valid compile-pass usage and verify the failure is for the intended contract, not a missing import, dependency, or unrelated syntax error.
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
- If a `.stderr` expectation is missing, trybuild writes the observed output under `wip/` and fails the test for review.
- If an existing `.stderr` expectation is stale, trybuild displays the expected and actual output and fails the test; it leaves the expectation unchanged and creates no `wip/` replacement.
- Use `TRYBUILD=overwrite cargo test ...` only when intentionally regenerating expectations.
- Always review `git diff` for changed `.stderr` files and disclose reviewed paths in the handoff.
- Keep fixtures independent of rustc wording that is likely to churn unless the wording itself is the public diagnostic contract.

## Rust 1.99-sensitive compiler contracts

Use the [baseline compiler and expectation compatibility notes](rust-1-99-testing-baseline.md#compiler-and-expectation-file-compatibility)
for release-specific acceptance, lints, escaping, and carried-forward compiler
contracts. Apply [doctest compatibility rules](doctests-and-examples.md#rust-199-compatibility)
without accepting an unrelated attribute error as proof of the intended invalid
usage.

Update expectations only when the changed acceptance or diagnostic is relevant
to the repository's public or type-level contract. If a lower MSRV is explicitly
supported, keep fixtures and expected diagnostics compatible with that lane or
use the repository's version-gated UI convention. Follow the fixture discipline
and harness workflow above when regenerating expectations.

## Validation

Run the narrowest test command that exercises the UI harness. When diagnostic expectations change, report both the validation command and the reviewed expectation path.

Common focused commands include:

- `cargo test -p <crate> <trybuild_test_name>` for a trybuild driver test;
- `TRYBUILD=overwrite cargo test -p <crate> <trybuild_test_name>` only when intentionally regenerating expectations;
- `cargo test --doc -p <crate>` for doctest `compile_fail` examples;
- repository-specific UI-test commands when already documented in CI or runner files.
