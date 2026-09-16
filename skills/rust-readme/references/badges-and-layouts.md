# Badge markup and README layouts

These are authoring templates, not files to paste unchanged. Resolve every
uppercase placeholder from repository evidence; remove unavailable badges,
unused definitions, optional headings, and instructional comments. The final
README must contain no template placeholders or absence inventory.

## Canonical badge paragraph

Use one badge per source line, without blank lines between them:

```markdown
[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io][crate-badge]][crate]
```

Reference definitions belong at the bottom of the README. The following URL
shapes illustrate a GitHub-hosted project; use actual provider-generated values
and percent-encode branch names and query parameters as needed:

```markdown
[ci-badge]: https://github.com/OWNER/REPO/actions/workflows/WORKFLOW.yml/badge.svg?branch=BRANCH
[ci]: https://github.com/OWNER/REPO/actions/workflows/WORKFLOW.yml
[codecov-badge]: https://codecov.io/gh/OWNER/REPO/branch/BRANCH/graph/badge.svg
[codecov]: https://codecov.io/gh/OWNER/REPO
[book-badge]: https://img.shields.io/badge/Book-mdBook-blue
[book]: https://VERIFIED-BOOK-HOST/VERIFIED-BOOK-ROOT/
[crate-badge]: https://img.shields.io/crates/v/PACKAGE.svg
[crate]: https://crates.io/crates/PACKAGE
```

The workflow filename need not be `ci.yml`; never rename a workflow to fit the
example. Confirm the Codecov URL and any provider-generated public badge token
against the actual project. Do not copy upload credentials into Markdown.
The static Book badge says where to read, not whether documentation CI passes.
Do not use mdBook's own crate badge in place of the target project's book.

Keep image/target pairs together when moving badges. Test both: an image that
renders successfully can still link to a wrong package, branch, or workflow.
Badge status may legitimately be failing; an error/unknown SVG from a fabricated
backend is not a useful status badge.

## Single-package layout

```markdown
# PACKAGE

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io][crate-badge]][crate]

PACKAGE provides CONCRETE-CAPABILITY for INTENDED-READER.

## Overview

ONLY-DISTINCTIVE-VERIFIED-CAPABILITIES-OR-CONSTRAINTS

## Example

ONE-SHORT-MAINTAINED-USAGE-EXAMPLE-WITHOUT-INSTALLATION-STEPS

## Contributing

See the [contribution guide][contributing].
```

Add only the definitions actually used, including a verified `contributing`
destination. Omit Overview, Example, or Contributing when they are unnecessary.
Do not add a Documentation section that repeats the Book badge, or an
Installation section to fill the gap when there is no book.

## Workspace root with an established primary package

Use the same opening and badge paragraph, but make the visible crates.io label
and alt text identify the primary package. For example, after the Book badge:

```markdown
[![crates.io: PRIMARY-PACKAGE][primary-badge]][primary]
```

```markdown
[primary-badge]: https://img.shields.io/crates/v/PRIMARY-PACKAGE.svg?label=PRIMARY-PACKAGE
[primary]: https://crates.io/crates/PRIMARY-PACKAGE
```

Describe the overall project, then add a `Crates` table when choosing a member
needs explanation. Other package routes belong in that table or their member
READMEs, not in an ambiguous workspace-version badge.

## Workspace root without a primary package

All named registry badges form one final group, sorted by exact package name:

```markdown
# PROJECT

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io: PACKAGE-A][package-a-badge]][package-a]
[![crates.io: PACKAGE-B][package-b-badge]][package-b]

PROJECT provides SHARED-CAPABILITY. Choose the crate for your use case below.

## Crates

| Crate | Purpose | Source |
| --- | --- | --- |
| `PACKAGE-A` | VERIFIED-PURPOSE-A | [README][package-a-readme] |
| `PACKAGE-B` | VERIFIED-PURPOSE-B | [README][package-b-readme] |

## Contributing

See the [contribution guide][contributing].
```

Use these additional definition shapes, with actual package names and routes:

```markdown
[package-a-badge]: https://img.shields.io/crates/v/PACKAGE-A.svg?label=PACKAGE-A
[package-a]: https://crates.io/crates/PACKAGE-A
[package-a-readme]: https://github.com/OWNER/REPO/blob/BRANCH/MEMBER-A/README.md
[package-b-badge]: https://img.shields.io/crates/v/PACKAGE-B.svg?label=PACKAGE-B
[package-b]: https://crates.io/crates/PACKAGE-B
[package-b-readme]: https://github.com/OWNER/REPO/blob/BRANCH/MEMBER-B/README.md
```

Do not add a registry link for the workspace name itself. Link an internal tool
row to its actual source if it belongs in the overview, without a crates.io
badge. If a member lacks a README, link to its existing source directory rather
than inventing a file. Do not repeat badge images inside the table.

## Workspace member layout

```markdown
# MEMBER-PACKAGE

[![Workspace CI][ci-badge]][ci]
[![Workspace Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io: MEMBER-PACKAGE][crate-badge]][crate]

MEMBER-PACKAGE provides MEMBER-CAPABILITY within [PROJECT][project].

## Example

ONE-SHORT-MEMBER-SPECIFIC-USAGE-EXAMPLE
```

Use member-specific CI/coverage labels instead when genuinely measured per
member. Reuse the workspace Book badge only when that book serves this member's
users. The crates.io image, visible label, and destination must all name this
member, not the primary package. A short verified API-reference link can provide
an additional route without introducing a docs.rs badge.

## Acceptance cases

Use these cases to review an authored README and to exercise future automation.
They describe expected behavior, not permission to create external services.

| Evidence | Expected README behavior |
| --- | --- |
| Single package; all four backends verified | Title, CI, Codecov, Book, crates.io, then description. |
| Published package; CI and crates.io only | CI then crates.io; no empty Book/Codecov slot or absence explanation. |
| Single package; no verified backend | Title and description; no badge paragraph or invented installation fallback. |
| Virtual workspace; public packages `alpha` and `zeta`; no primary | Shared badges first; named `alpha` then `zeta` registry badges; crate map; no workspace-version badge. |
| Workspace primary `zeta`; other public and internal members | Named `zeta` top-level registry badge; relevant member routes in the map; no registry badge for an internal-only package. |
| Member `alpha`; shared CI/coverage | `Workspace CI`, `Workspace Codecov`, applicable Book, then only `alpha` on crates.io. |
| Book sources without a verified deployment | No Book badge and no substitute links to book chapters or source navigation. |
| Failing CI or low coverage from a verified service | Keep the real status; do not hide it or create a static passing badge. |
| Registry name exists but belongs to another project | No registry badge until ownership/package identity is resolved. |
| Shared inherited README that describes the wrong member | Report the package/readme mapping issue; do not silently publish a misleading overview or change manifests out of scope. |
| Review-only request on generated READMEs | Report proposed source/template changes; do not regenerate or modify files. |

For every case, reject license sections/badges/boilerplate, installation commands
or dependency snippets, mdBook chapter links, repeated book landing links, extra
badge categories, and unused template placeholders. Confirm any optional usage
example is not setup material under a different heading.
