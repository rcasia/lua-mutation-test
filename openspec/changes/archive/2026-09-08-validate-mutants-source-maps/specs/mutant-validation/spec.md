## ADDED Requirements

### Requirement: Syntactic validation of mutated source
The system SHALL re-parse each mutated source with the Lua parser to verify that it is syntactically valid.

#### Scenario: Valid mutant passes validation
- **WHEN** a generated mutant is syntactically valid Lua
- **THEN** the system SHALL mark it as valid and queue it for test execution

#### Scenario: Invalid mutant fails validation
- **WHEN** a generated mutant is not syntactically valid Lua
- **THEN** the system SHALL mark it as invalid

### Requirement: Invalid mutants are marked as error
The system SHALL mark invalid mutants as `error` and exclude them from test runs.

#### Scenario: Invalid mutant is skipped
- **WHEN** a mutant fails syntactic validation
- **THEN** the system SHALL record its result as `error` and SHALL NOT execute it against the test suite

### Requirement: Temporary file generation preserves relative paths
The system SHALL write valid mutated sources to temporary files while preserving the original relative path structure.

#### Scenario: Mutant in nested module
- **WHEN** a mutant originates from `src/util/math.lua`
- **THEN** the system SHALL write the mutated source to a temporary path matching `src/util/math.lua`

### Requirement: Preserve original line numbers
The system SHALL preserve original line numbers in mutated sources by avoiding edits that insert or remove lines.

#### Scenario: Mutation replaces an operator on the same line
- **WHEN** a mutation replaces an operator without adding or removing lines
- **THEN** the mutated source SHALL have the same number of lines and each line SHALL map to the same original line number

### Requirement: SourceMap for debug output
The system SHALL provide a `SourceMap` that maps mutated source locations back to original source locations for debug output.

#### Scenario: Map mutated line to original line
- **WHEN** the system generates a source map for a valid mutant
- **THEN** the source map SHALL map each line in the mutated file to the corresponding line in the original source file
