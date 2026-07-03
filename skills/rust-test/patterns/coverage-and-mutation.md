# Coverage and mutation-testing patterns

Use this pattern when the user asks whether Rust tests are strong enough, whether coverage should be improved, or whether mutation testing would expose weak assertions.

## Principle

Coverage and mutation results are supporting evidence, not proof of correctness. They can reveal unexercised code or assertions that fail to detect behavior changes, but they do not replace contract-focused tests, property/fuzz validation, doctests, compile-fail cases, or integration/e2e coverage where those are the right fit.

## Coverage guidance

Coverage is useful for:

- finding public branches, error paths, feature-gated code, and target-specific paths that are never exercised;
- checking whether new regression tests actually touch the intended code;
- spotting dead or unreachable test scaffolding;
- supporting risk discussion in audits.

Coverage is a poor fit for:

- proving behavioral correctness;
- ranking tests by line count alone;
- forcing tests through unreachable defensive code without a meaningful contract;
- CI gates that encourage brittle tests or superficial assertions.

Use repository-standard coverage commands when present. Generic LLVM coverage or `cargo-llvm-cov` commands should be recommended only when the repository already uses them, the user requested coverage setup, or the recommendation is clearly labeled as **Recommended**.

## Mutation-testing guidance

Mutation testing is useful for:

- identifying assertions that execute code but do not check meaningful behavior;
- finding missing negative cases and boundary conditions;
- prioritizing high-risk logic, parsers, validators, serializers, and safety wrappers;
- auditing whether property tests and integration tests actually kill plausible behavior changes.

Mutation testing is a poor fit for:

- very slow full-workspace runs without scoping;
- generated code with many equivalent mutants;
- noisy async, timing, external-service, or nondeterministic tests;
- treating every surviving mutant as a bug without review.

Use mutation tooling only when configured, requested, or clearly labeled as **Recommended**. Scope runs to the affected package, module, or file first.

## Handoff

Report coverage or mutation evidence carefully:

- `Reviewed coverage output: <path or summary>` when a repository-generated report was inspected.
- `Validated with: cargo llvm-cov -p <crate>` only when that repository command ran successfully.
- `Reviewed mutation output: <path or summary>` when mutation results were inspected.
- `Reviewed only; not executed because: coverage tooling is not configured in this repository.`
- `Not validated; outside requested scope: mutation testing would require introducing a new tool.`
