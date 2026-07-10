# Rust test checklist

## Evidence

- [ ] The evidence tier matched the request size: brief command advice, focused patch, cross-surface audit, or tool-adoption strategy.
- [ ] Relevant manifests, dev-dependencies, feature flags, feature matrices, bench targets, fuzz manifests, runner files, and CI were inspected.
- [ ] Cargo test semantics were considered: package selection, target selection, features, doctests, examples, test filters, libtest arguments, build parallelism, runtime test threads, warning policy, and selected lockfile.
- [ ] MSRV policy, package `rust-version`, `rust-toolchain.toml`, `.cargo/config.toml`, Cargo 1.97 `build.warnings` or `resolver.lockfile-path`, target triples, platform-specific `cfg`s, `no_std`/WASM/embedded support, Rust 1.97-sensitive doctest or target configuration, and feature-combination expectations were inspected when affected.
- [ ] Existing test, fixture, snapshot, benchmark, fuzz, doctest, async/concurrency, unsafe-code, and e2e layouts were preserved.
- [ ] New test, fuzzing, benchmark, snapshot, async-runtime, fake-time, Miri, sanitizer, loom, coverage, mutation-testing, cargo-hack, or command-runner dependencies were added only when evidenced, requested, or clearly labeled as recommendations.
- [ ] Public docs, README examples, CLI behavior, generated output, diagnostics, protocols, schemas, target/platform guarantees, performance claims, or compatibility files affected by the tests were inspected.

## Test fit

- [ ] Unit tests cover pure logic and narrow invariants.
- [ ] Integration tests cover public APIs, facade behavior, and cross-module seams.
- [ ] End-to-end tests cover the smallest public workflow that proves behavior across real boundaries.
- [ ] Doctests cover public documentation examples and API samples, not private implementation details, and use rustdoc mechanics such as hidden setup lines, `no_run`, and `compile_fail` intentionally.
- [ ] Snapshots are used only when reviewable diffs are clearer than ordinary assertions.
- [ ] Compile-fail/UI tests cover proc macros, diagnostics, type-level contracts, or compile-time failures when relevant, and `.stderr`/UI expectations were generated through the repository workflow and reviewed.
- [ ] Property tests cover invariants, round trips, or generated-input behavior when the repository already uses or requests that style.
- [ ] Fuzz tests cover untrusted-input, parser, deserializer, protocol, or unsafe-code surfaces when fuzzing is configured or requested.
- [ ] Async/concurrency tests cover cancellation, shutdown, timeouts, channels, streams, task joins, or synchronization contracts without relying on arbitrary sleeps.
- [ ] Unsafe-code tests pair public functional behavior with invariant-focused regressions, and Miri/sanitizer/loom/fuzz validation is used only when configured, requested, or clearly recommended.
- [ ] Feature, `cfg`, target, MSRV, `no_std`, WASM, embedded, or platform-specific support is validated when it is part of the contract, with mutually exclusive features handled explicitly.
- [ ] Criterion or benchmark tests cover performance-sensitive behavior without pretending to prove functional correctness.
- [ ] Coverage or mutation-testing suggestions are framed as supporting evidence, not a replacement for contract-focused tests.
- [ ] Fixtures are small, named by behavior, and not coupled across unrelated tests.
- [ ] Golden/generated outputs are produced from the source of truth, not hand-edited without reason.
- [ ] Generated Rust or proc-macro output is tested through public behavior first; snapshots of generated shape are used only when stable, normalized, and review-relevant.
- [ ] Structural assertions are preferred for typed errors, variants, spans, exit codes, and machine-readable fields; on the Rust 1.97 baseline, `assert_matches!` is used for single-pattern variant assertions when it improves failure output and the repository MSRV is Rust 1.96 or newer; exact text is asserted only when wording is part of the public contract.
- [ ] Mock-centric tests were not introduced as the default strategy; boundary behavior is covered through public seams, integration/e2e tests, fixtures, or local fakes only when justified by repository evidence.
- [ ] The test skill did not drift into broad Rust implementation guidance covered by `rust-best-practices` unless that code pattern was itself the tested contract.

## Determinism and hygiene

- [ ] Paths, timestamps, random IDs, map order, environment-specific data, hostnames, process IDs, platform-specific separators, scheduling artifacts, runtime timing, and Rust 1.97 symbol-mangling or path-remapping differences are normalized or intentionally reviewed.
- [ ] Snapshots, golden files, corpora, benchmark fixtures, generated outputs, and minimized regressions avoid credentials, tokens, private URLs, personal data, secrets, and machine-local paths.
- [ ] Property strategies and fuzz targets are bounded, reproducible when failing, and free from uncontrolled network/time/environment dependencies.
- [ ] Async/concurrency tests use deterministic synchronization, fake or paused time, local fakes, and joined tasks where practical.
- [ ] Flaky tests were triaged for root causes such as sleeps, shared temp resources, ports, global state, unjoined tasks, nondeterministic order, or environment dependence before adding retries or serial execution.
- [ ] Benchmarks isolate setup from measurement and disclose noisy or unrepresentative environments.
- [ ] Coverage or mutation results, when present, are treated as supporting evidence rather than proof of test quality.
- [ ] Expected-output, corpus, diagnostic, snapshot, generated-output, or benchmark-reference updates were reviewed intentionally.

## Validation

- [ ] The narrowest proving command was run when possible, and the handoff does not overclaim surfaces that command did not exercise.
- [ ] Workspace package graph, dependent crate impact, feature combinations, MSRV, Cargo warning policy, selected lockfile, and target/platform commands were considered when relevant.
- [ ] When nextest is used, doctests were validated separately when affected.
- [ ] Rust 1.97-specific Cargo/rustdoc/target behavior, such as `build.warnings`, `resolver.lockfile-path`, rustdoc output/path flags, v0 symbol mangling, or WASM target linking, was validated only with commands that actually exercise that surface.
- [ ] When `trybuild` is used, missing `wip/` outputs, `TRYBUILD=overwrite`, and changed `.stderr` files were handled intentionally.
- [ ] Snapshot, golden, diagnostic, corpus, generated-output, or benchmark diffs/results were reviewed or explicitly marked not reviewed.
- [ ] The handoff distinguishes `Validated with`, `Attempted validation with`, `Reviewed only`, and `Not validated`.
- [ ] Failed validation is summarized with whether the failure appears related to the change.
- [ ] Remaining risks and missing evidence are disclosed.
