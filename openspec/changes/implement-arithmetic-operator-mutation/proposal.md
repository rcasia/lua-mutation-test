## Why

Arithmetic operators are among the most common sources of logic bugs in Lua. Replacing them with alternative operators helps expose gaps in test assertions and branch coverage (GitHub issue #1).

## What Changes

- Add an `ArithmeticOperatorMutator` to the mutant generation pipeline.
- Replace each Lua arithmetic operator in a `binary_expression` with a defined set of alternative operators:
  - `+`, `-`, `*`, `/` are replaced with each other.
  - `%`, `//`, and `^` are replaced with `*` and `/`.
- Emit one distinct mutant per replacement, including the original operator, replacement operator, and source location.
- (Optional v1) Skip replacements that would introduce obvious runtime errors when a lightweight safety check is available.

## Capabilities

### New Capabilities
- `arithmetic-operator-mutation`: Generate mutants by replacing arithmetic operators in Lua binary expressions.

### Modified Capabilities
- None.

## Impact

- Mutant generation pipeline and operator-mutator registry.
- Existing mutation tests and reporting output.
- No breaking CLI or API changes.
