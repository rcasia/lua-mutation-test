## 1. Setup

- [x] 1.1 Create the `ControlFlowMutator` module file.
- [x] 1.2 Register `ControlFlowMutator` with the mutation operator registry.
- [x] 1.3 Add test fixtures for nested and edge-case control-flow structures.

## 2. Branch Mutations

- [x] 2.1 Implement inversion of the `if` condition.
- [x] 2.2 Implement swapping of `then` and `else` bodies.
- [x] 2.3 Implement removal of `else` branches.
- [x] 2.4 Implement removal of `elseif` branches.
- [x] 2.5 Add unit tests for branch mutations.

## 3. Loop Mutations

- [x] 3.1 Implement `while false do` and `while true do` mutations.
- [x] 3.2 Implement numeric `for` boundary changes (`a +/- 1`, `b +/- 1`).
- [x] 3.3 Implement `repeat ... until` condition inversion.
- [x] 3.4 Add unit tests for loop mutations.

## 4. Return Mutations

- [x] 4.1 Implement removal of `return` statements.
- [x] 4.2 Implement replacement of return expressions with `nil`.
- [x] 4.3 Add unit tests for return mutations.

## 5. Validation and Integration

- [x] 5.1 Validate generated control-flow mutants parse as valid Lua.
- [x] 5.2 Remove invalid mutants before they reach the runner.
- [x] 5.3 Run the full mutation pipeline on a sample Lua file and inspect results.
