# Fixtures, golden files, and generated output

Use fixtures and golden files when the input/output relationship is clearer as files than inline test data.

## Fixture rules

- Keep fixtures small enough to review, unless the fixture represents a real public compatibility case.
- Name fixtures after the behavior or public contract they protect.
- Avoid fixture reuse that couples unrelated tests.
- Put generated or golden outputs next to the harness when the repository already uses that layout; otherwise follow local conventions.
- Normalize nondeterministic data before writing golden output.
- Before adding or updating fixtures, snapshots, goldens, corpora, benchmark inputs, or minimized regressions, check that they do not contain credentials, tokens, private URLs, personal data, secrets, or machine-local paths.
- When a property test or fuzz target finds a bug, reduce the failing case and add the minimized input as a focused regression fixture when practical.

## Generated output

- Change the source generator, schema, inventory, metadata, or template before changing checked-in generated output.
- Regenerate outputs only with evidenced repository commands.
- Review generated diffs for unexpected unrelated churn.
- Add or update focused tests for the generator behavior that caused the output change.
- For generated Rust or proc-macro output, test the public behavior first. Snapshot expanded or generated code only when the generated shape is itself review-relevant, stable, and normalized.
- Do not snapshot compiler-expanded output that includes unstable formatting, incidental hygiene, nondeterministic spans, or machine-local paths unless the repository already normalizes and reviews that output.
- On Rust 1.97, account for v0 symbol mangling, linker warnings, and rustdoc path remapping before accepting changes to backtrace, symbol, stderr, or documentation goldens.

## Assertion style

Prefer the clearest assertion for the contract:

- exact equality for small deterministic values;
- structural assertions for typed data where only selected fields matter;
- `assert_matches!` on the Rust 1.97 baseline for single-pattern variants or typed errors where mismatch debug output matters, provided the repository MSRV is Rust 1.96 or newer;
- structural error assertions for multi-field typed errors, error kinds, spans, exit codes, and machine-readable fields;
- snapshots for large or nested deterministic output;
- property tests when invariants matter across many generated cases and the repository already uses or requests that style;
- fuzz tests when robustness over untrusted or complex inputs is the risk and fuzzing is configured or requested;
- end-to-end fixture tests for public workflows and boundary behavior.

Assert exact user-facing text only when wording is part of the public contract, such as CLI output, diagnostics, snapshots, documented messages, or compatibility formats.

Avoid test suites made mostly of `assert!(text.contains(...))` when the output has meaningful structure or a snapshot would make intent clearer. Avoid mock-only assertions for generated output or public workflows; assert the resulting artifact or observable behavior instead.
