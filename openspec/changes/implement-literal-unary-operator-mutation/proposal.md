## Why

Literal values and unary operators encode implicit assumptions in Lua code: boundary values, default flags, sign, length, and boolean polarity. Mutating them exposes tests that do not verify these assumptions (GitHub issue #3).

## What Changes

- Add a `LiteralMutator` that mutates boolean, number, string, and `nil` literals.
  - Boolean: `true` <-> `false`.
  - Number: increment, decrement, sign flip, and replacement with `0` or `1`.
  - String: replacement with an empty string.
  - Nil: replacement with `false` where safe.
- Add a `UnaryOperatorMutator` that removes unary operators (`-`, `not`, `#`) by replacing the `unary_expression` with its operand.
- Record replacement text and source location for every mutant.
- Document known equivalent mutant cases.

## Capabilities

### New Capabilities
- `literal-unary-operator-mutation`: Generate mutants by mutating Lua literals and removing unary operators.

### Modified Capabilities
- None.

## Impact

- Mutant generation pipeline and mutator registry.
- Documentation updated with known equivalent mutants.
- No breaking CLI or API changes.
