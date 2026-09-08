## Context

tree-sitter-lua represents relational and logical expressions as `binary_expression` nodes with operator tokens. Control-flow statements (`if`, `while`, `repeat`) contain condition expressions. The project already has a pattern for adding mutators that emit `Mutant` values with source locations.

## Goals / Non-Goals

**Goals:**
- Implement `RelationalOperatorMutator` covering all six relational operators.
- Implement `LogicalOperatorMutator` swapping `and` and `or`.
- Implement condition negation for bare condition expressions in control-flow statements.
- Record original operator, replacement, and byte range for every mutant.

**Non-Goals:**
- Simulating Lua short-circuit semantics.
- Mutating unary `not` (handled by the unary operator mutation change).
- Type-aware boundary analysis.

## Decisions

- **Separate mutators for relational and logical operators**: Relational and logical operators have different AST precedence and replacement sets. Splitting them keeps each mutator focused and testable.
- **Replace each relational operator with every other relational operator**: This satisfies the acceptance criterion that all six operators are covered and is simple to maintain via a dispatch table.
- **Condition negation as a dedicated mutator**: `ConditionNegationMutator` matches `if`, `while`, and `repeat ... until` condition expressions and wraps them in `not (...)`. Keeping it separate from logical-operator mutation makes intent and tests clearer.
- **Operator text matching**: Match operator tokens by their literal text (`==`, `~=`, `and`, `or`, etc.). This avoids depending on internal tree-sitter field indices.

## Risks / Trade-offs

- **Equivalent mutants**: Negating a condition that is already a boolean literal or already contains `not` may produce equivalent or trivially killed mutants. This is accepted and documented.
- **Large mutant counts**: Replacing every relational operator with five alternatives can generate many mutants; this is acceptable for correctness and can be optimized later.
