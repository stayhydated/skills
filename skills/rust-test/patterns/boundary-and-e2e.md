# Boundary, integration, and end-to-end patterns

Use integration or end-to-end tests when the contract depends on multiple components working together through a public seam.

## Good fits

- public crate facades that coordinate multiple modules;
- CLI flows, file formats, config loading, exit codes, and user-visible stderr/stdout;
- protocol, database, filesystem, subprocess, network, or service boundaries;
- compatibility workflows that should behave like a downstream user;
- regression tests for bugs that happened at the seam between otherwise-correct units.

## Scope discipline

- Prefer the smallest public workflow that proves the contract.
- Keep broad e2e coverage intentionally small and stable; do not replace focused unit and integration tests with a giant slow suite.
- Make external resources hermetic when possible: temp directories, local subprocesses, local test servers, checked-in fixtures, or repository-standard containers.
- Assert observable behavior: returned values, files written, exit status, stdout/stderr after normalization, state transitions, or public API effects.
- Avoid assertions over internal call order, private helper calls, or incidental implementation choreography.

## CLI tests without extra dependencies

For binary integration tests, first consider Cargo's built-in binary path support before adding CLI helper crates.

- Integration tests can use `CARGO_BIN_EXE_<name>` to locate the compiled binary for a package binary target.
- For small CLI e2e tests, `std::process::Command` plus assertions over exit status, stdout, stderr, and filesystem effects may be enough.
- Add crates such as `assert_cmd`, `predicates`, snapshot helpers, or local command harnesses only when the repository already uses them, the user asks to standardize on them, or the addition is clearly labeled as **Recommended**.
- Normalize paths, line endings, environment-dependent output, and temporary directories before asserting CLI text.

## Mock boundary

Do not introduce a mock framework or mock-centric test pattern by default. Mock-heavy tests often verify that an implementation called a collaborator, not that the public behavior works.

Use a test double only when all of these hold:

1. the real boundary is slow, unavailable, unsafe, expensive, nondeterministic, or outside the repository's normal validation environment;
2. the repository already uses the style or the strategy is explicitly requested and labeled;
3. the double preserves the public contract closely enough for the test's purpose; and
4. at least one integration/e2e, contract, fixture, or compatibility test covers the real boundary where practical.

Prefer lightweight fakes, in-memory implementations, fixtures, or local servers over mocks that only verify method-call choreography.

## Handoff

When selecting integration/e2e over mocks, name the public seam being protected and the smaller tests that still cover pure logic. When a real dependency is not exercised, disclose the missing evidence and the remaining integration risk.
