# Rust README research and source boundaries

These maintained Rust projects are structural references, not a claim that all
Rust repositories follow one README standard. The `rust-readme` house policy is
intentionally stricter: only CI, Codecov, Book, and crates.io badges in that
order; no README license content, installation recipes, or book chapter routes.
Recheck upstream files before relying on details that may have changed.

## Repository examples

| Project and source | Useful pattern | Adaptation for this skill |
| --- | --- | --- |
| [Bytes README](https://github.com/tokio-rs/bytes/blob/master/README.md) and [manifest](https://github.com/tokio-rs/bytes/blob/master/Cargo.toml) | A single-package library identifies its purpose briefly and provides status, registry, and API-documentation routes. | Keep package-focused orientation; use the fixed badge order and omit dependency setup and license prose. |
| [Tokio README](https://github.com/tokio-rs/tokio/blob/master/README.md) | A project overview leads into a concrete example and maintained documentation, while crate-specific resources remain identifiable. | Separate workspace orientation from member ownership; do not copy the long release history, installation snippet, or license section. |
| [clap README](https://github.com/clap-rs/clap/blob/master/README.md) | A compact landing page states the purpose and directs readers to API docs and examples. | Retain the concise routing approach, not its installation command, license text, or additional badge categories. |
| [mdBook README](https://github.com/rust-lang/mdBook/blob/main/README.md) | A small project introduction delegates detailed feature, installation, and usage material to a maintained user guide. | Make the deployed guide's root a Book badge; keep contribution routing separate and omit its license badge and section. |

Do not paste upstream README text or badge URLs into a target repository.
A popular project's license section, badge order, or onboarding recipe is not
an exception to the requested policy. The useful common pattern is a clear
identity and a small number of routes to maintained detail.

## Authoritative mechanics

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
not to these maintainer-facing research references. These references document
why the authoring and validation rules exist; they are not README sections.
