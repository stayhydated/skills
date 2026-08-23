---
name: use-subagents-codex
description: Orchestrate one or more Codex subagents for a user-requested task.
---

# Use Codex Subagents

Orchestrate Codex subagents for the requested task. Keep the skill and custom-agent identity stable across worker-model upgrades. The bundled TOML is the sole source of the subagent model and reasoning effort; the parent model and reasoning effort selected by the user are outside this managed profile.

## Execution contract

- Use at least one subagent unless multi-agent tooling is unavailable or doing so would violate a safety or permission boundary.
- Do not skip delegation merely because the parent could complete the task alone. For a small or tightly coupled task, use one subagent as the focused executor or independent verifier.
- The parent remains responsible for decomposition, user-facing decisions, synthesis, verification, and the final answer.

## Preserve the parent runtime

- Leave the parent model and reasoning effort exactly as selected for the current Codex CLI or ChatGPT Desktop Work mode session.
- Never set, override, or persist a parent `model` or `model_reasoning_effort` on behalf of this skill.
- Do not change model or reasoning settings in `$CODEX_HOME/config.toml`, a profile, command-line options, application settings, or the active parent session.
- Do not derive the subagent model or effort from the parent, and do not apply the subagent settings to the parent.
- Apply the bundled model and reasoning effort only to workers spawned by this skill.

## Reconcile the managed custom agent

The bundle owns one stable custom-agent identity with a bundle-controlled worker profile:

- bundled source: `assets/use-subagents-codex.toml`
- installed target: `$CODEX_HOME/agents/use-subagents-codex.toml`, falling back to `~/.codex/agents/use-subagents-codex.toml` when `CODEX_HOME` is unset
- custom-agent name: `use_subagents_codex`

Treat the bundled file as canonical. This managed target is intentionally replaced when the bundle changes so a successor worker model or revised worker effort can be adopted without renaming the skill or agent. Local customizations belong in a different custom-agent file with a different `name`.

1. Resolve this skill directory and the effective Codex home.
2. Compare the bundled source and installed target byte-for-byte using normal file tools. No helper script or Python runtime is required.
3. If they match, continue.
4. If the target is missing or differs, show the intended change. Obtain approval through the execution environment before writing outside the workspace.
5. When replacing an existing target, create a timestamped backup beside it. Then copy the bundled file atomically where supported and restrict permissions to the current user where supported.
6. Do not modify `$CODEX_HOME/config.toml` preemptively. If the effective runtime reports that multi-agent tools are disabled, explain the blocker and request approval before changing only `[agents].enabled` to `true`. Do not override managed policy or alter any model or reasoning setting.
7. Do not use a Codex version heuristic unless a concrete runtime failure requires compatibility diagnosis.

If the managed file was created or updated during the current session, do not assume the session reloaded it. Prefer an exact-runtime fallback from the next section, or require a fresh session rather than claiming the new definition ran.

## Select the worker

Use the first available path that preserves both worker settings selected by the bundle:

1. Spawn the custom agent named `use_subagents_codex` when the current session has already loaded the canonical definition.
2. Otherwise, only when the spawn surface accepts explicit model **and** explicit reasoning-effort overrides, read the top-level `model` and `model_reasoning_effort` values from `assets/use-subagents-codex.toml` and pass both to a built-in `worker`. Include the worker constraints below in the assignment.
3. Do not use an explicit-model fallback that cannot also preserve the bundled reasoning effort; it could accidentally inherit the parent's effort.
4. If neither exact path is available, report that the requested subagent profile cannot be executed in the current session. Do not silently substitute the parent model, another worker model, or another reasoning level.

When runtime metadata exposes the selected worker model, reasoning effort, or custom-agent path, verify it. Otherwise state only what the runtime actually confirms. Never treat the installed TOML alone as proof of the runtime used.

## Choose the delegation topology

Use the minimum number of workers that satisfies the explicit request:

- **One worker by default.** Use one for a coherent implementation, investigation, transformation, test pass, or independent verification.
- **Multiple workers only for independent work packages.** Parallelize when each package has distinct inputs or read scopes and can return a useful result without another worker's intermediate output.
- **Read-heavy work parallelizes more safely than writes.** For concurrent write work, assign disjoint files or directories and account for shared manifests, lockfiles, generated outputs, and migrations. If ownership overlaps, serialize the work or use one worker.
- Honor a user-specified worker count or decomposition when feasible, but reduce it when concurrency, permissions, or conflicting write scopes make that unsafe or ineffective. State the adjustment.

Do not let workers recursively spawn more agents.

## Build each assignment

Give every worker a self-contained brief containing:

1. objective and acceptance criteria;
2. allowed inputs and relevant context;
3. explicit read-only or write scope;
4. prohibited actions and stop conditions;
5. expected output format;
6. checks or evidence required before returning.

Use these constraints in every brief, including exact-runtime fallback workers:

- Work only on the bounded assignment.
- If the objective, inputs, scope, or acceptance criteria are missing or contradictory, stop and report the ambiguity instead of guessing.
- Remain read-only unless a write scope is explicitly assigned.
- Modify only assigned files or directories and preserve unrelated user changes.
- Do not broaden the task, make unrequested product or architecture decisions, publish changes, push commits, contact people, or perform irreversible external actions.
- Do not spawn subagents.
- Run targeted checks that could falsify the result when useful and permitted.
- Return the result, supporting evidence, files changed or `none`, checks and outcomes, and any blocker or uncertainty.

## Consolidate and verify

Wait for all requested workers. Resolve ambiguity and cross-worker conflicts in the parent rather than widening a worker's scope. Inspect material changes, reconcile outputs, and run proportional final verification. The final response must distinguish worker findings from parent verification and must not overstate worker model, reasoning-effort, or runtime certainty.
