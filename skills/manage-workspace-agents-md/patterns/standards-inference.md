# Standards inference patterns

Use this file to infer repository standards for `AGENTS.md` guidance. It is not a recommendation list and must not be copied wholesale into generated output.

## What counts as a standard

A repository standard is a repeated or canonical local practice that agents must follow to avoid wrong edits. Good standards are supported by one or more of these evidence types:

- CI jobs, runner recipes, manifests, lockfiles, and documented setup commands;
- repeated file organization, naming, fixture, generator, or snapshot layout;
- existing root or nested `AGENTS.md`, contributor docs, or README instructions;
- public entry points, exported APIs, examples, docs, schemas, CLI help, or generated outputs;
- tests that encode expected behavior or review workflow.

## Inference rules

- Infer command standards from CI, runner files, package scripts, manifests, and contributor docs. Do not invent commands from ecosystem defaults.
- Infer package-manager standards from lockfiles, install docs, CI setup, and existing scripts. Do not recommend switching managers.
- Infer workspace ownership from manifests, package metadata, README links, exported modules, docs, examples, and directory layout.
- Infer public contract surfaces from user-facing docs, CLI behavior, schemas, protocols, macro diagnostics, examples, package exports, SDKs, and API docs.
- Infer Rust crate or workspace compatibility expectations from `Cargo.toml`, workspace package versions, and explicit compatibility docs; for evidenced versions below `1.0.0`, do not infer backward/legacy compatibility unless repository docs explicitly require it.
- Infer synchronization rules only for docs, examples, generated outputs, fixtures, snapshots, and `AGENTS.md` guidance that exist.
- Infer local code standards only when repeated practice affects how agents should route, validate, or synchronize edits.

## What not to infer

Do not infer that a repository wants a tool, dependency, framework, lint rule, type-system pattern, or testing library merely because it would be a good general practice.

Keep these out of generated guidance unless already evidenced or explicitly requested:

- new dependencies or dev-dependencies;
- new package managers, runtimes, frameworks, or validation stacks;
- migrations from one library or workflow to another;
- language-specific style rules not visible in the repository;
- broad modernization advice;
- backward/legacy compatibility for Rust crates or workspaces with evidenced versions below `1.0.0` unless explicitly documented.

## Evidence ledger shape

Keep a brief ledger while working. It may remain internal unless the user asks for it.

| Claim | Class | Evidence | Output decision |
| --- | --- | --- | --- |
| `just test` is the default validation command. | Observed | `justfile` and CI use it. | Include. |
| `examples/basic` is public documentation. | Inferred | Root README links users there. | Include narrowly. |
| Add a snapshot testing library. | Recommended | Useful for long output, but not present. | Audit note only, not guide text. |

## Output rule

Generated guide text should state the standard and the evidence-backed action, not the research process. Example:

<!-- EXAMPLE ONLY: replace every command, docs path, and test path with repository evidence before use. -->

```md
When public CLI output changes, update `docs/cli.md` and the matching tests under `tests/cli/`, then run `just test-cli`.
```

Do not include the words **Observed**, **Inferred**, or **Recommended** in the generated guide unless the user asked for an audit or evidence table.
