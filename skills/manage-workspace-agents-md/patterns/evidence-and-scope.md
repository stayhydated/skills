# Evidence and scope patterns

These patterns govern repository claims and intended working agreements. Use them as source material; do not paste this file wholesale into a generated guide. Audit, Alignment, and Checklist remain read-only under the mode contract in `SKILL.md`; proposed removals and corrections are findings, not permission to edit.

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
| **Observed** | Directly found in repository files or explicit maintainer instructions, including working agreements established in the current request. | Yes; distinguish intended policy from implementation facts. |
| **Inferred** | Strongly implied by repeated local practice, manifests, imports, layout, generated files, examples, tests, or CI. | Yes, when stated narrowly and consistent with explicit requirements. |
| **Recommended** | Plausible or useful, but neither current repository truth nor an adopted maintainer requirement. | No, unless the maintainer explicitly adopts it; asking for proposals is not adoption. |

An explicit maintainer instruction or existing `AGENTS.md` rule can establish intended policy without matching code, CI enforcement, or another document. It does not prove that a referenced path or command exists, that implementation follows the policy, or that a check succeeds. Verify factual details separately and report discrepancies without silently replacing the requirement with observed behavior.

Keep a brief evidence ledger while drafting or patching. It may stay internal unless the user asks for it.

## Minimum evidence by mode

### Draft

Inspect explicit maintainer requirements, the top-level tree, existing root or nested `AGENTS.md`, README or contributor docs, repository manifests, CI workflows, runner files when present, and the docs/examples/tests/generated outputs that may be named. Look for useful coding conventions, constraints, pitfalls, and completion requirements as well as routing information.

### Patch

Inspect the guide being patched, applicable maintainer requirements, evidence for every changed command/path/convention/constraint/sync rule/ownership claim, and local guidance that applies to the edited scope.

### Audit

Inspect the `AGENTS.md` guide and enough repository evidence and stated requirements to verify or challenge its commands, conventions, constraints, workspace map when present, synchronization rules, and validation guidance. If evidence is incomplete, mark findings as evidence-limited. Do not treat a policy as unsupported solely because it appears only in `AGENTS.md`.

### Alignment

Compare the guide against explicit requirements, CI, runner files, manifests, lockfiles, README/contributor docs, relevant code conventions, generated-output sources, examples, tests, and scoped `AGENTS.md` guidance. Distinguish stale factual claims from implementation that does not yet satisfy intended policy. Report drift without applying corrections.

### Checklist

Inspect the `AGENTS.md` guide and every file or requirement needed for checked items. Mark unsupported items `Not checked` instead of guessing. Report failures without applying corrections.

## Repository inspection playbook

For repository-specific work:

1. Read the request and identify explicit maintainer requirements and applicable existing guidance.
2. List top-level files and directories.
3. Identify manifests, lockfiles, runner files, and CI workflows already used by the repository.
   When a command runner or command index exists, such as `justfile`, `Makefile`,
   `Taskfile.yml`, or package scripts, inspect it before proposing common
   commands and prefer pointing agents to that source when it is the canonical
   command list. Preserve instructions about when specific checks are required.
4. Identify workspace members and public entry points.
5. Read existing root and nested `AGENTS.md` guidance, README files, and contributor docs for local conventions, working agreements, supported workflows, and completion requirements.
6. Inspect relevant implementation, tests, examples, and configuration to verify local practices and known pitfalls without turning isolated patterns into policy.
7. Inspect docs, examples, tests, snapshots, generated files, and fixtures that may need sync rules.
8. Build a workspace map only when it resolves meaningful ownership or entry-point ambiguity.
9. Draft sync rules only for surfaces that exist and need coordinated changes.
10. Final-check every command and path used in the output; distinguish existence, intended usage, and successful execution.

Use manifests, lockfiles, dependency automation config, runner files, and CI as evidence for workspace shape, package manager, dependency tooling, commands, and validation. Do not list them merely to inventory the repository. Include instructions about them when they clarify a useful action, prevent a known mistake, or express a maintainer requirement, even when the procedure is routine. Preserve existing working agreements unless they are stale, misleading, irrelevant, redundant, or explicitly superseded. Do not delete lockfile, dependency, or CI guidance solely because of its file category or because it is documented only in `AGENTS.md`. In reporting modes, propose changes without applying them.

Use `.gitignore` as exclusion evidence, not as an inventory to repeat in `AGENTS.md`. Ignored build outputs, cache directories, dependency caches, and tool output directories are not source surfaces by default; name them when a repository-owned workflow or explicit working agreement makes the path relevant to editing, synchronization, cleanup, or validation.

## Alignment drift checks

When comparing guidance against repository truth and intended policy, check:

