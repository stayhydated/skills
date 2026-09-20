---
name: rust-readme
description: >
  Create, revise, or audit concise Rust READMEs with idiomatic usage examples for
  single-package repositories, workspace roots, and member crates. Use for README
  structure, concise project overviews, crate navigation, publication-aware links,
  and verified scope-appropriate badges ordered CI, Codecov, Book, crates.io.
  Omit license content, installation instructions, contribution sections or
  links, and mdBook chapter links or duplicated book navigation.
---

# Rust README

Make the README a concise landing page: what the project does, who it serves,
which crate to choose, and where its maintained documentation lives. It is not
an installation guide, a book contents page, or a copy of registry metadata.

## Scope and write boundary

- **Edit:** create or revise only the requested READMEs and their authorized
  sources. Preserve unrelated edits and existing project identity.
- **Review:** audit or checklist requests report findings and proposed changes
  without modifying files. When write permission is ambiguous, choose Review.
  Use non-mutating checks or an isolated copy; do not run generators or
  formatters against the original checkout.

This skill owns README presentation and routing. Use `mdbook-user-docs` for
user-guide chapters, `mdbook-internals` for maintainer architecture, `rust-test`
for example validation, and `rust-best-practices` for implementation or API
style. Those are separate tasks, not permission to expand a README edit.

## Idiomatic Rust requirement

Every Rust usage example authored, retained, or recommended must demonstrate
idiomatic use of the package's actual public API for its edition, MSRV, and
feature set. Prefer existing tested examples, but review their style and behavior
rather than treating compilation as proof of idiomaticity.

- Use conventional names, straightforward control flow, and the public API's
  intended ownership model. Avoid unnecessary clones, allocations, wrappers, or
  invented APIs merely to shorten the example.
- Prefer `?` for fallible usage and handle meaningful `Option`/`Result` cases.
  Use `expect` only for an explained invariant, not to hide normal input or I/O
  failures. Hidden rustdoc setup is appropriate only when the actual README
  rendering and test workflow supports it.
- Keep examples small and runnable or explicitly compile-only through the
  existing harness. Do not add dependencies, raise the MSRV, or refactor
  production APIs solely to make README code look more idiomatic.

Use `rust-best-practices` for example-level implementation guidance and
`rust-test` for validation without expanding the README write boundary. Report
an API limitation that needs a separate change instead of inventing a better API.

## Required evidence

Inspect the smallest relevant set before writing:

1. Repository and directory instructions, current READMEs, generators or
   templates, and the working diff. Edit the source of a generated README and
   follow its existing regeneration workflow only in Edit mode.
2. Root and relevant member `Cargo.toml` files: package names, targets,
   descriptions, workspace membership, inherited metadata, `readme`,
   `repository`, `documentation`, and `publish`. Resolve member globs, exclusions,
   and local path members; do not equate the directory name with the crate name.
3. The actual CI workflow and branch, Codecov upload/report configuration,
   `book.toml` and book deployment, and relevant published crates.io pages.
   Verify destinations and ownership rather than constructing plausible URLs.
4. Public API docs, CLI help, tests, or maintained examples that substantiate
   the description and any example. Read existing user-support routes before
   linking them.

Treat Cargo packages as the publication unit even when casually called crates.
A package may contain both library and binary targets. Count workspace members,
not Rust source targets, when choosing a README layout. A `[workspace]` without
`[package]` is a virtual root, not a package that can be published. A root with
both tables is a workspace with a root package; it is not automatically a
single-package repository. Follow the evidence even in a one-member workspace.

See [Cargo, example, and badge mechanics](references/technical-references.md) for sources.
Use those references to verify mechanics, not to infer README structure or
import another project's authoring conventions.

## Fixed badge policy

The badge order and exclusions are this skill's editorial policy, not universal
Rust conventions.

Put one compact badge paragraph immediately below the H1, before the opening
prose. Keep badges on consecutive Markdown source lines with no blank lines
between them. Use linked images with meaningful alt text and reference-style
links; keep the reference definitions together at the end of the file.

The only allowed badge categories, in this exact order, are **CI → Codecov → Book → crates.io**.

Select the applicable, verified entries in that order. Omit missing categories
without placeholders, empty badges, explanatory absence prose, or reordering
what remains. Never add a backend solely to complete the row. With no verified
backends, start the description after the title without a badge paragraph.

