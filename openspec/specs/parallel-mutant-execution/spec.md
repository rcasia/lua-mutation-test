# parallel-mutant-execution Specification

## Purpose
TBD - created by archiving change parallelize-mutant-execution. Update Purpose after archive.
## Requirements
### Requirement: Concurrent mutant execution
The system SHALL execute generated mutants concurrently using a configurable number of workers.

#### Scenario: Default worker count
- **WHEN** the user runs the mutation test command without specifying a worker count
- **THEN** the system uses a number of workers equal to the available logical CPUs

#### Scenario: Custom worker count
- **WHEN** the user provides a `--workers <n>` option
- **THEN** the system executes mutants using exactly `n` workers

### Requirement: Isolated worker environments
The system SHALL ensure each concurrent worker operates in an isolated temporary directory.

#### Scenario: No directory collision
- **WHEN** two workers execute mutants simultaneously
- **THEN** each worker writes to a distinct temporary directory and neither sees the other's files

### Requirement: Progress reporting
The system SHALL report execution progress while mutants are running.

#### Scenario: Progress updates emitted
- **WHEN** mutants are being executed in parallel
- **THEN** the system periodically emits progress showing the number of completed mutants and the total number of mutants

### Requirement: Graceful shutdown
The system SHALL stop accepting new mutant work and complete in-flight work when interrupted by `SIGINT`.

#### Scenario: Ctrl-C during execution
- **WHEN** the user sends `Ctrl-C` while mutants are running
- **THEN** the system stops spawning new mutants, finishes the mutants already in flight, and exits cleanly

### Requirement: Deterministic results
The system SHALL produce identical mutation results regardless of the order in which mutants finish.

#### Scenario: Repeated runs produce identical reports
- **WHEN** the same mutation test is run multiple times on the same source and tests
- **THEN** the reported killed, survived, and timed-out status for each mutant is identical across runs

