# Property and fuzz test patterns

Use property tests and fuzz tests when the interesting behavior is defined over a broad input space rather than a small set of hand-picked examples.

## Property tests

Property tests are a good fit for:

- round trips such as parse/print/parse, serialize/deserialize, encode/decode, or normalize/idempotence;
- algebraic, ordering, validation, or compatibility invariants;
- generated inputs where edge cases are hard to enumerate by hand;
- state-machine or sequence behavior when the repository already uses a suitable property-testing framework.

Use the repository's existing property-testing framework, commonly `proptest` or `quickcheck`. Do not introduce one in a patch unless requested or already standardized.

Property-test discipline:

- Write the invariant first; do not only reimplement the production algorithm as the oracle.
- Bound generated sizes so tests stay reliable in normal `cargo test` or `nextest` runs.
- Generate valid and invalid cases intentionally instead of filtering most inputs away.
- Keep failure output useful by naming strategies and adding context.
- Preserve failing seeds or minimized cases when the framework does so, and add focused regression tests for important discoveries.

## Fuzz tests

Fuzz tests are a good fit for:

- parsers, deserializers, decoders, decompressors, and protocol handlers;
- unsafe code and FFI boundaries;
- code that accepts untrusted bytes or text;
- state machines with many edge-case transitions;
- denial-of-service risks such as pathological input size, nesting, or recursion.

Use the repository's existing fuzzer and layout, commonly `cargo-fuzz` under `fuzz/`. Do not introduce a fuzzer, nightly-only toolchain requirement, platform-specific setup, large corpus, or CI fuzz job unless requested or already part of the repository policy.

Fuzz-target discipline:

- Keep targets deterministic, hermetic, and free from uncontrolled network, clock, filesystem, or environment dependencies.
- Prefer exercising the real public parser/decoder/API seam rather than mocking collaborators.
- Treat panics, hangs, excessive allocation, and assertion failures as findings only when they violate the intended contract.
- Minimize crashing inputs and commit only small, non-secret corpus/regression files that protect meaningful behavior.
- Convert important minimized failures into ordinary regression tests when they are cheap and stable enough for normal CI.

## Validation

Use the repository's documented property/fuzz command when present. Common focused commands include:

- `cargo test -p <crate> <property_test_name>` for property tests included in the normal test suite;
- `cargo fuzz run <target>` for configured fuzz targets;
- `cargo fuzz run <target> <artifact_or_corpus_path>` to reproduce a known failing input when the repository uses cargo-fuzz.

Disclose time limits, unavailable tools, platform constraints, and whether corpus or minimized artifacts were reviewed.
