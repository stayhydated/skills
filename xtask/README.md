# xtask

[![Codecov: xtask][codecov-badge]][codecov]

`xtask` is the repository's internal maintenance CLI for keeping Rust-based
skill documentation aligned. It checks tracked Rust versions against the stable
channel and validates executable Rust snippets embedded in skill Markdown.

Requires Rust 1.99 or newer. This is the tool's declared minimum; individual
skill examples have their own generated-crate setup.

## Commands

| Command | Purpose |
| --- | --- |
| `cargo xtask check-rust-stable` | Compare tracked Rust-version mentions with the current stable channel. |
| `cargo xtask check-skill-snippets` | Discover, lint, compile, and run Rust code fences from the skill bundles. |

Run commands from the workspace root. `check-rust-stable` reads Rust version
mentions under `skills/` and fetches the current stable channel manifest. It
prints the comparison and any proposed sync issue; creating an issue requires
the explicit `--create-issue` option and GitHub environment configuration.

Use each command's `--help` output for its focused filters and automation
options.

## Skill snippet checks

`cargo xtask check-skill-snippets` discovers Rust fences under `skills/`, builds
one generated test crate per skill, applies the workspace Clippy policy, and runs
the accepted examples. Generated manifests, source, and build output stay under
`target/skill-snippets/`.

For example, list and then validate the maintained type-state examples:

```sh
cargo xtask check-skill-snippets --list --filter type-state-pattern
cargo xtask check-skill-snippets --deny-ignored --filter type-state-pattern
```

The first command reports matching file locations and fence modes without
compiling them. The second lints, compiles, and runs those examples, and rejects
ignored Rust fences. `--filter` matches a Markdown file's relative path;
`--skills-root` selects another skill directory.

The common Rust fence modes are:

- `rust`: lint and compile items, execute statements, and run contained tests.
- `rust,no_run`: lint and compile without executing contained tests.
- `rust,compile_fail`: require compilation to fail.
- `rust,should_panic`: require the example to panic through rustdoc.
- `rust,ignore (reason)`: record an explicit exclusion; the repository's strict
  check rejects ignored snippets.

Edition attributes and `standalone_crate` retain their rustdoc semantics. The
checker rejects unknown Rust fence attributes.

Other languages, untagged fences, inline code, and Rust fences shown inside an
outer Markdown example are outside the snippet check. The reported count covers
Rust snippets rather than shell commands, template fragments, or agent behavior.

[codecov-badge]: https://codecov.io/github/stayhydated/skills/graph/badge.svg?token=34CV04UOU1&component=xtask
[codecov]: https://codecov.io/github/stayhydated/skills
