# Cargo test semantics

Use this pattern before selecting validation commands or explaining what a Rust test command proves. Cargo's test model affects which contracts are exercised, which targets are compiled, and which arguments are interpreted by Cargo versus the test harness.

## Default behavior to account for

- `cargo test` builds and runs selected test targets for selected packages. In common library crates this includes unit tests, integration tests, and doctests unless target selection changes that behavior.
- Package selection matters in workspaces. Prefer `-p <crate>` for focused validation, and expand to dependents only when a public API, shared fixture, feature, or workspace dependency change can affect them.
- Feature selection changes the tested contract. `--features`, `--all-features`, and `--no-default-features` can compile different APIs, dependencies, `cfg` branches, doctests, and examples.
- MSRV and package `rust-version` affect which test idioms are allowed. A test-only cleanup to Rust 1.96 `assert_matches!` is not valid for crates that still promise older compilers.
- Target selection changes both compilation and executability. Cross-target validation may be limited to `cargo check` when the target cannot run in the current environment.
- Arguments before `--` are Cargo arguments. Arguments after `--` are test harness arguments, such as a libtest filter option or `--test-threads=1`.
- `-j <n>` controls Cargo build parallelism. `-- --test-threads=<n>` controls libtest runtime test parallelism.

## Test target selection

Common focused target selections:

- `cargo test -p <crate> <filter>` runs matching unit and integration tests for one package and may also build other relevant test targets.
- `cargo test -p <crate> --lib <filter>` narrows to the library test target.
- `cargo test -p <crate> --test <integration_test> <filter>` narrows to one integration test target.
- `cargo test --doc -p <crate>` runs the package's library doctests only.
- `cargo test -p <crate> --example <example_name>` runs tests for a selected example target when applicable.
- `cargo test -p <crate> --bins` or `--bin <name>` is relevant when binary targets contain unit tests or when binary-specific compilation matters.

Do not claim that a selected command proves doctests, examples, benches, target-specific builds, or all feature combinations unless the command actually selects them.

## Integration tests and binaries

For binary crates or crates with CLI targets, first consider Cargo's built-in binary path support before adding CLI test helper crates:

- Integration tests can use `CARGO_BIN_EXE_<name>` to locate the compiled binary for a package binary target.
- This is often enough for small CLI e2e tests that assert exit status, stdout, stderr, and filesystem effects.
- Add helper crates such as `assert_cmd`, `predicates`, or snapshot tools only when the repository already uses them, the user asks for standardization, or the addition is clearly labeled as **Recommended**.

## Working directories and environment

- Test executables normally run with the package root as the working directory, but doctest compilation and execution have distinct rustdoc behavior.
- Avoid relying on incidental current-directory behavior when a test can use explicit fixture paths or `CARGO_MANIFEST_DIR`.
- Keep test environment assumptions visible: environment variables, locale, path separators, target OS, feature flags, and current time can all affect assertions and snapshots.
- Inspect `.cargo/config.toml` when doctests or target validation are affected; cfg-specific `rustdocflags` can make a doctest command cover a different rustdoc configuration than a plain host run.

## Handoff discipline

When reporting validation, state exactly what the command covered. Examples:

- `Validated with: cargo test -p parser parse_round_trip` covers matching normal tests for that package, not all feature combinations.
- `Validated with: cargo test --doc -p parser` covers doctests under the selected package, target, feature, and rustdoc configuration, not integration tests.
- `Validated with: cargo check -p parser --target wasm32-unknown-unknown` covers target compilation, not runtime behavior on that target.
