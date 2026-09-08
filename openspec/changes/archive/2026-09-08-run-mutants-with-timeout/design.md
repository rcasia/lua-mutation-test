## Context

Once mutants are generated and a baseline test run is established, each mutant must be evaluated individually. Running mutants in the same process as the test runner risks cross-contamination and cannot recover from infinite loops. The acceptance criteria require process isolation, a default timeout, explicit timeout status, cleanup, and graceful handling of spawn failures.

## Goals / Non-Goals

**Goals:**
- Run one mutant per process.
- Enforce a configurable timeout defaulting to 30 seconds.
- Classify outcomes as `killed`, `survived`, `timed_out`, or `errored`.
- Clean up temporary files after every run.
- Handle command-spawning failures without panicking.

**Non-Goals:**
- Parallel execution of mutants (handled by a separate change).
- Resource pooling or sandboxing beyond OS process boundaries.
- Mutant application to source files (assumed to be provided by existing utilities).

## Decisions

- **Decision:** Use `std::process::Command` with a timeout wrapper rather than an async runtime.
  - **Rationale:** Avoids adding a dependency such as `tokio` and keeps the implementation small and synchronous.
  - **Alternative considered:** `tokio::process::Command` with a timeout future. Rejected to minimize dependencies.
- **Decision:** Represent the timeout as a `Duration` parsed from a user-provided seconds value.
  - **Rationale:** Simple configuration while still supporting fractional seconds if desired.
- **Decision:** Run the same adapter command used for the baseline, but with the mutant-injected source path.
  - **Rationale:** Ensures consistency between baseline and mutant execution and reuses adapter logic.
- **Decision:** Clean up temporary files in a `Drop` guard or explicit `finally`-style block.
  - **Rationale:** Guarantees cleanup even if the child process times out or a panic occurs.

## Risks / Trade-offs

- [Risk] Very short timeouts may cause false-positive `timed_out` results on slow hosts.
  - → Mitigation: Allow configuration and document the 30-second default as a starting point.
- [Risk] Process isolation alone does not prevent mutants from modifying external resources.
  - → Mitigation: Document the limitation and recommend running mutation tests in a disposable environment.
