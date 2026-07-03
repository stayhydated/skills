# Benchmarks and performance patterns

Use benchmarks when performance is part of the contract, a suspected regression, or a change claims to improve speed, memory behavior, throughput, or allocation patterns.

## Good fits

Criterion or the repository's existing benchmark harness is a good fit for:

- hot algorithms, parsers, serializers, data structures, alloc-heavy code, or tight loops;
- comparing two public implementations or feature flags;
- measuring throughput for bytes, items, requests, or operations;
- guarding performance-sensitive generated code;
- reproducing a reported performance regression with a stable input.

## Poor fits

Prefer ordinary tests, profiling, or manual analysis for:

- functional correctness;
- tiny behavior where optimizer artifacts dominate the measurement;
- uncontrolled network, wall-clock, filesystem, scheduling, logging, or service dependencies;
- benchmarks that allocate fresh huge fixtures inside the measured loop by accident;
- CI pass/fail thresholds based on noisy wall-clock timing unless the repository already has a calibrated performance gate.

## Criterion discipline

- Follow the repository's existing `benches/` layout, `[[bench]]` manifest entries, and benchmark runner.
- For Criterion benchmark targets, use `harness = false` in the manifest when that is the repository convention or required by the custom harness.
- Benchmark the public API surface unless local conventions expose a benchmark-only helper; Cargo bench targets under `benches/` compile like separate crates.
- Keep setup outside the measured loop; use batched setup when each iteration needs fresh input.
- Use `std::hint::black_box` when the repository MSRV supports it; otherwise follow the local Criterion version and imports.
- Use stable, behavior-named benchmark IDs and groups so historical output remains comparable.
- Record throughput units when they make results easier to interpret.
- Do not treat a successful benchmark run as proof of correctness; pair benchmarks with correctness tests.

## Validation and handoff

Use the repository's documented benchmark command when present. Common focused commands include:

- `cargo bench --bench <bench_name>` for one benchmark target;
- `cargo bench --bench <bench_name> --no-run` when only compilation can be validated;
- `cargo bench -p <crate> --bench <bench_name>` in a workspace;
- `cargo criterion ...`, `just bench`, or CI-specific benchmark scripts only when evidenced in the repository.

When reporting validation, distinguish compile validation from performance measurement:

- `Validated with: cargo bench --bench parse_bench --no-run` means the benchmark compiled but performance was not measured.
- `Validated with: cargo bench --bench parse_bench` means the benchmark ran, but disclose if the environment was noisy or not comparable to CI.
- `Reviewed benchmark output: <path or summary>` only when benchmark reports, baselines, or generated outputs were inspected.
