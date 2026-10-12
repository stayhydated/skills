# Badge markup and README layouts

These are authoring templates, not files to paste unchanged. Resolve every
uppercase placeholder from repository evidence; remove unavailable badges,
unused definitions, optional headings, and instructional comments. The final
README must contain no template placeholders or absence inventory.

Include a requirements paragraph only when supported by the package manifests
and relevant compatibility evidence. At a workspace root, identify which
packages the statement covers and qualify differing requirements.

## Core badge paragraph

Use one badge per source line, without blank lines between them:

```markdown
[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io][crate-badge]][crate]
```

Add other relevant, verified badges to the same paragraph, usually after this
core group. For example, a project using Crowdin may add:

```markdown
[![Crowdin][crowdin-badge]][crowdin]
```

Resolve `crowdin-badge` and `crowdin` from the actual project's public badge
image and localization page. These layouts show common core badges; adapt them
to include or retain useful project-specific badges with verified scope.

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

For a verified workspace-member component, add the exact configured
`component_id` to the badge image and identify the member in the badge label:

```markdown
[![Codecov: MEMBER-PACKAGE][codecov-badge]][codecov]

[codecov-badge]: https://codecov.io/gh/OWNER/REPO/branch/BRANCH/graph/badge.svg?component=COMPONENT-ID
[codecov]: https://codecov.io/gh/OWNER/REPO
```

Use a verified component-filtered Codecov UI route as the link target when one
is available. Otherwise the verified project dashboard is an honest target; do
not construct an undocumented component deep link. Confirm that the component's
path or flag filters isolate the member and that uploaded coverage populates it.

## Single-package layout

```markdown
# PACKAGE

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io][crate-badge]][crate]

PACKAGE provides CONCRETE-CAPABILITY for INTENDED-READER.

Requires Rust VERIFIED-MSRV or newer.

## Overview

ONLY-DISTINCTIVE-VERIFIED-CAPABILITIES-OR-CONSTRAINTS

## Features

COMPLETE-FEATURE-TABLE-USING-THE-FEATURE-REFERENCE-LAYOUT-BELOW

## Example

REQUIRED-FILE-PACKAGE-OR-RUNTIME-CONTEXT

ONE-SHORT-MAINTAINED-USAGE-EXAMPLE-WITHOUT-INSTALLATION-STEPS

EXPECTED-RESULT-OR-EFFECT
```

Add only the definitions actually used. Omit Overview or Example when they are
unnecessary, and Features when the package exposes no Cargo features. Do not add
a Contributing section or contribution-guide link, including under Support. Do
not add a Documentation section that repeats the Book badge, or an Installation
section to fill the gap when there is no book.

## Workspace root with an established primary package

Use the same opening and badge paragraph, with exactly one verified crates.io
badge for the evidenced public facade or primary consumer package. Make its
visible label and alt text identify that package. For example, after Book:

```markdown
[![crates.io: PRIMARY-PACKAGE][primary-badge]][primary]
```

```markdown
[primary-badge]: https://img.shields.io/crates/v/PRIMARY-PACKAGE.svg?label=PRIMARY-PACKAGE
[primary]: https://crates.io/crates/PRIMARY-PACKAGE
```

Describe the overall project, then add a `Crates` table when choosing a member
needs explanation. Use the two-column `Crate` / `Purpose` layout below, linking
each crate name to its README or source directory. Other package routes belong
in that table or their member READMEs. Establish the primary package from actual
consumer guidance and API routes, never from the repository name or
`default-members`. If its publication is unverified, omit its badge rather than
selecting an underlying package's version.

## Workspace root without a primary package

Keep verified shared CI, Codecov, and applicable Book badges plus relevant
additional badges. Use zero crates.io badges at the root; the crate table routes
readers to each member's README and its own package badge:

```markdown
# PROJECT

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]

PROJECT provides SHARED-CAPABILITY. Choose the crate for your use case below.

Requires Rust VERIFIED-MSRV or newer.

## Crates

| Crate | Purpose |
| --- | --- |
| [`PACKAGE-A`][package-a-readme] | VERIFIED-PURPOSE-A |
| [`PACKAGE-B`][package-b-readme] | VERIFIED-PURPOSE-B |
```

Use these additional definition shapes, with actual package names and routes:

```markdown
[package-a-readme]: https://github.com/OWNER/REPO/blob/BRANCH/MEMBER-A/README.md
[package-b-readme]: https://github.com/OWNER/REPO/blob/BRANCH/MEMBER-B/README.md
```

Do not add a registry link for the workspace name itself. Link an internal tool
row to its actual source if it belongs in the overview, without a crates.io
badge. If a member lacks a README, link to its existing source directory rather
than inventing a file. Do not repeat badge images inside the table.

## Workspace member layout

