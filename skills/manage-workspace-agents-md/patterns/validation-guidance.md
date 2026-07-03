# Validation guidance patterns

These patterns are source material. Do not copy commands into generated `AGENTS.md` unless the repository defines them.

## Validation and editing rules pattern

Use language like this after inspecting repository evidence:

<!-- EXAMPLE ONLY: include only validation rules and commands backed by repository evidence. -->

```md
## Validation and Editing Rules

### Validation After Changes

- Run the narrowest command that proves the edited behavior works for the affected package, crate, docs, example, generator, fixture, or public surface.
- Prefer targeted checks before full-workspace validation.
- Use repository runner recipes only when they exist and fit the changed surface.
- If validation cannot be run, state why and what remains unvalidated.
- Do not claim a change works unless it was validated or the remaining risk is explicitly documented.
```

## Validation examples by change type

Use these examples to choose evidence to look for. Do not copy commands unless evidenced.

| Change type | Prefer validating with |
| --- | --- |
| One package or crate | Package-specific check or test command if evidenced. |
| README-only change | Docs lint, link check, doctest, or reviewed-only handoff if no command exists. |
| Generated output | Generator command plus focused diff or snapshot review, only when the command exists. |
| Snapshot update | Targeted test plus snapshot review command if configured. |
| Public CLI behavior | Focused CLI, integration, or documented example command if evidenced. |
| Public API or schema change | Focused API, schema, type, or compatibility check if evidenced. |
| Guide-only change | Markdown lint or static review, depending on repository tooling. |

## Validation wording

The canonical validation contract lives in `SKILL.md`. Handoffs and templates must use the exact applicable line or lines from that section rather than synonyms or local rewrites. This file explains how to choose evidence and validation scope; it is not the source of truth for the wording itself.

## Avoid false certainty

- Do not say `passes`, `tested`, `validated`, or `works` for static review.
- Do not use broad full-workspace validation as the default when a narrower command proves the change.
- Do not invent formatting, linting, type-checking, snapshot, or regeneration commands.
- When a runner file or command index exists, such as `justfile`, `Makefile`,
  `Taskfile.yml`, or package scripts, inspect it before choosing or documenting
  validation commands.
