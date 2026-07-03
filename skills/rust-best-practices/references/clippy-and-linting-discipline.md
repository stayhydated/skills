# Clippy and Linting Discipline

Clippy is part of the Rust toolchain. Do not hard-code a Clippy version in docs or
scripts; ask the toolchain what is installed.

```sh
rustc -Vv
cargo clippy -V
rustup component add clippy
```

## Default Commands

Use a strict command for local review and CI:

```sh
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --all-features --locked
cargo doc --workspace --all-features --no-deps --locked
```

Use `--all-features` when features are additive and compatible. If features are
mutually exclusive, test the documented feature matrix instead of pretending all
features can be enabled together.

## Workspace Lints

Prefer central lint policy in the workspace root, with crates opting in through
`[lints] workspace = true`.

```toml
[workspace.lints.rust]
future-incompatible = "deny"
nonstandard_style = "deny"
unsafe_op_in_unsafe_fn = "deny"
unused_must_use = "deny"

[workspace.lints.rustdoc]
broken_intra_doc_links = "deny"
bare_urls = "deny"

[workspace.lints.clippy]
all = "deny"
clone_on_copy = "deny"
large_enum_variant = "deny"
manual_ok_or = "deny"
needless_collect = "deny"
redundant_clone = "deny"
unnecessary_wraps = "deny"
```

In member crates:

```toml
[lints]
workspace = true
```

Consider `clippy::pedantic` and `clippy::nursery` as review aids, not automatic
policy for every repository. Avoid `clippy::restriction` as a group; enable only
specific restriction lints that match team policy.

## Fix Warnings Before Suppressing Them

A lint suppression is a local design decision. Prefer `#[expect]` with a reason so
future Clippy runs warn when the expectation becomes stale.

```rust
#[expect(
    clippy::large_enum_variant,
    reason = "wire frames are matched in a hot loop and profiling favors locality"
)]
enum WireFrame {
    Ping,
    Payload([u8; 2048]),
}
```

When the warning is correct, fix the code instead:

```rust
enum WireFrame {
    Ping,
    Payload(Box<[u8; 2048]>),
}
```

Do not use crate-wide `#![allow(...)]` unless the crate has a documented policy
reason. Local `#[expect]` is usually easier to audit.

## Lints Worth Treating as Design Feedback

| Lint | Design signal |
| --- | --- |
| `redundant_clone` | Ownership can likely be simplified. |
| `clone_on_copy` | A small value is being cloned needlessly. |
| `needless_borrow` | Borrowing is obscuring type flow. |
| `manual_ok_or` | `Option`/`Result` conversion can be expressed directly. |
| `needless_collect` | An intermediate allocation may be avoidable. |
| `large_enum_variant` | Enum layout may waste stack or cache space. |
| `unnecessary_wraps` | The function's return type may overstate fallibility. |
| `missing_errors_doc` | A public fallible API needs better caller guidance. |

Treat lints as prompts for design review, not as mechanical chores.

## Compatibility and MSRV

Clippy evolves with the compiler. When a repository has an MSRV lower than the
active toolchain, pin CI to the MSRV plus stable, or document that lint policy is
checked on stable only.

For libraries, avoid "fixing" a lint by raising MSRV unless the release notes and
crate policy justify that MSRV bump.
