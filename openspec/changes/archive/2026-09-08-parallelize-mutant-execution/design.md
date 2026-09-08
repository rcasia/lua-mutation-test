## Context

The current mutation test runner executes mutants one at a time. For projects with many mutants, this serial execution dominates total runtime even though each mutant run is independent. The project already generates and deduplicates mutants, so the next logical optimization is to run them in parallel.

## Goals / Non-Goals

**Goals:**
- Reduce total mutation-test wall-clock time by utilizing available CPU cores.
- Make concurrency configurable and resource-aware.
- Preserve deterministic mutation results independent of execution order.
- Provide feedback to the user while mutants are running.
- Handle interruption gracefully without leaking worker processes or temp directories.

**Non-Goals:**
- Distributed execution across multiple machines.
- Dynamic load balancing beyond a fixed-size worker pool.
- Changing mutation semantics or scoring rules.

## Decisions

- **Use `rayon` for data parallelism.** Rationale: it provides a thread pool with minimal code changes and composes well with iterator-based mutant collections. Async (`tokio`) is more complex than needed for CPU-bound work.
- **Each worker receives its own temporary directory.** Rationale: prevents filesystem collisions when multiple mutants write artifacts simultaneously; the `tempfile` crate can create per-task directories.
- **Progress is reported via a shared atomic counter.** Rationale: simple, lock-free, and independent of worker result ordering.
- **Signal handling uses `ctrlc` to set a shutdown flag.** Rationale: cross-platform `SIGINT` handling; the worker pool stops accepting new work and waits for in-flight tasks.

## Risks / Trade-offs

- **[Risk]** CPU contention between the worker pool and the Lua test subprocesses. → **Mitigation:** default workers to the number of logical CPUs and expose `--workers` so users can throttle on resource-constrained machines.
- **[Risk]** Non-deterministic results if workers share mutable state. → **Mitigation:** pass inputs by value and collect immutable result structs; do not share RNGs or mutable report builders.
- **[Risk]** Temporary directories not cleaned up after interruption. → **Mitigation:** use `tempfile::TempDir` with explicit cleanup in a scope, and ensure the shutdown path drops active workers.