```markdown
# MEMBER-PACKAGE

[![crates.io: MEMBER-PACKAGE][crate-badge]][crate]

MEMBER-PACKAGE provides MEMBER-CAPABILITY for INTENDED-DIRECT-CONSUMERS within [PROJECT][project].

Requires Rust VERIFIED-MSRV or newer.

## Features

COMPLETE-FEATURE-TABLE-USING-THE-FEATURE-REFERENCE-LAYOUT-BELOW

## Example

REQUIRED-FILE-PACKAGE-OR-RUNTIME-CONTEXT

ONE-SHORT-MEMBER-SPECIFIC-USAGE-EXAMPLE

EXPECTED-RESULT-OR-EFFECT
```

When applications normally consume this member through a facade, name and link
that facade in the opening paragraph, including any required feature. Explain
the concrete use case for depending on the member directly when one exists.

The member template intentionally omits workspace-wide CI and Codecov. Add a CI
or Codecov line before crates.io only when its linked backend exposes genuinely
member-specific status or coverage; a shared workflow or report that happens to
include the member is insufficient. Never add a Book badge or repeat the
workspace book link in a member README; the root README owns the workspace book
route. For Codecov, prefer an
`individual_components` entry whose paths or flags isolate the package, and use
its exact `component_id` in the badge query. The crates.io image, visible label,
and destination must all name this member, not the primary package. A short
verified API-reference link can provide an additional route without introducing
a docs.rs badge. Include Features whenever the member exposes Cargo features.

## Feature reference layout

Use this layout in the owning package README, with one row for every exposed
Cargo feature. Keep the `default` row only when that feature is declared; derive
all rows and default activation from the actual package feature graph.

```markdown
## Features

DEFAULT-FEATURE-SET-SUMMARY

| Feature | Default | Purpose |
| --- | --- | --- |
| `default` | Yes | Enables EXACT-DEFAULT-MEMBERS. |
| `FEATURE-A` | YES-OR-NO | VERIFIED-CAPABILITY-AND-RELEVANT-CONSTRAINTS. |
| `FEATURE-B` | YES-OR-NO | VERIFIED-CAPABILITY-OR-EXACT-BUNDLE-MEMBERS. |
```

List each individual feature even when an aggregate includes it. Describe
internal or experimental features with their evidenced intended use. A virtual
workspace root uses the crate table to route readers to these member inventories;
a root README serving a root package documents that package's features.

## Example forms

Choose the form that shows the package's actual use. Use `Example`, `Commands`,
or `Configuration` as appropriate rather than adding every heading.

| Package purpose | Example shape | Context and result |
| --- | --- | --- |
| Rust library | A short public-API call or declaration with an observable result. | Identify required features, application-owned values, and runtime initialization. |
| CLI | A representative command or short workflow; optionally a `Command` / `Purpose` table. | State the working directory and input, then describe output and file or state changes. |
| Configuration parser or loader | A minimal valid file in its actual format. | Name the file, its location, its consuming API or tool, and its effect. |
| Protocol or tool integration | A representative request, message, or payload. | Name the operation, distinguish a payload from a complete message, and show or describe the response or effect. |

For excerpts, label the missing host context and link a maintained complete
example. Verify that the excerpt agrees with that example and report what its
existing harness validates. Keep shared behavior in its owning API reference or
crate README; retain only the detail needed to understand this example and use
the package correctly.

## Acceptance cases

Use these cases to review an authored README and to exercise future automation.
They describe expected behavior, not permission to create external services.

