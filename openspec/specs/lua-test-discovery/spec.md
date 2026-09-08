# lua-test-discovery Specification

## Purpose
TBD - created by archiving change discover-lua-test-files. Update Purpose after archive.
## Requirements
### Requirement: Discover test files by default patterns
The system SHALL discover Lua test files matching the default glob patterns `*_spec.lua`, `*_test.lua`, and `test_*.lua`.

#### Scenario: Project with mixed test files
- **WHEN** the project contains `foo_spec.lua`, `bar_test.lua`, `test_baz.lua`, and `helper.lua`
- **THEN** the discovered test files are `foo_spec.lua`, `bar_test.lua`, and `test_baz.lua`

### Requirement: Configure test file glob patterns
The system SHALL allow users to override the default test file discovery patterns through configuration.

#### Scenario: Custom glob pattern
- **WHEN** the user configures the pattern `tests/**/*.lua`
- **THEN** discovery returns only files matching that pattern

### Requirement: Adapter for busted
The system SHALL provide an adapter that runs discovered test files with the `busted` framework.

#### Scenario: Run with busted
- **WHEN** the busted adapter is selected and tests are discovered
- **THEN** the adapter invokes `busted` with the discovered test files

### Requirement: Adapter for luaunit
The system SHALL provide an adapter that runs discovered test files with the `luaunit` framework.

#### Scenario: Run with luaunit
- **WHEN** the luaunit adapter is selected and tests are discovered
- **THEN** the adapter invokes `lua` with the luaunit entry logic for each test file

### Requirement: Generic shell command adapter
The system SHALL provide a generic adapter that runs a user-provided shell command against the discovered test files.

#### Scenario: Custom test command
- **WHEN** the user provides the command `lua run-tests.lua`
- **THEN** the generic adapter executes that command

### Requirement: Establish baseline test run
The system SHALL run the discovered tests against the original, unmutated source and record the baseline result.

#### Scenario: Baseline passes
- **WHEN** the baseline test command runs against the original source
- **THEN** the baseline is marked as passing and made available to the mutant runner

#### Scenario: Baseline fails
- **WHEN** the baseline test command fails against the original source
- **THEN** mutation testing is aborted and the failure is reported

