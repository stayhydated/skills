# AGENTS.md

This is the working guide for contributors and coding agents in the `skills`
workspace.

Use it to decide where a change belongs, which skill metadata must stay in sync,
which Rust maintenance surfaces are affected, and which local validation command
is the narrowest proof for the edit.

Start here:

- `skills/*/SKILL.md` is the entry point for each skill bundle.
- When a task names a skill from this repository, use the checked-out
  `skills/*/SKILL.md` and referenced files in that same skill directory rather
  than a globally installed skill with the same name.
- `skills/*/agents/openai.yaml` carries the skill's OpenAI display metadata and
  default prompt text.
- `justfile` is the local command index; run `just --list` before adding or
  changing validation commands.
- `scripts/check_skills.py` validates every skill bundle against the Agent Skills
  contract and this repository's required OpenAI display metadata shape.
- `scripts/test_skills.py` tests skill validation and worker-profile installation.
  `scripts/skill-behavior-cases.md` defines separate agent evaluations.
- `xtask/src/main.rs` starts the CLI; `xtask/src/cli.rs` parses and dispatches
  commands. Each command owns its implementation and resources under
  `xtask/src/commands/`.

## Quick Decision Flow

1. Find the owning surface in the workspace map before editing.
2. For skill behavior, activation, or routing changes, edit the owning
   `SKILL.md` and any referenced support files in the same skill directory.
3. For displayed skill names, short descriptions, or default prompts, keep the
   matching `agents/openai.yaml` aligned with the skill frontmatter and user
   instructions.
4. For skill contract or OpenAI metadata shape changes, keep
   `scripts/check_skills.py`, `scripts/test_skills.py`, the `just check-skills`
   recipe, and the CI `skills` job aligned.
5. For Rust baseline guidance, update all affected `skills/` mentions together;
   the `xtask` sync command scans files under `skills/` for tracked Rust minor
   versions.
6. For Rust-stable sync tooling changes, keep `xtask/src/commands/check_rust_stable/` and
   `.github/workflows/check-rust-stable.yml` aligned when CLI flags,
   environment variables, issue text, or the workflow invocation changes.
7. Validate with the smallest evidenced command that proves the edited surface.

## Workspace Map

### Skill Bundles

- `skills/mdbook-internals/`
  Role: maintainer-facing mdBook architecture, component, flow, operations, and
  design-decision documentation guidance.
  Sync: keep `SKILL.md`, `references/`, `assets/`, and `agents/openai.yaml`
  aligned when workflow, resource routing, or visible metadata changes.

- `skills/mdbook-user-docs/`
  Role: user-facing mdBook tutorial, how-to, concept, reference, migration, and
  troubleshooting guidance.
  Sync: keep `SKILL.md`, `references/`, `assets/`, and `agents/openai.yaml`
  aligned when workflow, resource routing, or visible metadata changes.

- `skills/pre-1-0-forward-only/`
  Role: durable guidance for forward-only pre-1.0 Rust workspace edits.
  Sync: keep `SKILL.md` and `agents/openai.yaml` aligned when activation text,
  display text, or default prompts change.

- `skills/rust-best-practices/`
  Role: Rust implementation, review, optimization, API, and documentation
  guidance.
  Sync: keep `SKILL.md`, `references/`, and `agents/openai.yaml` aligned when
  the Rust baseline, referenced chapters, or visible skill description changes.

- `skills/rust-test/`
  Role: Rust test strategy and validation guidance.
  Sync: keep `SKILL.md`, `patterns/`, `checklists/`, and `agents/openai.yaml`
  aligned when test categories, Rust-version-specific guidance, or validation
  wording changes.

- `skills/manage-workspace-agents-md/`
  Role: evidence-based `AGENTS.md` creation, patching, audit, alignment, and
  checklist guidance.
  Sync: keep `SKILL.md`, `patterns/`, `templates/`, `checklists/`, and
  `agents/openai.yaml` aligned when modes, handoff wording, evidence rules, or
  validation wording change.

