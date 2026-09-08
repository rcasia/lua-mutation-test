## ADDED Requirements

### Requirement: MutantResult enum represents all possible run outcomes
The system SHALL define a `MutantResult` enum with variants `killed`, `survived`, `timed_out`, and `error`, each carrying the stable mutant id and captured output snippets.

#### Scenario: Successful run with exit code 0
- **WHEN** a mutant test run exits with code 0
- **THEN** the interpreter returns `MutantResult::survived` for that mutant

#### Scenario: Successful run with non-zero exit code
- **WHEN** a mutant test run exits with a non-zero code
- **THEN** the interpreter returns `MutantResult::killed` for that mutant

### Requirement: Timeouts are classified as timed_out
The system SHALL map a mutant test run that exceeds the configured timeout to `MutantResult::timed_out`.

#### Scenario: Timeout occurs
- **WHEN** a mutant test run exceeds its timeout limit
- **THEN** the interpreter returns `MutantResult::timed_out`

### Requirement: Runner failures are classified as error
The system SHALL map invalid mutants and runner failures (missing executable, crash, or inability to run) to `MutantResult::error`.

#### Scenario: Runner executable is missing
- **WHEN** the configured runner cannot be invoked
- **THEN** the interpreter returns `MutantResult::error`

#### Scenario: Invalid mutant prevents execution
- **WHEN** a mutant is syntactically invalid and cannot be executed
- **THEN** the interpreter returns `MutantResult::error`

### Requirement: Output snippets are stored with every result
The system SHALL store bounded stdout and stderr snippets alongside every `MutantResult` for reporting and debugging.

#### Scenario: Run produces output
- **WHEN** a mutant test run produces stdout or stderr
- **THEN** the resulting `MutantResult` includes the captured snippets

### Requirement: Test failures are distinguished from runner failures
The system SHALL distinguish a test failure (non-zero exit code from a working runner) from a runner failure (runner could not execute).

#### Scenario: Tests fail cleanly
- **WHEN** the runner executes successfully and the tests fail
- **THEN** the interpreter returns `MutantResult::killed`

#### Scenario: Runner crashes
- **WHEN** the runner process crashes before returning a meaningful exit code
- **THEN** the interpreter returns `MutantResult::error`
