## Why

Equivalent mutants do not change program behavior, so they never fail tests and skew mutation scores while still consuming execution time. Simple static heuristics can detect obviously equivalent mutants (such as arithmetic identities) before they are run, reducing false positives and improving runtime.

## What Changes

- Add heuristic detection of obviously equivalent mutants (e.g., `x + 0`).
- Flag mutants that are likely equivalent for reviewer attention instead of executing them.
- Create sample Lua projects under `tests/fixtures/` for integration testing.
- Add an integration test suite that exercises the tool against real Lua projects.
- Add a benchmark suite measuring mutation execution time.
- Add a CI workflow that runs integration tests.
- Document known equivalent-mutant cases.

## Capabilities

### New Capabilities
- `equivalent-mutant-heuristics`: Detect and flag obviously equivalent mutants using lightweight static heuristics before execution.

### Modified Capabilities
- None

## Impact

- Mutant generation and filtering pipeline.
- New `tests/fixtures/` sample Lua projects.
- New integration test and benchmark modules.
- New CI workflow.
- Mutation testing documentation.
