# arithmetic-operator-mutation Specification

## Purpose
TBD - created by archiving change implement-arithmetic-operator-mutation. Update Purpose after archive.
## Requirements
### Requirement: Arithmetic operator replacement set
The system SHALL generate mutants that replace arithmetic operators in Lua `binary_expression` nodes with the configured alternative operators.

#### Scenario: Plus operator is replaced
- **WHEN** the source contains a binary expression `a + b`
- **THEN** the arithmetic operator mutator emits mutants for `a - b`, `a * b`, and `a / b`

#### Scenario: Minus operator is replaced
- **WHEN** the source contains a binary expression `a - b`
- **THEN** the arithmetic operator mutator emits mutants for `a + b`, `a * b`, and `a / b`

#### Scenario: Multiply operator is replaced
- **WHEN** the source contains a binary expression `a * b`
- **THEN** the arithmetic operator mutator emits mutants for `a + b`, `a - b`, and `a / b`

#### Scenario: Divide operator is replaced
- **WHEN** the source contains a binary expression `a / b`
- **THEN** the arithmetic operator mutator emits mutants for `a + b`, `a - b`, and `a * b`

#### Scenario: Modulo operator is replaced
- **WHEN** the source contains a binary expression `a % b`
- **THEN** the arithmetic operator mutator emits mutants for `a * b` and `a / b`

#### Scenario: Floor division operator is replaced
- **WHEN** the source contains a binary expression `a // b`
- **THEN** the arithmetic operator mutator emits mutants for `a / b` and `a * b`

#### Scenario: Exponent operator is replaced
- **WHEN** the source contains a binary expression `a ^ b`
- **THEN** the arithmetic operator mutator emits mutants for `a * b` and `a / b`

### Requirement: Distinct mutant metadata
The system SHALL record the original operator, replacement operator, and source location for every arithmetic operator mutant.

#### Scenario: Mutant metadata is complete
- **WHEN** the arithmetic operator mutator generates a mutant for `a + b`
- **THEN** the mutant contains the original operator `+`, the replacement operator, and the byte range of the operator token

### Requirement: Mutator integration
The system SHALL expose `ArithmeticOperatorMutator` through the operator-mutator registry so the pipeline can discover it.

#### Scenario: Mutator is discoverable
- **WHEN** the mutation pipeline enumerates registered mutators
- **THEN** `ArithmeticOperatorMutator` is present and produces mutants for arithmetic expressions