- `skills/use-subagents-codex/`
  Role: user-requested Codex subagent orchestration and management of the
  bundled worker profile.
  Sync: keep `SKILL.md`, `assets/use-subagents-codex.toml`,
  `scripts/configure_worker_profile.py`, and `agents/openai.yaml` aligned when
  orchestration behavior, worker settings, installation requirements, or visible
  metadata changes. Cover renderer changes in `scripts/test_skills.py` at the
  repository root.

- `skills/use-windows-vm-computer-use-codex/`
  Role: SSH-orchestrated, VM-local Codex computer-use in interactive Windows
  sessions, including forced full-access execution, approval, task lifecycle,
  and troubleshooting guidance.
  Sync: keep `SKILL.md` and `agents/openai.yaml` aligned when activation text,
  workflow boundaries, full-access guidance, or display text changes.

### Skill Contract Validation

- `scripts/check_skills.py`
  Role: validates every immediate directory under `skills/` with the pinned
  Agent Skills reference validator, then checks progressive-disclosure resources
  and `agents/openai.yaml` metadata.
  Sync: keep `scripts/test_skills.py`, the `just check-skills` recipe, and the CI
  `skills` job aligned when the command, dependency pin, or validated metadata
  shape changes.

- `scripts/test_skills.py`
  Role: isolated filesystem tests for the worker renderer and fixtures for
  skill-validator behavior. The script uses the same pinned reference-validator
  dependency as `scripts/check_skills.py`.
  Sync: keep tests aligned with validator and renderer contracts. The
  `WorkerProfileTests` subset runs without third-party Python packages; the full
  suite runs through `just check-skills` and the CI `skills` job.

- `xtask/src/commands/check_skill_snippets/`
  Role: discover Rust code fences under `skills/`, generate one Rustdoc test
  crate per skill under `target/skill-snippets/`, and attach every Markdown file
  from that skill with `include_str!`. Per-skill dependency setups live under
  `xtask/src/commands/check_skill_snippets/setups/`.
  Sync: keep those setups, source examples, the `just check-skill-snippets`
  recipe, and the CI `skill-snippets` job aligned. See `xtask/README.md` for fence
  attributes, exclusions, and focused checks.

- `scripts/skill-behavior-cases.md`
  Role: agent-evaluation scenarios for reporting modes, validation claims,
  runtime settings, release boundaries, audience routing, and dependency scope.
  Sync: update affected cases when those skill contracts change. These scenarios
  require an actual agent run; tooling tests do not establish their results.

### Rust Maintenance Tooling

- `xtask/`
  Role: `cargo xtask check-rust-stable` checks the current
  Rust stable channel against Rust minor versions mentioned under `skills/`.
  Implementation: `xtask/src/commands/check_rust_stable/` separates command
  orchestration, channel manifests, version scanning, GitHub access, and reporting.
  Sync: if CLI flags, environment variables, issue text, or scan behavior
  change, update the colocated unit tests, `xtask/tests/check_rust_stable.rs`, and
  the GitHub workflow invocation when applicable.

- `.github/workflows/check-rust-stable.yml`
  Role: scheduled and manual workflow that runs the Rust-stable sync command with
  `--create-issue`.
  Sync: keep it aligned with `xtask` CLI and environment variable changes.

## Validation and Editing Rules

- Use `just --list` as the command index instead of inventing new validation
  commands.
- Use `just fmt` for repository formatting when Markdown, Rust, or TOML formatting
  is part of the change.
- Use `just check`, `just clippy`, or `just test` for focused Rust workspace
  validation when the edited surface affects typechecking, lints, or tests.
- Use `just check-skills` after changing a skill's `SKILL.md`, bundled resources,
  `agents/openai.yaml`, or skill-validation tooling. It runs both contract
  validation and tooling regression tests.
- Use `just check-skill-snippets` after changing Rust examples or their runner.
  It requires Cargo and resolves each skill's example dependencies in generated crates under `target/`
  without changing the workspace lockfile or accepting snapshots.
- Use `just ci` for the full local suite when a change spans skill text, Rust
  tooling, manifests, and CI wiring.
- For local Rust-stable sync checks, run
  `cargo xtask check-rust-stable` without `--create-issue`.
- Report successful checks separately from attempted checks that failed. Do not
  claim validation ran unless it was executed; if a command is skipped, state
  what remains unvalidated.
