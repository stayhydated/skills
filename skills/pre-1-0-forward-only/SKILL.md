---
name: pre-1-0-forward-only
description: >
  Apply forward-only editing, documentation, and review rules for Rust crates or
  workspaces whose package or workspace version is below 1.0. Use when editing,
  reviewing, or documenting pre-1.0 Rust APIs, READMEs, AGENTS.md guidance,
  examples, release notes, migration text, or workflow docs: omit durable
  absent-surface negative prose such as "does not currently..." and avoid
  backward/legacy compatibility obligations, old API preservation, aliases,
  wrappers, or shims unless explicit repository policy or the user requires them.
  Keep review-only requests read-only and account for release compatibility.
---

# Pre-1.0 Forward Only

Use this skill to keep pre-1.0 Rust workspaces focused on the current API and
current repository shape. It pairs two rules:

1. Durable docs should route to the surfaces that exist instead of cataloging
   absent surfaces.
2. Pre-1.0 Rust crates and workspaces need not preserve obsolete APIs with
   compatibility bridges unless explicit project policy or the user requires
   them; release-versioning requirements still apply.

## Scope Check

Before applying the rules:

1. Read the relevant `Cargo.toml`.
2. Treat `workspace.package.version = "0.x.y"` as applying to every workspace
   crate that inherits it.
3. Treat a crate-local `version = "0.x.y"` as applying to that crate.
4. If the relevant version is `1.0.0` or later, or project docs promise
   compatibility, follow that compatibility policy instead.
5. If the user explicitly asks to preserve an old API, command, document shape,
   or migration path, do that and state the tradeoff.
6. Identify whether the request authorizes edits. Reviews and audits report
   findings without changing files, even when a fix is obvious. An editing
   request authorizes only its requested surfaces, not a repository-wide cleanup.
7. Before changing a published or externally consumed interface, inspect release
   policy, publication status, known consumers, and affected dependency ranges.
   Do not infer that a `0.x` version means there are no consumers.

## Release Compatibility

Forward-only development does not waive release-versioning requirements. Cargo
can treat multiple pre-1.0 versions as compatible: a dependency requirement of
`"0.4.2"` permits later `0.4` patch releases, but not `0.5.0`. Removing a public
API in `0.4.3` can therefore break consumers that accepted the earlier version.
Use [Cargo's version-requirement rules](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#version-requirement-syntax)
when evaluating an affected range.

Put a breaking interface change behind an appropriate incompatible release
boundary, or report the unresolved release requirement. Check persisted data,
protocols, and deployment boundaries separately when they are affected; a crate
version alone does not settle their compatibility requirements. This does not
require adding aliases, shims, or migration infrastructure by default.

Do not bump versions, publish releases, or change release policy solely because
this check finds a breaking change. Make those edits only when authorized;
otherwise record the release requirement in the handoff. Keep unpublished,
internal-only work proportional to its evidenced consumers and local policy.

## Durable Docs

For READMEs, `AGENTS.md`, crate docs, examples, and other durable repository
guidance:

- Omit absence inventory such as `does not currently contain`,
  `currently does not include`, `not yet`, `currently lacks`, and similar
  phrasing.
- Prefer positive routing to evidenced surfaces: name the command, crate,
  module, test, generated source, or document that owns the work.
- Remove stale references instead of replacing them with prose about missing
  paths, but apply removals only in an authorized editing request.
- Use negative wording only when it changes an action: `Do not edit generated
  output`, `do not commit secrets`, or `not validated because ...` in a handoff.
- Preserve evidenced limitations and compatibility constraints that affect user
  actions or safety; they are not absence inventory.
- Keep temporary uncertainty in the final response or review note, not in the
  durable repository guide.

Example rewrite pattern:

```md
Start with `just --list`; the `justfile` is the repository command index. The
main Rust workspace is defined in `Cargo.toml`.
```

## Forward-Only API Rule

For pre-1.0 crates and workspaces without an explicit compatibility promise:

- Treat the current API, command surface, route shape, generated output, and
  documented workflow as the contract, subject to the release boundary above.
- Apply this as task procedure; do not add a standing pre-1.0 policy sentence to
  durable docs or `AGENTS.md` solely because a manifest version is below `1.0`.
- Within an authorized change, move implementation, public exports, tests,
  examples, generated outputs, README text, crate docs, and `AGENTS.md` guidance
  to the current shape together when they name the changed surface. Report
  required follow-up changes that fall outside the requested write scope.
- Remove obsolete docs, tests, examples, and expectation files once they no
  longer describe the current behavior, only within the authorized edit scope.
- Avoid adding legacy aliases, compatibility wrappers, deprecated exports,
  migration layers, old command paths, or "keep the old API" wording solely for
  backward compatibility when an appropriate incompatible release is intended.
- If a compatibility bridge is explicitly required, mark it as user-requested or
  policy-backed and keep it scoped.

## Review and Editing Workflow

1. Confirm the pre-1.0 scope, requested write boundary, and any affected release
   compatibility requirements before invoking forward-only rules.
2. Search durable docs and guidance for absence-inventory phrasing.
3. Search affected code/docs for compatibility language such as
   `backward compatibility`, `backwards compatibility`, `legacy`, `deprecated`,
   `old API`, `compatibility shim`, and `migration`.
4. Inspect matches manually; these searches are prompts, not mechanical delete
   rules.
5. For a review or audit, report the finding, evidence, proposed fix, and release
   implications without modifying files. For an authorized editing request,
   patch only the owning implementation and supporting surfaces within scope.
6. Validate authorized edits with the narrowest command that proves the changed
   surface. In a read-only review, use non-mutating checks or state static review.
   Distinguish successful validation from attempted checks that failed.
