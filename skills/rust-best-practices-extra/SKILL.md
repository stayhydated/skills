---
name: rust-best-practices-extra
description: >
  Add Bon builders, Statum lifecycles, dependency adoption policy, Strum/i18n
  boundaries, code generation, and Rust 1.99 guidance to BeautifulRust's
  rust-best-practices. Use alongside that core when writing, refactoring,
  reviewing, optimizing, or documenting Rust; apply these explicit preferences
  and overrides within the requested scope.
---

# Rust Best Practices Extra

Use [BeautifulRust's `rust-best-practices`](https://github.com/morganlinton/BeautifulRust/tree/main/skills/rust-best-practices)
as the general Rust baseline. Load its installed entry point together with this
skill and read supporting references only for the task at hand. If the core is
unavailable, apply these additions and report that the core was not loaded;
do not silently install or substitute another same-named skill.

This stable companion was validated with
[BeautifulRust revision `2b2d53a`](https://github.com/morganlinton/BeautifulRust/tree/2b2d53a66f6f36ab4b8d6a93070183d91e77fad4/skills/rust-best-practices).
When updating the core, review its changed guidance against these overrides.

User instructions and repository requirements govern the work. Within those
constraints, the additions and explicit overrides below take precedence over
conflicting advice in the core or its source digests. Core guidance owns the
remaining general Rust idioms; keep house style distinct from correctness.

Assume **Rust 1.99 stable** and **edition 2024** unless the repository or user
specifies another target. Respect toolchain files, MSRV, features, targets,
`no_std`, workspace lints, and release policy.

## Overrides When Using Both Skills

- **Construction:** use Bon for suitable builders, including struct, function,
  and method builders. Its consuming setters and compile-time required fields
  replace a blanket preference for manual `&mut self` builders or universally
  fallible `build()`. Put runtime validation in the constructor or function body.
  A simple constructor or struct literal remains appropriate for simple inputs.
- **Lifecycles:** use Statum when phases change which operations are legal. Use
  an enum for local runtime branching and Bon for construction. Bon and Statum
  macros may transform declarations and signatures as designed; this is an
  intentional exception to the core's macro restrictions.
- **Named booleans:** a clearly named boolean builder setter can express an
  independent flag. Use an enum when the domain represents distinct modes.
- **Testing:** assert observable state, outputs, artifacts, and public protocol
  effects. Use real boundaries, deterministic stubs, or small fakes as needed.
  Do not require `new_mocked()`, mock-object call expectations, or a mocking
  framework. Use `rust-test` for detailed test strategy when available.
- **Validation:** use evidenced repository commands and the narrowest relevant
  package, feature, target, and MSRV lanes. Use `--all-features` only when the
  features are compatible. Do not install audit tools, introduce quality gates,
  change lint policy, or add tests for every reversible edit solely to satisfy
  the core's checklists. Report executed checks and remaining evidence gaps.
- **Domain constraints:** preserve framework-required thread affinity, ownership,
  and public type contracts. `Send`/`Sync`, pointer hiding, and plain-integer API
  preferences are design guidance, not reasons to weaken an invariant or replace
  a necessary single-threaded type.
- **Release policy:** use `pre-1-0-forward-only` when applicable and available;
  an authorized incompatible pre-1.0 change need not gain deprecation shims solely
  to satisfy the core's general evolution advice. Respect consumers and release
  boundaries.

## Dependency Adoption

Bon (`bon`) and Statum (`statum`) are authorized by default in both new and
existing repositories. Use Bon for suitable builder work and Statum for real
lifecycle or protocol type-state work; type-state is not a generic replacement
for builders or enums. Add the appropriate dependency when needed for the
requested implementation or refactor; do not ask the user to
approve these dependencies separately. Their absence from `Cargo.toml`, an
existing manual implementation, or a dependency-free crate is not an opt-out.

Inspect the applicable cargo-deny configuration for an explicit opt-out, including
crate- and version-specific bans on `bon` or `statum`. It is commonly `deny.toml`,
`.deny.toml`, or `.cargo/deny.toml`; follow the configuration selected by repository
tooling when it uses a different path. With no matching ban, proceed without a
dependency-approval prompt. A missing cargo-deny configuration is not a blocker
and does not require introducing cargo-deny. If a ban applies, identify its file
and rule and use a permitted alternative. Do not remove or weaken the rule to
force adoption. Honor an explicit user request to avoid a dependency or use a
manual pattern.

Default authorization does not waive MSRV, feature, `no_std`, target, public API,
license, source, or security constraints, including restrictions affecting
transitive dependencies. Select compatible versions and follow the repository's
manifest and lockfile conventions. Report a concrete incompatibility rather than
inventing an approval requirement. Keep adoption within the requested change;
do not migrate unrelated constructors, state machines, or APIs.

For other libraries, including Strum, template engines, and error-handling
libraries, preserve established dependencies, versions, patterns, and dependency
policy unless adoption or migration is part of the requested scope. Apply their
house-style preferences when already adopted or explicitly authorized; otherwise
use the established implementation and label alternatives as recommendations.
The Bon and Statum default authorization takes precedence over generic
adoption-or-authorization language in the references.

Reviews are read-only unless the user asks to apply changes. Separate correctness
findings from house-style suggestions; a manual implementation is not a defect
solely because a preferred library could generate it.

## References

Read only the references relevant to the requested work:

- [Builder Pattern](references/builder-pattern.md): Bon struct, function, method,
  and explicitly overwritable setters.
- [Type State Pattern](references/type-state-pattern.md): Statum lifecycle
  transitions, persisted-state validation, and the builder boundary.
- [Enum String Conversions](references/enum-string-conversions.md): Strum static
  labels, parse aliases, discriminants, and localization ownership.
- [Code Generation Templates](references/code-generation-templates.md): typed
  models, file renderers, token emitters, escaping, diagnostics, and generated
  output validation.
- [Workspace and Toolchain Preferences](references/workspace-and-toolchain.md):
  inherited dependencies, warning policy, profiles, and scoped CI checks.
- [Rust and Cargo Baseline Additions](references/rust-cargo-baseline.md): selected
  version-sensitive APIs, compiler diagnostics, and tooling contracts.
