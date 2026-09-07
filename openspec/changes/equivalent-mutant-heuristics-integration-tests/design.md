## Context

Equivalent mutants are a well-known problem in mutation testing: they survive not because tests are weak, but because the mutation is semantically identical to the original code. Perfect equivalence detection is undecidable, but lightweight heuristics catch common cases such as arithmetic identities. This change also establishes integration tests and benchmarks so the project can be validated against realistic Lua code.

## Goals / Non-Goals

**Goals:**
- Detect and skip obviously equivalent mutants using static heuristics.
- Surface likely-equivalent mutants in reports for reviewer review.
- Provide repeatable integration tests against sample Lua projects.
- Measure execution performance with a benchmark suite.
- Run integration tests in CI.
- Document known equivalent-mutant patterns.

**Non-Goals:**
- Proving equivalence formally.
- Covering every possible equivalent mutant.
- Modifying existing mutation operators beyond filtering their output.

## Decisions

- **Apply heuristics at mutant generation time.** Rationale: avoids creating and scheduling mutants that will never fail, saving execution time.
- **Initial heuristic: arithmetic identities** (`x + 0`, `x - 0`, `x * 1`, `x / 1`, `x // 1`). Rationale: common, easy to detect statically, and high confidence.
- **Flag equivalent mutants in reports with a reason.** Rationale: keeps scores honest and gives users visibility into skipped mutants.
- **Integration fixtures live in `tests/fixtures/`.** Rationale: standard Rust convention; keeps test data discoverable and version controlled.
- **Benchmarks use Criterion.** Rationale: standard Rust benchmarking harness with stable statistical reporting.

## Risks / Trade-offs

- **[Risk]** False negatives — real equivalent mutants are not flagged. → **Mitigation:** accept heuristic coverage and document known cases; perfect detection is out of scope.
- **[Risk]** False positives — non-equivalent mutants are flagged. → **Mitigation:** keep heuristics conservative and require a literal identity operand.
- **[Risk]** Integration tests become slow or flaky. → **Mitigation:** keep fixtures small and pin Lua interpreter versions in CI.
