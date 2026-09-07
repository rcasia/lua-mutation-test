## 1. Literal mutator

- [ ] 1.1 Define `LiteralMutator` implementing the `Mutator` trait
- [ ] 1.2 Match boolean, number, string, and `nil` literal AST nodes
- [ ] 1.3 Implement boolean toggle
- [ ] 1.4 Implement number increment, decrement, sign flip, and `0`/`1` replacement
- [ ] 1.5 Implement string replacement with empty string
- [ ] 1.6 Implement safe `nil` to `false` replacement

## 2. Unary operator mutator

- [ ] 2.1 Define `UnaryOperatorMutator` implementing the `Mutator` trait
- [ ] 2.2 Match `unary_expression` nodes with operators `-`, `not`, and `#`
- [ ] 2.3 Emit identity mutants by replacing the expression with its operand

## 3. Metadata and documentation

- [ ] 3.1 Record replacement text and source location for every literal and unary mutant
- [ ] 3.2 Document known equivalent mutant cases

## 4. Tests

- [ ] 4.1 Test boolean literal toggle
- [ ] 4.2 Test number literal mutations
- [ ] 4.3 Test string literal empty replacement
- [ ] 4.4 Test `nil` to `false` replacement
- [ ] 4.5 Test unary operator removal for `-`, `not`, and `#`
- [ ] 4.6 Test that every mutant carries location metadata
