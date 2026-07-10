# Feature, target, and MSRV matrix patterns

Use this pattern when behavior, compilation, public API shape, dependencies, doctests, examples, unsafe paths, or platform support changes under feature flags, target triples, `cfg` gates, `no_std`, WASM, embedded builds, or MSRV policy.

## Evidence to inspect

Before recommending matrix validation, inspect the relevant subset of:

- workspace and package `Cargo.toml` feature definitions, optional dependencies, and mutually exclusive feature notes;
- README support claims, crate docs, package metadata, and release policy;
- `rust-toolchain.toml`, package `rust-version`, CI MSRV jobs, and documented minimum supported Rust version;
- `.cargo/config.toml`, Cargo 1.97 `build.warnings` and `resolver.lockfile-path`, target-specific linker/config settings, cfg-specific rustdoc flags, and platform-specific `cfg`s;
- existing CI matrices, `cargo hack` usage, target check jobs, `no_std` checks, WASM checks, and embedded recipes.

## Feature validation discipline

- Test default features when they are the normal user contract.
- Test `--no-default-features` when disabled-default support is claimed or changed.
- Test `--features <feature>` for each affected feature-gated contract.
- Test `--all-features` only when features are compatible.
- For mutually exclusive features, use documented combinations rather than forcing `--all-features`.
- Prefer targeted feature combinations over exhaustive powersets when features are numerous or expensive.
- Include doctests or examples separately when public docs or examples are feature-gated.
- On the Rust 1.97 baseline, `assert_matches!` is available, but do not introduce it when the tested crate still promises an MSRV older than Rust 1.96.
- Do not introduce Rust 1.97-only integer APIs or Cargo configuration when the tested crate or tooling contract promises an older compiler.

## Target and MSRV validation discipline

- Use `cargo check --target <target>` when the target is compile-only in the current environment.
- Use `cargo test --target <target>` only when the target can execute locally or in the configured runner.
- For `no_std`, WASM, or embedded contracts, prefer the repository's documented check recipe over invented commands.
- For WASM contracts on the Rust 1.97 baseline, treat undefined-symbol link failures as target-validation evidence; do not assume a host test run proves the WASM boundary.
- For MSRV, use the repository's documented toolchain or CI job. Do not claim MSRV validation unless the command actually ran under the minimum supported toolchain.
- If `resolver.lockfile-path` is configured, identify the selected `Cargo.lock` for each toolchain lane; the setting itself requires Cargo 1.97 or newer.
- If `build.warnings = "deny"` is configured, report warning-policy failures separately from behavioral test failures and verify whether older toolchain lanes honor the setting.

## Rust 1.97-specific matrix notes

Use `patterns/rust-1-97-testing-baseline.md` when a test change depends on Rust 1.97. Keep matrix guidance narrow:

- `assert_matches!` and `debug_assert_matches!` are available on the Rust 1.97 baseline but require Rust 1.96 or newer.
- Rust 1.97 integer methods should trigger test work only when public API, serialization, fixtures, or compatibility claims expose those operations; otherwise leave implementation style to `rust-best-practices`.
- Cargo `build.warnings` and `resolver.lockfile-path` can make otherwise identical matrix commands use different failure policy or dependency state.
- cfg-specific `rustdocflags`, rustdoc `--emit`, and `--remap-path-prefix` can affect doctest compilation or generated documentation; inspect `.cargo/config.toml` and runner scripts before choosing or interpreting doctest validation.
- v0 symbol mangling is the Rust 1.97 default, so symbol-based target checks, backtrace goldens, profilers, and post-processing tools may need explicit compatibility review.
- Target-specific compiler or linker behavior should be validated with the repository's target command, not inferred from ordinary host tests.

## cargo-hack guidance

Use `cargo hack` only when the repository already uses it or when clearly labeled as **Recommended** in a strategy or audit. It is useful for feature-combination and version-range validation, but it should not become implicit repository policy.

Common recommended shapes:

- `cargo hack check --each-feature --no-dev-deps`
- `cargo hack check --feature-powerset --no-dev-deps`
- `cargo hack check --feature-powerset --depth 2 --no-dev-deps`
- `cargo hack check --version-range <min>..=<max>`

Avoid exhaustive feature powersets when they are too slow or semantically invalid. Prefer `--depth`, documented groups, or CI-evidenced combinations.

## Handoff

State which axis was validated and which was not:

- `Validated with: cargo test -p my_crate --no-default-features`
- `Validated with: cargo check -p my_crate --target wasm32-unknown-unknown`
- `Reviewed only; not executed because: MSRV toolchain is not installed in this environment.`
- `Not validated; outside requested scope: full feature powerset would require introducing cargo-hack, which the repository does not currently use.`
