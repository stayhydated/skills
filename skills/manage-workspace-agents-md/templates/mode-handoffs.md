# Mode handoff templates

Use the smallest template that fits the selected mode. These templates shape the final handoff only; keep them out of generated `AGENTS.md` files. For small requests, compact the template while preserving selected mode, changed or audited scope, validation wording, missing evidence, and actionable next steps.

For validation sections, use only the lines that apply. If a section has no content, write `None.` rather than leaving placeholder bullets. Insert the exact applicable validation line or lines from `SKILL.md` > `Validation wording`; do not restate the validation contract in this template.

## Draft mode handoff

```md
Mode: Draft

## Created

- File or section created:
- Basis:

## Evidence used

- Observed:
- Inferred:

## Assumptions

-

## Validation

- <Exact applicable validation line from `SKILL.md` > `Validation wording`.>

## Remaining uncertainties

-

## Next steps

-
```

## Patch mode handoff

Use this for targeted edits and refactors that preserve repository facts. If edits were applied, include changed file paths. If edits could not be applied directly, include a unified diff or exact replacement sections before this handoff.

```md
Mode: Patch

## Changed

-

## Patch summary

-

## Evidence for project-specific changes

-

## Assumptions

-

## Validation

- <Exact applicable validation line from `SKILL.md` > `Validation wording`.>

## Remaining uncertainties

-

## Next steps

-
```

## Audit mode handoff

Use an artifact-specific title such as `# AGENTS.md Audit` or `# Agent Guidance Audit`.

If a severity has no findings, write `None found.` Do not leave placeholder bullets in the final response.

```md
Mode: Audit

# Agent Guidance Audit

## Critical

- Finding:
  Evidence:
  Risk:
  Fix:

## Important

- Finding:
  Evidence:
  Risk:
  Fix:

## Nice to have

- Finding:
  Evidence:
  Risk:
  Fix:

## Evidence coverage

- Scope:
- Confidence:

## Assumptions

-

## Validation

- <Exact applicable validation line from `SKILL.md` > `Validation wording`.>

## Missing evidence or unresolved conflicts

-

## Next steps

-
```

## Alignment mode handoff

```md
Mode: Alignment

## Drift found

- Area:
  Guide says:
  Repository evidence says:
  Fix:

## No drift found

-

## Evidence inspected

-

## Assumptions

-

## Not checked

-

## Validation

- <Exact applicable validation line from `SKILL.md` > `Validation wording`.>

## Next steps

-
```

## Checklist mode handoff

```md
Mode: Checklist

## Pass

-

## Fail

- Item:
  Evidence:
  Fix:

## Not applicable

-

## Not checked

- Item:
  Reason:

## Assumptions

-

## Validation

- <Exact applicable validation line from `SKILL.md` > `Validation wording`.>
```