| Category | Evidence required | Link target and scope |
| --- | --- | --- |
| CI | An enabled workflow or existing CI service that actually validates this project, with a usable status endpoint. | The matching workflow/results page; use the actual default or documented support branch. Prefer the aggregate CI workflow rather than separate OS, lint, or release badges. |
| Codecov | An existing Codecov project with coverage reports and a working badge, not merely a coverage command or configuration file. A member badge requires a configured component or flag whose filters isolate that package. | Use the matching project dashboard or another verified Codecov UI route. For a component badge, use the exact configured `component_id` in the badge query and identify the member in its label. |
| Book | An existing mdBook and its verified, deployed reader-facing site. A source-only `book.toml` is insufficient. In a workspace, the canonical user book must cover the public member set. | Use the book's landing/root URL on a single-package or workspace-root README, never on a workspace-member README. Never target a chapter, anchor, source directory, or deployment workflow. A static `Book` / `mdBook` badge is navigation, not a claim that a build passes. |
| crates.io | A published package belonging to this project, verified by its exact Cargo package name and registry metadata. Publication permission or a manifest version alone is insufficient. | The package's crates.io landing page, with a dynamic version badge rather than a manually maintained version. |

A failing CI run or low coverage does not make a backend unavailable: preserve
truthful status, never hide it or replace it with a static passing badge.
Existing verified badges may be retained when a service is temporarily
unreachable; disclose live-verification limits in the handoff. Do not invent new
badges when evidence is missing. Do not expose secrets or reuse another
repository's badge tokens; use public URLs or an explicitly public badge token.

Do not add badges for licenses, docs.rs, MSRV, downloads, dependencies, security
scores, chat, sponsorship, stars, or anything outside the four categories.
Ordinary, verified API-reference or user-support links are not badges, but add
them only when they provide a distinct useful route.

### Workspace badge selection

- Keep repository-wide or workspace-wide CI and Codecov badges on the workspace
  root README. Do not repeat the same status in member READMEs merely because the
  workflow or report includes that member. A member README may include CI or
  Codecov only when the linked backend exposes genuinely member-specific status
  or coverage; label and link it at that member scope.
- Prefer a Codecov component for member coverage when
  `component_management.individual_components` maps an exact `component_id` to
  that package through verified path or flag filters. Use the component ID in
  the badge's `component` query parameter. Confirm that uploaded reports populate
  the component and that the badge works; configuration alone is insufficient.
  Link to a verified component-filtered Codecov page when one is available,
  otherwise link to the verified project dashboard rather than inventing a deep
  URL.
- Put the canonical user-facing Book badge on the workspace root README only.
  The workspace book should account for every public member and provide all
  needed user documentation. Never repeat the Book badge or its link in member
  READMEs. If the book does not cover a member, report that workspace-book
  documentation gap rather than adding member navigation or selecting an
  internals book.
- A single-package or member README has at most its own crates.io badge.
- At a workspace root, use the evidenced primary public package's badge when
  there is one. Otherwise use a named crates.io badge for each published,
  user-facing member represented by the overview, sorted lexicographically by
  exact Cargo package name. Keep all of these after Book. Never imply one
  member's version is a workspace-wide release version.
- Do not infer a primary package from the repository name, directory order,
  `default-members`, or a shared workspace version. Name each package in both
  the visible badge label and alt text whenever the root could be ambiguous.
- Omit crates.io badges for unpublished, internal-only, or private-registry
  packages. Keep useful internal members in the workspace map with source links,
  not dead registry links. `publish = false` is a warning to inspect publication
  history, not proof that no historical release exists; do not present an old
  release as the current checkout when its role has changed.

## README structure

Use the following order, omitting optional sections that add no information.
Do not create empty headings to satisfy a template.

1. **Project or package name** as the single H1.
2. **Verified badge paragraph** in the fixed order above.
3. **Opening description:** one short paragraph naming the problem, intended
   reader, and concrete value. Prefer supported capabilities to marketing.
4. **Overview** only when a few differentiating capabilities or action-changing
   constraints add information beyond the opening paragraph.
5. **Crates** for a multi-package workspace whose members need explanation:
   a compact map of package name, purpose, and the owning README/source route.
6. **Example** only when one short, maintained API or CLI example communicates
   the project's value. It must not become an installation walkthrough.
7. **Support** only for an existing, distinct user-support destination. Do not
   use it to reintroduce contribution guidance or contributor setup.
8. **Link definitions** for reference-style links, with no visible heading.

### Single-package repository

Describe the package directly, using its real package and command names. Do not
add a workspace table or separate duplicate READMEs merely because it contains
multiple binaries, examples, or a library target. Use its published crate badge
when available and its relevant book badge when deployed.

### Workspace root

