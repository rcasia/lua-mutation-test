## Context

The test runner currently launches a Lua process against a mutated file but does not yet classify the outcome. This change introduces a small interpreter layer that normalizes the raw process result into a domain type the rest of the system can rely on.

## Goals / Non-Goals

**Goals:**
- Map exit code 0 to `survived`.
- Map non-zero exit code to `killed`.
- Map timeout events to `timed_out`.
- Map runner failures and invalid mutants to `error`.
- Store stdout/stderr snippets for reporting.
- Distinguish test failures from runner failures.

**Non-Goals:**
- Framework-specific output parsing that overrides exit codes.
- Persisting full output logs to disk.
- Defining the reporting UI or score calculation.

## Decisions

- **Separate result type from runner type.** Keep `MutantResult` as a pure domain enum while the runner remains responsible for process execution. This prevents the runner from leaking into the scorer and reporter.
- **Use an interpreter function rather than methods on the runner.** An interpreter that accepts exit code, stdout, stderr, and a failure reason is easier to unit test and reuse if alternative runners are added later.
- **Store output snippets, not full logs.** Full logs can be large; keeping bounded snippets (e.g., last N lines) balances debugging value with memory use.
- **Distinguish runner failures by an explicit `RunnerError` variant.** A missing executable or crash is different from a test failure, so the interpreter needs a signal separate from the exit code.

## Risks / Trade-offs

- **[Risk]** Some Lua test frameworks may exit 0 even when tests fail.  
  **Mitigation:** Document that the default mapping trusts the exit code; future output parsing can override this without changing the `MutantResult` shape.
- **[Risk]** Large stdout/stderr can bloat memory.  
  **Mitigation:** Cap snippet length and expose the limit in configuration later.
