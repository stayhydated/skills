# Agent Skills

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]

Agent Skills is a collection of focused instructions and supporting resources for
coding agents working with Rust repositories, mdBook documentation, and workspace
guidance. Choose the skill that matches the task below.

## Skills

| Skill | Use it for |
| --- | --- |
| [`manage-workspace-agents-md`][manage-workspace-agents-md] | Explicitly requested creation, revision, or audit of repository `AGENTS.md` guidance. |
| [`mdbook-internals`][mdbook-internals] | Maintainer-facing architecture, operations, and design documentation in mdBook. |
| [`mdbook-user-docs`][mdbook-user-docs] | User-facing tutorials, guides, concepts, references, and troubleshooting in mdBook. |
| [`pre-1-0-forward-only`][pre-1-0-forward-only] | Forward-only API and documentation work in pre-1.0 Rust projects. |
| [`rust-best-practices-extra`][rust-best-practices-extra] | Bon, Statum, dependency policy, and focused additions to BeautifulRust's Rust guidance. |
| [`rust-readme`][rust-readme] | Concise, evidence-backed Rust README creation and review. |
| [`rust-test`][rust-test] | Rust test design, implementation, audit, and validation strategy. |

Use [`rust-best-practices` from BeautifulRust][beautiful-rust] as the general Rust
core alongside this repository's `rust-best-practices-extra`. The extra skill
defines our preferences and resolves conflicts while preserving user and
repository constraints.

`rust-best-practices-extra` is a stable companion skill.

[ci-badge]: https://github.com/stayhydated/skills/actions/workflows/ci.yml/badge.svg?branch=master
[ci]: https://github.com/stayhydated/skills/actions/workflows/ci.yml
[codecov-badge]: https://codecov.io/github/stayhydated/skills/graph/badge.svg?token=34CV04UOU1
[codecov]: https://codecov.io/github/stayhydated/skills
[manage-workspace-agents-md]: skills/manage-workspace-agents-md/SKILL.md
[mdbook-internals]: skills/mdbook-internals/SKILL.md
[mdbook-user-docs]: skills/mdbook-user-docs/SKILL.md
[pre-1-0-forward-only]: skills/pre-1-0-forward-only/SKILL.md
[rust-best-practices-extra]: skills/rust-best-practices-extra/SKILL.md
[beautiful-rust]: https://github.com/morganlinton/BeautifulRust/tree/main/skills/rust-best-practices
[rust-readme]: skills/rust-readme/SKILL.md
[rust-test]: skills/rust-test/SKILL.md
