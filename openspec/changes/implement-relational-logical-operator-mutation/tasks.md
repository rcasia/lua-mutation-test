## 1. Relational operator mutator

- [ ] 1.1 Define `RelationalOperatorMutator` implementing the `Mutator` trait
- [ ] 1.2 Map each relational operator (`==`, `~=`, `<`, `>`, `<=`, `>=`) to replacements covering the other five
- [ ] 1.3 Match `binary_expression` nodes with relational operators and emit mutants

## 2. Logical operator mutator

- [ ] 2.1 Define `LogicalOperatorMutator` implementing the `Mutator` trait
- [ ] 2.2 Replace `and` with `or` and `or` with `and`
- [ ] 2.3 Match `binary_expression` nodes with logical operators and emit mutants

## 3. Condition negation

- [ ] 3.1 Define `ConditionNegationMutator` (or equivalent) for control-flow conditions
- [ ] 3.2 Match condition expressions in `if`, `while`, and `repeat ... until` statements
- [ ] 3.3 Wrap each matched condition in `not (...)`

## 4. Metadata and tests

- [ ] 4.1 Record original operator, replacement, and source location for relational/logical mutants
- [ ] 4.2 Add tests for all six relational operators
- [ ] 4.3 Add tests for `and` and `or` replacement
- [ ] 4.4 Add tests for condition negation
- [ ] 4.5 Verify that `~=` is parsed and matched correctly
