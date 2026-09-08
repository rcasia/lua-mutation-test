## Why

Executing the test suite against every mutant can be expensive and dangerous: a single infinite loop introduced by a control-flow mutation could hang the entire run, and side effects from one mutant could leak into the next. Isolating each mutant in its own process, enforcing a timeout, and cleaning up afterwards is necessary for a robust and trustworthy mutation testing workflow.

## What Changes

- Implement a mutant runner that executes the configured test command in the context of a single mutant.
- Spawn each mutant run as a separate OS process using `std::process::Command`.
- Apply a configurable timeout with a default of 30 seconds.
- Mark mutants that exceed the timeout as `timed_out` in the results.
- Clean up temporary files and scratch directories after each run.
- Handle process-spawning errors gracefully with a clear `errored` status.
- Integrate with the existing test-result capture component to record pass/fail/timeout/error outcomes.

## Capabilities

### New Capabilities
- `mutant-execution`: Run the test suite against each mutant in an isolated process with timeout enforcement and cleanup.

### Modified Capabilities
<!-- No existing capabilities require requirement-level changes. -->

## Impact

- New runner module in the Rust codebase.
- Use of `std::process::Command` for process isolation; no async runtime dependency introduced.
- Adds `timeout` and related fields to configuration.
- Result capture component extended with `timed_out` and `errored` statuses.
