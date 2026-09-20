# xtask

[![Codecov: xtask][codecov-badge]][codecov]

`xtask` is the repository's internal maintenance CLI for keeping Rust-based
skill documentation aligned. It checks tracked Rust versions against the stable
channel and validates executable Rust snippets embedded in skill Markdown.

## Commands

| Command | Purpose |
| --- | --- |
| `cargo xtask check-rust-stable` | Compare tracked Rust-version mentions with the current stable channel. |
| `cargo xtask check-skill-snippets` | Discover, lint, compile, and run Rust code fences from the skill bundles. |

Use each command's `--help` output for its focused filters and automation
options.

## Skill snippet checks

`cargo xtask check-skill-snippets` discovers Rust fences under `skills/`, builds
one generated test crate per skill, applies the workspace Clippy policy, and runs
the accepted examples. Generated manifests, source, and build output stay under
`target/skill-snippets/`.

The focused options are:

```sh
cargo xtask check-skill-snippets --list
cargo xtask check-skill-snippets --filter type-state-pattern
cargo xtask check-skill-snippets --skills-root /path/to/skills
cargo xtask check-skill-snippets --deny-ignored
```

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
