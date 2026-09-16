---
name: manage-workspace-agents-md
description: Create, update, refactor, audit, or align repository AGENTS.md working instructions from repository evidence and explicit maintainer requirements without inventing tools or policies.
---

# manage-workspace-agents-md

Create or improve concise, scoped root or nested `AGENTS.md` working instructions for a software workspace. Help coding agents work correctly with useful commands, conventions, constraints, pitfalls, completion requirements, and pointers to further guidance, without turning the guide into a policy dump, language handbook, or generic best-practices essay.

Treat these instructions as a procedure, not as content to copy wholesale into a generated guide.

## Operating principle

Optimize for agent usability: concrete rules, stable headings, shallow hierarchy, and the smallest set of useful instructions for the scope. Capture verified repository facts and explicit maintainer requirements. The skill may infer repository standards from evidence, but it must not invent preferred dependencies, package managers, frameworks, test libraries, or language-specific coding styles. Existing local coding conventions and working agreements are valid guidance in their own right; they need not concern routing or documentation synchronization. Treat well-documented code, tests, executable examples, schemas, fixtures, and generator inputs as evidence for internal behavior. Point to those surfaces and existing maintainer documentation when useful rather than requiring new narrative documents for implementation rationale.

A strong guide helps answer the questions relevant to its scope:

1. How should an agent set up, run, and work in the project?
2. Which conventions, constraints, and known pitfalls affect correct changes?
3. Where should an agent start, and which local guidance applies?
4. Which related docs, examples, tests, generated files, or fixtures must change together when applicable?
5. Which focused checks help during iteration, and which checks and completion requirements must be satisfied before finishing?

## Decision kernel

1. Choose the smallest mode that answers the request.
2. Inspect repository evidence and explicit maintainer requirements before making repository-specific claims.
3. Add only **Observed** facts or requirements, or clearly justified **Inferred** practices, to generated guidance.
4. Keep **Recommended** tools, dependencies, migrations, commands, frameworks, and workflow changes out of mandatory guide text unless explicitly adopted by the maintainer.
5. Preserve existing local standards and working agreements instead of replacing them with ecosystem preferences.
6. Keep root guidance short; split path-specific procedures into nested `AGENTS.md` or local docs.
7. Start validation narrowly without weakening applicable completion checks; never imply commands were run when they were only reviewed.

## Out of scope

- General repository documentation unrelated to contributor or coding-agent guidance.
- Authoring release notes, README content, troubleshooting guides, or other non-`AGENTS.md` documents.
- Generic language, framework, or ecosystem best-practices guidance.
- Unsolicited recommendations for new dependencies, package managers, test frameworks, type checkers, formatters, linters, or runtime frameworks.
- Writing or implementing code-level tests. Use a dedicated testing or language skill when available; for Rust test work, use `rust-test` when available.
- Creating, updating, auditing, or aligning `.agents/` directories, `.agents/*` guidance, skill files, custom-agent files, or persona files.
- Overriding local subtree guidance when a nested `AGENTS.md` file owns that scope.

## Reference map

Supporting files are source material and handoff scaffolding, not default output:

- `templates/mode-handoffs.md`: final response shapes for Draft, Patch/refactor, Audit, Alignment, and Checklist modes.
- `patterns/evidence-and-scope.md`: evidence gates, claim classification, conflict precedence, limited-evidence handling, and placeholder hygiene.
- `patterns/standards-inference.md`: how to capture maintainer requirements and infer repository standards without recommending new tools.
- `patterns/agent-guide-layout.md`: optional guide sections, audience labels, sync rules, workspace map patterns, size budget, and split triggers.
- `patterns/validation-guidance.md`: validation wording, focused checks, required completion checks, and validation-by-change-type routing.
- `checklists/agent-guide-audit.md`: final checklist for generated or patched `AGENTS.md` guidance.

Before using any example or bullet from a reference file, replace it with repository-specific guidance or omit it. Before final output, search for copied pattern headings, placeholder paths, and example commands from `patterns/*` or `templates/*`; remove or replace anything not backed by repository or artifact evidence or explicit maintainer requirements.

## Output modes

Always identify the selected mode in one sentence before the main output.

| User asks for | Choose | Output |
| --- | --- | --- |
| “Create an `AGENTS.md`” and no suitable guide exists | Draft | A complete guide based on repository evidence and explicit maintainer requirements. |
| “Improve this file,” “clean this up,” or “make targeted edits” | Patch | Minimal edits; preserve structure unless structure is the problem. |
| “Refactor,” “reorganize,” or “make this guide easier to scan” | Patch | Structural edits that preserve repository facts and working agreements; verify changed or newly emphasized claims. |
| “Review,” “is this good,” or “could it be improved?” | Audit | Findings by severity with concrete fixes. |
| “Compare against the repo,” “is this stale,” or “does this match CI/docs?” | Alignment | Drift report against repository truth and intended policy. |
| “Apply the checklist” | Checklist | Pass/fail/not-checked notes. |

