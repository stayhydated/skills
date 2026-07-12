# Evidence and scope patterns

These patterns govern repository claims. Use them as source material; do not paste this file wholesale into a generated guide.

## Contents

- [Claim classes](#claim-classes)
- [Minimum evidence by mode](#minimum-evidence-by-mode)
- [Repository inspection playbook](#repository-inspection-playbook)
- [Alignment drift checks](#alignment-drift-checks)
- [Conflict and guidance precedence](#conflict-precedence)
- [Agent platform compatibility](#agent-platform-compatibility)
- [High-risk and limited-evidence handling](#high-risk-edit-surfaces)
- [Output hygiene and recommendation boundaries](#placeholder-hygiene)

## Claim classes

Before adding project-specific instructions, classify each claim:

| Class | Meaning | Can become mandatory guide text? |
| --- | --- | --- |
| **Observed** | Directly found in repository files. | Yes. |
| **Inferred** | Strongly implied by manifests, imports, layout, generated files, examples, tests, or CI. | Yes, when stated narrowly. |
| **Recommended** | Plausible or useful, but not current repository truth. | No, unless the user explicitly asks to standardize. |

Keep a brief evidence ledger while drafting or patching. It may stay internal unless the user asks for it.

## Minimum evidence by mode

### Draft

Inspect the top-level tree, existing root or nested `AGENTS.md`, README or contributor docs, repository manifests, CI workflows, runner files when present, and the docs/examples/tests/generated outputs that may be named.

### Patch

Inspect the guide being patched, evidence for every changed command/path/sync rule/ownership claim, and local guidance that applies to the edited scope.

### Audit

Inspect the `AGENTS.md` guide and enough repository evidence to verify or challenge its commands, paths, workspace map, synchronization rules, and validation guidance. If evidence is incomplete, mark findings as evidence-limited.

### Alignment

Compare the guide against CI, runner files, manifests, lockfiles, README/contributor docs, generated-output sources, examples, tests, and scoped `AGENTS.md` guidance.

### Checklist

Inspect the `AGENTS.md` guide and every file needed for checked items. Mark unsupported items `Not checked` instead of guessing.

## Repository inspection playbook

For repository-specific work:

1. List top-level files and directories.
2. Identify manifests, lockfiles, runner files, and CI workflows already used by the repository.
   When a command runner or command index exists, such as `justfile`, `Makefile`,
   `Taskfile.yml`, or package scripts, inspect it before proposing common
   commands and prefer pointing agents to that source when it is the canonical
   command list.
3. Identify workspace members and public entry points.
4. Read existing root and nested `AGENTS.md` guidance.
5. Read README and contributor docs to understand audience and supported workflows.
6. Inspect docs, examples, tests, snapshots, generated files, and fixtures that may need sync rules.
7. Build a workspace map from observed owned editing surfaces only.
8. Draft sync rules only for surfaces that exist.
9. Final-check every command and path used in the output.

Use manifests, lockfiles, dependency automation config, runner files, and CI as evidence for workspace shape, package manager, dependency tooling, commands, and validation. Do not list generic root manifests, lockfiles, dependency metadata, dependency automation config, package-manager config, runner files, or workflow YAML as workspace-map entries merely because they exist or own configuration. Include them only when repository guidance or docs define a non-obvious editing procedure for that exact surface that agents must follow during ordinary work. In Patch, Alignment, and Checklist modes, remove existing entries that only say to keep lockfiles aligned, update dependency automation schedules/groups/labels, or route ordinary dependency metadata edits.

Use `.gitignore` as exclusion evidence, not as an inventory to repeat in `AGENTS.md`. Ignored build outputs, cache directories, dependency caches, and tool output directories are not source surfaces by default; name them only when another repository-owned workflow makes the path relevant to editing, synchronization, cleanup, or validation.

## Alignment drift checks

When comparing guidance against repository truth, check:

| Area | Compare against |
| --- | --- |
| Validation commands | CI, runner files, manifests, and contributor docs. |
| Package manager | Manifests, lockfiles, install docs, package-manager config, and CI setup. |
| Workspace members | Workspace config, package metadata, manifests, and directory layout. |
| Public entry points | README files, package/crate metadata, examples, docs, exported APIs, and facade modules. |
| Documentation sync rules | README, books, public site docs, API docs, examples, tutorials, generated outputs, fixtures, and `AGENTS.md`. |
| Generated outputs | Generators, schemas, registries, templates, checked-in generated files, snapshots, and fixtures. |
| Testing guidance | Test directories, snapshot files, UI-test harnesses, runner files, package scripts, and CI jobs. |
| Scoped guidance | Nested `AGENTS.md` guidance. |
| Public contracts | CLI behavior, config, schemas, macro syntax, diagnostics, plugin interfaces, package exports, and API docs. |

## Conflict precedence

When instructions, commands, or docs disagree, separate merge requirements from local command selection:

1. CI workflows are the highest-confidence source for what must be proven before merge.
2. Runner files, manifests, and contributor docs are the highest-confidence local command sources.
3. Existing root or nested `AGENTS.md` guidance defines agent-facing scope and local exceptions.
4. README and contributor docs define intended user-facing workflows.
5. Manifest conventions define workspace shape and package metadata.
6. Ecosystem defaults are fallback context only, not enough to create a standard.

Only copy CI commands into a guide when directly runnable in a normal checkout or when no local equivalent exists. Preserve unresolved conflicts in the handoff instead of guessing.

## Nested guidance precedence

Use the most local applicable guidance for files in a subtree. Keep root guidance for repository-wide rules. Put local exceptions, generated-output procedures, tool-specific details, and path-specific workflows in nested `AGENTS.md` or local docs.

## Agent platform compatibility

When the user request, repository files, or existing guidance targets a specific agent runtime, check the platform behavior before deciding where `AGENTS.md` instructions belong. Relevant compatibility facts include:

- supported instruction filenames;
- global, repository, nested, and override-file discovery locations;
- root-vs-subtree precedence;
- merge, layering, replacement, or nearest-file behavior;
- size, byte, or truncation limits for loaded guidance;
- whether root and nested `AGENTS.md` files are actually read by the target runtime.

Do not add platform-specific behavior to generated repository guidance unless the platform is evidenced by the user request, repository files, existing guidance, or inspected platform documentation. If platform behavior matters but was not checked, mark it `Not checked` in the handoff rather than guessing.

## High-risk edit surfaces

When repository evidence identifies restricted or sensitive edit surfaces, include a short, mechanical boundary in generated guidance. Common evidence-backed boundaries include:

- change generated outputs through the generator, schema, inventory, metadata, template, or registry when one exists;
- ask before changing release, deployment, migration, production configuration, or compatibility files when local guidance marks them sensitive;
- do not edit secrets, credentials, vendored dependencies, or checked-in third-party artifacts as ordinary source.

Do not invent high-risk areas that are not evidenced by repository files or existing guidance. Keep boundary notes short enough to help routing rather than turning the guide into a policy document.

Do not treat ignored build, cache, dependency, or tool-output paths as high-risk edit surfaces merely because `.gitignore` names them. Omit these paths from guide text unless they require a non-obvious generator, cleanup step, validation step, or edit-routing rule.

## Limited-evidence mode

If repository access is incomplete, produce an evidence-limited draft, patch, audit, or checklist. Clearly separate visible facts, assumptions, missing evidence, validation performed, and validation not performed. Do not name commands, paths, docs, packages, generated outputs, or tools that were not provided or otherwise evidenced. For non-`AGENTS.md` agent artifacts, this skill is out of scope.

## Placeholder hygiene

Before handoff, search the output for:

- placeholder names such as `my-workspace`, `foo`, `bar`, `baz`, or `example` when they are not real repository names;
- commands not found in repository evidence;
- copied template paths, package names, crate names, app names, fixture names, or generated-output names;
- unchecked Markdown links or local paths;
- copied pattern headings or fenced examples from `patterns/*` or `templates/*` without replacing placeholders and backing claims with evidence.

## Recommendation boundary

Recommendations are useful in audits and proposals, but they are not repository guidance. Do not present a preferred tool, dependency, framework, package manager, test library, type checker, linter, formatter, or workflow as current policy unless repository evidence or an explicit user request supports it.

When the user asks for an `AGENTS.md` patch, exclude unsupported recommendations from the guide and mention them separately only when they are directly relevant.

## Other languages and tools

Include any language or tool in the workspace map or validation guidance only when it owns user-facing, public-integration, generated, validation, or contributor workflow surfaces. Use observed evidence and local conventions. Do not invent language-specific rules.
