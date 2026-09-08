## 1. Domain Model

- [x] 1.1 Define a `MutationScore` struct holding counts for `killed`, `survived`, `timed_out`, `error`, and `skipped`
- [x] 1.2 Define a score breakdown type that groups scores by file, operator, and function
- [x] 1.3 Define a categorized result type mapping `MutantResult` to score categories

## 2. Aggregation Logic

- [x] 2.1 Implement overall mutation score calculation as `killed / (total - errors - timeouts)`
- [x] 2.2 Implement percentage derivation from counts
- [x] 2.3 Add tests for overall score with various category mixes

## 3. Categorization

- [x] 3.1 Map `MutantResult::killed` to the `killed` category
- [x] 3.2 Map `MutantResult::survived` to the `survived` category
- [x] 3.3 Map `MutantResult::timed_out` to the `timed_out` category
- [x] 3.4 Map `MutantResult::error` to the `error` category
- [x] 3.5 Support `skipped` category for future filtering use

## 4. Breakdowns

- [x] 4.1 Implement per-file score aggregation
- [x] 4.2 Implement per-operator score aggregation
- [x] 4.3 Implement per-function score aggregation when function metadata exists
- [x] 4.4 Provide a fallback entry for mutants outside any named function
- [x] 4.5 Add tests for each breakdown dimension

## 5. Reporting Interface

- [x] 5.1 Expose score data as a plain, formatting-free data structure
- [x] 5.2 Add tests verifying reporters can consume score data without scoring logic

## 6. Documentation

- [x] 6.1 Document the default scoring formula and category definitions
- [x] 6.2 Document the design for future configurable scoring formulas
