# Agent guide layout patterns

These patterns are source material, not default output. Before using any bullet or section in a generated guide, replace it with repository-specific guidance or omit it. Fenced examples are illustrative only; do not copy them without replacing every path, command, surface, and sync rule with repository evidence. Removal and synchronization rules describe authorized edits; in Audit, Alignment, and Checklist modes, report findings without changing files.

## Contents

- [Default guide layout](#default-guide-layout-pattern)
- [Size budget](#size-budget)
- [Split guidance triggers](#split-guidance-triggers)
- [Quick decision flow](#quick-decision-flow-pattern)
- [Audience labels](#audience-labels)
- [Documentation placement](#documentation-placement-rules)
- [Synchronization rules](#synchronization-rules)
- [Edit boundaries and workspace maps](#edit-boundary-notes)

## Default guide layout pattern

`AGENTS.md` is ordinary Markdown with no required headings or schema. Start with the instructions that most help an agent work correctly, not a fixed outline. The following are optional content areas, not sections to fill:

- **Working commands:** setup, run, build, test, lint, and generation commands that are useful in this scope.
- **Conventions and constraints:** local implementation and testing conventions, compatibility guarantees, dependency policies, security boundaries, and review expectations.
- **Verification and completion:** focused checks for iteration and all applicable checks or requirements before finishing or opening a PR.
- **Pointers and pitfalls:** where to start, applicable local guidance, and known mistakes worth preventing.
- **Routing and synchronization:** a workspace map, decision flow, audience labels, or docs/generated-output sync rules only when they resolve real editing ambiguities.

For small repositories, a few commands and working agreements may be enough. Preserve a useful existing structure. Do not require a project summary, audience taxonomy, documentation policy, decision flow, or workspace map merely to match this pattern.

Keep the file actionable. Prefer bullets and short paragraphs over dense prose. When common setup, test, lint, generation, or docs commands are central to everyday work, place a short evidenced command block early rather than burying commands near the end.
If the repository already has a canonical command runner or command index, such
as a `justfile`, `Makefile`, `Taskfile.yml`, or package scripts, prefer an early
pointer to that source over copying a long command list. Preserve concise
instructions about which commands to run, when to run them, and what is required
before finishing; a command index alone does not communicate those obligations.
Verify each listed command exists.

## Size budget

Prefer concise root guidance unless repository complexity clearly justifies more:

- Root `AGENTS.md`: as short as practical; usually under 220 lines. Many small repositories should be well below 120 lines. These are concision heuristics, not format requirements, target lengths, or completeness tests.
- Workspace map entry, when useful: 2-5 lines per important surface.
- Validation section: distinguish focused checks from required completion checks; preserve both even when a required check is broad or infrequent.
- Common commands: point to the evidenced runner file when it is the canonical command list; do not duplicate the whole runner.
- Sync rules: only existing surfaces with evidenced or explicitly required synchronized edits.
- Manifests, lockfiles, dependency automation config, package-manager config, runner files, and CI files: use as evidence, not an inventory to reproduce. Retain useful working agreements about them, including routine ones, when they clarify an action, prevent a mistake, or express a maintainer requirement.
- Long procedures: move to nested `AGENTS.md` or docs without dropping essential root-level constraints or completion requirements.
- Target-platform limits: when an evidenced agent runtime has an instruction size or truncation limit, keep the applicable root and nested guidance below that limit or split local procedures closer to the files they govern.

If exceeding 220 lines or a known platform limit, name the reason in the handoff: required working agreements, distinct public surfaces, unavailable nested guidance, generated-output complexity, public-contract complexity, or explicit user request. A known platform limit needs a mitigation, not just an explanation.

## Split guidance triggers

Prefer a nested `AGENTS.md` or local docs when:

- one subtree has a different runner, release flow, generator, or public contract evidenced by repository files;
- a procedure needs more than 8-10 bullets;
- a rule is specific to one subtree rather than repository-wide;
- validation requires specialized setup not needed for most edits;
- generated output or snapshot review has a multi-step workflow;
- public docs, examples, or APIs have local conventions that would clutter the root guide.

## Quick decision flow pattern

Use a decision flow only when routing changes is genuinely ambiguous. A small repository does not need a classification step before every edit. Adapt this optional pattern to actual workflows:

<!-- EXAMPLE ONLY: replace every route, surface, command, and sync rule with repository evidence before use. -->

```md
## Quick Decision Flow

1. **Find the owning surface and local guidance.** Consult the workspace map when ownership is unclear.
2. **Follow repository working agreements.** Use the implementation conventions, dependency policy, runner, generated-output flow, and test style required by this repository.
3. **Place documentation by content and audience.** User-facing workflows belong in existing user guides, examples, books, public site docs, or API docs. Keep implementation rationale close to the code and supporting evidence. Preserve and update existing maintainer books and design records when they document the changed contract; do not require new narrative documents for every internal change.
4. **Sync affected contracts.** When behavior, commands, generated output, feature flags, API shape, or recommended usage changes, update the relevant examples, user or maintainer docs, and existing `AGENTS.md` guidance in the same change when applicable.
5. **Validate in stages.** Start with focused checks, then satisfy all applicable completion requirements, including required broader checks, before finishing. Report any checks that could not be run.
```

## Audience labels

Use labels only when distinguishing audiences helps agents choose the correct editing or documentation workflow. If used, define them once and reuse them in the workspace map; otherwise omit the taxonomy.

<!-- EXAMPLE ONLY: keep, rename, or omit labels based on repository evidence. -->

```md
## Audience Labels

These labels describe the package, crate, app, tool, or surface itself, not the documentation file being edited:

- **User-facing**: normal entry points for application developers or end users.
- **Public integration**: public crates, packages, macros, schemas, protocols, plugins, SDKs, or tooling meant for extensions, integrations, or deeper customization.
- **Generated/source-of-truth**: generators, schemas, inventories, templates, or checked-in generated outputs that must stay synchronized.
- **Validation**: tests, fixtures, snapshots, examples-as-tests, or harnesses that encode expected behavior.
- **Internal**: workspace plumbing, implementation details, maintenance tooling, demos, benchmarks, experiments, and contributor-only workflow surfaces.
```

Treat these as public contracts when present: CLI flags and output, configuration files, schemas, protocols, macro syntax, diagnostics, public examples, package exports, facade APIs, plugin interfaces, SDKs, and API docs.

## Documentation placement rules

Include these rules only when the repository has the named surfaces and the guidance helps an agent place or update documentation:

- Determine a document's audience from its stated purpose, readers, and repository usage, not its filename or rendering system. READMEs and books may be user-facing, maintainer-facing, or mixed; classify mixed books by chapter when needed.
- Keep user-facing docs example-first. Prefer runnable commands or tested snippets over prose-only descriptions.
- Keep implementation details, subsystem boundaries, data flow, generated-output mechanics, and maintenance-only procedures close to the code, tests, fixtures, schemas, generator inputs, or comments that prove the behavior.
- Preserve existing maintainer documentation, architecture books, and design records. Route their updates by the contracts and workflows they document, without imposing end-user writing constraints or requiring new narrative docs for every implementation change.

## Synchronization rules

Make useful sync rules explicit and mechanical. Use “when X changes, update Y” language. Do not require a separate synchronization section when a concise instruction near the relevant command or convention is enough.

Common sync contract pattern:

<!-- EXAMPLE ONLY: include only surfaces that exist and have evidenced or explicitly required synchronized edits. -->

```md
## Synchronization Rules

When a substantive change modifies a public workflow, public feature, generated output, feature flag story, CLI behavior, supported inventory, macro syntax, diagnostic text, configuration shape, schema, protocol, or user-visible API shape:

1. Update the owning implementation.
2. Update the canonical executable example when relevant.
3. Update affected user-facing README files.
4. Update matching user or maintainer book, public site, tutorial, API reference, or example README pages when they exist and describe the changed contract.
5. Update relevant root or nested `AGENTS.md` guidance when it already names the changed workflow or boundary.
6. Update code comments, tests, fixtures, schemas, generator inputs, examples, existing maintainer documentation, or `AGENTS.md` guidance when they already encode the changed boundary, data flow, generated output, or internal behavior.
7. Keep these surfaces aligned in the same change unless there is a documented reason not to.
```

For generated output:

- Prefer changing the source generator, schema, inventory, metadata, template, or registry over hand-editing generated output.
- Keep generator tests, snapshots, golden files, fixtures, and relevant user or maintainer docs aligned.
- State exact regeneration commands only after confirming they exist.
- Do not create a standalone generated or ignored-output section,
  workspace-map entry, or validation/editing rule merely to restate
  `.gitignore` or build-output paths. Mention generated outputs where they
  change editing behavior: in a sync rule, workspace-map entry, or short
  boundary note that names the owning generator or source.

## Edit boundary notes

Include a short edit-boundary note when repository evidence or an explicit maintainer requirement identifies a surface as generated, vendored, release-critical, deployment-sensitive, migration-sensitive, production-configured, or otherwise restricted. Keep it mechanical:

- name the exact path or surface;
- state whether to edit the source, ask first, or avoid ordinary edits;
- point to the owning generator, docs, checklist, or local guidance when it exists.

Do not add broad safety language that does not help an agent work correctly in this repository.

## Workspace map pattern

A workspace map is optional. Include one when it reduces meaningful uncertainty about ownership or entry points, and keep it accurate and terse. Map useful editing surfaces, not every important file. Do not categorically exclude manifests, lockfiles, dependency configuration, runner files, or CI; include an entry only when it helps an agent act correctly. A useful working agreement about such a file can belong elsewhere in the guide without needing a map entry. Remove or shorten stale, misleading, irrelevant, or redundant entries only in authorized Patch mode; reporting modes recommend changes without applying them.

<!-- EXAMPLE ONLY: use real repository paths and omit sections that do not apply. -->

```md
## Workspace Map

### Main User-Facing Entry Points

- `packages/main`
  Audience: **User-facing**
  Docs: `packages/main/README.md`
  Role: default public entry point and compatibility boundary.

### Public Integration Surfaces

- `packages/plugin-api`
  Audience: **Public integration**
  Role: extension interface for integrations. Most users should start with `packages/main` instead.

### Generated and Validation Surfaces

- `tools/generate`
  Audience: **Generated/source-of-truth**
  Role: generator and maintenance tooling for checked-in outputs.

- `tests/fixtures`
  Audience: **Validation**
  Role: fixtures for public behavior and generated-output tests.
```

Use real repository paths and omit sections that do not apply.
