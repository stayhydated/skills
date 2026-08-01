# [TODO: Component name]

<!-- Replace every TODO and remove every section that does not support the maintenance decision. -->

[TODO: State the component's responsibility, boundary, and the main fact a maintainer must know before changing it.]

## Responsibilities

### Owns

- [TODO: Responsibility]

### Does not own

- [TODO: Neighboring responsibility handled elsewhere]

## Interface

| Input or event | Preconditions | Result | Errors or rejection |
|---|---|---|---|
| `[TODO: Input]` | [TODO: Required state] | [TODO: Observable result] | [TODO: Failure contract] |

## State and invariants

| Invariant | Enforced by | Evidence |
|---|---|---|
| [TODO: Condition that must remain true] | `[TODO: Code, schema, or transaction]` | [TODO: Test or assertion] |

## Normal flow

[TODO: Describe the trigger through meaningful completion. Add a small supported sequence diagram only when actor order matters.]

## Concurrency and ordering

[TODO: Describe ownership, synchronization, idempotency, backpressure, and ordering only where applicable.]

## Failure behavior

| Failure | Retry safe? | Cleanup or compensation | Signal |
|---|---:|---|---|
| [TODO: Failure class] | [TODO: Yes, no, or conditional] | [TODO: Behavior] | [TODO: Log, metric, error, or trace] |

## Configuration and operations

| Setting or signal | Default or expected value | Effect or interpretation |
|---|---|---|
| `[TODO: Name]` | `[TODO: Value]` | [TODO: Meaning] |

## Modify or extend safely

1. [TODO: Supported extension point or change sequence.]
2. [TODO: Invariant or compatibility constraint to preserve.]
3. [TODO: Tests and validation to run.]

## Source map

- **Implementation:** `[TODO: Repository path]`
- **Contract tests:** `[TODO: Repository path]`
- **Schema or configuration:** `[TODO: Repository path]`
- **Related decision:** [TODO: Add a relative chapter link when applicable]