Audit, Alignment, and Checklist are read-only modes. Report proposed removals, replacements, and synchronization changes without editing repository files. Use read-only checks or dry runs; do not run formatters, generators, or fix commands that change files in these modes. Apply changes only when the user authorizes Draft or Patch. This boundary governs removal and synchronization rules in every supporting file, even when a defect and its fix are obvious.

For existing `AGENTS.md` artifacts, treat “check,” “review,” “is this good,” and “could this be improved” as Audit. Treat “edit,” “apply,” “rewrite,” “patch,” or “update the file” as Patch, except that “apply the checklist” selects the read-only Checklist mode. If a prompt says “improve” without an edit verb, prefer Audit unless the user clearly expects changed `AGENTS.md` content.

Ambiguous prompt examples:

- “Improve this AGENTS.md” -> Patch.
- “Could this AGENTS.md be improved?” -> Audit.
- “Apply your recommended improvements” -> Patch.

If the user provides only an `AGENTS.md` guide and no repository, perform an evidence-limited Audit or Patch. Preserve explicit working agreements, but do not invent repository commands, paths, manifests, sync rules, or validation claims.

In Patch mode, make the file changes when repository write access is available. If edits cannot be applied directly, provide a unified diff or exact replacement sections before the handoff. Do not provide only advisory findings unless the selected mode is Audit.

When the request is ambiguous, choose the smallest useful mode. Prefer Audit for existing guidance unless the user clearly asked for edits.

### Severity definitions for audits

- **Critical**: likely to cause wrong edits, false validation claims, broken commands, stale public-contract guidance, weakened required checks, or unsafe repository-wide assumptions.
- **Important**: likely to slow agents down, duplicate guidance, obscure ownership, drop useful working agreements, miss required docs/tests synchronization, or overstate recommendations as repository truth.
- **Nice to have**: improves readability, routing, concision, examples, or handoff quality without materially changing correctness.

## Required workflow

1. **Select mode.** Declare Draft, Patch, Audit, Alignment, or Checklist.
2. **Inspect evidence and requirements.** For `AGENTS.md` work, use `patterns/evidence-and-scope.md`.
3. **Check agent platform compatibility when applicable.** If the request or repository names a target agent runtime, use `patterns/evidence-and-scope.md` to check supported instruction filenames, discovery locations, precedence, merge behavior, and size limits before choosing where `AGENTS.md` guidance belongs. Do not assume one agent platform reads another platform's files.
4. **Capture local standards.** Use `patterns/standards-inference.md` to classify facts, requirements, and local conventions as **Observed**, **Inferred**, or **Recommended**.
5. **Use reference files selectively.** Layout, evidence, validation, and checklist files are inputs to judgment, not sections to paste.
6. **Draft, patch, audit, align, or checklist-review.** Keep changes as small as the selected mode allows; reporting modes do not apply edits.
7. **Check for support-file leakage.** Before final output, search for copied pattern headings, placeholder paths, example commands, and fenced examples from `patterns/*` or `templates/*` that are not supported by evidence or explicit requirements. Remove or replace them in the proposed output, or in repository files only when editing is authorized.
8. **Validate or disclose.** Run available checks that fit the scope and mode, including required completion checks, then use exact validation wording in the handoff. Disclose any required check that could not be run.

## Evidence floor

Match evidence depth to the selected mode:

- **Draft and Alignment:** inspect explicit maintainer requirements and broad repository evidence, including the top-level tree, existing root and nested `AGENTS.md` guidance, README/contributor docs, manifests, lockfiles, workspace config, package metadata, CI workflows, runner files, and named docs/examples/tests/generated surfaces.
- **Patch:** inspect the guide being patched, applicable local guidance and maintainer requirements, and evidence for every changed or added command, path, convention, constraint, ownership claim, synchronization rule, or validation claim.
- **Audit:** inspect enough repository evidence and stated requirements to verify or challenge the artifact's claims. If coverage is incomplete, mark findings evidence-limited.
- **Checklist:** inspect only the files and requirements needed to check each item. Mark unsupported items `Not checked`.
- **Platform compatibility:** when a target agent or runtime is named, inspect evidenced platform rules for instruction discovery, local precedence, override behavior, and size limits. Mark uninspected platform behavior as `Not checked`.

For large repositories, inspect every named surface and representative evidence for each claimed workflow category. If full coverage is impractical, state what was inspected, mark remaining areas as not checked, and avoid repository-wide claims for uninspected categories.

## Output boundaries

