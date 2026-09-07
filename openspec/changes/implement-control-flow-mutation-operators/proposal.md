## Why

Control-flow constructs are the backbone of conditional logic and iteration in Lua programs, yet the current mutation engine does not target them. Adding control-flow operators is essential to expose weaknesses in tests that only exercise happy paths and to measure true behavioral coverage.

## What Changes

- Introduce a new `ControlFlowMutator` module that registers with the existing mutation operator registry.
- Add branch mutations for `if`/`elseif`/`else` statements:
  - Invert the `if` condition.
  - Swap `then` and `else` bodies.
  - Remove `else` and `elseif` branches.
- Add loop mutations:
  - Replace `while cond do` with `while false do` and `while true do`.
  - Adjust numeric `for` boundaries (`a +/- 1`, `b +/- 1`).
  - Invert the condition in `repeat ... until cond`.
- Add return-statement mutations:
  - Remove `return` statements entirely.
  - Replace return expressions with `nil`.
- Validate generated mutants for syntactic correctness before they are queued for execution.
- Add unit tests covering each operator and edge cases such as nested control flow.

## Capabilities

### New Capabilities
- `control-flow-mutation`: Generate, deduplicate, and queue control-flow mutants for conditionals, loops, and return statements.

### Modified Capabilities
<!-- No existing capabilities require requirement-level changes. -->

## Impact

- New mutation operator module in the Rust codebase.
- Dependency on the existing parser/tree-sitter Lua AST traversal utilities.
- Potential increase in mutant count per run; validated to avoid syntax errors.
