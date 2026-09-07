## Context

The mutation testing pipeline already parses Lua source files and applies arithmetic, relational, logical, literal, and unary operators. The next logical expansion is control-flow operators, which are commonly used in mutation testing frameworks and directly challenge test-suite thoroughness. The issue acceptance criteria enumerate specific transformations for branches, loops, and returns.

## Goals / Non-Goals

**Goals:**
- Implement all control-flow mutations listed in the acceptance criteria.
- Keep the operator consistent with the existing mutator registration pattern.
- Ensure every generated mutant parses as valid Lua before it reaches the test runner.
- Provide focused unit tests that demonstrate each transformation.

**Non-Goals:**
- Parallelizing mutant execution (handled by a separate change).
- Detecting semantic equivalence or equivalent mutants.
- Adding new CLI flags beyond what the existing operator registry already supports.

## Decisions

- **Decision:** Implement a single `ControlFlowMutator` struct/module rather than separate mutators per construct.
  - **Rationale:** All listed transformations operate on control-flow nodes; grouping them keeps the operator registry small and simplifies shared validation logic.
  - **Alternative considered:** One mutator per construct (branch, loop, return). Rejected because it would create many small files with overlapping AST traversal patterns.
- **Decision:** Validate mutated source with the parser before emitting the mutant.
  - **Rationale:** Branch removal can produce invalid Lua (e.g., empty `if` block followed by `else`). Filtering at generation time avoids wasting test runs.
  - **Alternative considered:** Let the test runner handle parse errors. Rejected because it conflates mutant generation failures with test failures.
- **Decision:** Preserve source-location metadata using the existing source-map utilities.
  - **Rationale:** Consistent reporting of mutant locations across operator types.

## Risks / Trade-offs

- [Risk] Numeric `for` boundary mutations (`a +/- 1`, `b +/- 1`) may generate loops that never execute or run indefinitely.
  - → Mitigation: The test runner's timeout/isolation logic (handled by a separate change) will cap runaway loops.
- [Risk] Removing `else`/`elseif` branches can introduce syntax errors or change semantics in unexpected ways.
  - → Mitigation: Syntactic validation and a test fixture covering branch removal.
