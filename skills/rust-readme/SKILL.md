---
name: rust-readme
description: >
  Create, revise, or audit concise Rust READMEs with idiomatic usage examples for
  single-package repositories, workspace roots, and member crates. Use for README
  structure, crate selection, requirements and compatibility, contextual usage
  examples, complete Cargo feature tables, publication-aware links, and verified
  scope-appropriate badges, including relevant project-specific badges.
  Omit license sections and prose, installation instructions,
  contribution sections or links, and mdBook chapter links or duplicated book
  navigation.
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

## Usage examples

Choose a short, maintained example that demonstrates the package's purpose.
Identify the required file, package, features, runtime state, or inputs and show
the expected result or effect. Keep this context close to the example.

Prefer a complete example. When a full host application would overwhelm the
README, label the code as an excerpt, name the values or initialization supplied
by its host, and link to the maintained complete example. Verify that the
excerpt agrees with that source and use its existing harness for validation.
If no complete source can be verified, use a self-contained example or omit it.

Match the example to the package:

- **Library API:** show the relevant call and its result; identify the context
  supplying any application-owned state.
- **CLI:** identify the working directory, required inputs, output, and file or
  state changes. A compact command table may explain the main operations.
- **Configuration:** name the file, its location, and the loader or tool that
  consumes it; show a minimal valid configuration and its effect.
- **Protocol:** identify the tool, operation, or endpoint, distinguish a payload
  from a complete message, and explain the expected response or effect.