| Area | Compare against |
| --- | --- |
| Validation commands | CI, runner files, manifests, contributor docs, and explicit completion requirements. |
| Working agreements | Explicit maintainer requirements and applicable root or nested `AGENTS.md`; distinguish intended policy from current enforcement. |
| Coding conventions and constraints | Relevant implementation, tests, configuration, contributor docs, and explicit requirements. |
| Package manager | Manifests, lockfiles, install docs, package-manager config, and CI setup. |
| Workspace members | Workspace config, package metadata, manifests, and directory layout. |
| Public entry points | README files, package/crate metadata, examples, docs, exported APIs, and facade modules. |
| Documentation sync rules | User and maintainer docs, README, books, public site docs, API docs, examples, tutorials, generated outputs, fixtures, and `AGENTS.md`, classified by actual audience. |
| Generated outputs | Generators, schemas, registries, templates, checked-in generated files, snapshots, and fixtures. |
| Testing guidance | Test directories, snapshot files, UI-test harnesses, runner files, package scripts, CI jobs, and explicit requirements. |
| Scoped guidance | Nested `AGENTS.md` guidance. |
| Public contracts | CLI behavior, config, schemas, macro syntax, diagnostics, plugin interfaces, package exports, and API docs. |

## Conflict precedence

Separate instruction authority, factual evidence, and local command selection rather than using a single source ranking for all three:

1. Explicit maintainer requirements and applicable root or nested `AGENTS.md` guidance establish intended working agreements. Apply the target runtime's instruction precedence; do not replace an explicit requirement with an inferred code or CI convention.
2. CI workflows are evidence for automated merge checks, not a complete inventory of every local or maintainer-required completion check.
3. Runner files, manifests, and contributor docs establish supported local commands. Verify that a documented command exists and can run in the intended environment.
4. README and contributor docs describe intended workflows; source, tests, and configuration provide evidence of implemented behavior.
5. Manifest conventions define workspace shape and package metadata.
6. Ecosystem defaults are fallback context only, not enough to create a standard.

Only copy CI commands into a guide when directly runnable in a normal checkout or when no local equivalent exists; document any required setup. Do not silently weaken required checks because a focused test is cheaper. Report unresolved conflicts and distinguish a stale command from an intended requirement that lacks implementation or enforcement.

## Nested guidance precedence

Use the most local applicable guidance for files in a subtree while preserving non-conflicting inherited requirements according to the target runtime. Keep root guidance for repository-wide rules. Put local exceptions, generated-output procedures, tool-specific details, and path-specific workflows in nested `AGENTS.md` or local docs.

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

When repository evidence or explicit maintainer requirements identify restricted or sensitive edit surfaces, include a short, mechanical boundary in generated guidance. Common supported boundaries include:

- change generated outputs through the generator, schema, inventory, metadata, template, or registry when one exists;
- ask before changing release, deployment, migration, production configuration, or compatibility files when local guidance marks them sensitive;
- do not edit secrets, credentials, vendored dependencies, or checked-in third-party artifacts as ordinary source.

Do not invent high-risk areas. Keep boundary notes short enough to guide correct edits rather than turning the guide into a policy document.

Do not treat ignored build, cache, dependency, or tool-output paths as high-risk edit surfaces merely because `.gitignore` names them. Omit these paths unless a repository workflow or working agreement makes them relevant to editing, cleanup, or validation.

## Limited-evidence mode

If repository access is incomplete, produce an evidence-limited draft, patch, audit, or checklist. Clearly separate visible facts, explicit requirements, assumptions, missing evidence, validation performed, and validation not performed. Do not name commands, paths, docs, packages, generated outputs, or tools that were not provided or otherwise evidenced. Preserve stated working agreements while marking unverified factual details. Treat non-`AGENTS.md` agent artifacts as out of scope.

## Placeholder hygiene

Before handoff, search the output for:

- placeholder names such as `my-workspace`, `foo`, `bar`, `baz`, or `example` when they are not real repository names;
- commands not found in repository evidence;
- copied template paths, package names, crate names, app names, fixture names, or generated-output names;
- unchecked Markdown links or local paths;
- copied pattern headings or fenced examples from `patterns/*` or `templates/*` without replacing placeholders and supporting claims with evidence or explicit requirements.

## Recommendation boundary

Recommendations are useful in audits and proposals, but are not adopted working agreements. Do not present a preferred tool, dependency, framework, package manager, test library, type checker, linter, formatter, or workflow as current policy unless repository evidence or explicit maintainer adoption supports it. A request for suggestions alone is not adoption.

When the user asks for an `AGENTS.md` patch, exclude unsupported recommendations from mandatory guide text and mention them separately only when they are directly relevant.

## Other languages and tools

Include language or tool guidance when it materially helps an agent work correctly in the repository. Local implementation conventions and working agreements are valid even when they are unrelated to routing or synchronization. Use observed evidence, explicit requirements, and narrowly inferred local practices; do not invent language-specific rules from ecosystem preferences.
