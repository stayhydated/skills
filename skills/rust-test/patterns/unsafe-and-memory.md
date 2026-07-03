# Unsafe and memory validation patterns

Use unsafe and memory validation guidance when tests touch `unsafe`, FFI, raw pointers, custom allocators, atomics, aliasing assumptions, panic/drop safety, interior mutability, or invariants that Rust's type system cannot fully enforce.

## Good fits

Unsafe and memory-focused validation is a good fit for:

- minimized regressions for soundness, panic-safety, drop-order, or ownership bugs;
- safe public APIs that encapsulate unsafe internals;
- FFI boundary behavior, nullability, buffer length, encoding, ownership-transfer, and error-code contracts;
- raw pointer, slice, alignment, lifetime, and aliasing invariants;
- custom allocators, arenas, reference-counting, pinning, or self-referential structures;
- atomic ordering and concurrent access contracts;
- parser, decoder, decompressor, or serializer fast paths that use unsafe for performance.

## Discipline

- Test the safe public API that encapsulates unsafe internals when possible; use private helpers only when local conventions already expose test-only invariant checks.
- Pair ordinary functional tests with invariant-focused regressions. Passing tests do not prove soundness.
- Keep UB or soundness regressions minimized, documented, non-secret, and reviewable.
- Prefer assertions about ownership, error handling, drop behavior, panic safety, bounds, and public effects over implementation call choreography.
- Use Miri, sanitizers, loom, stress tests, or fuzzing only when configured, available, requested, or clearly labeled as **Recommended**.
- For FFI, avoid relying on host-global state or external libraries unless the repository's test environment already provides them.
- For atomics or concurrent unsafe code, prefer deterministic synchronization and small state-space tests before broad stress loops.
- Do not present a clean test, Miri, sanitizer, or fuzz run as a complete proof of memory safety; report it as evidence for the exercised contracts.

## Practical Miri constraints

When Miri is configured, available, or requested:

- Use `cargo miri test ...` or `cargo +nightly miri test ...` according to repository policy and toolchain setup.
- Treat Miri as evidence for the executed paths, not a proof of soundness.
- Expect unsupported operations, target differences, FFI limitations, host isolation, and performance overhead.
- For nondeterministic or scheduler-sensitive unsafe behavior, consider multiple seeds or repository-standard Miri flags only when already used or explicitly requested.
- Keep ordinary regression tests for public behavior even when a Miri run passes.

## Validation

Use the repository's documented unsafe-code validation when present. Potential commands, only when evidenced or requested, include:

- `cargo test -p <crate> <test_name>` for stable regression tests;
- `cargo miri test -p <crate>` or `cargo +nightly miri test -p <crate>` when Miri is configured or requested;
- repository-standard Miri flags, such as multiple-seed execution, only when already documented or requested;
- repository-specific sanitizer commands, `RUSTFLAGS`, or toolchain scripts used by CI;
- `cargo test -p <crate> --features <unsafe_or_ffi_feature>` when unsafe or FFI behavior is feature-gated;
- `cargo fuzz run <target> <artifact_or_corpus_path>` to reproduce a minimized unsafe-surface crash when the repository uses cargo-fuzz;
- loom-specific test commands only when loom is already part of the repository workflow.

When reporting validation, distinguish ordinary functional coverage from memory-model evidence:

- `Validated with: cargo test -p my_crate pointer_regression` means the regression test passed.
- `Validated with: cargo miri test -p my_crate` means the configured Miri run passed for the exercised tests.
- `Attempted validation with: <command>` means the command ran but failed; summarize whether the failure appears related to the unsafe-code change.
- `Reviewed only; not executed because: <reason>` applies when Miri, sanitizer, loom, fuzzing, or target-specific tooling was unavailable.
