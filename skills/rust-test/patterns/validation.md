# Rust test validation patterns

## Contents

- [Command selection](#command-selection)
- [Common command shapes](#common-command-shapes)
- [Cargo configuration boundaries](#cargo-argument-and-configuration-boundaries)
- [Nextest](#cargo-nextest-guidance)
- [Feature, target, and MSRV validation](#feature-target-and-msrv-validation)
- [Rust 1.99 validation](#rust-199-specific-validation)
- [Expectation-file handoff](#expectation-file-handoff)
- [Coverage and mutation evidence](#coverage-and-mutation-evidence)
- [Validation wording](#validation-wording)

Use the narrowest command that proves the affected behavior. Do not invent repository-specific commands; inspect manifests, CI, runner files, and docs first. Generic `cargo` commands are appropriate only when they are directly runnable in the repository and match the affected package, target, feature, or test surface.

Before reporting validation, use [Cargo test semantics](cargo-test-semantics.md)
to establish the selected contracts and active configuration.

## Command selection

Prefer, in order:

1. The repository's focused runner recipe for the affected surface.
2. A package-specific, test-name-filtered, target-filtered, benchmark-name-filtered, doctest, feature-specific, target-specific, fuzz reproduction, or expectation-review command.
3. Dependent package tests when a public API, shared fixture, generated output, feature, or workspace dependency change can affect downstream crates in the workspace.
4. The repository's standard full Rust test command when the change spans surfaces or no narrower command proves the contract.
5. Static review only, with explicit disclosure, when commands are unavailable.

In workspaces, identify the affected package graph before selecting validation. Prefer package-scoped commands first, then dependent package tests when public APIs or shared fixtures changed. Avoid defaulting to full-workspace tests unless the change crosses package boundaries or no narrower command proves the contract.

## Common command shapes

Use only when evidenced or directly runnable in the repository:

- `cargo test -p <crate> <test_name>` for focused crate tests.
- `cargo test -p <crate> --lib <test_name>` for a library unit-test target.
- `cargo test -p <crate> --test <integration_test>` for a specific integration test target.
- `cargo test -p <crate> --example <example_name>` when an example target has tests or must compile.
- `cargo test -p <crate>` for one crate.
- `cargo test --doc -p <crate>` for doctests.
- `cargo test -p <crate> -- --test-threads=1` only when serial runtime execution is required by repository policy or the test contract.
- `cargo insta test` and `cargo insta review` when `cargo-insta` is part of the workflow.
- `cargo nextest run ...` when the repository uses nextest for normal unit/integration test execution.
- `cargo bench --bench <bench_name>` for an existing benchmark target.
- `cargo bench --bench <bench_name> --no-run` when benchmark compilation is the only practical validation.
- `cargo fuzz run <target>` when the repository uses cargo-fuzz and the command is in scope.
- `cargo miri test ...`, sanitizer commands, or loom commands only when configured, requested, or clearly labeled as recommended.
- `just <recipe>` or `make <recipe>` when the repository has a fitting recipe.

## Cargo argument and configuration boundaries

Use [Cargo argument and configuration semantics](cargo-test-semantics.md#default-behavior-to-account-for)
for Cargo versus libtest arguments, build versus runtime parallelism, profiles,
feature inheritance, warning policy, CI incremental defaults, and lockfile
selection. Check [active configuration sources](cargo-test-semantics.md#working-directories-configuration-and-environment)
and the [target-selection limits](cargo-test-semantics.md#test-target-selection)
before claiming what a command proves. Report compile-only checks separately
from execution, and verify reported counts and ignored tests rather than treating
zero matching tests as behavioral validation.

## cargo-nextest guidance

When the repository uses cargo-nextest:

- Prefer `cargo nextest run -p <crate> <filter>` for normal unit and integration tests when that is the repository workflow.
- Inspect `.config/nextest.toml` before changing retries, slow-test settings, timeout policy, status levels, or per-test overrides.
- Do not assume nextest covers doctests. Run or recommend `cargo test --doc -p <crate>` separately when doctests are affected.
- Treat retries as flake classification or quarantine, not a fix. Report flaky-pass behavior rather than calling it fully stable.
- Preserve repository-standard profiles, such as default, CI, or slow-test profiles, rather than inventing new command policy.

## Feature, target, and MSRV validation

Use the [feature, target, and MSRV pattern](feature-msrv-matrix.md) to identify the
affected axes, choose [focused matrix commands](feature-msrv-matrix.md#focused-matrix-commands),
and apply [cargo-hack guidance](feature-msrv-matrix.md#cargo-hack-guidance) within
the skill's dependency boundary.

For target-specific doctests, follow the
[doctest execution and compilation boundary](doctests-and-examples.md#validation):
a target check does not prove doctests. Report an existing verified
snippet-compilation workflow separately from execution, or mark those doctests
not run.

Disclose mutually exclusive features, missing target toolchains, unavailable
linkers, MSRV toolchain gaps, baseline APIs or configuration unavailable on an
explicitly declared lower-MSRV lane, and target tests checked but not executed.

## Rust 1.99-specific validation

Use the [baseline pattern](rust-1-99-testing-baseline.md) for detailed API edge
cases, compiler/expectation compatibility, and target contracts. Select validation
that proves the affected contract:

- Run the smallest package/test-target command for an assertion change, with a
  separate doctest run when the assertion appears there.
- Validate algebraic floating-point tolerances in the relevant optimization
  profile. Use the repository's benchmark workflow when allocation, throughput,
  or vectorization is the claim.
- Pair target-aware compilation with the configured runtime, concurrency,
  safe-wrapper, ABI, or platform evidence when the contract requires it. Host
  functional tests alone do not prove soundness, cross-target calling
  conventions, or filesystem precision.
- Apply [Cargo configuration boundaries](cargo-test-semantics.md#default-behavior-to-account-for)
  and [matrix discipline](feature-msrv-matrix.md#rust-199-specific-matrix-notes)
  before reporting selected features, profiles, warning policy, or lockfiles.
- Apply [doctest compatibility guidance](doctests-and-examples.md#rust-199-compatibility)
  to actual feature/target gates; run the documentation build/lint command
  separately when rustdoc lint behavior is affected.
- Regenerate expectations through the repository workflow and review the diff
  using [baseline compatibility notes](rust-1-99-testing-baseline.md#compiler-and-expectation-file-compatibility).

## Expectation-file handoff

For snapshots, golden files, diagnostics, fuzz corpora, generated outputs, and benchmark reports, say exactly what happened:

- `Reviewed snapshot diff: <path>` when inspected.
- `Reviewed diagnostic expectation: <path>` when inspected.
- `Reviewed golden diff: <path>` when inspected.
- `Reviewed generated-output diff: <path>` when inspected.
- `Reviewed corpus artifact: <path>` when inspected.
- `Reviewed benchmark output: <path or summary>` when inspected.
- `Not reviewed; snapshot tool unavailable: <reason>` when not inspected.
- `Not reviewed; benchmark environment unavailable or noisy: <reason>` when performance output was not meaningful.
- `Not updated; behavior change did not affect expectations` when applicable.

For `trybuild`, distinguish new expectations written under `wip/` from mismatches
against existing `.stderr` files. Report whether expectations were intentionally
regenerated with `TRYBUILD=overwrite` and reviewed through `git diff`.

## Coverage and mutation evidence

Use coverage or mutation-testing results only as supporting evidence. Do not introduce coverage or mutation tooling as a default patch. When the repository already uses such tools or the user asks for them:

- run the narrowest repository-standard coverage or mutation command;
- review reports for missed contracts, weak assertions, or surviving mutants rather than reporting only a percentage;
- disclose noise, unsupported targets, generated code, equivalent mutants, and unexecuted feature combinations.

## Validation wording

Use the exact [validation status templates](../SKILL.md#final-response-format)
from the skill entrypoint, including failure summaries and whether attempted
validation failures appear related to the change.
