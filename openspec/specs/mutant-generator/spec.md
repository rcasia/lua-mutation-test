# mutant-generator Specification

## Purpose
TBD - created by archiving change generate-deduplicate-serialize-mutants. Update Purpose after archive.
## Requirements
### Requirement: Mutant struct contains required metadata
The system SHALL define a `Mutant` struct containing a stable id, file path, operator name, start and end byte positions, line and column, original text, and replacement text.

#### Scenario: Create a mutant from a candidate
- **WHEN** a mutation operator produces a candidate for a source location
- **THEN** the generator constructs a `Mutant` with all required metadata fields populated

### Requirement: Mutant ids are stable and content-based
The system SHALL compute each mutant id deterministically from the tuple `(file_path, start_byte, end_byte, replacement)`.

#### Scenario: Regenerate mutants for the same file
- **WHEN** the generator runs twice on the same parsed file with the same operators
- **THEN** identical mutants receive identical ids

### Requirement: MutantGenerator composes mutators
The system SHALL provide a `MutantGenerator` that accepts a list of mutators and produces mutants for a parsed Lua file.

#### Scenario: Generate mutants with multiple operators
- **WHEN** the generator is configured with multiple mutators and given a parsed file
- **THEN** it returns mutants produced by all configured mutators

### Requirement: Identical mutants are deduplicated
The system SHALL deduplicate generated mutants by the tuple `(file, location, replacement)` so that only one mutant exists per unique source transformation.

#### Scenario: Two operators produce the same replacement
- **WHEN** two mutators produce mutants with the same file, byte range, and replacement
- **THEN** the generator emits a single mutant for that transformation

### Requirement: Mutants serialize to and deserialize from JSON
The system SHALL support JSON serialization and deserialization of `Mutant` values.

#### Scenario: Round-trip a mutant through JSON
- **WHEN** a mutant is serialized to JSON and then deserialized
- **THEN** the deserialized mutant equals the original

### Requirement: CLI can list mutants without running tests
The system SHALL provide a CLI subcommand that lists generated mutants for a given file or project without executing tests.

#### Scenario: List mutants for a source file
- **WHEN** the user invokes the list-mutants subcommand on a Lua file
- **THEN** the system prints the generated mutants, one per line, in JSON format

