# Agent guidance audit checklist

Use this checklist before finalizing a generated or patched `AGENTS.md` guide, running an alignment review, or reviewing guide quality. Checklist results are read-only unless the user separately authorizes edits. Optional sections are not required merely because this checklist mentions them.

## Mode and output boundary

- [ ] The handoff states the selected mode: Draft, Patch, Audit, Alignment, or Checklist.
- [ ] The selected mode is the smallest useful mode for the request.
- [ ] Audit, Alignment, and Checklist report proposed changes without editing files or running fix commands.
- [ ] Generated or patched guidance does not copy the skill procedure, templates, pattern files, or checklist wholesale.
- [ ] Pattern examples were replaced with repository-specific guidance or omitted.
- [ ] Patch mode, including refactors, preserves useful repository facts, working agreements, and local style unless there is a concrete reason to change them.
- [ ] Recommendations, migrations, new dependencies, and new tools stay out of mandatory guide text unless evidenced or explicitly adopted by the maintainer; a request for proposals alone is not adoption.

## Repository evidence

- [ ] Workspace name and any named entry points are real.
- [ ] Every listed crate, package, app, example, docs surface, generated file, fixture, or `AGENTS.md` surface exists.
- [ ] Every listed path exists or is explicitly identified as an unverified detail in an evidence-limited handoff.
- [ ] Commands are evidenced by CI, runner files, manifests, or project docs; a request to adopt a policy is not proof that its command exists.
- [ ] Each validation command exists and fits its iteration or completion role.
- [ ] Project-specific claims are **Observed** facts or explicit requirements, or clearly justified **Inferred** practices.
- [ ] Intended policy is distinguished from implementation facts and successful execution.
- [ ] **Recommended** items are labeled as recommendations outside mandatory guide text unless explicitly adopted.
- [ ] Limited-evidence status is disclosed when full repository evidence was unavailable.
- [ ] Unresolved conflicts are listed instead of guessed away or resolved by silently weakening requirements.
- [ ] Target agent or platform compatibility was checked when a specific runtime was named or evidenced.
- [ ] Supported instruction filenames, discovery locations, precedence, merge behavior, and size limits are respected or marked `Not checked`.

## Standards inference

- [ ] Local standards are supported by explicit maintainer requirements, existing working agreements, or repeated or canonical repository practice.
- [ ] Ecosystem defaults were not treated as repository standards.
- [ ] Useful local implementation, error-handling, testing, security, compatibility, dependency, and review conventions are allowed even when unrelated to routing, synchronization, ownership, or validation.
- [ ] A working agreement is not rejected merely because it appears only in `AGENTS.md` or is not yet enforced in code or CI.
- [ ] An inferred practice does not override an explicit applicable requirement.
- [ ] New dependency, framework, package-manager, linter, formatter, type-checker, or testing-library suggestions are not presented as current policy unless evidenced or adopted.
- [ ] Existing package managers, test frameworks, type checkers, linters, and build tools are preserved unless migration was requested.

## Scope and readability

- [ ] The guide prioritizes useful commands, conventions, constraints, pitfalls, and completion requirements for its scope.
- [ ] Headings are shallow and stable; no fixed outline, audience taxonomy, workspace map, or decision flow is required.
- [ ] Rules are concrete enough to act on and review.
- [ ] Commands are in short bullets or command lists.
- [ ] Common commands appear early when useful, or the guide points early to an evidenced runner file when that is the canonical command list.
- [ ] A pointer to a command index does not replace instructions about when checks are required.
- [ ] The guide does not duplicate a canonical command index, `.gitignore`, ignored build/cache paths, or build-output inventory unless the item adds useful instructions or context.
- [ ] Root guidance is short enough to scan; the usual 220-line budget is a concision heuristic, not a target or completeness test.
- [ ] Known target-platform size or truncation limits are respected, or mitigation is identified.
- [ ] Long or path-specific procedures are moved closer to their scope without dropping essential inherited constraints or completion checks.
- [ ] Workspace maps, audience labels, decision flows, and synchronization sections are included only when they resolve real editing ambiguities.
- [ ] Manifests, lockfiles, dependency automation config, package-manager config, runner files, and CI files are used as evidence, not an inventory to reproduce.
- [ ] Useful working agreements about those files are preserved even when routine or documented only in `AGENTS.md`.
- [ ] Proposed removals have a concrete staleness, correctness, relevance, duplication, or supersession reason, not a categorical ban on file types or topics.
- [ ] Placeholder names, copied template paths, and unchecked links are removed from proposed output; repository edits remain subject to the selected mode.

