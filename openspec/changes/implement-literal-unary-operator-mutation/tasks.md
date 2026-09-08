## 1. Literal mutator

- [x] 1.1 Define `LiteralMutator` implementing the `Mutator` trait
- [x] 1.2 Match boolean, number, string, and `nil` literal AST nodes
- [x] 1.3 Implement boolean toggle
- [x] 1.4 Implement number increment, decrement, sign flip, and `0`/`1` replacement
- [x] 1.5 Implement string replacement with empty string
- [x] 1.6 Implement safe `nil` to `false` replacement

## 2. Unary operator mutator

- [x] 2.1 Define `UnaryOperatorMutator` implementing the `Mutator` trait
- [x] 2.2 Match `unary_expression` nodes with operators `-`, `not`, and `#`
- [x] 2.3 Emit identity mutants by replacing the expression with its operand

## 3. Metadata and documentation

- [x] 3.1 Record replacement text and source location for every literal and unary mutant
- [x] 3.2 Document known equivalent mutant cases

## 4. Tests

- [x] 4.1 Test boolean literal toggle
- [x] 4.2 Test number literal mutations
- [x] 4.3 Test string literal empty replacement
- [x] 4.4 Test `nil` to `false` replacement
- [x] 4.5 Test unary operator removal for `-`, `not`, and `#`
- [x] 4.6 Test that every mutant carries location metadata
