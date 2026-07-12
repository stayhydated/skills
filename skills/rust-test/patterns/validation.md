# Rust test validation patterns

## Contents

- [Command selection](#command-selection)
- [Common command shapes](#common-command-shapes)
- [Cargo configuration boundaries](#cargo-argument-and-configuration-boundaries)
- [Nextest](#cargo-nextest-guidance)
- [Feature, target, and MSRV validation](#feature-target-and-msrv-validation)
- [Rust 1.97 validation](#rust-197-specific-validation)
- [Expectation-file handoff](#expectation-file-handoff)
- [Coverage and mutation evidence](#coverage-and-mutation-evidence)
- [Validation wording](#validation-wording)

Use the narrowest command that proves the affected behavior. Do not invent repository-specific commands; inspect manifests, CI, runner files, and docs first. Generic `cargo` commands are appropriate only when they are directly runnable in the repository and match the affected package, target, feature, or test surface.

Before reporting validation, account for Cargo test semantics: package selection, target selection, feature selection, doctests, examples, integration test targets, test filters, arguments after `--`, build parallelism, runtime test parallelism, package `rust-version`, MSRV, and applicable Cargo configuration. Use `patterns/cargo-test-semantics.md` for command interpretation.

## Command selection

Prefer, in order:

1. The repository's focused runner recipe for the affected surface.
2. A package-specific, test-name-filtered, target-filtered, benchmark-name-filtered, doctest, feature-specific, target-specific, fuzz reproduction, or expectation-review command.
3. Dependent package tests when a public API, shared fixture, generated output, feature, or workspace dependency change can affect downstream crates in the workspace.
4. The repository's standard full Rust test command when the change spans surfaces or no narrower command exists.
5. Static review only, with explicit disclosure, when commands are unavailable.

In workspaces, identify the affected package graph before selecting validation. Prefer package-scoped commands first, then dependent package tests when public APIs or shared fixtures changed. Avoid defaulting to full-workspace tests unless the change crosses package boundaries or no narrower command proves the contract.

## Common command shapes

Use only when evidenced or directly runnable in the repository:

- `cargo test -p <crate> <test_name>` for focused crate tests.
- `cargo test -p <crate> --lib <test_name>` for a library unit-test target.
- `cargo test -p <crate> --test <integration_test> <filter>` for a specific integration test target.
- `cargo test -p <crate> --example <example_name>` when an example target has tests or must compile.
- `cargo test -p <crate>` for one crate.
- `cargo test -p <crate> --no-default-features` for behavior or compilation with default features disabled.
- `cargo test -p <crate> --all-features` for all compatible feature flags.
- `cargo test -p <crate> --features <feature>` for a specific feature-gated contract.
- `cargo test -p <crate> --features <feature_a>,<feature_b>` for a specific compatible feature combination.
- `cargo check -p <crate> --target <target>` for target-specific compilation when the target cannot run locally.
- `cargo test -p <crate> --target <target>` when the target can run in the environment.
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

Do not confuse Cargo arguments with test harness arguments:

- Cargo/package/feature/target arguments go before `--`.
- Libtest/test-harness arguments go after `--`.
- `-j <n>` controls Cargo build parallelism.
- `-- --test-threads=<n>` controls libtest runtime test parallelism.

On Cargo 1.97, also inspect applicable configuration:

- `build.warnings = "deny"` can turn local-package lint warnings into command failures;
- `resolver.lockfile-path` changes the lockfile used by resolution and `--locked`;
- configuration may come from parent directories or the user's Cargo home, not only the repository.

Do not claim a command validates doctests, examples, all packages, all targets, all features, a particular lockfile, or binary behavior unless those surfaces were selected and the applicable configuration was verified.

## cargo-nextest guidance

When the repository uses cargo-nextest:

- Prefer `cargo nextest run -p <crate> <filter>` for normal unit and integration tests when that is the repository workflow.
- Inspect `.config/nextest.toml` before changing retries, slow-test settings, timeout policy, status levels, or per-test overrides.
- Do not assume nextest covers doctests. Run or recommend `cargo test --doc -p <crate>` separately when doctests are affected.
- Treat retries as flake classification or quarantine, not a fix. Report flaky-pass behavior rather than calling it fully stable.
- Preserve repository-standard profiles, such as default, CI, or slow-test profiles, rather than inventing new command policy.

## Feature, target, and MSRV validation

Treat feature flags, `cfg` gates, target triples, `no_std`, WASM, embedded support, Rust 1.97-sensitive doctests, and MSRV as part of the test contract when affected. Useful evidence includes manifests, package `rust-version`, CI matrices, `.cargo/config.toml`, `rust-toolchain.toml`, README support claims, package metadata, and existing target-specific tests.

Validation examples, only when applicable:

- `cargo test -p <crate> --no-default-features` for disabled-default support.
- `cargo test -p <crate> --all-features` when features are compatible.
- `cargo test -p <crate> --features <feature_a>,<feature_b>` for specific feature combinations.
- `cargo check -p <crate> --target <target>` for cross-target compilation.
- `cargo test -p <crate> --target <target>` when the target can run locally or in the configured environment.
- Repository-specific MSRV commands only when documented or present in CI.
- `cargo test --doc -p <crate> --target <target>` only when target-specific rustdoc configuration or doctest compilation is part of the contract and the target can be checked in the environment.

Use `cargo hack` only when the repository already uses it or the recommendation is clearly labeled as **Recommended**. Common recommended shapes include:

- `cargo hack check --each-feature --no-dev-deps`
- `cargo hack check --feature-powerset --no-dev-deps`
- `cargo hack check --feature-powerset --depth 2 --no-dev-deps`
- `cargo hack check --version-range <min>..=<max>`

Disclose mutually exclusive features, missing target toolchains, unavailable linkers, MSRV toolchain gaps, Rust 1.97-only APIs or configuration that were not MSRV-safe, or target tests that could be checked but not executed.

## Rust 1.97-specific validation

Use `patterns/rust-1-97-testing-baseline.md` when the patch or recommendation depends on Rust 1.97. Apply these validation rules narrowly:

- For assertion-only changes from `assert!(matches!(...))` to `assert_matches!`, run the smallest package/test-target command that exercises the changed tests, and disclose if the repository's Rust 1.96 minimum for that macro was reviewed but not executed.
- On the Rust 1.97 baseline, `assert_matches!` is available; do not apply it to a crate whose declared MSRV is older than Rust 1.96.
- For doctests using `assert_matches!`, run or recommend `cargo test --doc ...`; a normal nextest run is not sufficient.
- For cfg-specific `rustdocflags`, rustdoc `--emit` or `--remap-path-prefix`, WASM, `no_std`, embedded, or custom target claims, use the repository's documented command. Host tests do not prove those contracts.
- When `build.warnings = "deny"` is active, distinguish a lint-policy failure from a test failure.
- When `resolver.lockfile-path` is active, identify the configured `Cargo.lock` before claiming `--locked` validation.
- For Rust 1.97 compiler diagnostic or compatibility changes, update UI/diagnostic/symbol expectations only through the repository workflow and review the resulting diffs.

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

For `trybuild`, disclose whether missing or changed `.stderr` files were written under `wip/`, regenerated with `TRYBUILD=overwrite`, and reviewed through `git diff`.

## Coverage and mutation evidence

Use coverage or mutation-testing results only as supporting evidence. Do not introduce coverage or mutation tooling as a default patch. When the repository already uses such tools or the user asks for them:

- run the narrowest repository-standard coverage or mutation command;
- review reports for missed contracts, weak assertions, or surviving mutants rather than reporting only a percentage;
- disclose noise, unsupported targets, generated code, equivalent mutants, and unexecuted feature combinations.

## Validation wording

Use exact validation wording:

- `Validated with: <command>` only when the command ran successfully.
- `Attempted validation with: <command>` when the command ran but failed; include the relevant failure summary and whether the failure appears related to the change.
- `Reviewed only; not executed because: <reason>` for static review without execution.
- `Not validated; missing repo access / command unavailable / outside requested scope: <reason>` when validation was not possible or not attempted.
