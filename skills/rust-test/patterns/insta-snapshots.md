# insta snapshot patterns

Use `insta` when a reviewable expected-output file communicates the contract better than many narrow assertions. Do not add it silently to repositories that do not use it unless the user asked for that standardization or the output is an explicitly labeled recommendation.

## Good fits

`insta` is usually a good fit for:

- generated Rust code, generated config, or generated documentation fragments when the generated shape is stable and review-relevant;
- parser, formatter, serializer, deserializer, and schema output;
- CLI help, CLI output, diagnostics, and structured logs after normalization;
- large structs, trees, ASTs, token streams, or nested error reports;
- regression tests where the whole shape matters and diffs are useful.

## Poor fits

Prefer normal assertions for:

- simple scalars, booleans, counters, or small enums;
- behavior where only one field or invariant matters;
- nondeterministic output that cannot be normalized;
- secrets, credentials, private URLs, personal data, machine-local paths, unstable timestamps, random IDs, or host-specific data;
- performance numbers, fuzz corpora, or generated data that changes without a stable reviewable contract;
- broad snapshots that change for unrelated reasons.

## Snapshot hygiene

- Keep snapshots focused on the public or regression-relevant contract.
- Normalize or redact paths, timestamps, random IDs, ordering, hostnames, process IDs, and platform-specific separators.
- Sort maps, sets, and generated inventories before snapshotting when order is not part of the contract.
- For generated Rust or proc-macro output, test public behavior first; snapshot expanded or generated code only when normalized, stable, and review-relevant.
- Do not snapshot unstable compiler-expanded output, incidental hygiene, nondeterministic spans, or machine-local paths unless the repository already normalizes and reviews that output.
- Prefer one clear snapshot per contract over a giant unrelated dump.
- Name tests and snapshots after the behavior being protected.
- Never accept snapshot changes blindly; inspect the diff and tie the update to the intentional behavior change.

## Workflow

Follow the repository's existing commands when present. Common `insta` workflows include running the targeted tests, then reviewing changed snapshots with `cargo insta review`, or using `cargo insta test` / `cargo insta test --review` when the repository uses `cargo-insta`.

When a snapshot changes, the handoff should say whether the snapshot diff was reviewed. Use wording such as:

- `Validated with: cargo test -p my_crate parser_snapshot`
- `Reviewed snapshot diff: tests/snapshots/parser__basic.snap`
- `Not reviewed; snapshot tool unavailable: cargo-insta is not installed in this environment`
