## 1. Define the arithmetic mutator

- [ ] 1.1 Create `ArithmeticOperatorMutator` struct implementing the `Mutator` trait
- [ ] 1.2 Define the operator-to-replacements mapping for `+`, `-`, `*`, `/`, `%`, `//`, and `^`

## 2. Match AST nodes

- [ ] 2.1 Match `binary_expression` nodes whose operator child is an arithmetic operator
- [ ] 2.2 Extract the operator text and byte range from the AST

## 3. Generate mutants

- [ ] 3.1 Emit one mutant per alternative operator, replacing only the operator token
- [ ] 3.2 Attach original operator, replacement operator, and source location metadata to each mutant

## 4. Test

- [ ] 4.1 Add unit tests covering replacement for `+`, `-`, `*`, `/`, `%`, `//`, and `^`
- [ ] 4.2 Add a test verifying that each mutant has distinct location metadata
