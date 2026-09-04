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

## Mock-free boundary

Refuse a mock-object test: a test that programs expectations for calls to a collaborator and passes or fails by verifying those calls, arguments, counts, or ordering. Do not introduce a mocking framework, extend mock expectations already present, or use a spy to make implementation choreography the assertion.

When asked to write a mock test:

1. explain briefly that this skill does not create interaction-verification mocks because they are coupled to implementation details;
2. identify the observable contract the requested mock was meant to protect;
3. implement a state-, output-, artifact-, or public-protocol-based test through the smallest practical seam; and
4. if no such seam is available within scope, report the missing evidence and the production seam needed instead of generating the mock.

Do not treat all test doubles as mocks. When the real boundary is slow, unavailable, unsafe, expensive, nondeterministic, or outside the normal validation environment, use the narrowest substitute that does not verify internal calls:

- a stub that supplies a deterministic response;
- a lightweight fake or in-memory implementation whose resulting state can be queried through its public contract;
- a checked-in fixture;
- a local server or subprocess that exposes the real protocol; or
- a repository-standard hermetic service or container.

Prefer the real implementation when it is fast and deterministic. Where practical, pair a substitute with an integration/e2e, contract, fixture, or compatibility test against the real boundary. If the interaction itself is a public protocol contract, capture and assert the semantic protocol result or transcript at that boundary; do not verify private method calls used to produce it.

When editing tests that already use mocks, do not add expectations. Replace mocks touched by the requested change with contract-focused coverage where scope permits, but do not rewrite unrelated tests without authorization.

This policy follows the distinction between mocks and other test doubles in Martin Fowler's [Mocks Aren't Stubs](https://martinfowler.com/articles/mocksArentStubs.html) and the public-API and state-testing guidance in Google's [Software Engineering at Google: Testing Overview](https://abseil.io/resources/swe-book/html/ch12.html#test_via_public_apis).

## Handoff

When selecting a mock-free alternative, name the public seam being protected and the smaller tests that still cover pure logic. When a real dependency is not exercised, disclose the missing evidence and the remaining integration risk.
