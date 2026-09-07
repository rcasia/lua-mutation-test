## Why

Mutation testing only provides actionable feedback when results are aggregated into a clear score. Users need an overall mutation score as well as per-file, per-operator, and per-function breakdowns to understand where their test suite is effective and where it needs improvement.

## What Changes

- Define a `MutationScore` struct that holds counts and percentages for each mutant category.
- Categorize mutants as `killed`, `survived`, `timed_out`, `error`, or `skipped`.
- Implement overall score calculation using the formula `killed / (total - errors - timeouts)` by default, with a documented path to make the formula configurable.
- Provide per-file, per-operator, and per-function score breakdowns.
- Expose score data in a reporting-friendly structure for later output formatting.
- Add unit tests for aggregation logic and category edge cases.

## Capabilities

### New Capabilities
- `mutation-scoring`: Aggregates mutant results into an overall mutation score with per-file, per-operator, and per-function breakdowns.

### Modified Capabilities
- None.

## Impact

- Adds a scoring module consumed by the reporting layer.
- Does not change existing mutation operators or runner behavior; it operates on the `MutantResult` data produced by the runner.
