## Why

Relational and logical operators determine control-flow and boundary behavior in Lua. Mutating these operators surfaces missing branch coverage and weak assertions around equality, ordering, and boolean logic (GitHub issue #2).

## What Changes

- Add a `RelationalOperatorMutator` that replaces each relational operator (`==`, `~=`, `<`, `>`, `<=`, `>=`) with the other relational operators.
- Add a `LogicalOperatorMutator` that swaps `and` with `or` and vice versa.
- Add condition-negation mutants for control-flow statements (e.g., `if x` becomes `if not x`).
- Record the original operator, replacement, and source location for each mutant.

## Capabilities

### New Capabilities
- `relational-logical-operator-mutation`: Generate mutants by replacing relational and logical operators and by negating conditions.

### Modified Capabilities
- None.

## Impact

- Mutant generation pipeline and mutator registry.
- Mutation reports will include additional relational, logical, and condition-negation mutants.
- No breaking CLI or API changes.