## Synchronization and public contracts

- [ ] Documentation audience, when relevant, is established from purpose and readers, not from a README filename or mdBook renderer.
- [ ] Public workflow changes name the docs, examples, API references, generated outputs, fixtures, or `AGENTS.md` guidance that must sync when applicable.
- [ ] Internal implementation changes route first to their source evidence; existing maintainer books and design records remain synchronization targets when they describe the changed contract, without requiring new narrative docs for every change.
- [ ] Generated output changes route to generators, schemas, inventories, templates, snapshots, fixtures, and docs when present.
- [ ] High-risk or restricted edit surfaces supported by evidence or explicit requirements are named with short mechanical boundaries.
- [ ] Sync instructions use “when X changes, update Y” rather than vague “keep docs updated.”
- [ ] Relevant public contracts are identified when useful: CLI behavior, config shape, schemas, protocols, macro syntax, diagnostics, package exports, SDKs, plugin APIs, and public examples.

## Tests and validation

- [ ] Validation starts with focused checks, preserves all applicable completion requirements, and respects the selected mode's write boundary.
- [ ] Required broader tests, lint, formatting, security, compatibility, or PR gates are not removed merely because a narrower check passes or CI does not enforce them.
- [ ] CI-only checks are distinguished from required local checks; every CI job is not automatically turned into a local prerequisite.
- [ ] The guide does not claim successful validation when checks were not run or failed.
- [ ] Handoff wording distinguishes `Validated with`, `Attempted validation with`, `Reviewed only`, and `Not validated`.
- [ ] Failed checks include their failure summary and relationship to the change.
- [ ] Skipped required checks are named with reasons and remaining uncertainty.
- [ ] Snapshot, compile-fail, schema, generated-output, or type-level test guidance appears only when configured, repo-standardized, or explicitly requested; commands are verified separately.
- [ ] Broad full-workspace validation is not the default iteration step unless needed, but required broader checks remain completion gates.

## Handoff quality

- [ ] The final response names what changed or what was audited.
- [ ] Audit findings use Critical, Important, and Nice to have consistently.
- [ ] Validation performed and not performed are stated precisely.
- [ ] Remaining ambiguities are listed.
- [ ] Recommendations are separated from repository facts and adopted requirements.
- [ ] Next steps are directly actionable.

## Regression review scenarios

Use these hypothetical cases to review changes to this skill. They are expected behaviors, not instructions to copy into a repository's guide or claims that a behavioral evaluation ran.

| Input situation | Expected behavior |
| --- | --- |
| A maintainer requires an existing domain error type at public API boundaries, with no routing or docs-sync consequence. | Include the useful local coding convention; verify the type separately and do not claim all code already complies. |
| An existing `AGENTS.md` requires manifest and lockfile changes together or approval before adding dependencies; no other document repeats the rule. | Preserve the applicable working agreement rather than deleting it as routine or unsupported. |
| A focused package test passes, but repository guidance requires a broader suite before finishing. | Preserve and run the required broader checks when available; otherwise report them as skipped, not satisfied. |
| A small repository needs only setup commands, test instructions, and a few constraints. | Do not add a workspace map, audience taxonomy, decision flow, or documentation policy just to fill a template. |
| A user asks for suggestions about a new test library but has not adopted one. | Keep the suggestion outside mandatory guidance; do not invent an installed dependency or executable command. |
