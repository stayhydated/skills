# Agent guidance audit checklist

Use this checklist before finalizing a generated or patched `AGENTS.md` guide, running an alignment review, or reviewing guide quality.

## Mode and output boundary

- [ ] The handoff states the selected mode: Draft, Patch, Audit, Alignment, or Checklist.
- [ ] The selected mode is the smallest useful mode for the request.
- [ ] Generated or patched guidance does not copy the skill procedure, templates, pattern files, or checklist wholesale.
- [ ] Pattern examples were replaced with repository-specific guidance or omitted.
- [ ] Patch mode, including refactors, preserves existing repository facts and local style unless the structure itself is defective.
- [ ] Recommendations, migrations, new dependencies, and new tools stay out of mandatory guide text unless evidenced or explicitly requested.

## Repository evidence

- [ ] Workspace name and default entry points are real.
- [ ] Every listed crate, package, app, example, docs surface, generated file, fixture, or `AGENTS.md` surface exists.
- [ ] Every listed path exists.
- [ ] Commands are evidenced by CI, runner files, manifests, or project docs.
- [ ] Each validation command exists and fits the changed surface.
- [ ] Project-specific claims are **Observed** or clearly justified **Inferred**.
- [ ] **Recommended** items are labeled as recommendations outside mandatory guide text.
- [ ] Limited-evidence status is disclosed when full repository evidence was unavailable.
- [ ] Unresolved conflicts are listed instead of guessed away.
- [ ] Target agent or platform compatibility was checked when a specific runtime was named or evidenced.
- [ ] Supported instruction filenames, discovery locations, precedence, merge behavior, and size limits are respected or marked `Not checked`.

## Standards inference

- [ ] Local standards were inferred from repeated repository practice, documented workflows, CI, manifests, tests, examples, or local guidance.
- [ ] Ecosystem defaults were not treated as repository standards.
- [ ] Language-specific rules appear only when they affect repository-specific routing, synchronization, ownership, or validation.
- [ ] New dependency, framework, package-manager, linter, formatter, type-checker, or testing-library suggestions are not presented as current policy unless evidenced or requested.
- [ ] Existing package managers, test frameworks, type checkers, linters, and build tools are preserved unless migration was requested.

## Scope and readability

- [ ] The first third of the guide answers where to start, how to classify changes, and what must sync.
- [ ] Headings are shallow and stable.
- [ ] Rules are concrete enough to verify.
- [ ] Commands are in short bullets or command lists.
- [ ] Common commands appear early only when they are evidenced and useful for
  most edits, or the guide points early to an evidenced runner file such as
  `justfile` when that is the canonical command list.
- [ ] The guide does not duplicate a canonical command index, `.gitignore`, or
  build-output inventory unless the duplicated item adds routing or validation
  value.
- [ ] Root guidance is short enough to scan, usually under 220 lines.
- [ ] Known target-platform size or truncation limits are respected, or local procedures are split closer to the files they govern.
- [ ] Long or path-specific procedures are moved to nested `AGENTS.md` or local docs.
- [ ] Workspace map entries describe owned editing surfaces, not every manifest or workflow file.
- [ ] Placeholder names, copied template paths, and unchecked links are removed.

## Synchronization and public contracts

- [ ] Public workflow changes name the docs, examples, API references, generated outputs, fixtures, or `AGENTS.md` guidance that must sync.
- [ ] Internal implementation changes are not routed to standalone narrative docs; source-of-truth expectations stay in code, tests, examples, fixtures, schemas, generated-source inputs, and existing `AGENTS.md` guidance.
- [ ] Generated output changes route to generators, schemas, inventories, templates, snapshots, fixtures, and docs.
- [ ] Evidence-backed high-risk or restricted edit surfaces are named with short mechanical boundaries.
- [ ] The guide uses “when X changes, update Y” rather than vague “keep docs updated.”
- [ ] Public contracts are identified when present: CLI behavior, config shape, schemas, protocols, macro syntax, diagnostics, package exports, SDKs, plugin APIs, and public examples.
- [ ] For Rust crates or workspaces with evidenced versions below `1.0.0`, breaking API changes are routed as updates to the current API shape in code, tests, examples, docs, fixtures, generated outputs, and guidance, not as legacy/backward-compatibility obligations unless local docs explicitly require them.

## Tests and validation

- [ ] Validation rules require the narrowest proving command.
- [ ] The guide does not claim validation when checks were not run.
- [ ] Handoff wording distinguishes `Validated with`, `Reviewed only`, and `Not validated`.
- [ ] Snapshot, compile-fail, schema, generated-output, or type-level test guidance appears only when configured, repo-standardized, or explicitly requested.
- [ ] Broad full-workspace validation is not the default unless it is the smallest reliable proof.

## Handoff quality

- [ ] The final response names what changed or what was audited.
- [ ] Audit findings use Critical, Important, and Nice to have consistently.
- [ ] Validation performed and not performed are stated precisely.
- [ ] Remaining ambiguities are listed.
- [ ] Recommendations are separated from repository facts.
- [ ] Next steps are directly actionable.