Generated guidance should contain repository-specific working instructions and useful context for coding agents. Commands, local coding conventions, testing expectations, security boundaries, dependency policies, PR requirements, and known pitfalls are all valid when supported by repository evidence or explicit maintainer requirements. Add maps, audience labels, and synchronization sections only when they resolve a real editing ambiguity. Do not copy this skill's procedure, output mode rules, evidence gates, templates, pattern files, or checklists into `AGENTS.md` unless a point is directly relevant to repository work and supported.

Do not add `AGENTS.md` guidance that only restates `.gitignore`, ignored build or cache directories, dependency caches, or tool output directories. Treat ignored outputs as non-source by default; mention them when a repository-owned workflow or explicit working agreement makes the path relevant to editing, synchronization, cleanup, or validation.

Evaluate each instruction by its usefulness, not its file category: does it help an agent perform the task, avoid a known mistake, or satisfy a repository requirement? Use manifests, lockfiles, dependency automation config, package-manager config, runner files, and CI as evidence, not as an inventory to reproduce. Include concise guidance about these surfaces when it answers that test, even when the procedure is routine. Do not remove an explicit lockfile, dependency, or CI working agreement merely because it is ordinary or documented only in `AGENTS.md`. In Patch mode, remove or shorten stale, misleading, irrelevant, or redundant text with a concrete reason; in Audit, Alignment, and Checklist modes, report the proposed change without editing files.

When repository evidence or explicit maintainer requirements identify high-risk edit surfaces, include a short boundary note: generated outputs should be changed through their generator when one exists; release, deployment, migration, or production configuration files may require extra approval when local guidance says so; secrets, credentials, and vendored dependencies must not be edited as ordinary source. Do not invent restricted areas.

Prefer a root `AGENTS.md` under 220 lines as a concision heuristic, not a required format, target length, or completeness test. Split into nested `AGENTS.md` or local docs when a procedure is path-specific, generated-output-heavy, release-only, troubleshooting-oriented, or longer than a short checklist. If the target agent has an evidenced instruction-size or truncation limit, keep the applicable guidance under that limit or split path-specific material closer to the files it governs. Preserve essential requirements when shortening a guide.

## Terminology boundary

Use these terms narrowly:

- `AGENTS.md`: durable repository or subtree working instructions and context for coding agents in that scope. Root and nested `AGENTS.md` files are the only agent-guidance artifacts in scope.
- Non-`AGENTS.md` agent artifacts, including skill directories, custom-agent files, persona files, plugins, MCP servers, tool integrations, scripts, and executable helpers, are out of scope.

Do not collapse these surfaces into one file type merely because the word “agent” appears in a path.

## Standards guidance boundary

Describe standards only when they are repository standards or explicitly requested maintainer requirements:

- **Observed:** directly evidenced by existing files, commands, tests, docs, manifests, CI, local guidance, or an explicit maintainer instruction. An instruction is evidence of intended policy, not proof that code implements it or that a command exists or succeeds.
- **Inferred:** strongly implied by repeated local practice, workspace structure, or public entry points; state narrowly and do not override explicit requirements with inferred practice.
- **Recommended:** plausible but not current repository truth or an adopted requirement; keep out of mandatory guide text unless the maintainer explicitly adopts it. A request for suggestions alone is not adoption.

Do not add language-specific rules merely because a language appears in the repository. Include useful evidenced or explicitly requested local conventions for implementation, error handling, testing, compatibility, security, dependencies, and review, even when they do not affect routing, ownership, synchronization, or validation. `AGENTS.md` may itself establish a maintainer's working agreement; it need not repeat a policy already documented elsewhere.

For detailed code-level advice, use the relevant dedicated skill when available. Do not omit concise repository-specific conventions because another skill covers the language. Point to existing repository guidance or a real `AGENTS.md` path when that avoids duplicating detailed material.

## Validation wording

Use these exact distinctions for validation wording in final handoffs and templates:

- `Validated with: <command>` only when the command or check ran successfully; state only the scope it actually covered.
- `Attempted validation with: <command>` when the command ran but failed; include the failure summary and whether it appears related to the change.
- `Reviewed only; not executed because: <reason>` for static review without execution.
- `Not validated; missing repo access / command unavailable / outside requested scope: <reason>` when validation was not possible or not attempted.

Do not say `validated`, `tested`, `passes`, or `works` for changes that were only inspected. Execution alone is not successful validation.

## Final response format

Use the appropriate template from `templates/mode-handoffs.md`. For small requests, compact the handoff while preserving selected mode, changed or audited scope, validation wording, missing evidence, and actionable next steps. Include:

- selected mode;
- what changed or what was audited;
- important assumptions;
- validation performed with exact wording;
- validation not performed and why;
- unresolved conflicts or missing evidence;
- directly actionable next steps, only when useful.
