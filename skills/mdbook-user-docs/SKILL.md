---
name: mdbook-user-docs
description: Creates, revises, and reviews en-US user-facing documentation in mdBook projects for products, CLIs, libraries, and APIs. Applies to tutorials, how-to guides, concepts, reference pages, migration guidance, and troubleshooting for end users. Excludes architecture and maintainer-only implementation documentation, which belongs in mdbook-internals.
---

# User-facing mdBook documentation

## Goal

Create accurate en-US documentation that helps users complete a task or understand public behavior with minimal cognitive load. Lead with the outcome, verify claims against authoritative sources, and include implementation detail only when it changes a user decision or observable result.

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

- Find the `book.toml` for the requested documentation. If the repository contains multiple books, use the one that contains the target chapter or serves the requested audience; do not edit every book by default.
- Read `[book].src` from `book.toml` and resolve it relative to the book root. Use `<book-src>/SUMMARY.md`; do not assume the source directory is `src/`.
- Read the target chapter, adjacent chapters, `SUMMARY.md`, and relevant repository or directory-level instructions.
- Inspect the current working tree or diff when available. Preserve unrelated edits and avoid broad formatting changes outside the requested scope.
- Work only on en-US content. Leave translated locale trees and translation catalogs unchanged.

### 2. Define the reader task and evidence

Before drafting, identify:

- **Reader:** who performs the task and what they already know.
- **Goal:** the observable result they need.
- **Starting state:** prerequisites, permissions, supported versions, and required context.
- **Success signal:** how they know the task worked.
- **Likely failure:** the one or two problems most worth preventing or diagnosing.

Verify behavior from the strongest available public sources: CLI help, API or configuration schemas, generated reference material, current tests, release notes, and working examples. Treat implementation details as evidence, not automatically as a supported public contract. When authoritative sources conflict, do not silently invent a resolution; document the supported behavior and flag the conflict in the completion report.

### 3. Choose one dominant page pattern

| Page type | Reader question | Default structure |
|---|---|---|
| How-to | How do I accomplish this? | Outcome, prerequisites, steps, verification, troubleshooting |
| Tutorial | Can you teach me through a working path? | Goal, progressive steps, checkpoints, result, next step |
| Concept | How should I understand this behavior? | Summary, mental model, boundaries, implications, related tasks |
| Reference | What exactly is supported? | Definition, syntax, fields or options, examples, errors, compatibility |
| Troubleshooting | Why did this fail, and what should I do? | Symptom, distinguishable cause, action, verification |
| Migration | What changed, and how do I adapt? | Impact, before and after, required actions, compatibility, rollback |

Split or link a focused page when a chapter serves unrelated reader jobs.

### 4. Apply the implementation-detail gate

Include an internal mechanism only when it:

1. Changes what the user must do or choose.
2. Explains observable behavior, ordering, timing, limits, or compatibility.
3. Is needed to diagnose or recover from a failure.
4. Has a security, privacy, reliability, or performance consequence.
5. Is part of a documented public contract or extension point.

State the user impact first, then include only the mechanism needed to explain that impact. Otherwise, omit the detail or link to maintainer documentation.

### 5. Write for action, scanning, and en-US consistency

- Put the answer, outcome, or required action in the opening paragraph.
- Use sentence-case headings, active voice, concrete verbs, and one main idea per paragraph.
- Use American spelling and punctuation in original prose, including `behavior`, `modeling`, and `acknowledgment`.
- Preserve commands, flags, API identifiers, source symbols, UI strings, product names, and quoted text exactly; do not “correct” a public literal for house style.
- Reuse the book's established terminology. Do not rename a public concept solely to enforce stylistic consistency.
- Put prerequisites before steps. Use ordered lists only for required sequence and bullets for unordered facts.
- Make examples minimal, safe, and copyable. Use documented public service URLs, including production API base URLs, when they are part of the supported workflow. Use obvious placeholders such as `<PROJECT_ID>` for user-specific values; never include live credentials, tokens, personal data, or sensitive deployment URLs.
- Give code fences accurate language identifiers and enough execution context to remove ambiguity.
- Show expected output only when it is a useful verification signal.
- Remove throat-clearing, repetition, marketing language, and minimizing words such as “easy,” “simple,” “just,” or “obvious.”

### 6. Use mdBook features safely

- Inspect `book.toml`, repository build wrappers, and dependency or CI configuration before using preprocessors or version-sensitive syntax.
- The built-in `links` and `index` preprocessors run by default unless `[build].use-default-preprocessors = false`. Confirm that `links` is active before using `{{#include ...}}`.
- Resolve include paths relative to the chapter that contains the include. Prefer named anchors over line ranges and keep each included region focused.
- Do not add Mermaid, admonitions, math, link-checking syntax, raw HTML, or another build dependency unless the project already supports it or the user requested the configuration change.
- Use a diagram only when it explains branching, interaction, state, or relationships more clearly than prose. Add a textual conclusion, and give images meaningful alt text.
- Preserve the valid structure of `SUMMARY.md`. Use relative `.md` links for chapters when practical, one level-one heading per chapter unless the book deliberately differs, and no skipped heading levels.
- When adding, moving, or renaming a chapter, update `SUMMARY.md` and affected inbound links intentionally. Avoid URL-changing renames unless they are required.

### 7. Review structure without mechanical limits

Use length and visual density as diagnostics, not quality scores. Review long paragraphs for mixed claims, long lists for a real grouping, long procedures for natural phases and checkpoints, and wide tables or large diagrams for multiple purposes. Read [references/writing-toolkit.md](references/writing-toolkit.md) when a structural choice is not obvious.

### 8. Validate and inspect the final diff

Apply the request mode above to every check. Report a check as successful only
when it ran successfully; distinguish failed attempts, static review, and checks
not run. In Review mode, inspect the original checkout for accidental changes.

1. Check `[build].create-missing` before builds or `mdbook test`. It defaults to
   `true`, so loading the book can create missing chapters from `SUMMARY.md`.
   In Edit mode, create or rename authorized files explicitly, then set
   `MDBOOK_BUILD__CREATE_MISSING=false` for each validation process. Confirm that
   wrappers preserve this override. Report remaining missing chapters as
   follow-ups; validation must not create files outside the requested scope.
   If the override cannot be honored, validate in a safely isolated copy or
   report static review. In Review mode, report missing chapters and build only
   in a safely isolated copy as required above.
2. Build with the repository wrapper or `mdbook build <book-root>`.
3. Run the repository's Markdown, link, spelling, and style checks.
4. Run `mdbook test <book-root>` only for testable Rust snippets. Validate other languages with project-native tooling.
5. Confirm navigation, local links, code fences, tables, images, and configured diagrams render correctly. Inspect normal content width for overflow and copyability.
6. Re-read from the user's perspective and remove detail that does not affect action or understanding.
7. Inspect the final diff for unrelated edits, accidental files, stale links, and unresolved placeholders.

Report the selected mode, changed or reviewed chapters, evidence consulted,
validation performed (including any isolated copy), and checks that failed or
could not run. Do not describe static review as a successful build or test.

## Resources

- For a new chapter, start from [assets/chapter-template.md](assets/chapter-template.md) and replace or remove every placeholder and unused section.
- For page patterns, tables, diagrams, callouts, includes, and structural review, read [references/writing-toolkit.md](references/writing-toolkit.md).
- Before finalizing, apply [references/review-checklist.md](references/review-checklist.md).
