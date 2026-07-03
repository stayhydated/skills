# Async, concurrency, and time patterns

Use async, concurrency, and time-focused tests when the contract involves task scheduling, cancellation, timeouts, streams, channels, locks, backpressure, background workers, or graceful shutdown.

## Good fits

Async/concurrency tests are a good fit for:

- cancellation, timeout, retry, and graceful shutdown behavior;
- channel send/receive, close, backpressure, buffering, and ordering contracts;
- stream polling, termination, and error propagation;
- task lifecycle, spawned task joins, panic/error propagation, and cleanup;
- synchronization around shared state, locks, atomics, or worker pools;
- regressions where real scheduling exposed a bug but a deterministic seam can reproduce it.

## Poor fits

Prefer ordinary unit or integration tests for:

- pure logic that can be tested without an async runtime;
- exact wall-clock durations, scheduler fairness, or performance timing unless the repository has a calibrated benchmark or runtime test harness;
- tests that require uncontrolled network, external services, or arbitrary sleeps to pass;
- broad race-hunting that belongs in an existing loom, stress, fuzz, or sanitizer workflow.

## Discipline

- Follow the repository's existing async runtime and macros, such as `#[tokio::test]`, `async_std::test`, or local runtime helpers.
- Do not introduce a runtime, fake-time crate, loom, or stress harness unless the repository already uses it, the user requested it, or it is clearly labeled as **Recommended**.
- Avoid real sleeps as synchronization. Prefer explicit channels, barriers, notifications, local servers, fake clocks, paused time, or deterministic hooks.
- Use timeout wrappers only to prevent hangs or express timeout contracts; do not make fragile assertions about precise scheduling.
- Join spawned tasks and assert their results. Do not leave background tasks running past test end.
- Make shutdown and cancellation observable through returned values, join handles, emitted events, closed channels, or public state transitions.
- Keep tests hermetic: local fakes, temporary directories, local ports only when safely allocated, no uncontrolled network, and no dependency on host time.
- For concurrency correctness, prefer small deterministic tests first. Use loom or stress loops only when already configured, requested, or clearly recommended.
- When using fake or paused time, advance time explicitly and ensure all relevant tasks have reached the awaited state before asserting.
- Treat retries, serial execution, and timeout increases as flake triage tools, not fixes. Prefer root-cause repairs such as explicit synchronization, task joins, scoped resources, and deterministic ordering.

## Validation

Use the repository's documented async/concurrency command when present. Common focused commands include:

- `cargo test -p <crate> <async_test_name>` for an async test included in the normal suite;
- `cargo nextest run -p <crate> <filter>` when the repository uses nextest;
- `cargo test -p <crate> --test <integration_test> <filter>` for async integration tests;
- `cargo test -p <crate> --features <runtime_or_time_feature> <test_name>` when the behavior is feature-gated;
- `cargo test -p <crate> -- --test-threads=1` only when the repository or test contract requires serial execution.

Disclose whether timing-sensitive behavior was reviewed but not executed, and summarize any failure as `Attempted validation with: <command>` rather than calling it validated. For nondeterministic failures, use `patterns/flaky-tests.md` before adding retries, sleeps, broad serial execution, or longer timeouts.