Use the appropriate fenced language for Rust, shell, TOML, JSON, or other actual
formats. Keep examples focused on usage; the existing installation exclusions
still apply. See the [example forms](references/badges-and-layouts.md#example-forms)
for layout guidance.

### Idiomatic Rust requirement

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
- Validate Rust examples as runnable or explicitly compile-only through the
  existing harness, including the surrounding code for a labeled excerpt.
  Do not add dependencies, raise the MSRV, or refactor production APIs solely to
  make README code look more idiomatic.

Use `rust-best-practices` for example-level implementation guidance and
`rust-test` for validation without expanding the README write boundary. Report
an API limitation that needs a separate change instead of inventing a better API.

## Required evidence

Inspect the smallest relevant set before writing:

1. Repository and directory instructions, current READMEs, generators or
   templates, and the working diff. Edit the source of a generated README and
   follow its existing regeneration workflow only in Edit mode.
2. Root and relevant member `Cargo.toml` files: package names, targets,
   descriptions, workspace membership, inherited metadata, `rust-version`, `readme`,
   `repository`, `documentation`, `publish`, `[features]`, and optional dependency
   declarations and framework version requirements. Resolve member globs,
   exclusions, inherited dependencies, and local path members; do not equate the
   directory name with the crate name. Inspect required patches and platform
   constraints when they affect consumers.
3. The actual CI workflow and branch, Codecov upload/report configuration,
   `book.toml` and book deployment, relevant published crates.io pages, and any
   additional badge services or metadata sources. Verify destinations and
   ownership rather than constructing plausible URLs.
4. Public API docs, CLI help, tests, or maintained examples that substantiate
   the description, feature behavior, re-exports and recommended dependency
   routes, compatibility claims, and example inputs and results. Read existing
   user-support routes before linking them.

Treat Cargo packages as the publication unit even when casually called crates.
A package may contain both library and binary targets. Count workspace members,
not Rust source targets, when choosing a README layout. A `[workspace]` without
`[package]` is a virtual root, not a package that can be published. A root with
both tables is a workspace with a root package; it is not automatically a
single-package repository. Follow the evidence even in a one-member workspace.

See [Cargo, example, and badge mechanics](references/technical-references.md) for sources.
Use those references to verify mechanics, not to infer README structure or
import another project's authoring conventions.

## Badge policy

The core badge order is this skill's editorial policy.

Put one compact badge paragraph immediately below the H1, before the opening
prose. Keep badges on consecutive Markdown source lines with no blank lines
between them. Use linked images with meaningful alt text and reference-style
links; keep the reference definitions together at the end of the file.

Keep applicable core badges in this relative order: **CI → Codecov → Book → crates.io**.

Include or retain other relevant, verified badges when they communicate useful
project status, metadata, or navigation, such as Crowdin translation status for
an existing localization project. Verify each badge's image, destination,
ownership, label, and scope. Use relevance and evidence to select badges; place
additional badges where they make sense, usually after the core group. Preserve
useful established ordering for those additional badges.

Select the applicable, verified core entries in that order. Omit missing
categories without placeholders, empty badges, explanatory absence prose, or
reordering what remains. Never add a backend solely to complete the row. With no
applicable verified badges, start the description after the title without a
badge paragraph.

| Category | Evidence required | Link target and scope |
| --- | --- | --- |
| CI | An enabled workflow or existing CI service that actually validates this project, with a usable status endpoint. | The matching workflow/results page; use the actual default or documented support branch. Prefer the aggregate CI workflow rather than separate OS, lint, or release badges. |
| Codecov | An existing Codecov project with coverage reports and a working badge, not merely a coverage command or configuration file. A member badge requires a configured component or flag whose filters isolate that package. | Use the matching project dashboard or another verified Codecov UI route. For a component badge, use the exact configured `component_id` in the badge query and identify the member in its label. |
| Book | A canonical mdBook with a verified deployment that serves the README's intended reader audience. A source-only `book.toml` is insufficient. Facade-level documentation may cover underlying members through their normal application use. | Use the book's landing/root URL on a single-package or workspace-root README, never on a workspace-member README. Never target a chapter, anchor, source directory, or deployment workflow. A static `Book` / `mdBook` badge is navigation, not a claim of complete package coverage or a passing build. |
| crates.io | A published package belonging to this project, verified by its exact Cargo package name and registry metadata. Publication permission or a manifest version alone is insufficient. | The package's crates.io landing page, with a dynamic version badge rather than a manually maintained version. |

A failing CI run or low coverage does not make a backend unavailable: preserve
truthful status, never hide it or replace it with a static passing badge.
Existing verified badges may be retained when a service is temporarily
unreachable; disclose live-verification limits in the handoff. Do not invent new
badges when evidence is missing. Do not expose secrets or reuse another
repository's badge tokens; use public URLs or an explicitly public badge token.

Keep every badge accurate and useful to the README's audience. Ordinary, verified
API-reference or user-support links may also provide a distinct useful route.

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
  Judge coverage by reader workflows: an application guide for the facade may
  document underlying runtime, derive, and integration members through that
  facade without separate chapters for each package. A canonical deployed guide
  that serves the README's application audience may remain when a distinct
  direct-consumer route, such as parser tooling, is not covered; report that gap
  in the handoff without claiming the book is comprehensive. Omit the badge when
  the book is absent, undeployed, non-canonical, or inapplicable to the README's
  audience. Never repeat the Book badge or its link in member READMEs, substitute
  an internals book, add chapter links, or expand a README edit into book writing.
- A single-package or member README has at most its own crates.io badge.
- At a workspace root, use exactly one crates.io badge for the clear public
  facade or primary consumer package when its publication is verified. Establish
  that role from maintained application guidance, examples, and public API
  dependency routes. Do not add badges for its underlying members. When no one
  clear facade or primary consumer package exists, use zero root crates.io
  badges: retain valid shared and additional badges, and use the crate table for
  package discovery. If the clear primary package is unpublished or unverified,
  omit its badge rather than substituting a sibling package. Member READMEs
  retain their own verified package badges. Never imply the primary package's
  version is a workspace-wide release version.
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
2. **Verified badge paragraph** with the core order and relevant additional badges above.
3. **Opening description:** one short paragraph naming the problem, intended
   reader, and concrete value. Prefer supported capabilities to marketing.
4. **Requirements paragraph** with the evidenced minimum Rust version and
   adoption-relevant compatibility constraints for this README's scope.
5. **Overview** only when a few differentiating capabilities or action-changing
   constraints add information beyond the opening paragraph.
6. **Crates** for a multi-package workspace whose members need explanation:
   a two-column `Crate` / `Purpose` table with each crate name linked to its
   owning README or source directory.
7. **Features** for a package with Cargo features: a complete table of feature
   names, default activation, and purpose in the README that owns that package.
8. **Example** when a short API, CLI, configuration, or protocol example
   communicates the project's value with its context and expected result.
   Use `Commands` or `Configuration` when that heading better describes the
   content. Keep it separate from installation instructions.
9. **Support** only for an existing, distinct user-support destination. Do not
   use it to reintroduce contribution guidance or contributor setup.
10. **Link definitions** for reference-style links, with no visible heading.

### Single-package repository

Describe the package directly, using its real package and command names. Do not
add a workspace table or separate duplicate READMEs merely because it contains
multiple binaries, examples, or a library target. Use its published crate badge
when available and its relevant book badge when deployed.

### Workspace root

Explain the shared project and help readers choose a public entry point. Use a
small `Crates` table with `Crate` and `Purpose` columns when useful; sort rows by
exact package name. Make each code-formatted
crate name a reference-style link to its existing member README, otherwise its
source directory. The crate name carries the navigation link; omit a separate
`Source` or `README` column that repeats the same route. Include an internal tool
only when it helps readers navigate the repository. Avoid exhaustive dependency
graphs and inventories of test crates.
Use the purpose cells to distinguish application entry points, optional
integrations, and lower-level building blocks. When the project has a recommended
starting crate, name it and link it near the table using repository evidence.
The root owns repository-wide CI and Codecov status so that member READMEs do
not repeat identical badges. It also owns the applicable canonical Book badge
for the workspace's intended reader audience.

A virtual root has no crate page of its own. A workspace with a root package
may reuse its root README for that package only if the content genuinely serves
both audiences. Do not silently apply the same ecosystem overview to every
published member.

### Workspace member

Make the member README independently understandable on its crates.io page:
package purpose first, then its role in the project and a canonical repository
route when useful. For facade, runtime, derive, and integration packages, explain
in the opening paragraph who should depend on the package directly. When normal
application usage goes through a facade, link that facade and name any required
feature; identify the use case for a direct dependency when one exists. Verify
the re-exports and intended dependency route from source.

Scope examples, capabilities, API links, and version badges to that package.
Do not repeat workspace-wide CI or Codecov badges, the
workspace map, or the full project overview. Include CI or Codecov only when its
linked result is specific to the member rather than a shared workspace run or
report. A verified Codecov component whose filters isolate the member is the
preferred coverage evidence. Do not add a Book badge or repeat the workspace
book link; the root README owns that navigation. Create a member README only when
publication or a distinct audience needs it.

Resolve explicit `package.readme` paths relative to the package manifest;
resolve inherited `readme.workspace = true` through `workspace.package.readme`
relative to the workspace root. Respect `readme = false` and existing shared or
generated sources. Flag an unsuitable shared README or manifest mapping rather
than changing manifests outside the authorized scope. Check image and link
resolution both on the repository host and in the published package context;
use verified absolute URLs where relative paths would not resolve correctly.

## Requirements and compatibility

Place one concise requirements paragraph after the opening description. Derive
the minimum Rust version from the package's `rust-version`, resolving workspace
inheritance, and use `Requires Rust X.Y or newer.` with the declared version.
Scope member statements to that package. At a workspace root, identify the
covered packages and qualify differing requirements instead of assigning every
member one package's minimum.

A pinned development or CI toolchain is separate from the supported minimum.
Include it only when it changes a consumer action, and label that role explicitly.
If the manifest provides no minimum, omit the Rust requirement claim and report
any requested verification gap in the handoff. Report conflicts between the
manifest, tests, and documented support policy instead of choosing a new minimum.

Include supported framework versions, platform restrictions, feature conditions,
or required upstream patches when they affect adoption. Verify them against the
relevant dependency declarations, implementation, and maintained support docs;
link the owning API or source reference when more detail is needed. Describe
the current supported configuration without accumulating historical version
matrices. Mention the Rust edition only when it changes how consumers use the
package. See [Cargo mechanics](references/technical-references.md) for the
distinction between declared support and a selected toolchain.

## Cargo feature documentation

For each single-package or workspace-member README, add a `Features` section
when the package exposes Cargo features. List every feature in a compact
`Feature` / `Default` / `Purpose` table, using exact Cargo names. Include the
declared `default` feature, individual integrations, aggregate features such as
`full`, and internal or experimental flags. Label restricted audiences or
stability expectations from repository evidence. An overview of selected
features or a bundle name alone is insufficient.

Build the inventory from the package's `[features]` table and any implicit
features exposed by optional dependencies. Honor `dep:` suppression of implicit
features; dependency activations and forwarded dependency features belong in
the owning feature's explanation, not as invented package feature names. Verify
each purpose against feature-gated code or maintained documentation. Explain
which features each aggregate enables, using its actual membership rather than
assuming `full` means every feature.

State the default feature set and mark each row according to activation through
the package's default feature graph, including indirect activation. When no
features are enabled by default, say so in the section. Keep descriptions short
and include prerequisites, platform limits, or conflicts when evidenced. This
table is the crate's feature reference; setup commands and dependency snippets
remain outside the README layout.

A virtual workspace root routes readers to member READMEs through the crate
table. Keep each member's feature inventory in its owning README. If the root
README also serves a root package, include that package's complete feature table
and identify its scope. Omit the section for a package with no exposed features.

## Keep documentation single-sourced

- Keep the opening and overview focused on package choice and first correct use.
  Summarize material constraints briefly and link their maintained API reference
  for detailed semantics. Avoid accumulating edge-case algorithms, protocol
  internals, or release-note details in the README.
- Omit README narration that only explains links, file organization, resource
  loading, or where examples are repeated or tested. Keep required package,
  feature, runtime, and input context plus observable results. Preserve shared
  snippets when repository instructions require them to stay aligned; explain
  duplication and test-harness evidence in the validation handoff, not the README.
  Keep useful API/source routes and concise domain-specific guidance, such as
  "See the API reference for accepted grammar," when it directs readers to
  maintained semantics or complete code. A necessary input-file location is
  usage context, not file-layout filler.
- Give shared behavior one documentation owner, such as an API reference or
  canonical crate README. Member READMEs retain integration-specific guidance
  and link that owner instead of repeating a shared policy. Preserve each
  package's complete Cargo feature table and adoption-relevant constraints.
  Report missing reference material as follow-up work when creating it would
  exceed the requested documentation scope.
- Omit README license sections and prose, including standalone license-file
  links, SPDX summaries, and contribution-license boilerplate. Leave `LICENSE*`,
  notices, and Cargo license metadata unchanged; this is a README policy, not a
  request to change the project's license.
- Omit README contribution sections, contribution-guide links, contributor
  setup, and contribution invitations, including under alternate headings such
  as `Support`. Leave `CONTRIBUTING.md` and other contributor guides unchanged;
  rely on repository-host discovery instead of repeating that route in READMEs.
- Do not add installation sections, `cargo add` / `cargo install` commands,
  dependency/version snippets, package-manager recipes, or clone/build setup.
  Verified package badges and owning member READMEs provide package discovery;
  the applicable single-package or workspace-root Book badge is the user-guide
  route. Do not duplicate those routes in a `Getting started` section.
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
- Avoid manual release versions, benchmark dumps, contributor commands, and
  manifest dumps. Retain the complete Cargo feature table. Keep other constraints
  concise and verified when they materially change a reader's choice or safe use.

## Workflow and validation

1. Select Edit or Review and identify the affected README audience(s).
2. Build an evidence map of package roles and dependency routes, publication,
   requirements and compatibility, doc ownership, Cargo features and defaults,
   example context and results, and each badge's image URL, target URL, label,
   and status scope.
3. Apply the layout, core badge order, and relevant additional badges. Use
   [badge markup, layouts, and acceptance cases](references/badges-and-layouts.md).
4. Remove duplication and stale references only within scope. For generated
   READMEs, update the owning source rather than editing generated output alone.
5. Check Markdown rendering, heading order, resolved links/images, badge labels
   and destinations, and the complete absence of disallowed README content.
   Confirm the root has one verified primary-package registry badge or no
   registry badges, as supported by consumer-route evidence. Check the book's
   intended reader audience and report uncovered direct-consumer routes.
   Compare each feature table with the package's complete Cargo feature inventory;
   check default activation, aggregate membership, and descriptions against source.
   Verify facade routes, requirements, and compatibility claims. Check example
   context, expected results, complete-source links, and shared-behavior links.
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