Explain the shared project and help readers choose a public entry point. Use a
small `Crates` table when useful; sort rows by exact package name, matching the
registry badge group. Link to existing member READMEs, otherwise their source
directories. Include an internal tool only when it helps readers navigate the
repository. Avoid exhaustive dependency graphs and inventories of test crates.
The root owns repository-wide CI and Codecov status so that member READMEs do
not repeat identical badges. It also owns the canonical Book badge for the
workspace-wide user guide that documents the public members.

A virtual root has no crate page of its own. A workspace with a root package
may reuse its root README for that package only if the content genuinely serves
both audiences. Do not silently apply the same ecosystem overview to every
published member.

### Workspace member

Make the member README independently understandable on its crates.io page:
package purpose first, then its role in the project and a canonical repository
route when useful. Scope examples, capabilities, API links, and version badges
to that package. Do not repeat workspace-wide CI or Codecov badges, the
workspace map, or the full project overview. Include CI or Codecov only when its
linked result is specific to the member rather than a shared workspace run or
report. A verified Codecov component whose filters isolate the member is the
preferred coverage evidence. Do not add a Book badge or repeat the workspace
book link; the root README owns that navigation and the workspace book owns
member documentation. Create a member README only when publication or a distinct
audience needs it.

Resolve explicit `package.readme` paths relative to the package manifest;
resolve inherited `readme.workspace = true` through `workspace.package.readme`
relative to the workspace root. Respect `readme = false` and existing shared or
generated sources. Flag an unsuitable shared README or manifest mapping rather
than changing manifests outside the authorized scope. Check image and link
resolution both on the repository host and in the published package context;
use verified absolute URLs where relative paths would not resolve correctly.

## Keep documentation single-sourced

- Omit all README license sections, license badges, license-file links, SPDX
  summaries, and contribution-license boilerplate. Leave `LICENSE*`, notices,
  and Cargo license metadata unchanged; this is a README policy, not a request
  to change the project's license.
- Omit README contribution sections, contribution-guide links, contributor
  setup, and contribution invitations, including under alternate headings such
  as `Support`. Leave `CONTRIBUTING.md` and other contributor guides unchanged;
  rely on repository-host discovery instead of repeating that route in READMEs.
- Do not add installation sections, `cargo add` / `cargo install` commands,
  dependency/version snippets, package-manager recipes, or clone/build setup.
  The crates.io badge is the package-discovery route and the single-package or
  workspace-root Book badge is the user-guide route. Do not duplicate those
  routes in a `Getting started` section.
- Do not link or enumerate mdBook chapters, reproduce `SUMMARY.md`, embed book
  content with includes, or add a second documentation table of contents. The
  single-package or workspace-root Book badge owns book navigation. Do not repeat
  its destination in body prose or member READMEs.
- A README without a book or registry page still stays concise. Do not invent a
  badge, create a book, or silently introduce installation material as fallback.
  Preserve verified, distinct API/source/support routes and report any requested
  onboarding gap in the handoff, not as permanent missing-feature prose.
- Keep a value-demonstrating example separate from setup. Use public APIs and
  real command names; mention necessary feature context briefly without adding
  dependency instructions. Prefer an existing tested example and omit it if
  making it meaningful would require reproducing a tutorial.
- Avoid manual release versions, exhaustive feature matrices, benchmark dumps,
  contributor commands, and repeated manifest metadata. Retain concise, verified
  constraints when they materially change a reader's choice or safe use.

## Workflow and validation

1. Select Edit or Review and identify the affected README audience(s).
2. Build an evidence map of package roles, publication, doc ownership, and each
   badge's image URL, target URL, label, and status scope.
3. Apply the layout and fixed badge order. Use
   [badge markup, layouts, and acceptance cases](references/badges-and-layouts.md).
4. Remove duplication and stale references only within scope. For generated
   READMEs, update the owning source rather than editing generated output alone.
5. Check Markdown rendering, heading order, resolved links/images, badge labels
   and destinations, and the complete absence of disallowed README content.
   Review each retained Rust example against the idiomatic-Rust requirement and
   the actual public API, edition, MSRV, and features.
6. Use existing repository lint/link/example checks. An arbitrary README Rust
   fence is not automatically a doctest; verify the actual harness before
   claiming that a Cargo test command checks it. Validate package/readme mapping
   when publication is affected, without publishing anything.
7. Review the diff for unrelated changes. Report changed paths, badge choices,
   successful checks, failed attempts, and unverified external destinations
   separately. In Review mode, give findings and proposed fixes, not edits.

Do not install tooling, alter CI, configure Codecov, deploy a book, change Cargo
publication settings, or publish a crate just to satisfy README presentation.
