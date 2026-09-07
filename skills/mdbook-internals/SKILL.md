---
name: mdbook-internals
description: Creates, revises, and reviews en-US maintainer-facing mdBook documentation for architecture, components, control and data flows, invariants, failure modes, operations, extension points, contributor workflows, and design decisions. Applies when readers need verified implementation context to modify, debug, operate, or review a system. Excludes end-user task documentation, which belongs in mdbook-user-docs.
---

# Internal mdBook documentation

## Goal

Give maintainers enough verified context to change, debug, operate, or review a system safely. Explain boundaries, contracts, invariants, flows, failure behavior, and rationale without turning the book into a prose copy of the repository.

## Request mode and write boundary

Choose the mode before following the workflow:

- **Edit:** create, revise, or apply changes only to the user-authorized chapters
  and supporting surfaces.
- **Review:** reviews, audits, checks, and checklist requests report findings and
  proposed corrections without editing files. Choose Review when edit permission
  is ambiguous; applying a checklist does not authorize applying its fixes.

Review mode governs every workflow step, template, and supporting checklist.
Do not create missing chapters, change navigation or configuration, accept
expectations, or run formatters, generators, builds, or tests that write into the
checkout. Use non-mutating checks or an isolated temporary copy containing the
required book, includes, configuration, and validation inputs. Check wrappers and
preprocessors for absolute paths or external side effects before running them;
a temporary build destination alone does not make a build read-only. When safe
isolation is unavailable, report static review rather than run the command.
Preserve tracked, untracked, and ignored files, including pre-existing user edits.

In Edit mode, make only the requested changes and report required follow-ups
outside that scope. A prior review is not standing permission to edit.

## Workflow

### 1. Locate the correct book and protect existing work

- Find the `book.toml` for the requested documentation. If the repository contains multiple books, use the one that covers the relevant subsystem or audience; do not edit every book by default.
- Read `[book].src` and resolve it relative to the book root. Use `<book-src>/SUMMARY.md`; do not assume the source directory is `src/`.
- Read the target chapter, adjacent chapters, relevant repository instructions, and only the architecture records, configuration, tests, schemas, and implementation needed for the requested topic.
- Inspect the current working tree or diff when available. Preserve unrelated edits and avoid broad reformatting outside the requested scope.
- Work only on en-US content. Leave translated locale trees and translation catalogs unchanged.

### 2. Define the documentation claim

Identify the intended reader and the maintenance decision or recurring question the page must support. Classify each material statement as one of these kinds of truth:

- **Contract:** behavior callers or components may rely on.
- **Invariant:** a condition that must remain true.
- **Current implementation:** how the contract is realized now and may change.
- **Rationale:** why the design exists.
- **Known limitation:** an unsupported or hazardous case.
- **Operational procedure:** an action with prerequisites, verification, and recovery.
- **Proposal:** an unimplemented direction with explicit status and ownership when known.

Do not present rationale as a guarantee, current behavior as an immutable interface, or a proposal as implemented behavior.

### 3. Verify before explaining

- Trace material behavior through code, tests, schemas, configuration, generated artifacts, and runtime evidence when available; do not infer behavior from names alone.
- Confirm ownership and boundaries from authoritative files such as code-owner rules, module interfaces, schemas, deployment configuration, or accepted design records.
- Treat comments and old design documents as leads when newer code, tests, or schemas disagree.
- Mark uncertainty, version dependence, inference, and missing evidence explicitly.
- Do not invent guarantees. When sources conflict, explain the conflict or report it instead of silently choosing the most convenient interpretation.

### 4. Choose one dominant page pattern

| Page type | Primary question | Core content |
|---|---|---|
| System overview | What are the major boundaries? | Responsibilities, dependencies, principal flows, trust boundaries |
| Component | What does this component own? | Inputs, outputs, state, invariants, collaborators, failure behavior |
| Data or control flow | How does work move through the system? | Trigger, sequence, transformations, persistence, retries, completion |
| Lifecycle or state | What states exist, and how do they change? | States, transitions, guards, side effects, terminal conditions |
| Operational | How is this run and diagnosed? | Configuration, signals, failure modes, recovery, capacity limits |
| Extension point | How can this be changed safely? | Contract, registration, lifecycle, constraints, compatibility, tests |
| Decision record | Why was this design chosen? | Context, forces, options, decision, consequences, status |

Split or link a focused page when the content serves unrelated maintenance decisions or mixes abstraction levels without a clear reason.

### 5. Cover the engineering facts that affect change safety

Include only what is relevant to the page's maintenance decision:

- Responsibilities, explicit non-responsibilities, inputs, outputs, and ownership.
- State, persistence, retention, schema evolution, and transaction boundaries.
- Control flow, concurrency, ordering, idempotency, backpressure, and lifecycle.
- Preconditions, postconditions, invariants, and enforcement points.
- Failure classes, propagation, retry safety, cleanup, recovery, and partial success.
- Security and trust boundaries, validation, authorization, and secret handling.
- Performance characteristics, scaling dimensions, limits, and expensive paths.
- Configuration, feature flags, compatibility, observability, extension points, hazards, and required tests.

Omit incidental detail that a maintainer can discover faster with symbol search and that does not alter design understanding, debugging strategy, operational action, or change safety.

### 6. Write concise, source-backed en-US prose

- Open with a short scope statement and the main conclusion.
- Use American spelling and punctuation in original prose, including `behavior`, `modeling`, and `acknowledgment`.
- Preserve identifiers, source symbols, configuration keys, log text, protocol terms, product names, and quotations exactly.
- Use the same component and event names as the implementation. Define acronyms and domain terms at first use.
- Organize by responsibility and behavior, not repository traversal or project chronology.
- Keep one primary engineering claim per paragraph. Support material or non-obvious claims with the relevant code, test, schema, configuration, metric, trace, or accepted decision record.
- State constraints near the behavior they constrain and label implementation-specific details as current behavior.
- Use examples to establish a contract, invariant, schema, failure mode, or extension pattern rather than reproduce a large implementation.

### 7. Use source, tables, diagrams, and mdBook features safely

- Link to stable files, symbols, schemas, tests, or generated references when the renderer and hosting setup support those links.
- The built-in `links` and `index` preprocessors run by default unless `[build].use-default-preprocessors = false`. Confirm that `links` is active before using `{{#include ...}}`.
- Resolve include paths relative to the chapter that contains the include. Prefer named anchors over line ranges and include only enough code to establish the relevant contract.
- Inspect `book.toml`, build wrappers, dependency files, and CI before using Mermaid or other version-sensitive syntax. Do not add a renderer or preprocessor dependency unless the project already supports it or the user requested the configuration change.
- Use tables for compact contracts and comparisons. Keep one comparison dimension per table and move nuanced rationale to prose.
- Use diagrams as models, not decoration. Keep one abstraction level and one engineering question per diagram; label material boundaries and follow the diagram with its invariants or implications.
- Preserve `SUMMARY.md`, use relative `.md` chapter links when practical, keep heading levels valid, and avoid raw HTML unless required and verified.
- When adding, moving, or renaming a chapter, update navigation and affected inbound links intentionally. Avoid URL-changing renames unless required.

Read [references/architecture-patterns.md](references/architecture-patterns.md) when selecting an architecture pattern, table, diagram, or structural split.

### 8. Validate against both source and renderer

Apply the request mode above to every check. Report a check as successful only
when it ran successfully; distinguish failed attempts, static review, and checks
not run. In Review mode, inspect the original checkout for accidental changes.

1. Re-check every material contract, invariant, state, and failure claim against the cited implementation evidence.
2. Check `[build].create-missing` before builds or `mdbook test`. It defaults to
   `true`, so loading the book can create missing chapters from `SUMMARY.md`.
   In Edit mode, create or rename authorized files explicitly, then set
   `MDBOOK_BUILD__CREATE_MISSING=false` for each validation process. Confirm that
   wrappers preserve this override. Report remaining missing chapters as
   follow-ups; validation must not create files outside the requested scope.
   If the override cannot be honored, validate in a safely isolated copy or
   report static review. In Review mode, report missing chapters and build only
   in a safely isolated copy as required above.
3. Build with the repository wrapper or `mdbook build <book-root>`.
4. Run repository Markdown, link, spelling, style, schema, and documentation checks.
5. Run `mdbook test <book-root>` for testable Rust snippets and project-native tests for other languages or schemas.
6. Confirm navigation, links, includes, tables, and configured diagrams render correctly at normal content width.
7. Review for mixed abstraction levels, stale line references, accidental promises, undocumented failure paths, and unresolved placeholders.
8. Inspect the final diff for unrelated edits or files created as a build side effect.

Report the selected mode, changed or reviewed chapters, evidence consulted,
validation performed (including any isolated copy), and checks that failed or
could not run. Do not describe static review as a successful build or test.

## Resources

- For a system or subsystem page, start from [assets/architecture-chapter-template.md](assets/architecture-chapter-template.md).
- For a component page, start from [assets/component-chapter-template.md](assets/component-chapter-template.md).
- For a durable design decision, use [assets/decision-record-template.md](assets/decision-record-template.md).
- For architecture, flow, lifecycle, failure, configuration, diagram, and source-inclusion patterns, read [references/architecture-patterns.md](references/architecture-patterns.md).
- Before finalizing, apply [references/review-checklist.md](references/review-checklist.md).
