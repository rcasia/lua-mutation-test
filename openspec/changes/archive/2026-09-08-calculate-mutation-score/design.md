## Context

Once mutant runs produce normalized `MutantResult` values, the next step is to aggregate them into scores. This change defines the scoring domain and aggregation logic, providing both an overall score and granular breakdowns.

## Goals / Non-Goals

**Goals:**
- Define `MutationScore` with counts for each category and an overall percentage.
- Categorize mutants as `killed`, `survived`, `timed_out`, `error`, or `skipped`.
- Calculate the overall score as `killed / (total - errors - timeouts)` by default.
- Provide per-file, per-operator, and per-function scores.
- Expose score data for reporting.

**Non-Goals:**
- Rendering scores to specific output formats (HTML, CLI table, JSON).
- Configurable scoring formulas in this iteration.
- Parallel aggregation for very large result sets.

## Decisions

- **Count-based `MutationScore` struct.** The struct holds raw counts and derives percentages on demand. Counts are easier to test and combine than pre-computed percentages.
- **Default formula excludes `error` and `timed_out` from the denominator.** This follows common mutation testing practice: only `killed` and `survived` mutants count toward the score, while `error` and `timed_out` are reported separately. Document this default clearly.
- **Breakdowns are derived from the same aggregation.** Per-file, per-operator, and per-function scores are computed by grouping the flat result list, reusing the same `MutationScore` type.
- **Make `skipped` a first-class category.** Even if no mutants are skipped yet, the category is part of the public model so future filtering features do not require a breaking change.
- **Expose a plain data object, not a formatter.** Keeping formatting separate lets reporters decide how to present the data.

## Risks / Trade-offs

- **[Risk]** Users may disagree with the default formula.  
  **Mitigation:** Document the formula explicitly and design the score struct so a configurable formula can be added later without breaking existing callers.
- **[Risk]** Per-function attribution depends on source-location metadata that may be missing.  
  **Mitigation:** Return per-function scores only when function metadata is available; group unattributable results under a fallback entry.
