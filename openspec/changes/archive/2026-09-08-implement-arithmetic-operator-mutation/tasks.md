## 1. Define the arithmetic mutator

- [x] 1.1 Create `ArithmeticOperatorMutator` struct implementing the `Mutator` trait
- [x] 1.2 Define the operator-to-replacements mapping for `+`, `-`, `*`, `/`, `%`, `//`, and `^`

## 2. Match AST nodes

- [x] 2.1 Match `binary_expression` nodes whose operator child is an arithmetic operator
- [x] 2.2 Extract the operator text and byte range from the AST

## 3. Generate mutants

- [x] 3.1 Emit one mutant per alternative operator, replacing only the operator token
- [x] 3.2 Attach original operator, replacement operator, and source location metadata to each mutant

## 4. Test

- [x] 4.1 Add unit tests covering replacement for `+`, `-`, `*`, `/`, `%`, `//`, and `^`
- [x] 4.2 Add a test verifying that each mutant has distinct location metadata