| Evidence | Expected README behavior |
| --- | --- |
| Single package; all four backends verified | Title, CI, Codecov, Book, crates.io, then description. |
| Verified Crowdin project or another relevant badge service | Include or retain its accurate badge in the compact paragraph, usually after the core group; verify its destination and scope. |
| Relevant license badge with verified project metadata and destination | Retain or add the badge; omit standalone license sections and prose. |
| Additional badge with an unrelated destination or unsupported claim | Omit or correct it based on evidence and reader value. |
| Published package; CI and crates.io only | CI then crates.io; no empty Book/Codecov slot or absence explanation. |
| Single package; no applicable verified badge | Title and description; no badge paragraph or invented installation fallback. |
| Virtual workspace; public packages `alpha` and `zeta`; no primary | Keep valid shared/additional badges with zero root registry badges; use the crate map for discovery and each member's own registry badge. |
| Workspace primary `zeta`; other public and internal members | Exactly one named `zeta` root registry badge after shared badges; other package routes in the crate map and own badges in their member READMEs. |
| Published public facade established by consumer guidance and API routes | Exactly one root registry badge for that facade; no additional root member-version badges. |
| Repository name or `default-members` points to a package, but consumer evidence establishes no primary | Zero root registry badges; keep valid shared/additional badges and the crate map. |
| Clear primary package has no verified publication | Omit its root registry badge; do not substitute a published underlying member. |
| Workspace root needs a crate map, with or without a primary package | Use `Crate` and `Purpose` columns. Link each crate name to its existing README or source directory; omit a redundant `Source` or `README` column. |
| Package has `fmt`, `full`, `full-fluent`, and individual integration features | List every feature, including each integration, with its purpose and default activation; describe the exact aggregate members. |
| Package exposes an internal or experimental feature | Include its row and label its intended use from repository evidence. |
| Optional dependencies expose implicit features; others use `dep:` | Include actual implicit features; explain suppressed dependency activations under their owning features. |
| Default features enable other features indirectly | Mark every feature enabled through the default graph and explain the default set. |
| Package exposes no Cargo features | Omit Features; keep the remaining README layout concise. |
| Virtual workspace with feature-rich members | Keep complete feature tables in the member READMEs linked from the root crate table. |
| Member `alpha`; workspace-wide CI/coverage and Book | Use `alpha`'s own crates.io badge; keep shared status and Book badges at the workspace root. Other relevant badges must have verified member scope. |
| Member `alpha`; verified Codecov component filters only `alpha` | `Codecov: alpha` component badge, then only `alpha` on crates.io. |
| Member `alpha`; component exists but filters or uploaded coverage are unverified | No member Codecov badge until its scope and rendered result are verified. |
| Member `alpha`; workspace book documents `alpha` | No member Book badge or repeated book link; retain the canonical Book badge at the workspace root. |
| Book sources without a verified deployment | No Book badge and no substitute links to book chapters or source navigation. |
| Canonical deployed application book documents underlying packages through a public facade | Retain the root Book badge; do not require a chapter for every package or repeat book links in member READMEs. |
| Useful canonical application book omits a separate direct parser or tooling route | Retain its root badge for the intended application audience and report the direct-consumer documentation gap in the handoff; do not claim complete coverage or write book chapters during a README edit. |
| Deployed book is non-canonical, internal-only, or inapplicable to the README's readers | Omit its Book badge; do not replace it with invented documentation navigation. |
| Failing CI or low coverage from a verified service | Keep the real status; do not hide it or create a static passing badge. |
| Registry name exists but belongs to another project | No registry badge until ownership/package identity is resolved. |
| Shared inherited README that describes the wrong member | Report the package/readme mapping issue; do not silently publish a misleading overview or change manifests out of scope. |
| Existing `CONTRIBUTING.md` or contributor guide | Omit contribution sections, links, and invitations from root and member READMEs, including under Support; leave contributor guides unchanged. |
| Review-only request on generated READMEs | Report proposed source/template changes; do not regenerate or modify files. |
| Example uses an existing manager, application context, or generated value | Label it as an excerpt, identify the supplied context and expected result, and link its maintained complete example. |
| Runtime, derive, or integration package normally consumed through a facade | Explain direct consumers and link the recommended facade and required feature in the opening paragraph. |
| Declared minimum Rust version differs from a development or CI pin | State the manifest-backed minimum; label the toolchain's separate role only when relevant to consumers. Include evidenced framework or platform constraints affecting adoption. |
| CLI, configuration, or protocol package | Use its natural example format, with required file or working-directory context and the expected output or effect. |
| Shared policy repeated across members or detailed edge cases accumulating in an overview | Keep concise adoption-relevant guidance and complete feature tables; link the existing documentation owner for detailed semantics. |
| Prose only explains links, file layout, resource loading, or where snippets are repeated/tested | Remove it; retain actual feature/runtime/input requirements and expected results. |
| Concise API/source link identifies accepted grammar, detailed semantics, or a complete example | Keep the useful route and its domain-specific context; do not treat it as navigation filler merely because it names a link. |
| Repository instructions require a root/member example to stay aligned | Preserve the aligned snippet while omitting duplicate/test-location narration from README prose; report the harness in validation evidence. |
| Usage example with unnecessary clones or panics for routine failures | Show idiomatic borrowing and fallible control flow using the actual public API; do not invent APIs or expand into production refactors. |
| Usage example exceeds the package's edition, MSRV, or enabled features | Use a compatible idiom and evidenced feature context; do not raise requirements for README styling. |
| README Rust fence outside the actual example harness | Review the example and disclose the execution gap; do not claim an unrelated Cargo test command validated it. |

For every case, reject standalone license sections/prose/boilerplate,
contribution sections or links, contributor setup or invitations, installation
commands or dependency snippets, mdBook chapter links, repeated book landing
links, inaccurate or irrelevant badges, registry badge piles at workspace roots,
navigation/testing filler, and unused template placeholders.
Check feature tables for complete names, accurate defaults, and exact aggregate
membership against package evidence.
Check requirements and facade routes against manifests and source. Verify each
example's context, expected result, and complete-source link when it is an excerpt.
Confirm any optional usage example is idiomatic for the actual public API,
edition, MSRV, and features, and is not setup material under a different heading.
Record what the example harness actually compiled or executed rather than
inferring coverage from a green build.
