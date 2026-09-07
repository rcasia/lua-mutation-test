## 1. Setup

- [ ] 1.1 Create the `ControlFlowMutator` module file.
- [ ] 1.2 Register `ControlFlowMutator` with the mutation operator registry.
- [ ] 1.3 Add test fixtures for nested and edge-case control-flow structures.

## 2. Branch Mutations

- [ ] 2.1 Implement inversion of the `if` condition.
- [ ] 2.2 Implement swapping of `then` and `else` bodies.
- [ ] 2.3 Implement removal of `else` branches.
- [ ] 2.4 Implement removal of `elseif` branches.
- [ ] 2.5 Add unit tests for branch mutations.

## 3. Loop Mutations

- [ ] 3.1 Implement `while false do` and `while true do` mutations.
- [ ] 3.2 Implement numeric `for` boundary changes (`a +/- 1`, `b +/- 1`).
- [ ] 3.3 Implement `repeat ... until` condition inversion.
- [ ] 3.4 Add unit tests for loop mutations.

## 4. Return Mutations

- [ ] 4.1 Implement removal of `return` statements.
- [ ] 4.2 Implement replacement of return expressions with `nil`.
- [ ] 4.3 Add unit tests for return mutations.

## 5. Validation and Integration

- [ ] 5.1 Validate generated control-flow mutants parse as valid Lua.
- [ ] 5.2 Remove invalid mutants before they reach the runner.
- [ ] 5.3 Run the full mutation pipeline on a sample Lua file and inspect results.
