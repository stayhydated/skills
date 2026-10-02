# Feature, target, and MSRV matrix patterns

Use this pattern when behavior, compilation, public API shape, dependencies, doctests, examples, unsafe paths, or platform support changes under feature flags, target triples, `cfg` gates, `no_std`, WASM, embedded builds, or MSRV policy.

## Evidence to inspect

Before recommending matrix validation, inspect the relevant subset of:

- workspace and package `Cargo.toml` feature definitions, optional dependencies, and mutually exclusive feature notes;
- README support claims, crate docs, package metadata, and release policy;
- `rust-toolchain.toml`, package `rust-version`, CI MSRV jobs, and documented minimum supported Rust version;
- `.cargo/config.toml`, Cargo 1.99 inherited `default-features`, profile selection, CI incremental defaults, `build.warnings` and `resolver.lockfile-path`, target-specific linker/config settings, cfg-specific rustdoc flags, and platform-specific `cfg`s;
- existing CI matrices, `cargo hack` usage, target check jobs, `no_std` checks, WASM checks, and embedded recipes.

## Feature validation discipline

- Test default features when they are the normal user contract.
- Test `--no-default-features` when disabled-default support is claimed or changed.
- Test `--features <feature>` for each affected feature-gated contract.
- Test `--all-features` only when features are compatible.
- For mutually exclusive features, use documented combinations rather than forcing `--all-features`.
- Prefer targeted feature combinations over exhaustive powersets when features are numerous or expensive.
- Include doctests or examples separately when public docs or examples are feature-gated.
- `assert_matches!` is available on the Rust 1.99 baseline, but do not introduce it or another current API when an explicitly declared lower-MSRV lane cannot compile it.
- Treat owned lossy UTF-8 decoding, deque tail retention, boxed-array iteration, `substr_range`, `subslice_range`, `strip_circumfix`, `NumBuffer`/`format_into`, algebraic floats, endian-specific UTF-16 constructors, non-zero radix parsing, and atomic view methods as matrix-sensitive only when the affected feature or target exposes them.

## Target and MSRV validation discipline

- Use `cargo check --target <target>` when the target is compile-only in the current environment.
- Use `cargo test --target <target>`, including `--doc`, only when the target can execute locally or in a working configured runner. Follow `patterns/doctests-and-examples.md` when only snippet compilation can be checked.
- For `no_std`, WASM, or embedded contracts, prefer the repository's documented check recipe over invented commands.
- For WASM contracts on the Rust 1.99 baseline, treat linker and ABI failures as target-validation evidence; do not assume a host test run proves the WASM boundary.
- For MSRV, use the repository's documented toolchain or CI job. Do not claim MSRV validation unless the command actually ran under the minimum supported toolchain.
- If `resolver.lockfile-path` is configured, identify the selected `Cargo.lock` for each toolchain lane and verify that every claimed toolchain lane supports the active configuration.
- If `build.warnings = "deny"` is configured, report warning-policy failures separately from behavioral test failures and verify whether older toolchain lanes honor the setting.

## Rust 1.99-specific matrix notes

Use `patterns/rust-1-99-testing-baseline.md` when a test change depends on Rust
1.99. Keep matrix guidance narrow:

- Cargo 1.99 permits edition-2024 members to override inherited
  `default-features`, including disabling workspace defaults. Test affected
  members in isolation and relevant workspace combinations: features enabled by
  other dependency edges are still unified. A root-package
  `--no-default-features` run does not by itself prove dependency defaults are off.
- Validate inherited overrides on lower-Cargo lanes separately; do not silently
  raise the repository's MSRV to match the skill baseline.
- Record explicit profile selection and CI incremental settings when comparing
  build behavior. The new `debug` profile currently inherits `dev`; ordinary
  tests still use `test`.
- Cargo `build.warnings` and `resolver.lockfile-path` can make otherwise identical
  commands use different failure policy or dependency state. These settings
  stabilized in Cargo 1.97, not 1.99.
- `str::substr_range` and `[T]::subslice_range` need provenance-focused tests;
  target-independent value-search tests are the wrong oracle.
- `Atomic*` view APIs can depend on target atomic support and primitive alignment;
  include the relevant `cfg` and compile target.
- Algebraic floating-point tests must tolerate optimization-dependent results and
  may need release-mode or benchmark evidence where vectorization is the claim.
- Endian-specific UTF-16 decoding is portable, but fixtures must state byte order
  explicitly.
- C-variadic ABI behavior, filesystem timestamp resolution, symlink handling,
  Emscripten exception handling, Solaris file locking, Thumb support, and other
  target claims require the repository's target job rather than host inference.
- `riscv64-unknown-linux-musl` is Tier 2 with host tools on 1.99; this does not
  automatically add it to the repository's supported target matrix.
- `doc(cfg(...))` no longer filters doctests. Validate actual feature and target
  gates; cfg-specific `rustdocflags`, `--emit`, and `--remap-path-prefix` can also
  affect doctest compilation or generated documentation.
- v0 symbols, version-specific escaping, and rustfmt module discovery can change
  expectations without changing runtime behavior; normalize or review them
  intentionally.

See the [Rust 1.99 release notes](https://doc.rust-lang.org/releases.html#version-1990-2026-10-01)
and [workspace dependency inheritance](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace).

## cargo-hack guidance

Use `cargo hack` only when the repository already uses it or when clearly labeled as **Recommended** in a strategy or audit. It is useful for feature-combination and version-range validation, but it should not become implicit repository policy.

Common recommended shapes:

- `cargo hack --each-feature --no-dev-deps check`
- `cargo hack --feature-powerset --no-dev-deps check`
- `cargo hack --feature-powerset --depth 2 --no-dev-deps check`
- `cargo hack --version-range <min>..=<max> check`

Avoid exhaustive feature powersets when they are too slow or semantically invalid. Prefer `--depth`, documented groups, or CI-evidenced combinations.

## Handoff

State which axis was validated and which was not:

- `Validated with: cargo test -p my_crate --no-default-features`
- `Validated with: cargo check -p my_crate --target wasm32-unknown-unknown`
- `Reviewed only; not executed because: MSRV toolchain is not installed in this environment.`
- `Not validated; outside requested scope: full feature powerset requires adopting cargo-hack.`
