# control-flow-mutation Specification

## Purpose
TBD - created by archiving change implement-control-flow-mutation-operators. Update Purpose after archive.
## Requirements
### Requirement: Invert if condition
The system SHALL generate a mutant that replaces `if cond then` with `if not cond then`.

#### Scenario: Simple if statement
- **WHEN** the source contains `if x > 0 then print("positive") end`
- **THEN** a mutant is produced as `if not (x > 0) then print("positive") end`

### Requirement: Swap if branches
The system SHALL generate a mutant that swaps the `then` and `else` bodies of an `if` statement.

#### Scenario: If-else statement
- **WHEN** the source contains `if x then A() else B() end`
- **THEN** a mutant is produced as `if x then B() else A() end`

### Requirement: Remove else branch
The system SHALL generate a mutant that removes the `else` branch of an `if` statement.

#### Scenario: If-else statement with else removal
- **WHEN** the source contains `if x then A() else B() end`
- **THEN** a mutant is produced as `if x then A() end`

### Requirement: Remove elseif branch
The system SHALL generate a mutant that removes an `elseif` branch from an `if` statement.

#### Scenario: If-elseif statement
- **WHEN** the source contains `if x then A() elseif y then B() end`
- **THEN** a mutant is produced as `if x then A() end`

### Requirement: Mutate while loop condition
The system SHALL generate mutants that replace the `while` condition with `false` and with `true`.

#### Scenario: While loop
- **WHEN** the source contains `while cond do work() end`
- **THEN** mutants are produced as `while false do work() end` and `while true do work() end`

### Requirement: Mutate numeric for boundaries
The system SHALL generate mutants that increment and decrement the start and end boundaries of a numeric `for` loop.

#### Scenario: Numeric for loop
- **WHEN** the source contains `for i = 1, 10 do work() end`
- **THEN** mutants are produced with boundaries `(2, 10)`, `(0, 10)`, `(1, 11)`, and `(1, 9)`

### Requirement: Invert repeat-until condition
The system SHALL generate a mutant that negates the condition of a `repeat ... until` statement.

#### Scenario: Repeat-until loop
- **WHEN** the source contains `repeat work() until done`
- **THEN** a mutant is produced as `repeat work() until not done`

### Requirement: Remove or nil return statements
The system SHALL generate mutants that remove `return` statements and replace return expressions with `nil`.

#### Scenario: Return with value
- **WHEN** the source contains `return x + 1`
- **THEN** mutants are produced as `return nil` and with the return statement removed

### Requirement: Validate control-flow mutants
The system SHALL discard any control-flow mutant that does not parse as valid Lua.

#### Scenario: Invalid branch removal
- **WHEN** a branch removal produces syntactically invalid Lua
- **THEN** the mutant is not emitted for execution

