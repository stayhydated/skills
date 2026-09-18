# Cargo, example, and badge reference mechanics

These references explain Cargo metadata, publication, Rust example conventions,
and badge URL mechanics. They are not README layout examples or an alternative
authoring policy. Follow `SKILL.md` and the bundled layouts for structure,
exclusions, and the fixed CI, Codecov, Book, crates.io badge order. Recheck
technical details before relying on behavior that may have changed.

## Authoritative mechanics

- [Rust API Guidelines: fallible examples](https://rust-lang.github.io/api-guidelines/documentation.html#c-question-mark)
  explains why public examples should demonstrate `?` rather than unwrapping
  routine failures. Apply the example guidance to the actual public API and
  supported toolchain, not as a reason to import crate-doc layouts into READMEs.
- [Cargo manifest: README](https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field)
  explains registry rendering, explicit README paths, default discovery, and
  `readme = false`.
- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
  distinguishes root packages from virtual manifests and documents membership
  and metadata inheritance. An inherited README path is relative to the
  workspace root; an ordinary package README path is package-relative.
- [Cargo manifest: publication](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field)
  describes publication permissions, which do not prove that a package has
  actually been published. Verify the real crates.io page separately.
- [GitHub workflow badges](https://docs.github.com/en/actions/how-tos/monitor-workflows/add-a-status-badge)
  documents workflow-file URLs and branch/event selection. Use the actual
  workflow, not a guessed name or a blanket passing label.
- [Codecov status badges](https://docs.codecov.com/docs/status-badges)
  describes badges backed by Codecov data. Use the target project's own badge
  and report scope rather than substituting a generic coverage claim.
- [Shields crates.io version badge](https://shields.io/badges/crates-io-version)
  provides the package-version image; the surrounding Markdown link must still
  target the correct crates.io package page.
- [Shields static badge](https://shields.io/badges/static-badge)
  supplies a navigation label for an existing deployed book. It is not a health
  check or proof of deployment.

The prohibition on chapter links applies to READMEs authored with this skill,
not to these maintainer-facing technical references. These references document
why the authoring and validation rules exist; they are not README sections.
