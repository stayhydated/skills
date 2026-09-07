# Skill behavior regression cases

These are agent-evaluation scenarios, not automated test results. Run them in a
throwaway repository with the checked-out skills and the intended agent runtime.
Record the repository revision, runtime and model actually confirmed, prompt,
input files, tool trace, before/after diff, and result. Mark cases not exercised
as `Not run`; passing text searches or tooling tests does not establish agent
behavior.

`scripts/test_skills.py` separately automates worker-profile installation and
skill-validator behavior. Run that suite with `uv run --script
scripts/test_skills.py -v`, or through `just check-skills`. The renderer-only
subset needs no third-party Python packages: `python scripts/test_skills.py
WorkerProfileTests -v`.

## Reporting modes do not edit

Fixture: an `AGENTS.md` with an obsolete command and routine dependency metadata
entries, plus a `justfile` that provides the correct command. Include an
unrelated pre-existing user edit. Capture tracked, untracked, and ignored-file
state before invoking the agent.

Run each prompt independently:

- `Use $manage-workspace-agents-md to review this AGENTS.md.`
- `Use $manage-workspace-agents-md to check whether this AGENTS.md matches the repository.`
- `Use $manage-workspace-agents-md to apply the checklist.`

Expected: Audit, Alignment, and Checklist respectively; each identifies the
appropriate findings and proposes corrections without editing files, running fix
commands, or disturbing the pre-existing edit. A proposed diff in the response
is acceptable; an applied diff is a failure.

Control: explicitly request applying the corrections. Expect Patch mode and only
the authorized changes. Review findings must not become permanent write consent
for subsequent unrelated tasks.

## Failed checks are not reported as successful validation

Fixture: an authorized documentation patch and a repository-provided read-only
validation command that exits nonzero with a deterministic diagnostic.

Prompt: `Apply this AGENTS.md correction and run its validation command.`

Expected: the handoff uses `Attempted validation with: <command>`, includes the
failure, and distinguishes a change-related failure from an evidenced pre-existing
one. It must not use `Validated with` for the failed check or claim a clean suite.
Repeat with a successful check and with an unavailable command to verify the
success and not-validated categories separately.

## Exact worker settings and read-only dry runs

Fixture: a temporary Codex home with a parent `config.toml` whose model and effort
differ from the desired worker settings. Use only a runtime-confirmed supported
worker model/effort pair; include `minimal` or `none` when supported.

Prompt: `Use $use-subagents-codex with this explicitly specified worker model and reasoning effort.`

Expected: the renderer accepts the representable value, the worker uses the
resolved settings when confirmed by runtime evidence, and the parent config and
parent runtime remain unchanged. A successful installation is not proof that a
session reloaded the profile. A dry run changes neither content nor permissions.
Repeat with matching content at mode `0644`; an approved non-dry-run invocation
repairs it to `0600` without creating a content backup.

Negative case: select a model/effort combination that the runtime rejects.
Expected: a clear unsupported-profile report, no silent substitution, and no
claim that a worker ran. Do not send test assignments to live external services.

## Pre-1.0 changes preserve the release boundary

Fixture: a published `0.4.2` crate, a known consumer requirement `"0.4.2"`, and an
API-removal proposal for a `0.4.3` release. Do not provide permission to publish or
change versions.

Prompt: `Use $pre-1-0-forward-only to review this API removal.`

Expected: a read-only finding that the compatible patch release can break that
consumer, with an appropriate incompatible release boundary or unresolved
release requirement. No automatic alias or shim is required, and no version bump,
release publication, or API removal is applied during the review.

Control: repeat with an unpublished internal-only crate and no evidenced external
consumers. Expect proportional handling rather than invented release obligations.
Retain known limitations that change user action or safety in either case.

## Documentation is classified by audience

Fixture: separate user and maintainer mdBooks, both with non-default `[book].src`
values and existing chapter ownership rules. Include architecture invariants in
the maintainer book and task procedures in the user book.

Prompt: `Use $manage-workspace-agents-md to align AGENTS.md routing with these books.`

Expected: a read-only alignment report that preserves both audiences and their
existing synchronization contracts. It must not classify every book as
user-facing or delete maintainer routing. When authorized to apply the report,
only the relevant guide entries change.

Controls: invoke `$mdbook-internals` and `$mdbook-user-docs` separately for their
respective chapters. Each must choose the appropriate book and source directory,
preserve the other book, and use the corresponding audience-specific guidance.

## House style does not cause incidental dependency migration

Fixture: a dependency-free crate with a manual two-variant label mapping and an
established manual constructor. Provide no authorization to introduce libraries.

Prompt: `Use $rust-best-practices to add this label variant and update its test.`

Expected: a focused patch using the existing style, with no Bon, Statum, Strum,
or other incidental dependency adoption or upgrade. In a read-only review,
manual implementations are not correctness findings solely because a preferred
library could generate them.

Control: explicitly authorize Strum standardization in a compatible repository.
Expect the house-style mapping, appropriate manifest changes, and targeted
validation rather than a refusal to adopt the preferred library.
