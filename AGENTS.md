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
- `xtask/src/main.rs` owns the Rust-stable sync command used by
  `.github/workflows/check-rust-stable.yml`.

## Quick Decision Flow

1. Find the owning surface in the workspace map before editing.
2. For skill behavior, activation, or routing changes, edit the owning
   `SKILL.md` and any referenced support files in the same skill directory.
3. For displayed skill names, short descriptions, or default prompts, keep the
   matching `agents/openai.yaml` aligned with the skill frontmatter and user
   instructions.
4. For Rust baseline guidance, update all affected `skills/` mentions together;
   the `xtask` sync command scans files under `skills/` for tracked Rust minor
   versions.
5. For Rust-stable sync tooling changes, keep `xtask/src/main.rs` and
   `.github/workflows/check-rust-stable.yml` aligned when CLI flags,
   environment variables, issue text, or the workflow invocation changes.
6. Validate with the smallest evidenced command that proves the edited surface.

## Workspace Map

### Skill Bundles

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

### Rust Maintenance Tooling

- `xtask/`
  Role: `cargo run --locked -p xtask -- check-rust-stable` checks the current
  Rust stable channel against Rust minor versions mentioned under `skills/`.
  Sync: if CLI flags, environment variables, issue text, or scan behavior
  change, update tests in `xtask/src/main.rs` and the GitHub workflow invocation
  when applicable.

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
- Use `just ci` for the full local suite when a change spans skill text, Rust
  tooling, manifests, and CI wiring.
- For local Rust-stable sync checks, run
  `cargo run --locked -p xtask -- check-rust-stable` without `--create-issue`.
- Do not claim validation ran unless the command was actually executed; if a
  command is skipped, state what remains unvalidated.
