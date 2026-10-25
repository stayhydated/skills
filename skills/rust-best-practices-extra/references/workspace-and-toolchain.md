# Workspace and Toolchain Preferences

These are house-style additions to BeautifulRust. Preserve existing manifests,
profiles, and CI unless changing them is within the user's request.

## Shared Dependencies

Prefer declaring shared direct dependencies in the workspace root's
`[workspace.dependencies]` table. Inherit them in member manifests with the
explicit `{ workspace = true }` inline-table form when the members can share the
same version and source. Prefer this form to dotted
`dependency.workspace = true` syntax so member-specific settings such as
`features` can be added to the same declaration. Keep a declaration member-local,
or use explicitly renamed workspace dependencies, when members genuinely
require different direct versions, sources, or feature defaults that their
supported Cargo versions and editions cannot express through inheritance. A transitive
disagreement, such as two dependencies requiring semver-incompatible versions
of a third crate, is not by itself a reason to keep the direct dependencies
local: Cargo resolves transitive versions independently and may build multiple
semver-incompatible versions.
Account for resolver failures and crates whose native `links` declarations
prevent multiple versions from coexisting.

On Cargo 1.99, edition-2024 members can override inherited `default-features`,
including `{ workspace = true, default-features = false }` when the workspace
enables defaults. This does not disable features enabled by other dependency
edges. Inspect feature unification and validate lower-Cargo lanes before using
this form; earlier editions and toolchains do not provide the same override.
See [inheriting workspace dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace).

## Cargo 1.99 Warning Policy

When a repository intentionally requires every ordinary Cargo build, check, or
test of local packages to reject lint warnings, use Cargo's `build.warnings`
configuration instead of a global `RUSTFLAGS=-Dwarnings` override:

```toml
[build]
warnings = "deny"
```

Use this when Rust 1.99 is the effective toolchain baseline and the repository has
adopted warning-free builds as policy. The setting was stabilized in Cargo 1.97;
it is not new in 1.99. Keep an explicit
`cargo clippy ... -- -D warnings` command for Clippy-specific review. When a
separate lower-MSRV lane exists, validate its behavior independently instead of
assuming it interprets the active Cargo configuration identically.

## CI Quality Jobs

Prefer explicit CI jobs for documentation and package validation instead of
burying those checks inside a broad test script:

```sh
cargo doc --workspace --all-features --no-deps --locked
cargo publish --workspace --dry-run --locked
```

For virtual workspaces with release-order tooling, use the repository's release
plan or package dry-run command instead of forcing a plain workspace publish. For
intentionally unpublished workspaces, use the narrow package assembly check that
matches the repository's packaging contract.

Do not add Linux mold linker setup as a default CI or `.cargo/config.toml`
requirement. Keep linker overrides opt-in and repository-evidenced; CI should
prefer the stock runner toolchain unless the repository documents a real linker
requirement.

## Workspace Profiles

Profile tuning is workload-specific, not an idiomatic-Rust requirement. Preserve
the repository's profiles unless tuning is in scope. For a runtime-heavy
workspace targeting Rust 1.99, this is one candidate to measure against Cargo's
defaults, not a universal build-speed or runtime improvement:

```toml
[profile.dev]
codegen-units = 16
debug = "limited"
opt-level = 1
split-debuginfo = "unpacked"

[profile.dev.package."*"]
opt-level = 3

[profile.release]
codegen-units = 1
lto = "thin"
opt-level = 3
```

Cargo 1.99 adds a built-in `debug` profile, which currently inherits `dev`.
The profile name is distinct from the `debug` debuginfo setting above. Do not
rename existing profiles mechanically. Incremental compilation is disabled by
default when Cargo detects CI through the `CI` environment variable; inspect
explicit configuration and `CARGO_INCREMENTAL` before comparing build timings.
See [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
and the [Rust 1.99 Cargo changes](https://doc.rust-lang.org/releases.html#cargo).

Use `debug = "limited"` instead of full debug info when stack traces and module
context are enough. Optimize non-workspace dependencies in dev profiles for UI,
graphics, parser, and async-heavy projects where dependency hot paths dominate
runtime behavior.

ThinLTO and fewer codegen units can improve optimized output at the cost of build
time; measure the trade-off instead of promising a universal speedup. Verify
`split-debuginfo` support on the repository's targets. Add `panic = "abort"` or
`strip = true` only for deliverable binaries that intentionally trade panic
unwinding or symbols for size.
