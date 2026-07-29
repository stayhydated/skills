# [TODO: System or subsystem name]

<!-- Replace every TODO and remove every section that does not support the maintenance decision. -->

[TODO: State the scope, explicit non-scope, and main architectural conclusion in one short paragraph.]

## Boundaries

| Component | Responsibility | State owned | External dependencies |
|---|---|---|---|
| `[TODO: Component]` | [TODO: Single responsibility] | [TODO: Durable or transient state] | [TODO: Dependencies] |

<!-- Add one supported diagram only when it contributes information beyond the table. Do not assume Mermaid is configured. -->

## Principal flow

[TODO: Describe the trigger, the principal path, and the meaning of successful completion.]

[TODO: Explain acknowledgment, durability, ordering, trust, or asynchronous boundaries that a diagram cannot guarantee by itself.]

## Contracts and invariants

| Contract or invariant | Enforcement point | Evidence | Consequence if violated |
|---|---|---|---|
| [TODO: Statement] | `[TODO: Symbol, schema, or boundary]` | [TODO: Test or constraint] | [TODO: Impact] |

## Failure and recovery

| Failure | Detection | Propagation | Recovery |
|---|---|---|---|
| [TODO: Failure class] | [TODO: Signal] | [TODO: Caller or component effect] | [TODO: Automatic or manual action] |

## Security and trust boundaries

[TODO: Describe authentication, authorization, validation, secret handling, and untrusted inputs only where relevant.]

## Performance and scaling

[TODO: Identify the scaling dimension, bounded resources, expensive path, and known limit.]

## Operations

| Signal or setting | Meaning | Action |
|---|---|---|
| `[TODO: Metric, log, trace, or configuration]` | [TODO: Interpretation] | [TODO: Response or tuning guidance] |

## Change guide

- **Extension points:** [TODO: Supported change surfaces]
- **Hazards:** [TODO: Invariants or compatibility constraints]
- **Required tests:** [TODO: Specific suites or scenarios]
- **Owners:** [TODO: Team or stable ownership source]

## Sources

- **Implementation:** `[TODO: Repository path or generated reference]`
- **Tests or schemas:** `[TODO: Repository path]`
- **Decision record:** [TODO: Add a relative chapter link when applicable]
