# Standards inference patterns

Use this file to capture explicit maintainer requirements and infer repository standards for `AGENTS.md` guidance. It is not a recommendation list and must not be copied wholesale into generated output.

## What counts as a standard

A repository standard is an explicit working agreement or a repeated or canonical local practice that agents should follow to work correctly. Good standards are supported by one or more of these evidence types:

- explicit maintainer instructions, including requirements being established in `AGENTS.md` for the first time;
- CI jobs, runner recipes, manifests, lockfiles, and documented setup commands;
- repeated implementation, error-handling, naming, file organization, fixture, generator, or snapshot practices;
- existing root or nested `AGENTS.md`, contributor docs, or README instructions;
- public entry points, exported APIs, examples, docs, schemas, CLI help, or generated outputs;
- tests that encode expected behavior or review workflow.

A maintainer instruction establishes intended policy, not proof that code already follows it. Verify referenced commands and paths separately. Do not discard a working agreement solely because it is not enforced by tooling or repeated outside `AGENTS.md`.

## Inference rules

- Infer command standards from CI, runner files, package scripts, manifests, and contributor docs. Do not invent commands from ecosystem defaults.
- Infer package-manager standards from lockfiles, install docs, CI setup, and existing scripts. Do not recommend switching managers.
- Infer workspace ownership from manifests, package metadata, README links, exported modules, docs, examples, and directory layout.
- Infer public contract surfaces from user-facing docs, CLI behavior, schemas, protocols, macro diagnostics, examples, package exports, SDKs, and API docs.
- Infer synchronization rules only for docs, examples, generated outputs, fixtures, snapshots, and `AGENTS.md` guidance that exist.
- Include local coding, testing, security, compatibility, dependency, and review conventions when they materially affect correct work; they need not affect routing or synchronization.
- Prefer explicit applicable requirements over inferred practice. Report conflicting code or docs as drift instead of silently weakening the requirement.

## What not to infer

Do not infer that a repository wants a tool, dependency, framework, lint rule, type-system pattern, or testing library merely because it would be a good general practice.

Keep these out of mandatory generated guidance unless already evidenced or explicitly adopted by the maintainer:

- new dependencies or dev-dependencies;
- new package managers, runtimes, frameworks, or validation stacks;
- migrations from one library or workflow to another;
- language-specific style rules with no local evidence or explicit requirement;
- broad modernization advice.

A request for proposals does not authorize presenting a proposed policy as adopted. When a maintainer does adopt a rule, describe it as a requirement without claiming the implementation or tooling already satisfies it.

## Evidence ledger shape

Keep a brief ledger while working. It may remain internal unless the user asks for it.

| Claim | Class | Evidence | Output decision |
| --- | --- | --- | --- |
| `just test` is the default validation command. | Observed | `justfile` and CI use it. | Include. |
| Public APIs must use the repository's domain error type. | Observed | Explicit maintainer instruction; the type exists in source. | Include as intended policy; do not claim universal implementation compliance. |
| Manifest and lockfile changes belong in the same change. | Observed | Existing `AGENTS.md` working agreement. | Preserve when applicable; routine does not mean irrelevant. |
| `examples/basic` is public documentation. | Inferred | Root README links users there. | Include narrowly. |
| Add a snapshot testing library. | Recommended | Useful for long output, but not present or adopted. | Audit note only, not mandatory guide text. |

## Output rule

Generated guide text should state the standard and the supported action, not the research process. Example:

<!-- EXAMPLE ONLY: replace every command, docs path, and test path with repository evidence before use. -->

```md
When public CLI output changes, update `docs/cli.md` and the matching tests under `tests/cli/`, then run `just test-cli`. Run any additional completion checks required by the applicable repository guidance before finishing.
```

Do not include the words **Observed**, **Inferred**, or **Recommended** in the generated guide unless the user asked for an audit or evidence table.
