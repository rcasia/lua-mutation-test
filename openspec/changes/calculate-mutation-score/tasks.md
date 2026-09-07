## 1. Domain Model

- [ ] 1.1 Define a `MutationScore` struct holding counts for `killed`, `survived`, `timed_out`, `error`, and `skipped`
- [ ] 1.2 Define a score breakdown type that groups scores by file, operator, and function
- [ ] 1.3 Define a categorized result type mapping `MutantResult` to score categories

## 2. Aggregation Logic

- [ ] 2.1 Implement overall mutation score calculation as `killed / (total - errors - timeouts)`
- [ ] 2.2 Implement percentage derivation from counts
- [ ] 2.3 Add tests for overall score with various category mixes

## 3. Categorization

- [ ] 3.1 Map `MutantResult::killed` to the `killed` category
- [ ] 3.2 Map `MutantResult::survived` to the `survived` category
- [ ] 3.3 Map `MutantResult::timed_out` to the `timed_out` category
- [ ] 3.4 Map `MutantResult::error` to the `error` category
- [ ] 3.5 Support `skipped` category for future filtering use

## 4. Breakdowns

- [ ] 4.1 Implement per-file score aggregation
- [ ] 4.2 Implement per-operator score aggregation
- [ ] 4.3 Implement per-function score aggregation when function metadata exists
- [ ] 4.4 Provide a fallback entry for mutants outside any named function
- [ ] 4.5 Add tests for each breakdown dimension

## 5. Reporting Interface

- [ ] 5.1 Expose score data as a plain, formatting-free data structure
- [ ] 5.2 Add tests verifying reporters can consume score data without scoring logic

## 6. Documentation

- [ ] 6.1 Document the default scoring formula and category definitions
- [ ] 6.2 Document the design for future configurable scoring formulas
