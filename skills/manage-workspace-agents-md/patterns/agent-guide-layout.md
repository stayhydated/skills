# Agent guide layout patterns

These patterns are source material, not default output. Before using any bullet or section in a generated guide, replace it with repository-specific guidance or omit it. Fenced examples are illustrative only; do not copy them without replacing every path, command, surface, and sync rule with repository evidence.

## Default guide layout pattern

Use this order unless the repository has a strong reason to differ:

1. `# AGENTS.md`
2. One paragraph: “This is the working guide for contributors and coding agents in the `<workspace>` workspace.”
3. `Use it to decide:` with 4-5 bullets about ownership, audience, docs sync, tests/fixtures/generated outputs, and validation.
4. `Start here` lines for the main facade, default package, app, docs surface, or runner file when it is the repository entry point.
5. Optional `## Common Commands` when commands are repository-evidenced and useful for most edits.
6. `## Project Summary`
7. `## Quick Decision Flow`
8. `## Audience Labels`
9. `## Documentation Placement`
10. `## Synchronization Rules`
11. `## Workspace Map`
12. `## Repository Standards`, only for standards evidenced by the repository.
13. `## Validation and Editing Rules`
14. Optional narrowly scoped sections for generated outputs, public docs, examples, macros, schemas, or path-specific workflows.

For small repositories, combine or omit sections aggressively. Do not create a section solely because it appears in this layout.

Keep the file example-first and command-focused. Prefer bullets and short paragraphs over dense prose. When common setup, test, lint, generation, or docs commands are central to everyday work, place a short evidenced command block in the first third of the guide rather than burying commands near the end.
If the repository already has a canonical command runner or command index, such
as a `justfile`, `Makefile`, `Taskfile.yml`, or package scripts, prefer an early
pointer to that source over copying a long command list. If the guide already
points to that source, such as `Start with just --list`, omit `## Common
Commands` unless 1-3 commands are non-obvious, not discoverable from the runner,
or needed before normal editing or handoff. Verify each listed command exists.

## Size budget

Prefer concise root guidance unless repository complexity clearly justifies more:

- Root `AGENTS.md`: as short as practical; usually under 220 lines. Many small repositories should be well below 120 lines.
- Workspace map entry: 2-5 lines per important surface.
- Validation section: only commands that exist and are commonly needed.
- Common commands: point to the evidenced runner file when it is the canonical
  command list; do not duplicate the whole runner.
- Sync rules: only surfaces that exist and routinely need synchronized edits.
- Manifests, lockfiles, dependency automation config, package-manager config,
  runner files, and CI files: use as evidence; omit from the guide unless an
  exact file has a documented, non-obvious editing procedure that agents must
  follow during ordinary work. In existing guides, remove entries that only say
  to keep lockfiles aligned, update dependency automation schedules/groups, or
  route ordinary dependency metadata edits.
- Long procedures: move to nested `AGENTS.md` or docs.
- Target-platform limits: when an evidenced agent runtime has an instruction size or truncation limit, keep the applicable root and nested guidance below that limit or split local procedures closer to the files they govern.

If exceeding 220 lines or a known platform limit, name the reason in the handoff: distinct public surfaces, unavailable nested guidance, generated-output complexity, public-contract complexity, or explicit user request.

## Split guidance triggers

Prefer a nested `AGENTS.md` or local docs when:

- one subtree has a different runner, release flow, generator, or public contract evidenced by repository files;
- a procedure needs more than 8-10 bullets;
- a rule applies to fewer than two top-level surfaces;
- validation requires specialized setup not needed for most edits;
- generated output or snapshot review has a multi-step workflow;
- public docs, examples, or APIs have local conventions that would clutter the root guide.

## Quick decision flow pattern

Use a decision flow near the top because it helps agents route changes before editing:

<!-- EXAMPLE ONLY: replace every route, surface, command, and sync rule with repository evidence before use. -->

