# Cargo, example, and badge reference mechanics

These references explain Cargo metadata, requirements, features, publication,
Rust example conventions, and badge URL mechanics. They are not README layout
examples or an alternative authoring policy. Follow `SKILL.md` and the bundled
layouts for structure, exclusions, core badge order, and selection of relevant
additional badges. Recheck technical details before relying on behavior that
may have changed.

## Authoritative mechanics

- [Rust API Guidelines: fallible examples](https://rust-lang.github.io/api-guidelines/documentation.html#c-question-mark)
  explains why public examples should demonstrate `?` rather than unwrapping
  routine failures. Apply the example guidance to the actual public API and
  supported toolchain, not as a reason to import crate-doc layouts into READMEs.
- [Cargo manifest: README](https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field)
  explains registry rendering, explicit README paths, default discovery, and
  `readme = false`.
- [Cargo Rust version](https://doc.rust-lang.org/cargo/reference/rust-version.html)
  explains the supported minimum declared by `rust-version` and differing
  support policies within a workspace. Resolve inherited values and scope the
  README statement to the relevant packages; a selected development or CI
  toolchain alone does not establish that minimum.
- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
  distinguishes root packages from virtual manifests and documents membership
  and metadata inheritance. An inherited README path is relative to the
  workspace root; an ordinary package README path is package-relative.
- [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html)
  explains declared and default features, implicit optional-dependency features,
  `dep:` suppression, and dependency-feature forwarding. Use the package's actual
  feature graph to document its complete inventory, defaults, and bundle members.
- [Cargo manifest: publication](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field)
  describes publication permissions, which do not prove that a package has
  actually been published. Verify the real crates.io page separately.
- [GitHub workflow badges](https://docs.github.com/en/actions/how-tos/monitor-workflows/add-a-status-badge)
  documents workflow-file URLs and branch/event selection. Use the actual
  workflow, not a guessed name or a blanket passing label.
- [Codecov status badges](https://docs.codecov.com/docs/status-badges)
  describes badges backed by Codecov data, including the `component` query for a
  component name or ID. Use the target project's own badge and verified report
  scope rather than substituting a generic coverage claim.
- [Codecov components](https://docs.codecov.com/docs/components) documents
  `component_management.individual_components`, component IDs, path and flag
  filters, and component statuses. For a workspace member, verify that the
  configured filters isolate that package and that uploaded coverage populates
  the component before treating its badge as member-specific.
- [Shields crates.io version badge](https://shields.io/badges/crates-io-version)
  provides the package-version image; the surrounding Markdown link must still
  target the correct crates.io package page.
- [Shields static badge](https://shields.io/badges/static-badge)
  supplies a navigation label for an existing deployed book. It is not a health
  check or proof of deployment.

The prohibition on chapter links applies to READMEs authored with this skill,
not to these maintainer-facing technical references. These references document
why the authoring and validation rules exist; they are not README sections.
