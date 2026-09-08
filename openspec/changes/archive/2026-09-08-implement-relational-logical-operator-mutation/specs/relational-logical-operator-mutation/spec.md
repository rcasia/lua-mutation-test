## ADDED Requirements

### Requirement: Relational operator replacement set
The system SHALL generate mutants that replace each Lua relational operator in a `binary_expression` with every other relational operator.

#### Scenario: Equality operator is replaced
- **WHEN** the source contains `a == b`
- **THEN** the relational operator mutator emits mutants for `a ~= b`, `a < b`, `a > b`, `a <= b`, and `a >= b`

#### Scenario: Not-equal operator is replaced
- **WHEN** the source contains `a ~= b`
- **THEN** the relational operator mutator emits mutants for `a == b`, `a < b`, `a > b`, `a <= b`, and `a >= b`

#### Scenario: Less-than operator is replaced
- **WHEN** the source contains `a < b`
- **THEN** the relational operator mutator emits mutants for `a == b`, `a ~= b`, `a > b`, `a <= b`, and `a >= b`

#### Scenario: Greater-than operator is replaced
- **WHEN** the source contains `a > b`
- **THEN** the relational operator mutator emits mutants for `a == b`, `a ~= b`, `a < b`, `a <= b`, and `a >= b`

#### Scenario: Less-than-or-equal operator is replaced
- **WHEN** the source contains `a <= b`
- **THEN** the relational operator mutator emits mutants for `a == b`, `a ~= b`, `a < b`, `a > b`, and `a >= b`

#### Scenario: Greater-than-or-equal operator is replaced
- **WHEN** the source contains `a >= b`
- **THEN** the relational operator mutator emits mutants for `a == b`, `a ~= b`, `a < b`, `a > b`, and `a <= b`

### Requirement: Logical operator replacement
The system SHALL generate mutants that swap the logical operators `and` and `or` in `binary_expression` nodes.

#### Scenario: And is replaced with or
- **WHEN** the source contains `a and b`
- **THEN** the logical operator mutator emits a mutant for `a or b`

#### Scenario: Or is replaced with and
- **WHEN** the source contains `a or b`
- **THEN** the logical operator mutator emits a mutant for `a and b`

### Requirement: Condition negation
The system SHALL generate mutants that wrap the condition expression of a control-flow statement in `not (...)`.

#### Scenario: If condition is negated
- **WHEN** the source contains `if x then ... end`
- **THEN** the condition-negation mutator emits a mutant for `if not (x) then ... end`

#### Scenario: While condition is negated
- **WHEN** the source contains `while x do ... end`
- **THEN** the condition-negation mutator emits a mutant for `while not (x) do ... end`

#### Scenario: Repeat-until condition is negated
- **WHEN** the source contains `repeat ... until x`
- **THEN** the condition-negation mutator emits a mutant for `repeat ... until not (x)`

### Requirement: Mutant metadata
The system SHALL record the original operator, replacement operator, and source location for every relational and logical operator mutant.

#### Scenario: Metadata is complete
- **WHEN** the relational or logical operator mutator generates a mutant
- **THEN** the mutant contains the original operator, the replacement operator, and the byte range of the operator token

### Requirement: Lua not-equal operator parsing
The system SHALL correctly recognize the `~=` operator token when parsing Lua source files.

#### Scenario: Not-equal operator is parsed
- **WHEN** the source contains `a ~= b`
- **THEN** the parser exposes the operator token as `~=` for the mutator to match
