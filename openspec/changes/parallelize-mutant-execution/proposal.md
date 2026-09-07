## Why

Mutation testing runs each generated mutant independently, so sequential execution wastes available CPU cores and makes feedback loops unnecessarily slow. Parallelizing mutant execution reduces wall-clock time proportionally to available hardware without changing mutation semantics.

## What Changes

- Add a configurable worker pool to execute mutants concurrently.
- Default the worker count to the number of logical CPUs.
- Ensure each worker uses an isolated temporary directory to avoid filesystem collisions.
- Add progress reporting during parallel execution.
- Add graceful shutdown on `Ctrl-C` / `SIGINT`.
- Keep mutation results deterministic regardless of the order in which mutants finish.

## Capabilities

### New Capabilities
- `parallel-mutant-execution`: Execute generated mutants concurrently using a bounded worker pool while preserving deterministic results, resource limits, and graceful interruption handling.

### Modified Capabilities
- None

## Impact

- Core mutant execution / test-runner module.
- CLI argument parsing for `--workers`.
- New concurrency dependency (e.g., `rayon`).
- Progress reporting output and signal handling.
