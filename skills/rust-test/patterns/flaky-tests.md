# Flaky test triage

Use this pattern when tests fail nondeterministically, pass only on retry, hang intermittently, differ between local and CI, depend on timing, or become unstable under parallel execution.

## Principle

Prefer fixing the root cause over adding retries, sleeps, or broad serialization. Retries can classify or quarantine known flakiness, but they do not validate deterministic behavior.

## Common root causes

Look for:

- arbitrary sleeps instead of synchronization;
- unjoined tasks, leaked background workers, or dropped handles that hide panics;
- shared temp paths, ports, files, environment variables, process-global state, current directories, or logger/global runtime initialization;
- wall-clock, timezone, locale, hostname, process ID, random ID, or scheduler dependence;
- hash map/set order assumptions and nondeterministic iteration order;
- uncontrolled network, external services, filesystem races, subprocess races, or resource limits;
- tests that pass only when run serially because they mutate shared state;
- feature, target, OS, architecture, or MSRV assumptions not represented in the selected command.

## Triage workflow

1. Identify the smallest failing test, package, feature set, and target.
2. Re-run the narrow command enough to classify whether the failure is deterministic, flaky, order-dependent, or environment-dependent.
3. Inspect whether the test relies on sleeps, real time, global state, temp paths, local ports, randomized order, or unjoined tasks.
4. Replace timing assumptions with explicit synchronization, fake or paused time, barriers, notifications, local fakes, scoped temp directories, allocated ports, or deterministic ordering.
5. Join spawned tasks and assert their results.
6. Preserve a focused regression that fails deterministically for the root cause where practical.
7. If quarantine or retries are required by repository policy, report the test as flaky and disclose the policy rather than treating a retry pass as proof.

## Nextest-specific handling

When the repository uses cargo-nextest:

- inspect `.config/nextest.toml` before changing retries, timeouts, slow-test settings, or per-test overrides;
- distinguish ordinary pass/fail results from flaky-pass results;
- do not use nextest retries as the only fix for timing or concurrency bugs;
- remember that doctest validation remains separate from normal nextest runs.

## Handoff

Report flake work with concrete wording:

- `Attempted validation with: cargo test -p my_crate flaky_case -- --test-threads=1; failed intermittently after 3 runs, likely due to shared temp path reuse.`
- `Validated with: cargo test -p my_crate shutdown_is_joined; repeated 10 times locally after replacing sleep-based synchronization with a channel notification.`
- `Reviewed only; not executed because: CI-only flake requires target <target> not available in this environment.`
