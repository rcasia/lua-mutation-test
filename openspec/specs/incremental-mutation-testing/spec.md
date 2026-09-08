# incremental-mutation-testing Specification

## Purpose
TBD - created by archiving change incremental-mutation-testing-watch-mode. Update Purpose after archive.
## Requirements
### Requirement: Result cache
The system SHALL cache mutation results keyed by source file path and mutant hash.

#### Scenario: Cache hit
- **WHEN** a mutant with the same file path and hash was previously run successfully
- **THEN** the system reuses the cached result without executing the mutant again

#### Scenario: Cache miss
- **WHEN** no cached entry exists for a mutant
- **THEN** the system executes the mutant and stores the result

### Requirement: Change detection
The system SHALL detect changed source files since the last run using git or file content hashes.

#### Scenario: File modified
- **WHEN** a source file has changed since the previous run
- **THEN** the system re-runs mutants for that file

#### Scenario: File unchanged
- **WHEN** a source file has not changed since the previous run
- **THEN** the system skips mutants for that file and uses cached results

### Requirement: Cache invalidation on configuration change
The system SHALL invalidate cached results when the mutation testing configuration changes.

#### Scenario: Config updated
- **WHEN** the configuration file or CLI options change between runs
- **THEN** the system invalidates all cached results affected by the configuration change

### Requirement: Watch mode
The system SHALL provide a `watch` subcommand that re-runs mutation tests when source files change.

#### Scenario: File change triggers re-run
- **WHEN** the user runs the `watch` subcommand and a tracked source file is modified
- **THEN** the system re-runs only the affected mutants and reports updated results

#### Scenario: Watch mode debouncing
- **WHEN** multiple file changes occur in rapid succession
- **THEN** the system debounces the events and performs a single incremental re-run

### Requirement: Persistent incremental state
The system SHALL store incremental state in `.lua-mutation-test/cache/`.

#### Scenario: State persists across invocations
- **WHEN** a mutation test run completes
- **THEN** the system writes cache data to `.lua-mutation-test/cache/`

#### Scenario: State is read on next run
- **WHEN** the mutation test command starts
- **THEN** the system reads existing incremental state from `.lua-mutation-test/cache/`