```md
## Quick Decision Flow

Before editing, classify the change:

1. **Find the surface in the workspace map.** Use its audience label to decide how much public explanation the change needs.
2. **Place documentation by content, not by crate or package audience.** User-facing workflows belong in READMEs, examples, books, public site docs, or API docs when those surfaces exist. Implementation rationale should stay close to the code, tests, fixtures, schemas, generator inputs, or examples that prove the behavior. Do not create separate narrative documentation as a required synchronization target.
3. **Follow repository standards, not ecosystem defaults.** Use the package manager, runner, generated-output flow, and test style evidenced by this repository.
4. **Sync public workflow changes.** If behavior, commands, generated output, feature flags, API shape, or recommended usage changes, update the relevant example, README, book page, public docs, and existing `AGENTS.md` guidance in the same change when applicable.
5. **Validate narrowly.** Run the smallest evidenced command that proves the edited behavior or documentation surface is still sound.
```

## Audience labels

Define labels once, then use them in the workspace map.

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

Include these rules only when the repository has the named surfaces:

- Treat root `README.md`, crate/package READMEs, example READMEs, books, public site docs, tutorials, and public API docs as user-facing.
- Keep user-facing docs example-first. Prefer runnable commands or tested snippets over prose-only descriptions.
- Keep implementation details, subsystem boundaries, data flow, generated-output mechanics, and maintenance-only procedures close to the code, tests, fixtures, schemas, generator inputs, or comments that prove the behavior.

## Synchronization rules

Make sync rules explicit and mechanical. Use “when X changes, update Y” language.

Common sync contract pattern:

<!-- EXAMPLE ONLY: include only surfaces that exist and routinely need synchronized edits. -->

```md
## Synchronization Rules

When a substantive change modifies a public workflow, public feature, generated output, feature flag story, CLI behavior, supported inventory, macro syntax, diagnostic text, configuration shape, schema, protocol, or user-visible API shape:

1. Update the owning implementation.
2. Update the canonical executable example when relevant.
3. Update affected user-facing README files.
4. Update matching book, public site, tutorial, API reference, or example README pages when they exist.
5. Update relevant root or nested `AGENTS.md` guidance when it already names the changed workflow or boundary.
6. Update code comments, tests, fixtures, schemas, generator inputs, examples, or existing `AGENTS.md` guidance when it already encodes the changed boundary, data flow, generated output, or internal behavior.
7. Keep these surfaces aligned in the same change unless there is a documented reason not to.
```

For generated output:

- Prefer changing the source generator, schema, inventory, metadata, template, or registry over hand-editing generated output.
- Keep generator tests, snapshots, golden files, fixtures, and user-facing docs aligned.
- State exact regeneration commands only after confirming they exist.
- Do not create a standalone generated or ignored-output section,
  workspace-map entry, or validation/editing rule merely to restate
  `.gitignore` or build-output paths. Mention generated outputs only where they
  change editing behavior: in a sync rule, workspace-map entry, or short
  boundary note that names the owning generator or source.

## Edit boundary notes

Include a short edit-boundary note only when repository evidence identifies a surface as generated, vendored, release-critical, deployment-sensitive, migration-sensitive, production-configured, or otherwise restricted. Keep it mechanical:

- name the exact path or surface;
- state whether to edit the source, ask first, or avoid ordinary edits;
- point to the owning generator, docs, checklist, or local guidance when it exists.

Do not add broad safety language that does not help an agent route edits in this repository.

## Workspace map pattern

The workspace map is the highest-value part of the file. Keep it accurate and terse. Map owned editing surfaces, not every important file. Exclude routine manifests, lockfiles, dependency automation config, package-manager config, runner files, and CI files unless the repository documents a non-obvious editing procedure for that exact file. Remove existing workspace-map entries for these files when they only describe standard dependency metadata, lockfile synchronization, update schedules, grouping, labels, or ordinary CI/dependency routing.

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
