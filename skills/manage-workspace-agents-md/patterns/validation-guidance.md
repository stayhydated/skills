# Validation guidance patterns

These patterns are source material. Do not copy commands into generated `AGENTS.md` unless the repository defines them.

## Validation and editing rules pattern

Use language like this after inspecting repository evidence and applicable completion requirements:

<!-- EXAMPLE ONLY: include only validation rules and commands backed by repository evidence or explicit requirements; verify commands separately. -->

```md
## Validation and Editing Rules

### Validation After Changes

- Start with focused checks for the affected package, crate, docs, example, generator, fixture, or public surface.
- Before finishing, run the checks required by applicable repository guidance, including broader checks where required. Focused checks do not replace required completion checks.
- Use repository runner recipes only when they exist and fit the changed surface.
- If validation cannot be run, state why, which required checks were skipped, and what remains unvalidated.
- Report the scope of successful checks without treating a focused test as proof of the entire change.
```

Separate iteration checks from completion requirements. Preserve explicit local test, lint, formatting, security, compatibility, and PR gates even when CI does not enforce them. Use CI as evidence of automated merge checks, not as permission to weaken additional maintainer requirements. Do not turn every CI job into a local prerequisite without checking the intended contributor workflow.

Respect the selected mode's write boundary. In Audit, Alignment, and Checklist modes, report required fix or generation commands without running them; use read-only equivalents when available and disclose remaining checks.

## Validation examples by change type

Use these examples to choose evidence to look for. Do not copy commands unless evidenced. These are starting checks, not replacements for required completion checks.

| Change type | Prefer starting with |
| --- | --- |
| One package or crate | Package-specific check or test command if evidenced. |
| README-only change | Docs lint, link check, doctest, or reviewed-only handoff if no command exists. |
| Generated output | Generator command plus focused diff or snapshot review, only when the command exists and edits are authorized. |
| Snapshot update | Targeted test plus snapshot review command if configured and permitted by the selected mode. |
| Public CLI behavior | Focused CLI, integration, or documented example command if evidenced. |
| Public API or schema change | Focused API, schema, type, or compatibility check if evidenced. |
| Guide-only change | Markdown lint or static review, depending on repository tooling. |

## Validation wording

The canonical validation contract lives in `SKILL.md`. Handoffs and templates must use the exact applicable line or lines from that section rather than synonyms or local rewrites. This file explains how to choose evidence and validation scope; it is not the source of truth for the wording itself.

## Avoid false certainty

- Do not say `passes`, `tested`, `validated`, or `works` for static review.
- Do not default to broad full-workspace validation during iteration, but never drop required broader checks to save effort or shorten a guide.
- Do not invent formatting, linting, type-checking, snapshot, or regeneration commands.
- When a runner file or command index exists, such as `justfile`, `Makefile`,
  `Taskfile.yml`, or package scripts, inspect it before choosing or documenting
  validation commands.
- Distinguish checks run locally, checks run by CI, failed attempts, and required checks not run. Report success only for the scope actually checked.
