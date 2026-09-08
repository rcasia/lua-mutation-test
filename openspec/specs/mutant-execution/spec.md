# mutant-execution Specification

## Purpose
TBD - created by archiving change run-mutants-with-timeout. Update Purpose after archive.
## Requirements
### Requirement: Run configured test command against a mutant
The system SHALL execute the configured test command in the context of a single mutant.

#### Scenario: Mutant run
- **WHEN** a mutant has been generated and applied to a temporary source file
- **THEN** the configured test command runs against that source file

### Requirement: Apply configurable timeout
The system SHALL enforce a configurable timeout for each mutant test run, defaulting to 30 seconds.

#### Scenario: Default timeout
- **WHEN** no timeout is configured
- **THEN** each mutant run is terminated after 30 seconds if still running

#### Scenario: Custom timeout
- **WHEN** the user configures a timeout of 60 seconds
- **THEN** each mutant run is terminated after 60 seconds if still running

### Requirement: Mark timed out mutants
The system SHALL classify any mutant whose test run exceeds the timeout as `timed_out`.

#### Scenario: Infinite loop mutant
- **WHEN** a mutant causes the test command to run past the timeout
- **THEN** the mutant result is recorded as `timed_out`

### Requirement: Run each mutant in a separate process
The system SHALL spawn each mutant test run as a separate OS process.

#### Scenario: Process isolation
- **WHEN** two mutants are executed sequentially
- **THEN** the second mutant run starts in a fresh process with no shared state from the first

### Requirement: Clean up temporary files
The system SHALL remove temporary files created for a mutant run after the run completes, regardless of outcome.

#### Scenario: Cleanup after timeout
- **WHEN** a mutant run times out
- **THEN** the temporary source file and any scratch files are deleted

### Requirement: Handle process spawning errors
The system SHALL gracefully handle errors when spawning the test command and record the mutant result as `errored`.

#### Scenario: Missing test command
- **WHEN** the configured test command cannot be found
- **THEN** the mutant result is recorded as `errored` and the tool continues with the next mutant

