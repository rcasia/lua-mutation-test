# literal-unary-operator-mutation Specification

## Purpose
TBD - created by archiving change implement-literal-unary-operator-mutation. Update Purpose after archive.
## Requirements
### Requirement: Boolean literal mutation
The system SHALL generate mutants that toggle boolean literals between `true` and `false`.

#### Scenario: True becomes false
- **WHEN** the source contains the literal `true`
- **THEN** the literal mutator emits a mutant replacing it with `false`

#### Scenario: False becomes true
- **WHEN** the source contains the literal `false`
- **THEN** the literal mutator emits a mutant replacing it with `true`

### Requirement: Number literal mutation
The system SHALL generate mutants that modify numeric literals through increment, decrement, sign flip, and replacement with `0` or `1`.

#### Scenario: Number is incremented
- **WHEN** the source contains the number `5`
- **THEN** the literal mutator emits a mutant replacing it with `6`

#### Scenario: Number is decremented
- **WHEN** the source contains the number `5`
- **THEN** the literal mutator emits a mutant replacing it with `4`

#### Scenario: Number sign is flipped
- **WHEN** the source contains the number `5`
- **THEN** the literal mutator emits a mutant replacing it with `-5`

#### Scenario: Number is replaced with zero
- **WHEN** the source contains the number `5`
- **THEN** the literal mutator emits a mutant replacing it with `0`

#### Scenario: Number is replaced with one
- **WHEN** the source contains the number `5`
- **THEN** the literal mutator emits a mutant replacing it with `1`

### Requirement: String literal mutation
The system SHALL generate mutants that replace non-empty string literals with an empty string.

#### Scenario: Non-empty string becomes empty
- **WHEN** the source contains the string `"hello"`
- **THEN** the literal mutator emits a mutant replacing it with `""`

### Requirement: Nil literal mutation
The system SHALL generate mutants that replace `nil` literals with `false` where syntactically valid.

#### Scenario: Nil becomes false
- **WHEN** the source contains the expression `x == nil`
- **THEN** the literal mutator emits a mutant replacing `nil` with `false`

### Requirement: Unary operator mutation
The system SHALL generate mutants that remove unary operators by replacing the `unary_expression` with its operand.

#### Scenario: Unary minus is removed
- **WHEN** the source contains the expression `-x`
- **THEN** the unary operator mutator emits a mutant replacing it with `x`

#### Scenario: Unary not is removed
- **WHEN** the source contains the expression `not x`
- **THEN** the unary operator mutator emits a mutant replacing it with `x`

#### Scenario: Unary length is removed
- **WHEN** the source contains the expression `#x`
- **THEN** the unary operator mutator emits a mutant replacing it with `x`

### Requirement: Mutant metadata
The system SHALL record the replacement text and source location for every literal and unary operator mutant.

#### Scenario: Metadata is complete
- **WHEN** the literal or unary operator mutator generates a mutant
- **THEN** the mutant contains the replacement text and the byte range of the mutated node

### Requirement: Equivalent mutant documentation
The system SHALL document known cases where literal or unary operator mutations may produce equivalent mutants.

#### Scenario: Documentation exists
- **WHEN** the literal/unary operator mutation capability is reviewed
- **THEN** the design or user documentation lists at least one known equivalent-mutant example

