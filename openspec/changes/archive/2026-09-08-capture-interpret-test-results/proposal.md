## Why

The mutation testing pipeline needs to classify the outcome of every mutant run so downstream reporting can compute scores and present useful diagnostics. Without a shared result type and interpreter, every runner would have to implement its own ad-hoc exit-code mapping, making the system inconsistent and harder to extend to new Lua test frameworks.

## What Changes

- Introduce a `MutantResult` enum to represent `killed`, `survived`, `timed_out`, and `error` outcomes.
- Add a result interpreter that consumes the runner's exit code, captured stdout/stderr, and optional timeout signal, then returns a normalized `MutantResult`.
- Store short stdout/stderr snippets alongside each result for reporting and debugging.
- Distinguish test failures (non-zero exit from a working runner) from runner failures (missing executable, crash, or invalid mutant).
- Add unit tests covering each mapping and edge case.

## Capabilities

### New Capabilities
- `result-interpreter`: Captures and normalizes the outcome of a mutant test run, mapping exit codes, timeouts, and runner failures to a stable `MutantResult` and storing output snippets for reporting.

### Modified Capabilities
- None.

## Impact

- Affects the test-runner integration module and the data structures shared between the runner and the scorer.
- No CLI breaking changes; this is an internal capability.
